use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    error::AppError,
    models::{AddCommentRequest, Comment},
    router::AppState,
};

pub async fn list_comments(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<Vec<Comment>>, AppError> {
    let comments = s.comments.list(&item_id).map_err(AppError::Internal)?;
    Ok(Json(comments))
}

pub async fn add_comment(
    State(s): State<AppState>,
    Path(item_id): Path<String>,
    Json(body): Json<AddCommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    {
        let overrides = s.metadata.load().map_err(AppError::Internal)?;
        let scanner = s.scanner.read().unwrap();
        if scanner.get_item(&item_id, &overrides).is_none() {
            return Err(AppError::NotFound);
        }
    }
    let comment = s
        .comments
        .add(&item_id, body.author, body.content, body.reference)
        .map_err(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(comment)))
}

/// Handles PATCH /api/comments/*item_id
/// If tail ends with "/{uuid}/resolve" → resolve that comment.
/// Otherwise 404.
pub async fn patch_handler(
    State(s): State<AppState>,
    Path(tail): Path<String>,
) -> Result<Json<Comment>, AppError> {
    if let Some(rest) = tail.strip_suffix("/resolve") {
        let (item_id, comment_id) = rest
            .rsplit_once('/')
            .ok_or_else(|| AppError::BadRequest("invalid path".to_owned()))?;
        let comment = s
            .comments
            .resolve(item_id, comment_id)
            .map_err(AppError::Internal)?
            .ok_or(AppError::NotFound)?;
        return Ok(Json(comment));
    }
    Err(AppError::NotFound)
}
