use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "revi", about = "Revi review server — drop-and-run")]
pub struct Cli {
    /// Workspace directory (scanned for review items)
    #[arg(long)]
    pub workspace: Option<PathBuf>,

    /// Data directory (comments, archive, metadata)
    #[arg(long)]
    pub data: Option<PathBuf>,

    /// Port to listen on
    #[arg(long)]
    pub port: Option<u16>,

    /// Path to config file (default: ./revi.toml or ~/.config/revi/config.toml)
    #[arg(long)]
    pub config: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// Config file (TOML, all fields optional)
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    pub workspace: Option<String>,
    pub data: Option<String>,
    pub port: Option<u16>,
}

// ---------------------------------------------------------------------------
// Runtime config (resolved, all fields present)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Config {
    pub workspace_path: Option<PathBuf>,
    pub data_path: PathBuf,
    pub port: u16,
    /// The config file that was loaded (or would be written to on save)
    pub config_file: PathBuf,
}

impl Config {
    /// Load config from CLI flags → config file → defaults.
    pub fn load(cli: &Cli) -> anyhow::Result<Self> {
        let config_file = cli.config.clone().unwrap_or_else(resolve_config_file_path);

        let file_cfg: ConfigFile = if config_file.exists() {
            let raw = std::fs::read_to_string(&config_file)?;
            toml::from_str(&raw)?
        } else {
            ConfigFile::default()
        };

        let workspace_path = cli
            .workspace
            .clone()
            .or_else(|| file_cfg.workspace.as_deref().map(PathBuf::from));

        let data_path = cli
            .data
            .clone()
            .or_else(|| file_cfg.data.as_deref().map(PathBuf::from))
            .unwrap_or_else(default_data_dir);

        let port = cli.port.or(file_cfg.port).unwrap_or(8000);

        Ok(Self { workspace_path, data_path, port, config_file })
    }

    pub fn workspace_configured(&self) -> bool {
        self.workspace_path.is_some()
    }

    pub fn effective_workspace(&self) -> PathBuf {
        self.workspace_path
            .clone()
            .unwrap_or_else(default_workspace_dir)
    }

    /// Persist current values to the config file.
    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.config_file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file_cfg = ConfigFile {
            workspace: self.workspace_path.as_ref().map(|p| p.display().to_string()),
            data: Some(self.data_path.display().to_string()),
            port: Some(self.port),
        };
        std::fs::write(&self.config_file, toml::to_string_pretty(&file_cfg)?)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn revi_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".revi")
}

fn default_workspace_dir() -> PathBuf {
    revi_home().join("workspace")
}

fn default_data_dir() -> PathBuf {
    revi_home().join("data")
}

/// Search order: `./revi.toml` → `~/.config/revi/config.toml`
fn resolve_config_file_path() -> PathBuf {
    let cwd = PathBuf::from("revi.toml");
    if cwd.exists() {
        return cwd;
    }
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/revi/config.toml")
}
