use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceIdentity {
    pub workspace_id: String,
    pub repo_name: String,
    pub app_name: String,
    pub default_surface: SurfaceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SurfaceKind {
    Web,
    Desktop,
    Docs,
    Demo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LaunchProfile {
    Dev,
    Debug,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RouteKind {
    Landing,
    RuntimeMap,
    DocsEntry,
    Demo,
    ReleaseFlow,
    StyleLab,
    StoryDetail,
    ReviewApp,
    Archive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DevLaunchRequest {
    pub surface: SurfaceKind,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub route_entry: Option<String>,
    pub profile: LaunchProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RouteDescriptor {
    pub route_id: String,
    pub path: String,
    pub route_kind: RouteKind,
    pub parent_route_id: Option<String>,
    pub static_fallback: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DocsNodeKind {
    Index,
    Architecture,
    Guide,
    Example,
    Demo,
    Theme,
    Api,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AudienceKind {
    Human,
    MainAgent,
    Subagent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocsNode {
    pub node_id: String,
    pub title: String,
    pub kind: DocsNodeKind,
    pub path: String,
    pub audience: AudienceKind,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThemeTokenSet {
    pub palette: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub component_rules: BTreeMap<String, String>,
}

impl ThemeTokenSet {
    pub fn nier_gray() -> Self {
        let palette = BTreeMap::from([
            ("bg.canvas".to_string(), "#111111".to_string()),
            ("bg.panel".to_string(), "#1B1A17".to_string()),
            ("fg.primary".to_string(), "#ECE6D9".to_string()),
            ("fg.secondary".to_string(), "#B7B0A3".to_string()),
            ("line.strong".to_string(), "#D7D0C4".to_string()),
            ("line.soft".to_string(), "#5F5A52".to_string()),
            ("accent.signal".to_string(), "#C2572B".to_string()),
        ]);
        let typography = BTreeMap::from([
            ("title.family".to_string(), "serif-display".to_string()),
            ("body.family".to_string(), "humanist-sans".to_string()),
            ("meta.family".to_string(), "monospace".to_string()),
        ]);
        let component_rules = BTreeMap::from([
            ("heading.syntax".to_string(), "NN / Section".to_string()),
            ("table.style".to_string(), "strong-border".to_string()),
            ("card.style".to_string(), "thin-border-panel".to_string()),
        ]);

        Self {
            palette,
            typography,
            component_rules,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReleaseChannel {
    Dev,
    Preview,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReleaseTarget {
    Site,
    Guide,
    RustBinary,
    FrontendBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReleaseDescriptor {
    pub channel: ReleaseChannel,
    pub targets: Vec<ReleaseTarget>,
    pub artifact_namespace: String,
    pub requires_signature: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeContract {
    pub web_runtime_id: String,
    pub web_entry_route: String,
    pub docs_index_path: String,
    pub pages_strategy: String,
    pub api_default_origin: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nier_gray_tokens_expose_expected_keys() {
        let tokens = ThemeTokenSet::nier_gray();

        assert!(tokens.palette.contains_key("bg.canvas"));
        assert!(tokens.palette.contains_key("fg.primary"));
        assert!(tokens.component_rules.contains_key("heading.syntax"));
    }

    #[test]
    fn dev_launch_request_preserves_network_fields() {
        let request = DevLaunchRequest {
            surface: SurfaceKind::Web,
            host: Some("127.0.0.1".to_string()),
            port: Some(8080),
            route_entry: Some("/review/plans/demo".to_string()),
            profile: LaunchProfile::Dev,
        };

        assert_eq!(request.host.as_deref(), Some("127.0.0.1"));
        assert_eq!(request.port, Some(8080));
        assert_eq!(request.route_entry.as_deref(), Some("/review/plans/demo"));
    }
}
