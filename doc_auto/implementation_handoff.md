# Implementation Handoff

## 当前已确认的基线
- Revi 的受支持运行时收敛为 Rust backend + Vue frontend。
- `common/` 作为 vendored base 进入 Revi 仓库，并通过 `server/crates/revi-server/src/base.rs` 建立 Revi app-owned 到 common contracts 的映射。
- `common` 前门负责 `dev / demo / docs / release` 语义；`revi` 二进制继续负责产品 API 与 workspace/data runtime。
- `doc_auto/` 与 `AGENTS.md` 现在是结构性变更的真相源入口。
- Playwright 已切换为 Rust server 启动路径，并使用 `testdata/e2e/` 复制出的测试 workspace/data，而不再启动 Python `uvicorn`。
- Revi 支持无 workspace 启动：`Config.workspace_path` 现为 `Option<PathBuf>`，不提供 `--workspace` 参数时服务器正常启动但返回空列表；前端展示 workspace 选择器页面，支持设定本地路径或连接远程 Revi 服务器。`PATCH /api/config` 现在热更新运行时 Config 和 WorkspaceScanner（通过 `RwLock`），无需重启。`/workspace/*` 静态文件服务改为动态 handler 以跟随配置变更。

## 2026-04-05 深度审查后的结构性变更

### 安全加固
- Upload handler：添加 filename 净化（`Path::file_name()` 提取纯文件名，拒绝路径穿越）。
- Upload handler：添加 50MB 文件大小上限（`MAX_UPLOAD_SIZE`），超限返回 `BadRequest`。
- `serve_workspace_file`：添加 dotfile 阻止（任何路径分段以 `.` 开头返回 403），防止 `.git`/`.env` 泄露。
- `serve_workspace_file`：区分错误类型（`NotFound` → 404, `PermissionDenied` → 403, 其他 → 500），不再统一 404。
- Frontend：添加 DOMPurify 对 `marked.parse()` 输出的 HTML 净化，防止 XSS。
- `api.upload`：修复 `res.ok` 缺失的错误处理，与 `request()` 保持一致。

### 可靠性加固
- 全部 14 处 `RwLock`/`Mutex` 裸 `.unwrap()` 已替换为 `.map_err()` + `AppError::Internal` 或描述性 `.expect()`，消除 lock poisoning 导致的 panic 传播。
- Frontend：添加 `app.config.errorHandler` 全局错误捕获。
- Frontend：`CommentPanel` 的 submit/resolve/archive 操作添加 try/catch 和用户可见错误提示。
- Frontend：`MarkdownViewer` 添加 `sel.rangeCount` 防护。

### 测试覆盖提升
- Rust server：新增 `test_config.rs`（8 tests）、`test_workspace.rs`（7 tests），增强 `test_upload.rs`（5 tests）。总计 79 integration tests。
- Frontend unit：新增 9 个组件/视图测试文件（66 tests），line coverage 83.7%。已配置 `@vitest/coverage-v8`。总计 118 unit tests。
- E2E：新增 `workspace-setup.spec.js`、`upload.spec.js`、`responsive.spec.js`（31 tests）。总计 56 E2E tests。
- Clippy：server + common 全部 clean（`-D warnings`）。

### UI/UX 优化
- CSS 对比度：5 个变量调整 lightness 达到 WCAG AA 4.5:1（`--text-dim`、`--text-faint`、`--accent`、`--green`、`--amber`）。
- A11y：添加 skip-link、`role`/`aria-*` 属性覆盖全部交互组件（sidebar/tabs/dialog/pins/steps）、`focus-visible` 全局样式。
- 响应式：全部 10 个组件/视图添加 `@media` 断点（768px/480px），sidebar 改为移动端 overlay，dashboard 网格自适应，review 视图内容/评论切换。
- 触控适配：`@media (hover: none)` 禁用 hover-only 效果。

### CI 优化
- `ci.yml`：移除冗余 `validate-rust-vue.sh` 最终步骤，新增 `cargo clippy`/`cargo fmt --check` 步骤。
- `release-artifacts.yml`：`linux-x86_64` 矩阵不再安装 zigbuild；权限收紧为 `contents: read` 默认 + `publish-release` 专属 `contents: write`。

### 清理
- 移除 `server/Cargo.toml` 中未使用的 `reqwest` workspace dependency。
- `<html lang>` 现跟随 i18n locale 切换（`en` / `zh-CN`）。

## 当前待关闭项
- Revi 是否需要把更多通用 release/Pages/test seam 回推到 `RustWebAppCommon`。
- `site/` 是否进一步消费 `common demo/docs` 生成的 JSON manifests。
- 如需更深 release contract（多平台签名、升级校验），仍需后续迭代。
- `MetadataStore::set_type` 的 read-modify-write 无序列化保护（P1，低频操作，后续评估）。
- `serve_workspace_file` 的 TOCTOU 窗口（P1，需要 `O_NOFOLLOW` 或 single-syscall 读取方案）。

## 建议验证命令
```bash
cargo test --manifest-path server/Cargo.toml
cargo clippy --manifest-path server/Cargo.toml -- -D warnings
cargo test --manifest-path common/core/Cargo.toml
cargo test --manifest-path common/adapters/Cargo.toml
cargo test --manifest-path common/cli/Cargo.toml
cd frontend && npm test
cd frontend && npx vitest run --coverage
cd frontend && npm run test:e2e
bash scripts/validate-rust-vue.sh
REVI_TARGET_PLATFORM=linux-x86_64 bash scripts/build-release.sh
```

## 当前已验证结果
1. `cargo test --manifest-path server/Cargo.toml`：通过（79 tests）。
2. `cargo clippy --manifest-path server/Cargo.toml -- -D warnings`：通过（0 warnings）。
3. `cargo test --manifest-path common/core/Cargo.toml`：通过。
4. `cargo test --manifest-path common/adapters/Cargo.toml`：通过。
5. `cargo test --manifest-path common/cli/Cargo.toml`：通过。
6. `cd frontend && npm test`：通过（118 tests，12 files）。
7. `cd frontend && npx vitest run --coverage`：通过（83.7% line coverage）。
8. `cd frontend && npm run test:e2e`：测试文件已创建（56 tests），需 Rust server 启动。
9. `REVI_TARGET_PLATFORM=linux-x86_64 bash scripts/build-release.sh`：通过，产物 `release/revi-linux-x86_64`（3.5MB），SHA256SUMS 校验通过，manifest 正确。
10. `bash scripts/validate-rust-vue.sh`：通过。

## Last Updated
- 2026-04-05T06:30:00+00:00
