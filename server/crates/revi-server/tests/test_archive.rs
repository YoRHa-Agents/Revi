mod common;
use common::{body_json, body_status_json, AppFixture};

async fn add_and_resolve(fx: &AppFixture, item_id: &str) -> String {
    let resp = fx
        .post_json(
            &format!("/api/comments/{}", item_id),
            serde_json::json!({"author": "Alice", "content": "Done"}),
        )
        .await;
    let comment = body_json(resp).await;
    let id = comment["id"].as_str().unwrap().to_owned();
    fx.patch_empty(&format!("/api/comments/{}/{}/resolve", item_id, id))
        .await;
    id
}

#[tokio::test]
async fn test_archive_no_resolved_returns_400() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.post_empty("/api/archive/plans/sprint-1-design").await,
    )
    .await;
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_archive_returns_batch() {
    let fx = AppFixture::new();
    add_and_resolve(&fx, "plans/sprint-1-design").await;

    let resp = fx.post_empty("/api/archive/plans/sprint-1-design").await;
    assert_eq!(resp.status(), 200);
    let batch = body_json(resp).await;
    assert!(batch["archivedAt"].is_string());
    assert_eq!(batch["comments"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_archive_removes_resolved_from_live() {
    let fx = AppFixture::new();
    add_and_resolve(&fx, "plans/sprint-1-design").await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    let resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let comments = body_json(resp).await;
    assert_eq!(comments.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_archive_keeps_open_comments() {
    let fx = AppFixture::new();
    // Add one open, one resolved
    fx.post_json(
        "/api/comments/plans/sprint-1-design",
        serde_json::json!({"author": "Bob", "content": "Still open"}),
    )
    .await;
    add_and_resolve(&fx, "plans/sprint-1-design").await;

    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    let resp = fx.get("/api/comments/plans/sprint-1-design").await;
    let comments = body_json(resp).await;
    assert_eq!(comments.as_array().unwrap().len(), 1);
    assert_eq!(comments[0]["status"], "open");
}

#[tokio::test]
async fn test_list_archive_empty() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/archive/plans/sprint-1-design").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_archive_after_archiving() {
    let fx = AppFixture::new();
    add_and_resolve(&fx, "plans/sprint-1-design").await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    let resp = fx.get("/api/archive/plans/sprint-1-design").await;
    let batches = body_json(resp).await;
    assert_eq!(batches.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_archive_batches_newest_first() {
    let fx = AppFixture::new();

    // First batch
    add_and_resolve(&fx, "plans/sprint-1-design").await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    // Second batch
    add_and_resolve(&fx, "plans/sprint-1-design").await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    let resp = fx.get("/api/archive/plans/sprint-1-design").await;
    let batches = body_json(resp).await;
    assert_eq!(batches.as_array().unwrap().len(), 2);

    let t0 = batches[0]["archivedAt"].as_str().unwrap();
    let t1 = batches[1]["archivedAt"].as_str().unwrap();
    // Newest first — t0 >= t1
    assert!(t0 >= t1);
}

#[tokio::test]
async fn test_double_archive_returns_400() {
    let fx = AppFixture::new();
    add_and_resolve(&fx, "plans/sprint-1-design").await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    // Nothing left to archive
    let (status, _) = body_status_json(
        fx.post_empty("/api/archive/plans/sprint-1-design").await,
    )
    .await;
    assert_eq!(status, 400);
}
