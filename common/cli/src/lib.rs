use common_adapters::{
    DemoSiteAdapter, DocsSiteAdapter, JsonManifestAdapter, ReleasePipelineAdapter,
    ScriptReleaseAdapter,
};
use common_core::{LaunchProfile, SurfaceKind};
use revi_server::base::{
    revi_docs_nodes, revi_release_descriptor, revi_route_descriptors, revi_runtime_contract,
    revi_theme_tokens, revi_workspace_identity,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    Dev,
    DemoBuild,
    DocsBuild,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliInvocation {
    pub command: CommandKind,
    pub surface: Option<SurfaceKind>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionBundle {
    pub messages: Vec<String>,
}

pub fn parse_cli_args<S>(args: &[S]) -> Result<CliInvocation, String>
where
    S: AsRef<str>,
{
    if args.is_empty() {
        return Err("expected a command: dev/demo/docs/release".to_string());
    }

    match args[0].as_ref() {
        "dev" => parse_dev_args(args),
        "demo" => Ok(CliInvocation {
            command: CommandKind::DemoBuild,
            surface: Some(SurfaceKind::Demo),
            host: None,
            port: None,
        }),
        "docs" => Ok(CliInvocation {
            command: CommandKind::DocsBuild,
            surface: Some(SurfaceKind::Docs),
            host: None,
            port: None,
        }),
        "release" => Ok(CliInvocation {
            command: CommandKind::Release,
            surface: Some(SurfaceKind::Desktop),
            host: None,
            port: None,
        }),
        other => Err(format!("unsupported command: {other}")),
    }
}

fn parse_dev_args<S>(args: &[S]) -> Result<CliInvocation, String>
where
    S: AsRef<str>,
{
    let mut surface = None;
    let mut host = None;
    let mut port = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_ref() {
            "--surface" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --surface".to_string());
                };
                surface = Some(parse_surface(value.as_ref())?);
            }
            "--host" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --host".to_string());
                };
                host = Some(value.as_ref().to_string());
            }
            "--port" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("missing value after --port".to_string());
                };
                let parsed = value
                    .as_ref()
                    .parse::<u16>()
                    .map_err(|_| "port must be a valid u16".to_string())?;
                port = Some(parsed);
            }
            other => {
                return Err(format!("unsupported dev option: {other}"));
            }
        }
        index += 1;
    }

    Ok(CliInvocation {
        command: CommandKind::Dev,
        surface,
        host,
        port,
    })
}

fn parse_surface(value: &str) -> Result<SurfaceKind, String> {
    match value {
        "web" => Ok(SurfaceKind::Web),
        "desktop" => Ok(SurfaceKind::Desktop),
        "docs" => Ok(SurfaceKind::Docs),
        "demo" => Ok(SurfaceKind::Demo),
        other => Err(format!("unsupported surface: {other}")),
    }
}

pub fn execute(invocation: &CliInvocation) -> Result<ExecutionBundle, String> {
    let repo_root = repo_root();
    let site_root = site_root();
    execute_with_roots(invocation, &repo_root, &site_root)
}

fn execute_with_roots(
    invocation: &CliInvocation,
    repo_root: &Path,
    site_root: &Path,
) -> Result<ExecutionBundle, String> {
    let mut messages = Vec::new();

    match invocation.command {
        CommandKind::Dev => {
            JsonManifestAdapter
                .build_demo_assets(
                    &site_root,
                    &revi_route_descriptors(),
                    &revi_theme_tokens(),
                    &revi_runtime_contract(),
                )
                .map_err(|error| error.to_string())?;
            JsonManifestAdapter
                .build_docs_assets(&site_root, &revi_docs_nodes())
                .map_err(|error| error.to_string())?;

            let host = invocation.host.as_deref().unwrap_or("127.0.0.1");
            let port = invocation.port.unwrap_or(5173);
            let profile = match invocation.surface.unwrap_or(SurfaceKind::Web) {
                SurfaceKind::Web => LaunchProfile::Dev,
                SurfaceKind::Desktop => LaunchProfile::Debug,
                SurfaceKind::Docs | SurfaceKind::Demo => LaunchProfile::Release,
            };
            messages.push(format!(
                "prepared base assets for {} ({profile:?})",
                revi_workspace_identity().app_name
            ));
            messages.push(format!(
                "run cargo --manifest-path server/Cargo.toml run --bin revi -- --port 8000"
            ));
            messages.push(format!(
                "run npm --prefix frontend run dev -- --host {host} --port {port}"
            ));
        }
        CommandKind::DemoBuild => {
            let result = JsonManifestAdapter
                .build_demo_assets(
                    &site_root,
                    &revi_route_descriptors(),
                    &revi_theme_tokens(),
                    &revi_runtime_contract(),
                )
                .map_err(|error| error.to_string())?;
            messages.push(format!("demo assets built under {}", result.output_root.display()));
        }
        CommandKind::DocsBuild => {
            let result = JsonManifestAdapter
                .build_docs_assets(&site_root, &revi_docs_nodes())
                .map_err(|error| error.to_string())?;
            messages.push(format!("docs assets built under {}", result.output_root.display()));
        }
        CommandKind::Release => {
            ScriptReleaseAdapter
                .run_release(repo_root, &revi_release_descriptor())
                .map_err(|error| error.to_string())?;
            messages.push("release bundle generated via scripts/build-release.sh".to_string());
        }
    }

    Ok(ExecutionBundle { messages })
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("common/cli should have parent")
        .parent()
        .expect("common should have parent")
        .to_path_buf()
}

fn site_root() -> PathBuf {
    std::env::var_os("REVI_SITE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("site"))
}

#[cfg(test)]
mod tests {
    use super::{execute_with_roots, parse_cli_args, repo_root, CommandKind, SurfaceKind};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_site_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("revi-common-cli-{unique}"));
        fs::create_dir_all(&path).expect("temp site root should exist");
        path
    }

    #[test]
    fn parses_demo_command() {
        let invocation = parse_cli_args(&["demo"]).expect("demo should parse");
        assert_eq!(invocation.command, CommandKind::DemoBuild);
        assert_eq!(invocation.surface, Some(SurfaceKind::Demo));
    }

    #[test]
    fn parses_dev_with_surface_host_and_port() {
        let invocation = parse_cli_args(&[
            "dev",
            "--surface",
            "web",
            "--host",
            "127.0.0.1",
            "--port",
            "5175",
        ])
        .expect("dev should parse");
        assert_eq!(invocation.command, CommandKind::Dev);
        assert_eq!(invocation.surface, Some(SurfaceKind::Web));
        assert_eq!(invocation.port, Some(5175));
    }

    #[test]
    fn demo_build_writes_base_manifests() {
        let site_root = temp_site_root();
        let invocation = parse_cli_args(&["demo"]).expect("demo should parse");
        let bundle =
            execute_with_roots(&invocation, &repo_root(), &site_root).expect("demo should execute");
        assert!(bundle.messages[0].contains("demo assets built"));
        assert!(site_root.join("assets/route-manifest.json").is_file());
        assert!(site_root.join("assets/theme-tokens.json").is_file());
        assert!(site_root.join("assets/runtime-contract.json").is_file());
    }

    #[test]
    fn docs_build_writes_docs_index_manifest() {
        let site_root = temp_site_root();
        let invocation = parse_cli_args(&["docs"]).expect("docs should parse");
        let bundle =
            execute_with_roots(&invocation, &repo_root(), &site_root).expect("docs should execute");
        assert!(bundle.messages[0].contains("docs assets built"));
        assert!(site_root.join("guide/docs-index.json").is_file());
    }
}
