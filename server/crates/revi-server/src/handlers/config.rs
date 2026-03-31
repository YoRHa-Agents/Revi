use std::path::PathBuf;

use axum::{extract::State, Json};

use crate::{
    config::Config,
    error::AppError,
    models::{ConfigResponse, UpdateConfigRequest},
    router::AppState,
};

pub async fn get_config(State(s): State<AppState>) -> Json<ConfigResponse> {
    Json(config_to_response(&s.config))
}

pub async fn update_config(
    State(s): State<AppState>,
    Json(req): Json<UpdateConfigRequest>,
) -> Result<Json<ConfigResponse>, AppError> {
    let current = &*s.config;

    let workspace_path = req
        .workspace_path
        .map(PathBuf::from)
        .unwrap_or_else(|| current.workspace_path.clone());

    let data_path = req
        .data_path
        .map(PathBuf::from)
        .unwrap_or_else(|| current.data_path.clone());

    let port = req.port.unwrap_or(current.port);

    let updated = Config {
        workspace_path,
        data_path,
        port,
        config_file: current.config_file.clone(),
    };

    updated.save()?;

    Ok(Json(config_to_response(&updated)))
}

fn config_to_response(cfg: &Config) -> ConfigResponse {
    ConfigResponse {
        workspace_path: cfg.workspace_path.display().to_string(),
        data_path: cfg.data_path.display().to_string(),
        port: cfg.port,
        config_file: cfg.config_file.display().to_string(),
    }
}
