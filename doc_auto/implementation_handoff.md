# Implementation Handoff

## 当前已确认的基线
- Revi 的受支持运行时收敛为 Rust backend + Vue frontend。
- `common/` 作为 vendored base 进入 Revi 仓库，并通过 `server/crates/revi-server/src/base.rs` 建立 Revi app-owned 到 common contracts 的映射。
- `common` 前门负责 `dev / demo / docs / release` 语义；`revi` 二进制继续负责产品 API 与 workspace/data runtime。
- `doc_auto/` 与 `AGENTS.md` 现在是结构性变更的真相源入口。
- Playwright 已切换为 Rust server 启动路径，并使用 `testdata/e2e/` 复制出的测试 workspace/data，而不再启动 Python `uvicorn`。

## 当前待关闭项
- Revi 是否需要把更多通用 release/Pages/test seam 回推到 `RustWebAppCommon`。
- `site/` 是否进一步消费 `common demo/docs` 生成的 JSON manifests。
- 如需更深 release contract（多平台签名、升级校验），仍需后续迭代。

## 建议验证命令
```bash
cargo test --manifest-path server/Cargo.toml
cargo test --manifest-path common/core/Cargo.toml
cargo test --manifest-path common/adapters/Cargo.toml
cargo test --manifest-path common/cli/Cargo.toml
cd frontend && npm test
cd frontend && npm run test:e2e
bash scripts/validate-rust-vue.sh
cargo run --manifest-path common/cli/Cargo.toml -- demo
cargo run --manifest-path common/cli/Cargo.toml -- docs
cargo run --manifest-path common/cli/Cargo.toml -- release
```

## 当前已验证结果
1. `cargo test --manifest-path server/Cargo.toml`：通过。
2. `cargo test --manifest-path common/core/Cargo.toml`：通过。
3. `cargo test --manifest-path common/adapters/Cargo.toml`：通过。
4. `cargo test --manifest-path common/cli/Cargo.toml`：通过。
5. `cd frontend && npm test`：通过。
6. `cd frontend && npm run test:e2e`：通过，且 Rust server 已替代 Python webServer。
7. `bash scripts/validate-rust-vue.sh`：通过。
8. `cargo run --manifest-path common/cli/Cargo.toml -- demo` 与 `docs`：通过，已生成 site/docs manifests。
9. `cargo run --manifest-path common/cli/Cargo.toml -- release`：通过，已联通 release script、Rust release build 与前端 build。
10. Rust 测试辅助已清理低价值 `dead_code` 警告；第二轮文档收口后，upload API 说明与 Rust 真实接口一致。

## Last Updated
- 2026-04-02T11:01:35+00:00
