/// End-to-end workflow tests matching the Python test_workflows.py
mod common;
use common::{body_json, AppFixture};

/// Full human review loop: add → resolve → archive
#[tokio::test]
async fn test_human_review_loop() {
    let fx = AppFixture::new();
    let item = "plans/sprint-1-design";

    let c1 = body_json(
        fx.post_json(
            &format!("/api/comments/{}", item),
            serde_json::json!({"author": "Alice", "content": "Needs more detail"}),
        )
        .await,
    )
    .await;
    fx.post_json(
        &format!("/api/comments/{}", item),
        serde_json::json!({"author": "Bob", "content": "Add diagrams"}),
    )
    .await;

    let id = c1["id"].as_str().unwrap();
    fx.patch_empty(&format!("/api/comments/{}/{}/resolve", item, id))
        .await;

    let review = body_json(fx.get(&format!("/api/reviews/{}", item)).await).await;
    assert_eq!(review["openCount"], 1);
    assert_eq!(review["resolvedCount"], 1);

    let batch = body_json(fx.post_empty(&format!("/api/archive/{}", item)).await).await;
    assert_eq!(batch["comments"].as_array().unwrap().len(), 1);

    let comments = body_json(fx.get(&format!("/api/comments/{}", item)).await).await;
    assert_eq!(comments.as_array().unwrap().len(), 1);
    assert_eq!(comments[0]["status"], "open");
}

/// Agent consume flow: poll export, add structured comment
#[tokio::test]
async fn test_agent_consume_flow() {
    let fx = AppFixture::new();
    let item = "plans/sprint-1-design";

    let export = body_json(fx.get(&format!("/api/export/{}", item)).await).await;
    assert_eq!(export["schemaVersion"], "1.0");
    assert_eq!(export["summary"]["open"], 0);

    let resp = fx
        .post_json(
            &format!("/api/comments/{}", item),
            serde_json::json!({
                "author": "ReviewAgent",
                "content": "The storage section needs SQLite rationale",
                "reference": {"type": "section", "value": "## Storage Layer"}
            }),
        )
        .await;
    assert_eq!(resp.status(), 201);

    let export = body_json(fx.get(&format!("/api/export/{}", item)).await).await;
    assert_eq!(export["summary"]["open"], 1);
    assert_eq!(export["openComments"][0]["reference"]["value"], "## Storage Layer");
}

/// Comments for different items don't interfere
#[tokio::test]
async fn test_multi_item_independence() {
    let fx = AppFixture::new();

    fx.post_json(
        "/api/comments/plans/sprint-1-design",
        serde_json::json!({"author": "Alice", "content": "Plan comment"}),
    )
    .await;
    fx.post_json(
        "/api/comments/designs/ui-mockup-v1",
        serde_json::json!({"author": "Bob", "content": "Design comment"}),
    )
    .await;

    let plan_comments =
        body_json(fx.get("/api/comments/plans/sprint-1-design").await).await;
    let design_comments =
        body_json(fx.get("/api/comments/designs/ui-mockup-v1").await).await;

    assert_eq!(plan_comments.as_array().unwrap().len(), 1);
    assert_eq!(design_comments.as_array().unwrap().len(), 1);
    assert_eq!(plan_comments[0]["content"], "Plan comment");
    assert_eq!(design_comments[0]["content"], "Design comment");
}

/// Double-resolve is idempotent; double-archive returns 400
#[tokio::test]
async fn test_edge_cases() {
    let fx = AppFixture::new();
    let item = "plans/sprint-1-design";

    let c = body_json(
        fx.post_json(
            &format!("/api/comments/{}", item),
            serde_json::json!({"author": "Alice", "content": "hi"}),
        )
        .await,
    )
    .await;
    let id = c["id"].as_str().unwrap();
    let resolve_path = format!("/api/comments/{}/{}/resolve", item, id);

    let r1 = body_json(fx.patch_empty(&resolve_path).await).await;
    let resolved_at_1 = r1["resolvedAt"].clone();

    let r2 = body_json(fx.patch_empty(&resolve_path).await).await;
    assert_eq!(r2["resolvedAt"], resolved_at_1);

    fx.post_empty(&format!("/api/archive/{}", item)).await;

    let resp = fx.post_empty(&format!("/api/archive/{}", item)).await;
    assert_eq!(resp.status(), 400);
}

/// Upload → discover → comment flow
#[tokio::test]
async fn test_upload_discover_comment() {
    let fx = AppFixture::new();
    let boundary = "testboundary1234567890";
    let content = b"# My Uploaded Plan\n\n## Section\n\nContent here.\n";
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(
        format!(
            "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"my-uploaded-plan.md\"\r\nContent-Type: text/markdown\r\n\r\n",
            b = boundary
        )
        .as_bytes(),
    );
    body.extend_from_slice(content);
    body.extend_from_slice(format!("\r\n--{b}--\r\n", b = boundary).as_bytes());

    let resp = fx.post_multipart("/api/upload", body, boundary).await;
    assert_eq!(resp.status(), 201);
    let upload = body_json(resp).await;
    let item_id = upload["itemId"].as_str().unwrap().to_owned();

    let reviews = body_json(fx.get("/api/reviews").await).await;
    let found = reviews
        .as_array()
        .unwrap()
        .iter()
        .any(|i| i["id"] == item_id);
    assert!(found, "uploaded item not found in reviews");

    let resp = fx
        .post_json(
            &format!("/api/comments/{}", item_id),
            serde_json::json!({"author": "Reviewer", "content": "Nice plan"}),
        )
        .await;
    assert_eq!(resp.status(), 201);
}
