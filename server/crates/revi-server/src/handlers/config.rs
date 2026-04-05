use std::path::PathBuf;

use axum::{extract::State, Json};

use crate::{
    config::Config,
    error::AppError,
    models::{ConfigResponse, UpdateConfigRequest},
    router::AppState,
    workspace::scanner::WorkspaceScanner,
};

pub async fn get_config(State(s): State<AppState>) -> Result<Json<ConfigResponse>, AppError> {
    let cfg = s.config.read().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;
    Ok(Json(config_to_response(&cfg)))
}

pub async fn update_config(
    State(s): State<AppState>,
    Json(req): Json<UpdateConfigRequest>,
) -> Result<Json<ConfigResponse>, AppError> {
    let updated = {
        let current = s.config.read().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;

        let workspace_path = if let Some(ref ws) = req.workspace_path {
            Some(PathBuf::from(ws))
        } else {
            current.workspace_path.clone()
        };

        let data_path = req
            .data_path
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| current.data_path.clone());

        let port = req.port.unwrap_or(current.port);

        Config {
            workspace_path,
            data_path,
            port,
            config_file: current.config_file.clone(),
        }
    };

    if let Some(ref ws) = updated.workspace_path {
        std::fs::create_dir_all(ws)
            .map_err(|e| AppError::BadRequest(format!("cannot create workspace dir: {e}")))?;
    }

    updated.save()?;

    {
        let new_scanner = WorkspaceScanner::new(updated.effective_workspace());
        let mut scanner = s.scanner.write().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;
        *scanner = new_scanner;
    }

    let response = config_to_response(&updated);

    {
        let mut cfg = s.config.write().map_err(|_| AppError::Internal(anyhow::anyhow!("lock poisoned")))?;
        *cfg = updated;
    }

    Ok(Json(response))
}

fn config_to_response(cfg: &Config) -> ConfigResponse {
    ConfigResponse {
        workspace_path: cfg.workspace_path.as_ref().map(|p| p.display().to_string()),
        workspace_configured: cfg.workspace_configured(),
        data_path: cfg.data_path.display().to_string(),
        port: cfg.port,
        config_file: cfg.config_file.display().to_string(),
    }
}
