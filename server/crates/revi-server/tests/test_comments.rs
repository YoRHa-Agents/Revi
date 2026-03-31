mod common;
use common::{body_json, body_status_json, AppFixture};

async fn add_comment(fx: &AppFixture, item_id: &str) -> serde_json::Value {
    let resp = fx
        .post_json(
            &format!("/api/comments/{}", item_id),
            serde_json::json!({"author": "Alice", "content": "Looks good"}),
        )
        .await;
    body_json(resp).await
}

#[tokio::test]
async fn test_add_comment_returns_201() {
    let fx = AppFixture::new();
    let resp = fx
        .post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({"author": "Alice", "content": "Good plan"}),
        )
        .await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn test_add_comment_body_fields() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    assert!(comment.get("id").is_some());
    assert_eq!(comment["author"], "Alice");
    assert_eq!(comment["content"], "Looks good");
    assert_eq!(comment["status"], "open");
    assert!(comment["createdAt"].is_string());
    assert!(comment["resolvedAt"].is_null());
    assert_eq!(comment["itemId"], "plans/sprint-1-design");
}

#[tokio::test]
async fn test_add_comment_id_is_uuid_format() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    let id = comment["id"].as_str().unwrap();
    // UUID v4: 8-4-4-4-12 hex digits
    assert_eq!(id.len(), 36);
    assert_eq!(id.chars().filter(|&c| c == '-').count(), 4);
}

#[tokio::test]
async fn test_add_comment_camel_case_fields() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    assert!(comment.get("createdAt").is_some(), "should have createdAt");
    assert!(comment.get("created_at").is_none(), "should not have created_at");
    assert!(comment.get("itemId").is_some(), "should have itemId");
    assert!(comment.get("item_id").is_none(), "should not have item_id");
}

#[tokio::test]
async fn test_add_comment_with_reference() {
    let fx = AppFixture::new();
    let resp = fx
        .post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({
                "author": "Bob",
                "content": "Section looks thin",
                "reference": {"type": "section", "value": "## Overview"}
            }),
        )
        .await;
    let comment = body_json(resp).await;
    assert_eq!(comment["reference"]["type"], "section");
    assert_eq!(comment["reference"]["value"], "## Overview");
}

#[tokio::test]
async fn test_list_comments_empty() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/comments/plans/sprint-1-design").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_comments_returns_added() {
    let fx = AppFixture::new();
    add_comment(&fx, "plans/sprint-1-design").await;
    add_comment(&fx, "plans/sprint-1-design").await;
    let resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let body = body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_add_comment_not_found_item() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.post_json(
            "/api/comments/plans/does-not-exist",
            serde_json::json!({"author": "Alice", "content": "Hi"}),
        )
        .await,
    )
    .await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_add_comment_missing_required_fields() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({"content": "Missing author"}),
        )
        .await,
    )
    .await;
    assert_eq!(status, 422);
}

#[tokio::test]
async fn test_resolve_comment() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    let id = comment["id"].as_str().unwrap();

    let resp = fx
        .patch_empty(&format!(
            "/api/comments/plans/sprint-1-design/{}/resolve",
            id
        ))
        .await;
    assert_eq!(resp.status(), 200);
    let resolved = body_json(resp).await;
    assert_eq!(resolved["status"], "resolved");
    assert!(resolved["resolvedAt"].is_string());
}

#[tokio::test]
async fn test_resolve_comment_idempotent() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    let id = comment["id"].as_str().unwrap();
    let path = format!("/api/comments/plans/sprint-1-design/{}/resolve", id);

    fx.patch_empty(&path).await;
    let first_resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let first_comments = body_json(first_resp).await;
    let first_resolved_at = first_comments[0]["resolvedAt"].clone();

    // Second resolve — resolvedAt should not change
    fx.patch_empty(&path).await;
    let second_resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let second_comments = body_json(second_resp).await;
    assert_eq!(second_comments[0]["resolvedAt"], first_resolved_at);
}

#[tokio::test]
async fn test_resolve_comment_not_found() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.patch_empty(
            "/api/comments/plans/sprint-1-design/00000000-0000-0000-0000-000000000000/resolve",
        )
        .await,
    )
    .await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_list_comments_shows_resolved() {
    let fx = AppFixture::new();
    let comment = add_comment(&fx, "plans/sprint-1-design").await;
    let id = comment["id"].as_str().unwrap();
    fx.patch_empty(&format!(
        "/api/comments/plans/sprint-1-design/{}/resolve",
        id
    ))
    .await;

    let resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let comments = body_json(resp).await;
    assert_eq!(comments[0]["status"], "resolved");
}
