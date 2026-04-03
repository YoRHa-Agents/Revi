use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    error::AppError,
    models::{ReviewItem, ReviewItemDetail, UpdateTypeRequest},
    router::AppState,
};

pub async fn list_reviews(State(s): State<AppState>) -> Result<Json<Vec<ReviewItem>>, AppError> {
    let overrides = s.metadata.load().map_err(AppError::Internal)?;
    let scanner = s.scanner.read().unwrap();
    let mut items = scanner.scan(&overrides);
    for item in &mut items {
        item.open_count = s.comments.open_count(&item.id).unwrap_or(0);
        item.resolved_count = s.comments.resolved_count(&item.id).unwrap_or(0);
    }
    Ok(Json(items))
}

pub async fn get_review(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<ReviewItemDetail>, AppError> {
    let overrides = s.metadata.load().map_err(AppError::Internal)?;
    let scanner = s.scanner.read().unwrap();
    let mut detail = scanner
        .get_detail(&item_id, "/workspace", &overrides)
        .ok_or(AppError::NotFound)?;
    detail.open_count = s.comments.open_count(&item_id).unwrap_or(0);
    detail.resolved_count = s.comments.resolved_count(&item_id).unwrap_or(0);
    Ok(Json(detail))
}

pub async fn update_review_type(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
    Json(body): Json<UpdateTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let valid = ["plan", "design", "prototype"];
    if !valid.contains(&body.item_type.as_str()) {
        return Err(AppError::BadRequest(format!(
            "invalid type '{}'; must be one of plan, design, prototype",
            body.item_type
        )));
    }
    {
        let overrides = s.metadata.load().map_err(AppError::Internal)?;
        let scanner = s.scanner.read().unwrap();
        if scanner.get_item(&item_id, &overrides).is_none() {
            return Err(AppError::NotFound);
        }
    }
    s.metadata
        .set_type(&item_id, &body.item_type)
        .map_err(AppError::Internal)?;

    let overrides = s.metadata.load().map_err(AppError::Internal)?;
    let scanner = s.scanner.read().unwrap();
    let mut item = scanner
        .get_item(&item_id, &overrides)
        .ok_or(AppError::NotFound)?;
    item.open_count = s.comments.open_count(&item_id).unwrap_or(0);
    item.resolved_count = s.comments.resolved_count(&item_id).unwrap_or(0);
    Ok((StatusCode::OK, Json(item)))
}
