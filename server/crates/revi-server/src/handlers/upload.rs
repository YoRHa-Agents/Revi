use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    error::AppError,
    models::UploadResponse,
    router::AppState,
};

const MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024; // 50 MB

const DESIGN_EXTS: &[&str] = &["png", "jpg", "jpeg", "svg", "webp", "pdf"];
const PLAN_EXTS: &[&str] = &["md"];
const PROTO_EXTS: &[&str] = &["html", "htm"];

fn ext_to_subfolder(ext: &str) -> Option<(&'static str, &'static str)> {
    let ext = ext.to_lowercase();
    if PLAN_EXTS.contains(&ext.as_str()) {
        Some(("plans", "plan"))
    } else if PROTO_EXTS.contains(&ext.as_str()) {
        Some(("prototypes", "prototype"))
    } else if DESIGN_EXTS.contains(&ext.as_str()) {
        Some(("designs", "design"))
    } else {
        None
    }
}

fn type_to_subfolder(item_type: &str) -> Option<&'static str> {
    match item_type {
        "plan" => Some("plans"),
        "prototype" => Some("prototypes"),
        "design" => Some("designs"),
        _ => None,
    }
}

pub async fn upload_file(
    State(s): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut filename: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut type_override: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        match field.name() {
            Some("file") => {
                filename = field.file_name().map(str::to_owned);
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError::BadRequest(e.to_string()))?
                        .to_vec(),
                );
            }
            Some("type") => {
                type_override = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::BadRequest(e.to_string()))?,
                );
            }
            _ => {}
        }
    }

    let raw_filename =
        filename.ok_or_else(|| AppError::BadRequest("missing file field".to_owned()))?;
    let file_bytes =
        file_bytes.ok_or_else(|| AppError::BadRequest("missing file data".to_owned()))?;

    if file_bytes.len() > MAX_UPLOAD_SIZE {
        return Err(AppError::BadRequest(format!(
            "file too large ({} bytes, max {} bytes)",
            file_bytes.len(),
            MAX_UPLOAD_SIZE
        )));
    }

    let filename = std::path::Path::new(&raw_filename)
        .file_name()
        .and_then(|f| f.to_str())
        .map(str::to_owned)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::BadRequest("invalid filename".to_owned()))?;

    let ext = std::path::Path::new(&filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let subfolder = if let Some(ref t) = type_override {
        type_to_subfolder(t)
            .ok_or_else(|| AppError::BadRequest(format!("invalid type '{}'", t)))?
    } else {
        ext_to_subfolder(ext)
            .map(|(sf, _)| sf)
            .ok_or_else(|| AppError::BadRequest(format!("unsupported extension '{}'", ext)))?
    };

    let workspace_path = {
        let cfg = s.config.read().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;
        cfg.effective_workspace()
    };

    let dest_dir = workspace_path.join(subfolder);
    std::fs::create_dir_all(&dest_dir)
        .map_err(|e| AppError::Internal(e.into()))?;

    let dest = dest_dir.join(&filename);
    let tmp = dest.with_extension("upload.tmp");
    std::fs::write(&tmp, &file_bytes).map_err(|e| AppError::Internal(e.into()))?;
    std::fs::rename(&tmp, &dest).map_err(|e| AppError::Internal(e.into()))?;

    let stem = std::path::Path::new(&filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&filename);

    let item_id = format!("{}/{}", subfolder, stem);
    let url = format!("/workspace/{}/{}", subfolder, filename);

    Ok((
        StatusCode::CREATED,
        Json(UploadResponse { item_id, filename, url }),
    ))
}
