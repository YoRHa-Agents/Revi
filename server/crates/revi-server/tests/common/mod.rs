#![allow(dead_code)]

use std::sync::Arc;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    response::Response,
    Router,
};
use http_body_util::BodyExt;
use serde_json::Value;
use tempfile::TempDir;
use tower::util::ServiceExt;

use revi_server::{
    config::Config,
    router::{build_router, AppState},
};

pub struct AppFixture {
    pub workspace: TempDir,
    pub data: TempDir,
    pub app: Router,
}

impl AppFixture {
    pub fn new() -> Self {
        let workspace = TempDir::new().unwrap();
        let data = TempDir::new().unwrap();

        for sub in &["plans", "designs", "prototypes"] {
            std::fs::create_dir_all(workspace.path().join(sub)).unwrap();
        }

        std::fs::write(
            workspace.path().join("plans/sprint-1-design.md"),
            "# Sprint 1 — System Design\n\n## Overview\n\n## Storage Layer\nJSON comments.\n",
        )
        .unwrap();
        std::fs::write(
            workspace.path().join("designs/ui-mockup-v1.png"),
            b"\x89PNG\r\n\x1a\n\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                as &[u8],
        )
        .unwrap();
        std::fs::write(
            workspace.path().join("prototypes/review-flow.html"),
            "<html><body><h1>Proto</h1></body></html>",
        )
        .unwrap();

        let config = Arc::new(Config {
            workspace_path: workspace.path().to_path_buf(),
            data_path: data.path().to_path_buf(),
            port: 0,
            config_file: std::path::PathBuf::from("revi.toml"),
        });
        let state = AppState::new(config).unwrap();
        let app = build_router(state);

        AppFixture { workspace, data, app }
    }

    pub async fn send_req(&self, req: Request<Body>) -> Response {
        self.app.clone().oneshot(req).await.unwrap()
    }

    pub async fn get(&self, path: &str) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::GET)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    pub async fn post_json(&self, path: &str, body: Value) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::POST)
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
    }

    pub async fn patch_json(&self, path: &str, body: Value) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::PATCH)
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
    }

    pub async fn patch_empty(&self, path: &str) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::PATCH)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    pub async fn post_empty(&self, path: &str) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::POST)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    pub async fn post_multipart(&self, path: &str, body: Vec<u8>, boundary: &str) -> Response {
        self.send_req(
            Request::builder()
                .method(Method::POST)
                .uri(path)
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
    }
}

pub async fn body_json(resp: Response) -> Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

pub async fn body_status_json(resp: Response) -> (StatusCode, Value) {
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}
