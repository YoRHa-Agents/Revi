use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    error::AppError,
    models::ArchiveBatch,
    router::AppState,
};

pub async fn archive_resolved(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<ArchiveBatch>, AppError> {
    let resolved = s
        .comments
        .remove_resolved(&item_id)
        .map_err(AppError::Internal)?;

    if resolved.is_empty() {
        return Err(AppError::BadRequest(
            "no resolved comments to archive".to_owned(),
        ));
    }

    let batch = s
        .archive
        .add_batch(&item_id, resolved)
        .map_err(AppError::Internal)?;

    Ok(Json(batch))
}

pub async fn list_archive(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<Vec<ArchiveBatch>>, AppError> {
    let batches = s.archive.list(&item_id).map_err(AppError::Internal)?;
    Ok(Json(batches))
}
