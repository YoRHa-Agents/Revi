use common_core::{DocsNode, ReleaseDescriptor, RouteDescriptor, RuntimeContract, ThemeTokenSet};
use serde::Serialize;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteBuildResult {
    pub output_root: PathBuf,
    pub generated_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocsIndexManifest {
    pub docs_nodes: Vec<DocsNode>,
}

pub trait DemoSiteAdapter {
    fn build_demo_assets(
        &self,
        site_root: &Path,
        routes: &[RouteDescriptor],
        theme: &ThemeTokenSet,
        runtime_contract: &RuntimeContract,
    ) -> io::Result<SiteBuildResult>;
}

pub trait DocsSiteAdapter {
    fn build_docs_assets(&self, site_root: &Path, docs_nodes: &[DocsNode]) -> io::Result<SiteBuildResult>;
}

pub trait ReleasePipelineAdapter {
    fn run_release(&self, repo_root: &Path, descriptor: &ReleaseDescriptor) -> io::Result<()>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct JsonManifestAdapter;

impl DemoSiteAdapter for JsonManifestAdapter {
    fn build_demo_assets(
        &self,
        site_root: &Path,
        routes: &[RouteDescriptor],
        theme: &ThemeTokenSet,
        runtime_contract: &RuntimeContract,
    ) -> io::Result<SiteBuildResult> {
        fs::create_dir_all(site_root.join("assets"))?;
        let route_manifest = site_root.join("assets").join("route-manifest.json");
        let theme_manifest = site_root.join("assets").join("theme-tokens.json");
        let runtime_manifest = site_root.join("assets").join("runtime-contract.json");
        fs::write(&route_manifest, serde_json::to_vec_pretty(routes).map_err(io::Error::other)?)?;
        fs::write(&theme_manifest, serde_json::to_vec_pretty(theme).map_err(io::Error::other)?)?;
        fs::write(
            &runtime_manifest,
            serde_json::to_vec_pretty(runtime_contract).map_err(io::Error::other)?,
        )?;

        Ok(SiteBuildResult {
            output_root: site_root.to_path_buf(),
            generated_files: vec![route_manifest, theme_manifest, runtime_manifest],
        })
    }
}

impl DocsSiteAdapter for JsonManifestAdapter {
    fn build_docs_assets(&self, site_root: &Path, docs_nodes: &[DocsNode]) -> io::Result<SiteBuildResult> {
        let guide_root = site_root.join("guide");
        fs::create_dir_all(&guide_root)?;
        let docs_index = guide_root.join("docs-index.json");
        fs::write(
            &docs_index,
            serde_json::to_vec_pretty(&DocsIndexManifest {
                docs_nodes: docs_nodes.to_vec(),
            })
            .map_err(io::Error::other)?,
        )?;

        Ok(SiteBuildResult {
            output_root: guide_root,
            generated_files: vec![docs_index],
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ScriptReleaseAdapter;

impl ReleasePipelineAdapter for ScriptReleaseAdapter {
    fn run_release(&self, repo_root: &Path, descriptor: &ReleaseDescriptor) -> io::Result<()> {
        let script = repo_root.join("scripts").join("build-release.sh");
        if !script.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("missing release script at {}", script.display()),
            ));
        }
        let status = Command::new("bash")
            .arg(&script)
            .env("REVI_RELEASE_NAMESPACE", &descriptor.artifact_namespace)
            .current_dir(repo_root)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "release script exited with status {status}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DemoSiteAdapter, DocsSiteAdapter, JsonManifestAdapter};
    use common_core::{AudienceKind, DocsNode, DocsNodeKind, RouteDescriptor, RouteKind, RuntimeContract, ThemeTokenSet};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
        fs::create_dir_all(&path).expect("temp dir should exist");
        path
    }

    #[test]
    fn demo_assets_write_route_theme_and_runtime_manifests() {
        let root = temp_dir("revi-demo-assets");
        let adapter = JsonManifestAdapter;
        let result = adapter
            .build_demo_assets(
                &root,
                &[RouteDescriptor {
                    route_id: "landing".to_string(),
                    path: "/".to_string(),
                    route_kind: RouteKind::Landing,
                    parent_route_id: None,
                    static_fallback: true,
                }],
                &ThemeTokenSet::nier_gray(),
                &RuntimeContract {
                    web_runtime_id: "revi".to_string(),
                    web_entry_route: "/".to_string(),
                    docs_index_path: "/guide/".to_string(),
                    pages_strategy: "static".to_string(),
                    api_default_origin: "http://localhost:8000".to_string(),
                },
            )
            .expect("demo assets should build");

        assert_eq!(result.generated_files.len(), 3);
        assert!(root.join("assets/route-manifest.json").is_file());
        assert!(root.join("assets/theme-tokens.json").is_file());
        assert!(root.join("assets/runtime-contract.json").is_file());
    }

    #[test]
    fn docs_assets_write_docs_index_manifest() {
        let root = temp_dir("revi-docs-assets");
        let adapter = JsonManifestAdapter;
        adapter
            .build_docs_assets(
                &root,
                &[DocsNode {
                    node_id: "readme".to_string(),
                    title: "README".to_string(),
                    kind: DocsNodeKind::Index,
                    path: "README.md".to_string(),
                    audience: AudienceKind::Human,
                    children: vec![],
                }],
            )
            .expect("docs assets should build");

        assert!(root.join("guide/docs-index.json").is_file());
    }
}
