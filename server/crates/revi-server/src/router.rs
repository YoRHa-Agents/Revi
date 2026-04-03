use std::sync::{Arc, RwLock};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

use crate::{
    config::Config,
    handlers::{archive, comments, config as config_handler, export, items, upload},
    storage::{archive::ArchiveStore, comment::CommentStore, metadata::MetadataStore},
    workspace::scanner::WorkspaceScanner,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub comments: Arc<CommentStore>,
    pub archive: Arc<ArchiveStore>,
    pub metadata: Arc<MetadataStore>,
    pub scanner: Arc<RwLock<WorkspaceScanner>>,
}

impl AppState {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let scanner = WorkspaceScanner::new(config.effective_workspace());
        Ok(Self {
            comments: Arc::new(CommentStore::new(&config.data_path)?),
            archive: Arc::new(ArchiveStore::new(&config.data_path)?),
            metadata: Arc::new(MetadataStore::new(&config.data_path)),
            scanner: Arc::new(RwLock::new(scanner)),
            config: Arc::new(RwLock::new(config)),
        })
    }
}

async fn serve_workspace_file(
    State(s): State<AppState>,
    Path(path): Path<String>,
) -> Response {
    let workspace = {
        let cfg = s.config.read().unwrap();
        cfg.effective_workspace()
    };
    let file_path = workspace.join(&path);

    let file_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };
    let ws_canon = match workspace.canonicalize() {
        Ok(p) => p,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };
    if !file_path.starts_with(&ws_canon) {
        return StatusCode::FORBIDDEN.into_response();
    }

    let bytes = match tokio::fs::read(&file_path).await {
        Ok(b) => b,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mime = mime_guess::from_path(&file_path)
        .first_or_octet_stream()
        .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .body(Body::from(bytes))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

pub fn build_cors() -> CorsLayer {
    use axum::http::HeaderValue;
    let origins: Vec<HeaderValue> = [
        "http://localhost:5173",
        "http://localhost:5174",
        "http://localhost:5175",
        "http://localhost:5176",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(AllowHeaders::list([CONTENT_TYPE, AUTHORIZATION, ACCEPT]))
        .allow_credentials(true)
}

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/reviews", get(items::list_reviews))
        .route(
            "/reviews/*item_id",
            get(items::get_review).patch(items::update_review_type),
        )
        .route(
            "/comments/*item_id",
            get(comments::list_comments)
                .post(comments::add_comment)
                .patch(comments::patch_handler),
        )
        .route(
            "/archive/*item_id",
            get(archive::list_archive).post(archive::archive_resolved),
        )
        .route("/export/*item_id", get(export::export_for_agent))
        .route("/upload", post(upload::upload_file))
        .route("/config", get(config_handler::get_config).patch(config_handler::update_config))
        .with_state(state.clone());

    Router::new()
        .nest("/api", api)
        .route("/workspace/*path", get(serve_workspace_file).with_state(state))
}
