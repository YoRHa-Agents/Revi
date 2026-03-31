mod common;
use axum::response::Response;
use common::{body_json, body_status_json, AppFixture};

fn multipart_body(boundary: &str, filename: &str, content: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(
        format!(
            "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{f}\"\r\nContent-Type: application/octet-stream\r\n\r\n",
            b = boundary, f = filename
        )
        .as_bytes(),
    );
    body.extend_from_slice(content);
    body.extend_from_slice(format!("\r\n--{b}--\r\n", b = boundary).as_bytes());
    body
}

fn multipart_body_with_type(boundary: &str, filename: &str, content: &[u8], item_type: &str) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(
        format!(
            "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{f}\"\r\nContent-Type: application/octet-stream\r\n\r\n",
            b = boundary, f = filename
        )
        .as_bytes(),
    );
    body.extend_from_slice(content);
    body.extend_from_slice(
        format!(
            "\r\n--{b}\r\nContent-Disposition: form-data; name=\"type\"\r\n\r\n{t}\r\n--{b}--\r\n",
            b = boundary, t = item_type
        )
        .as_bytes(),
    );
    body
}

async fn upload(fx: &AppFixture, filename: &str, content: &[u8]) -> Response {
    let boundary = "uploadboundary42";
    fx.post_multipart(
        "/api/upload",
        multipart_body(boundary, filename, content),
        boundary,
    )
    .await
}

async fn upload_with_type(fx: &AppFixture, filename: &str, content: &[u8], item_type: &str) -> Response {
    let boundary = "uploadboundary43";
    fx.post_multipart(
        "/api/upload",
        multipart_body_with_type(boundary, filename, content, item_type),
        boundary,
    )
    .await
}

#[tokio::test]
async fn test_upload_markdown_returns_201() {
    let fx = AppFixture::new();
    let resp = upload(&fx, "new-plan.md", b"# New Plan\n").await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn test_upload_markdown_item_id() {
    let fx = AppFixture::new();
    let body = body_json(upload(&fx, "my-feature.md", b"# Feature\n").await).await;
    assert_eq!(body["itemId"], "plans/my-feature");
    assert_eq!(body["filename"], "my-feature.md");
    assert!(body["url"].as_str().unwrap().contains("/workspace/plans/"));
}

#[tokio::test]
async fn test_upload_html_goes_to_prototypes() {
    let fx = AppFixture::new();
    let body =
        body_json(upload(&fx, "review-v2.html", b"<html><body></body></html>").await).await;
    assert!(body["itemId"].as_str().unwrap().starts_with("prototypes/"));
}

#[tokio::test]
async fn test_upload_png_goes_to_designs() {
    let fx = AppFixture::new();
    let body = body_json(upload(&fx, "mockup.png", b"\x89PNG\r\n\x1a\n").await).await;
    assert!(body["itemId"].as_str().unwrap().starts_with("designs/"));
}

#[tokio::test]
async fn test_upload_type_override() {
    let fx = AppFixture::new();
    let body =
        body_json(upload_with_type(&fx, "weird.md", b"content", "design").await).await;
    assert!(body["itemId"].as_str().unwrap().starts_with("designs/"));
}

#[tokio::test]
async fn test_upload_unsupported_extension_returns_400() {
    let fx = AppFixture::new();
    let (status, _) = body_status_json(upload(&fx, "file.xyz", b"data").await).await;
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_uploaded_item_discoverable() {
    let fx = AppFixture::new();
    upload(&fx, "fresh-plan.md", b"# Fresh Plan\n").await;

    let reviews = body_json(fx.get("/api/reviews").await).await;
    let found = reviews
        .as_array()
        .unwrap()
        .iter()
        .any(|i| i["id"] == "plans/fresh-plan");
    assert!(found);
}
