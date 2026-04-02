# Docs Index

## Start Here
- `README.md`：产品总览、快速启动与接入 `RustWebAppCommon` 的现状。
- `AGENTS.md`：主 agent / subagent 先读入口。
- `doc_auto/base_adoption_boundary.md`：base 与 Revi app-owned 的边界裁决。

## Choose Your Path
### Human
- `README.md`
- `docs/user-guide.md`
- `docs/deploy-pages.md`

### Main Agent
- `AGENTS.md`
- `docs/guides/dev.md`
- `docs/guides/release.md`

### Subagent
- `docs/agent-guide.md`
- `demo/storyboard.md`
- `doc_auto/implementation_handoff.md`

## Core Navigation
- `docs/guides/dev.md`
- `docs/guides/release.md`
- `docs/user-guide.md`
- `docs/agent-guide.md`
- `docs/load-test.md`
- `demo/storyboard.md`
- `doc_auto/base_adoption_boundary.md`
- `doc_auto/implementation_handoff.md`

## Current Decisions
- Revi 以 vendored `common/` 方式接入 `RustWebAppCommon` 的 base 模式。
- `revi` 仍是产品 runtime；`common` 负责 `dev / demo / docs / release` 统一前门。
- Revi 的 review domain 继续保持 `app_owned`，不写回 `common_core`。
- Python backend 不再是受支持运行时；任何残留 Python 资产仅作为历史参考或工具用途。
