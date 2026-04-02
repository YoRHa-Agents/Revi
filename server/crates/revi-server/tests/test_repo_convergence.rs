use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn base_adoption_files_exist() {
    let root = repo_root();
    for path in [
        root.join("AGENTS.md"),
        root.join("doc_auto/README.md"),
        root.join("doc_auto/base_adoption_boundary.md"),
        root.join("common/core/Cargo.toml"),
        root.join("common/adapters/Cargo.toml"),
        root.join("common/cli/Cargo.toml"),
        root.join(".github/workflows/release-artifacts.yml"),
        root.join("docs/index.md"),
        root.join("docs/guides/dev.md"),
        root.join("docs/guides/release.md"),
        root.join("scripts/build-release.sh"),
        root.join("scripts/release-contract.sh"),
        root.join("scripts/validate-release.sh"),
        root.join("scripts/validate-rust-vue.sh"),
        root.join("scripts/load-smoke.sh"),
        root.join("testdata/e2e/workspace/plans/sprint-1-design.md"),
        root.join("testdata/e2e/workspace/designs/ui-mockup-v1.svg"),
        root.join("testdata/e2e/workspace/prototypes/review-flow.html"),
        root.join("testdata/e2e/data/metadata.json"),
    ] {
        assert!(path.exists(), "missing {}", path.display());
    }
}

#[test]
fn docs_and_automation_no_longer_point_to_python_runtime() {
    let root = repo_root();
    let readme = fs::read_to_string(root.join("README.md")).unwrap();
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    let user_guide = fs::read_to_string(root.join("docs/user-guide.md")).unwrap();
    let agent_guide = fs::read_to_string(root.join("docs/agent-guide.md")).unwrap();
    let load_guide = fs::read_to_string(root.join("docs/load-test.md")).unwrap();
    let playwright = fs::read_to_string(root.join("frontend/playwright.config.js")).unwrap();
    let new_plan = fs::read_to_string(root.join("scripts/new-plan.sh")).unwrap();
    let new_design = fs::read_to_string(root.join("scripts/new-design.sh")).unwrap();
    let new_prototype = fs::read_to_string(root.join("scripts/new-prototype.sh")).unwrap();

    for content in [
        &readme,
        &makefile,
        &user_guide,
        &agent_guide,
        &load_guide,
        &playwright,
        &new_plan,
        &new_design,
        &new_prototype,
    ] {
        assert!(!content.contains("uvicorn"));
        assert!(!content.contains("backend/workspace"));
        assert!(!content.contains("/api/upload/{subfolder}"));
    }

    assert!(playwright.contains("testdata/e2e/workspace"));
    assert!(playwright.contains("cargo run --manifest-path server/Cargo.toml --bin revi"));
    assert!(!root.join("backend/main.py").exists());
}
