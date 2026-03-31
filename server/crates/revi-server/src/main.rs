use std::{net::SocketAddr, sync::Arc};

use clap::Parser;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use revi_server::{
    config::{Cli, Config},
    router::{build_cors, build_router, AppState},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "revi_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    let config = Arc::new(Config::load(&cli)?);

    tracing::info!("workspace: {}", config.workspace_path.display());
    tracing::info!("data:      {}", config.data_path.display());
    tracing::info!("port:      {}", config.port);
    tracing::info!("config:    {}", config.config_file.display());

    std::fs::create_dir_all(&config.data_path)?;
    std::fs::create_dir_all(&config.workspace_path)?;

    let state = AppState::new(config.clone())?;

    let workspace_dir = config.workspace_path.clone();
    let app = build_router(state)
        .nest_service("/workspace", ServeDir::new(&workspace_dir))
        .layer(build_cors());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
