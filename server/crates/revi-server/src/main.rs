use std::net::SocketAddr;

use clap::Parser;
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
    let config = Config::load(&cli)?;

    if let Some(ref ws) = config.workspace_path {
        tracing::info!("workspace: {}", ws.display());
    } else {
        tracing::info!("workspace: (not configured — set via web UI or PATCH /api/config)");
    }
    tracing::info!("data:      {}", config.data_path.display());
    tracing::info!("port:      {}", config.port);
    tracing::info!("config:    {}", config.config_file.display());

    std::fs::create_dir_all(&config.data_path)?;
    if let Some(ref ws) = config.workspace_path {
        std::fs::create_dir_all(ws)?;
    }

    let state = AppState::new(config.clone())?;

    let app = build_router(state)
        .layer(build_cors());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
