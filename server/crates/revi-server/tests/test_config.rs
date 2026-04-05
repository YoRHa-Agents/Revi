mod common;
use axum::body::Body;
use axum::http::{Method, Request};
use common::{body_json, body_status_json, AppFixture};

#[tokio::test]
async fn test_get_config_returns_expected_fields() {
    let fx = AppFixture::new();
    let resp = fx.get("/api/config").await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert!(body.get("workspaceConfigured").is_some());
    assert!(body.get("dataPath").is_some());
    assert!(body.get("port").is_some());
    assert!(body.get("configFile").is_some());
}

#[tokio::test]
async fn test_get_config_workspace_configured_true() {
    let fx = AppFixture::new();
    let body = body_json(fx.get("/api/config").await).await;
    assert_eq!(body["workspaceConfigured"], true);
    assert!(body["workspacePath"].is_string());
}

#[tokio::test]
async fn test_patch_config_updates_workspace_path() {
    let fx = AppFixture::new();
    let new_ws = fx.workspace.path().join("custom-ws");
    let resp = fx
        .patch_json(
            "/api/config",
            serde_json::json!({"workspacePath": new_ws.to_str().unwrap()}),
        )
        .await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["workspacePath"], new_ws.to_str().unwrap());
    assert_eq!(body["workspaceConfigured"], true);
}

#[tokio::test]
async fn test_patch_config_updates_port() {
    let fx = AppFixture::new();
    let resp = fx
        .patch_json("/api/config", serde_json::json!({"port": 9999}))
        .await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["port"], 9999);
}

#[tokio::test]
async fn test_patch_config_invalid_json_returns_client_error() {
    let fx = AppFixture::new();
    let resp = fx
        .send_req(
            Request::builder()
                .method(Method::PATCH)
                .uri("/api/config")
                .header("content-type", "application/json")
                .body(Body::from("not valid json"))
                .unwrap(),
        )
        .await;
    assert!(
        resp.status().is_client_error(),
        "expected 4xx for invalid JSON, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn test_patch_config_empty_body_returns_client_error() {
    let fx = AppFixture::new();
    let resp = fx
        .send_req(
            Request::builder()
                .method(Method::PATCH)
                .uri("/api/config")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert!(
        resp.status().is_client_error(),
        "expected 4xx for empty body, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn test_patch_config_traversal_path_stored_as_is() {
    let fx = AppFixture::new();
    let traversal = format!("{}/sub/../injected", fx.workspace.path().display());
    let resp = fx
        .patch_json(
            "/api/config",
            serde_json::json!({"workspacePath": &traversal}),
        )
        .await;
    assert_eq!(resp.status(), 200);
    let body = body_json(resp).await;
    assert_eq!(body["workspacePath"], traversal);
}

#[tokio::test]
async fn test_patch_config_persists_across_get() {
    let fx = AppFixture::new();
    let new_ws = fx.workspace.path().join("persisted-ws");
    let (status, _) = body_status_json(
        fx.patch_json(
            "/api/config",
            serde_json::json!({"workspacePath": new_ws.to_str().unwrap()}),
        )
        .await,
    )
    .await;
    assert_eq!(status, 200);

    let body = body_json(fx.get("/api/config").await).await;
    assert_eq!(body["workspacePath"], new_ws.to_str().unwrap());
}
