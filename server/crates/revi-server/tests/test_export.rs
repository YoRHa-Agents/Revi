mod common;
use common::{body_json, AppFixture};

#[tokio::test]
async fn test_export_schema_version() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/export/plans/sprint-1-design").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["schemaVersion"], "1.0");
}

#[tokio::test]
async fn test_export_item_fields() {
    let fx = AppFixture::new();
    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let item = &body["item"];
    assert_eq!(item["id"], "plans/sprint-1-design");
    assert_eq!(item["type"], "plan");
    assert!(item["title"].is_string());
}

#[tokio::test]
async fn test_export_summary_zero_initially() {
    let fx = AppFixture::new();
    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let summary = &body["summary"];
    assert_eq!(summary["total"], 0);
    assert_eq!(summary["open"], 0);
    assert_eq!(summary["resolved"], 0);
}

#[tokio::test]
async fn test_export_summary_counts_correctly() {
    let fx = AppFixture::new();

    // Add 3 comments
    for i in 0..3 {
        let resp = fx
            .post_json(
                "/api/comments/plans/sprint-1-design",
                serde_json::json!({"author": "Agent", "content": format!("Comment {}", i)}),
            )
            .await;
        let c = body_json(resp).await;
        // Resolve first 2
        if i < 2 {
            let id = c["id"].as_str().unwrap();
            fx.patch_empty(&format!(
                "/api/comments/plans/sprint-1-design/{}/resolve",
                id
            ))
            .await;
        }
    }

    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let summary = &body["summary"];
    assert_eq!(summary["total"], 3);
    assert_eq!(summary["open"], 1);
    assert_eq!(summary["resolved"], 2);
}

#[tokio::test]
async fn test_export_open_comments_only() {
    let fx = AppFixture::new();

    let resp = fx
        .post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({"author": "Alice", "content": "Keep this open"}),
        )
        .await;
    let c1 = body_json(resp).await;

    let resp = fx
        .post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({"author": "Bob", "content": "Close this"}),
        )
        .await;
    let c2 = body_json(resp).await;
    let c2_id = c2["id"].as_str().unwrap();
    fx.patch_empty(&format!(
        "/api/comments/plans/sprint-1-design/{}/resolve",
        c2_id
    ))
    .await;

    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let open = body["openComments"].as_array().unwrap();
    assert_eq!(open.len(), 1);
    assert_eq!(open[0]["id"], c1["id"]);
}

#[tokio::test]
async fn test_export_open_comments_fields() {
    let fx = AppFixture::new();
    fx.post_json(
        "/api/comments/plans/sprint-1-design",
        serde_json::json!({
            "author": "Agent",
            "content": "Needs work",
            "reference": {"type": "section", "value": "## Overview"}
        }),
    )
    .await;

    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let comment = &body["openComments"][0];
    assert!(comment.get("id").is_some());
    assert_eq!(comment["author"], "Agent");
    assert_eq!(comment["content"], "Needs work");
    assert!(comment["createdAt"].is_string());
    assert_eq!(comment["reference"]["type"], "section");
    // Exported comments should NOT have status/resolvedAt/itemId
    assert!(comment.get("status").is_none());
    assert!(comment.get("itemId").is_none());
}

#[tokio::test]
async fn test_export_not_found() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/export/plans/no-such-item").await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_export_has_exported_at() {
    let fx = AppFixture::new();
    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    assert!(body["exportedAt"].is_string());
}

#[tokio::test]
async fn test_export_archived_not_in_counts() {
    let fx = AppFixture::new();

    // Add and resolve a comment, then archive it
    let resp = fx
        .post_json(
            "/api/comments/plans/sprint-1-design",
            serde_json::json!({"author": "Alice", "content": "Archived"}),
        )
        .await;
    let c = body_json(resp).await;
    let id = c["id"].as_str().unwrap();
    fx.patch_empty(&format!(
        "/api/comments/plans/sprint-1-design/{}/resolve",
        id
    ))
    .await;
    fx.post_empty("/api/archive/plans/sprint-1-design").await;

    // After archiving, counts should be 0
    let body = body_json(fx.get("/api/export/plans/sprint-1-design").await).await;
    let summary = &body["summary"];
    assert_eq!(summary["total"], 0);
    assert_eq!(summary["open"], 0);
    assert_eq!(summary["resolved"], 0);
}
