mod common;
use common::{body_json, body_status_json, AppFixture};

#[tokio::test]
async fn test_list_reviews_returns_three_items() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn test_list_reviews_camel_case_fields() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews").await;
    let items = body_json(resp).await;
    let item = &items[0];
    assert!(item.get("openCount").is_some(), "missing openCount");
    assert!(item.get("resolvedCount").is_some(), "missing resolvedCount");
    assert!(item.get("updatedAt").is_some(), "missing updatedAt");
    assert!(item.get("open_count").is_none(), "should not have open_count");
}

#[tokio::test]
async fn test_list_reviews_has_required_fields() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews").await;
    let items = body_json(resp).await;
    for item in items.as_array().unwrap() {
        assert!(item.get("id").is_some());
        assert!(item.get("type").is_some());
        assert!(item.get("title").is_some());
    }
}

#[tokio::test]
async fn test_list_reviews_plan_title_extracted_from_h1() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews").await;
    let items = body_json(resp).await;
    let plan = items
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"] == "plans/sprint-1-design")
        .unwrap()
        .clone();
    assert_eq!(plan["title"], "Sprint 1 — System Design");
}

#[tokio::test]
async fn test_get_review_detail_plan() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews/plans/sprint-1-design").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["id"], "plans/sprint-1-design");
    assert!(body["contentText"].is_string(), "plan should have contentText");
    assert!(body["contentUrl"].is_string(), "plan should have contentUrl");
}

#[tokio::test]
async fn test_get_review_detail_design_has_no_content_text() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews/designs/ui-mockup-v1").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert!(body["contentText"].is_null(), "design should not have contentText");
    assert!(body["contentUrl"].is_string());
}

#[tokio::test]
async fn test_get_review_not_found() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/reviews/plans/does-not-exist").await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_patch_review_type() {
    let fx = AppFixture::new();
    let resp = fx
        .patch_json(
            "/api/reviews/plans/sprint-1-design",
            serde_json::json!({"type": "design"}),
        )
        .await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["type"], "design");
}

#[tokio::test]
async fn test_patch_review_type_invalid() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.patch_json(
            "/api/reviews/plans/sprint-1-design",
            serde_json::json!({"type": "bogus"}),
        )
        .await,
    )
    .await;
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_patch_review_type_not_found() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(
        fx.patch_json(
            "/api/reviews/plans/no-such-item",
            serde_json::json!({"type": "design"}),
        )
        .await,
    )
    .await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_open_and_resolved_counts_populated() {
    let fx = AppFixture::new();
    // Add a comment then check counts
    fx.post_json(
        "/api/comments/plans/sprint-1-design",
        serde_json::json!({"author": "Alice", "content": "Test comment"}),
    )
    .await;

    let resp = fx.get("/api/reviews").await;
    let items = body_json(resp).await;
    let plan = items
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"] == "plans/sprint-1-design")
        .unwrap()
        .clone();
    assert_eq!(plan["openCount"], 1);
    assert_eq!(plan["resolvedCount"], 0);
}
