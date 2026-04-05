use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;

use crate::{
    error::AppError,
    models::{ExportComment, ExportItem, ExportResponse, ExportSummary},
    router::AppState,
};

pub async fn export_for_agent(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<ExportResponse>, AppError> {
    let overrides = s.metadata.load().map_err(AppError::Internal)?;
    let item = {
        let scanner = s.scanner.read().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;
        scanner
            .get_item(&item_id, &overrides)
            .ok_or(AppError::NotFound)?
    };

    let all_comments = s.comments.list(&item_id).map_err(AppError::Internal)?;
    let open_count = all_comments.iter().filter(|c| c.status == "open").count() as i64;
    let resolved_count =
        all_comments.iter().filter(|c| c.status == "resolved").count() as i64;
    let total = open_count + resolved_count;

    let disk_comments = s.comments.list_disk(&item_id).map_err(AppError::Internal)?;
    let open_comments: Vec<ExportComment> = disk_comments
        .into_iter()
        .filter(|c| c.status == "open")
        .map(ExportComment::from)
        .collect();

    let resp = ExportResponse {
        schema_version: "1.0".to_owned(),
        item: ExportItem {
            id: item.id,
            item_type: item.item_type,
            title: item.title,
        },
        summary: ExportSummary { total, open: open_count, resolved: resolved_count },
        open_comments,
        exported_at: Utc::now(),
    };

    Ok(Json(resp))
}
