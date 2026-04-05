mod common;
use common::AppFixture;

#[tokio::test]
async fn test_workspace_serves_valid_file() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/sprint-1-design.md").await;
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn test_workspace_nonexistent_file_returns_404() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/does-not-exist.md").await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_workspace_traversal_etc_passwd() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/../../etc/passwd").await;
    let status = resp.status();
    assert!(
        status.is_client_error(),
        "traversal to ../../etc/passwd should be rejected, got {status}"
    );
}

#[tokio::test]
async fn test_workspace_deep_traversal_etc_passwd() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/../../../etc/passwd").await;
    let status = resp.status();
    assert!(
        status.is_client_error(),
        "deep traversal to /etc/passwd should be rejected, got {status}"
    );
}

#[tokio::test]
async fn test_workspace_traversal_cargo_toml() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/../../../Cargo.toml").await;
    let status = resp.status();
    assert!(
        status.is_client_error(),
        "traversal to Cargo.toml should be rejected, got {status}"
    );
}

#[tokio::test]
async fn test_workspace_url_encoded_traversal() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/plans/%2e%2e/%2e%2e/etc/passwd").await;
    let status = resp.status();
    assert!(
        status.is_client_error(),
        "URL-encoded traversal should be rejected, got {status}"
    );
}

#[tokio::test]
async fn test_workspace_double_dot_only() {
    let fx = AppFixture::new();
    let resp = fx.get("/workspace/..").await;
    let status = resp.status();
    assert!(
        status.is_client_error(),
        "bare .. traversal should be rejected, got {status}"
    );
}
