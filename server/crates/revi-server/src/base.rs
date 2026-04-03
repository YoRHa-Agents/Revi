use common_core::{
    AudienceKind, DocsNode, DocsNodeKind, ReleaseChannel, ReleaseDescriptor, ReleaseTarget,
    RouteDescriptor, RouteKind, RuntimeContract, SurfaceKind, ThemeTokenSet, WorkspaceIdentity,
};

pub fn revi_workspace_identity() -> WorkspaceIdentity {
    WorkspaceIdentity {
        workspace_id: "revi".to_string(),
        repo_name: "Revi".to_string(),
        app_name: "Revi".to_string(),
        default_surface: SurfaceKind::Web,
    }
}

pub fn revi_route_descriptors() -> Vec<RouteDescriptor> {
    vec![
        RouteDescriptor {
            route_id: "landing".to_string(),
            path: "/".to_string(),
            route_kind: RouteKind::Landing,
            parent_route_id: None,
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "review-app".to_string(),
            path: "/review/:itemId+".to_string(),
            route_kind: RouteKind::ReviewApp,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "archive".to_string(),
            path: "/archive".to_string(),
            route_kind: RouteKind::Archive,
            parent_route_id: Some("review-app".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "demo".to_string(),
            path: "/demo/".to_string(),
            route_kind: RouteKind::Demo,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
        RouteDescriptor {
            route_id: "guide".to_string(),
            path: "/guide/".to_string(),
            route_kind: RouteKind::DocsEntry,
            parent_route_id: Some("landing".to_string()),
            static_fallback: true,
        },
    ]
}

pub fn revi_docs_nodes() -> Vec<DocsNode> {
    vec![
        DocsNode {
            node_id: "readme".to_string(),
            title: "README".to_string(),
            kind: DocsNodeKind::Index,
            path: "README.md".to_string(),
            audience: AudienceKind::Human,
            children: vec!["docs-index".to_string()],
        },
        DocsNode {
            node_id: "agents".to_string(),
            title: "AGENTS".to_string(),
            kind: DocsNodeKind::Guide,
            path: "AGENTS.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec!["docs-index".to_string()],
        },
        DocsNode {
            node_id: "docs-index".to_string(),
            title: "Docs Index".to_string(),
            kind: DocsNodeKind::Index,
            path: "docs/index.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![
                "dev-guide".to_string(),
                "release-guide".to_string(),
                "user-guide".to_string(),
                "agent-guide".to_string(),
                "deploy-pages".to_string(),
                "load-test".to_string(),
            ],
        },
        DocsNode {
            node_id: "dev-guide".to_string(),
            title: "Development Guide".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/guides/dev.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec![],
        },
        DocsNode {
            node_id: "release-guide".to_string(),
            title: "Release Guide".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/guides/release.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec![],
        },
        DocsNode {
            node_id: "user-guide".to_string(),
            title: "User Guide".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/user-guide.md".to_string(),
            audience: AudienceKind::Human,
            children: vec![],
        },
        DocsNode {
            node_id: "agent-guide".to_string(),
            title: "Agent Guide".to_string(),
            kind: DocsNodeKind::Api,
            path: "docs/agent-guide.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![],
        },
        DocsNode {
            node_id: "deploy-pages".to_string(),
            title: "Deploy Pages".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/deploy-pages.md".to_string(),
            audience: AudienceKind::MainAgent,
            children: vec![],
        },
        DocsNode {
            node_id: "load-test".to_string(),
            title: "Load Testing".to_string(),
            kind: DocsNodeKind::Guide,
            path: "docs/load-test.md".to_string(),
            audience: AudienceKind::Subagent,
            children: vec![],
        },
    ]
}

pub fn revi_release_descriptor() -> ReleaseDescriptor {
    ReleaseDescriptor {
        channel: ReleaseChannel::Preview,
        targets: vec![
            ReleaseTarget::Site,
            ReleaseTarget::Guide,
            ReleaseTarget::RustBinary,
            ReleaseTarget::FrontendBundle,
        ],
        artifact_namespace: "revi".to_string(),
        requires_signature: false,
    }
}

pub fn revi_runtime_contract() -> RuntimeContract {
    RuntimeContract {
        web_runtime_id: "revi_rust_vue".to_string(),
        web_entry_route: "/review/plans/sprint-1-design".to_string(),
        docs_index_path: "/guide/".to_string(),
        pages_strategy: "static_site_plus_hash_spa".to_string(),
        api_default_origin: "http://localhost:8000".to_string(),
    }
}

pub fn revi_theme_tokens() -> ThemeTokenSet {
    ThemeTokenSet::nier_gray()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_core::{AudienceKind, RouteKind, SurfaceKind};

    #[test]
    fn workspace_identity_defaults_to_web() {
        let identity = revi_workspace_identity();
        assert_eq!(identity.workspace_id, "revi");
        assert_eq!(identity.default_surface, SurfaceKind::Web);
    }

    #[test]
    fn revi_routes_cover_review_archive_and_site_entries() {
        let routes = revi_route_descriptors();
        let paths: Vec<&str> = routes.iter().map(|route| route.path.as_str()).collect();
        assert!(paths.contains(&"/review/:itemId+"));
        assert!(paths.contains(&"/archive"));
        assert!(paths.contains(&"/demo/"));
        assert!(routes
            .iter()
            .any(|route| route.route_kind == RouteKind::ReviewApp));
    }

    #[test]
    fn docs_nodes_cover_human_main_and_subagent_readers() {
        let docs_nodes = revi_docs_nodes();
        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::Human));
        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::MainAgent));
        assert!(docs_nodes
            .iter()
            .any(|node| node.audience == AudienceKind::Subagent));
    }

    #[test]
    fn release_descriptor_uses_revi_namespace() {
        let descriptor = revi_release_descriptor();
        assert_eq!(descriptor.artifact_namespace, "revi");
        assert!(!descriptor.requires_signature);
    }
}
