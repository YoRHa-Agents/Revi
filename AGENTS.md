# AGENTS
> Revi 采用 `RustWebAppCommon` 边界模型后的主 agent / subagent 入口。

## 先读顺序
1. `README.md`
2. `docs/index.md`
3. `doc_auto/base_adoption_boundary.md`
4. `doc_auto/implementation_handoff.md`

## 当前仓库边界
- `common/core/`：共享术语、route/docs/release/theme 等稳定契约。
- `common/adapters/`：site/docs/release 等可复用适配层，不承载 Revi 业务域模型。
- `common/cli/`：统一 `dev / demo / docs / release` 前门。
- `server/`：Revi app-owned review domain，包括 review items、comments、archive、export、upload、config、workspace scanner 与存储。
- `frontend/`：Revi Vue UI、state、review/archive 视图与 E2E/unit tests。
- `site/`、`docs/`、`demo/`：静态前门、用户文档与 storyboard。
- `doc_auto/`：接入 base 后的结构性变更真相源。

## 不要猜的内容
- 不要把 Revi 的 review 产品语义写回 `common_core`。
- 不要把 Python backend 当作受支持运行时。
- 不要假定 Pages 只服务静态 HTML，不需要考虑 `frontend/` 的 hash 路由和 Rust API。
- 不要假定 `common` 前门会替代 `revi` 二进制；`revi` 仍是产品 runtime。

## 当前工作规则
- 先冻结 `base` 与 `app_owned` 边界，再做代码迁移。
- 结构性代码改动后同步更新 `doc_auto/` 并追加修改时间。
- docs/site/guide/demo/README/agent docs 必须描述同一条 Rust + Vue 单轨运行链路。
- 修改 Rust 代码后运行 `cargo test --manifest-path server/Cargo.toml`。
- 修改前端或文档入口后至少运行 `cd frontend && npm test`。
- 修改 Playwright、Pages 或运行入口后运行 `cd frontend && npm run test:e2e` 或等价 smoke。
