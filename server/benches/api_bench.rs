use std::sync::Arc;

use axum::{body::Body, http::Request, Router};
use criterion::{criterion_group, criterion_main, Criterion};
use http_body_util::BodyExt;
use tempfile::TempDir;
use tower::ServiceExt;

use revi_server::{
    config::Config,
    router::{build_router, AppState},
};

struct Fixture {
    app: Router,
    _workspace: TempDir,
    _data: TempDir,
}

fn setup() -> Fixture {
    let workspace = TempDir::new().unwrap();
    let data = TempDir::new().unwrap();

    for sub in &["plans", "designs", "prototypes"] {
        std::fs::create_dir_all(workspace.path().join(sub)).unwrap();
    }
    std::fs::write(
        workspace.path().join("plans/sprint-1-design.md"),
        "# Sprint 1 — System Design\n\n## Overview\n\nJSON comments.\n",
    )
    .unwrap();
    std::fs::write(
        workspace.path().join("designs/ui-mockup-v1.png"),
        b"\x89PNG\r\n\x1a\n\x00\x00\x00\x00" as &[u8],
    )
    .unwrap();
    std::fs::write(
        workspace.path().join("prototypes/review-flow.html"),
        "<html><body></body></html>",
    )
    .unwrap();

    let config = Arc::new(Config {
        workspace_path: workspace.path().to_path_buf(),
        data_path: data.path().to_path_buf(),
        port: 0,
    });
    let state = AppState::new(config).unwrap();
    let app = build_router(state);

    Fixture { app, _workspace: workspace, _data: data }
}

fn add_comments(fx: &Fixture, n: usize) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    for i in 0..n {
        let req = Request::builder()
            .method("POST")
            .uri("/api/comments/plans/sprint-1-design")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({
                    "author": "bench",
                    "content": format!("comment {}", i)
                }))
                .unwrap(),
            ))
            .unwrap();
        rt.block_on(async {
            fx.app.clone().oneshot(req).await.unwrap();
        });
    }
}

fn bench_list_reviews(c: &mut Criterion) {
    let fx = setup();
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("GET /api/reviews", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = Request::builder()
                    .method("GET")
                    .uri("/api/reviews")
                    .body(Body::empty())
                    .unwrap();
                let resp = fx.app.clone().oneshot(req).await.unwrap();
                resp.into_body().collect().await.unwrap();
            })
        })
    });
}

fn bench_get_detail(c: &mut Criterion) {
    let fx = setup();
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("GET /api/reviews/plans/sprint-1-design", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = Request::builder()
                    .method("GET")
                    .uri("/api/reviews/plans/sprint-1-design")
                    .body(Body::empty())
                    .unwrap();
                let resp = fx.app.clone().oneshot(req).await.unwrap();
                resp.into_body().collect().await.unwrap();
            })
        })
    });
}

fn bench_add_comment(c: &mut Criterion) {
    let fx = setup();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let payload = serde_json::to_vec(&serde_json::json!({
        "author": "bench",
        "content": "benchmark comment",
        "reference": {"type": "general", "value": null}
    }))
    .unwrap();
    c.bench_function("POST /api/comments/plans/sprint-1-design", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = Request::builder()
                    .method("POST")
                    .uri("/api/comments/plans/sprint-1-design")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.clone()))
                    .unwrap();
                let resp = fx.app.clone().oneshot(req).await.unwrap();
                resp.into_body().collect().await.unwrap();
            })
        })
    });
}

fn bench_export(c: &mut Criterion) {
    let fx = setup();
    add_comments(&fx, 10);
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("GET /api/export/plans/sprint-1-design", |b| {
        b.iter(|| {
            rt.block_on(async {
                let req = Request::builder()
                    .method("GET")
                    .uri("/api/export/plans/sprint-1-design")
                    .body(Body::empty())
                    .unwrap();
                let resp = fx.app.clone().oneshot(req).await.unwrap();
                resp.into_body().collect().await.unwrap();
            })
        })
    });
}

criterion_group!(benches, bench_list_reviews, bench_get_detail, bench_add_comment, bench_export);
criterion_main!(benches);
