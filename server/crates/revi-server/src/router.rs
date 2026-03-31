use std::sync::Arc;

use axum::{
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
    pub config: Arc<Config>,
    pub comments: Arc<CommentStore>,
    pub archive: Arc<ArchiveStore>,
    pub metadata: Arc<MetadataStore>,
    pub scanner: Arc<WorkspaceScanner>,
}

impl AppState {
    pub fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        Ok(Self {
            comments: Arc::new(CommentStore::new(&config.data_path)?),
            archive: Arc::new(ArchiveStore::new(&config.data_path)?),
            metadata: Arc::new(MetadataStore::new(&config.data_path)),
            scanner: Arc::new(WorkspaceScanner::new(&config.workspace_path)),
            config,
        })
    }
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
        .with_state(state);

    Router::new().nest("/api", api)
}
