# Base Adoption Boundary

## 目标
明确 Revi 接入 `RustWebAppCommon` 之后，哪些能力复用 base，哪些能力继续保持为 Revi `app_owned`。

## Base 提供的能力
- `common/core/`：`WorkspaceIdentity`、`RouteDescriptor`、`DocsNode`、`ThemeTokenSet`、`ReleaseDescriptor` 等共享契约。
- `common/adapters/`：site/demo/docs manifest 生成与 release script 接缝。
- `common/cli/`：`dev / demo / docs / release` 的统一前门。
- `AGENTS.md` + `doc_auto/`：结构性变更的真相源与 handoff 协议。

## Revi app-owned 能力
- `server/crates/revi-server/src/router.rs` 中的 reviews/comments/archive/export/upload/config 路由。
- `server/crates/revi-server/src/models.rs` 中的 `ReviewItem`、`Reference`、comment/export/config 线协议。
- `server/crates/revi-server/src/storage/` 与 `workspace/` 中的产品存储与扫描逻辑。
- `frontend/` 中的 plan/design/prototype review UI、comment panel、archive 与 agent export preview。
- `site/` 中的 Revi landing/demo/guide 产品叙事。

## 明确不进入 common 的内容
- Revi 的 anchored comments、archive workflow、agent export schema、workspace item 分类与业务路由。
- 任何 Python backend 的残留语义。
- 针对单个 review item 的产品级 UX 命名、文案和行为。

## 当前接入拓扑
- Revi 保持独立产品仓库。
- `common/` 直接 vendored 到 Revi 仓库根目录，作为基座实现与统一命令前门。
- `server/` 通过 path dependency 复用 `common_core`，并在 `server/crates/revi-server/src/base.rs` 中把 Revi 产品元数据映射到 common 契约。
- `frontend/`、`site/`、`docs/`、`.github/workflows/` 继续留在 Revi 仓库中，但改为描述和验证 Rust + Vue 单轨路径。

## 当前已落地的共用接缝
- `common/core/`：已落地 shared contracts，并被 `revi_server::base` 使用。
- `common/adapters/`：已落地 site/docs manifest 与 release script 适配层。
- `common/cli/`：已落地 `dev / demo / docs / release` 统一前门。
- `site/assets/route-manifest.json`、`site/assets/runtime-contract.json`、`site/guide/docs-index.json`：已可由 `common demo/docs` 生成。
- 文档与站点入口已对齐到 Rust-only 运行时，上传 API 说明也已与 Rust 实现的 `POST /api/upload` 保持一致。

## Last Updated
- 2026-04-02T11:01:35+00:00
