# Development Guide

## 目标
统一 Revi 在接入 base 之后的 Rust + Vue 开发入口、site/docs 资产生成与最小验证路径。

## 当前命令语义
```bash
make dev
make dev-rust
make demo
make docs
make release
```

## 细化命令
```bash
cargo run --manifest-path common/cli/Cargo.toml -- dev --surface web --host 127.0.0.1 --port 5173
cargo run --manifest-path common/cli/Cargo.toml -- demo
cargo run --manifest-path common/cli/Cargo.toml -- docs
cargo run --manifest-path common/cli/Cargo.toml -- release
cargo run --manifest-path server/Cargo.toml --bin revi -- --port 8000
cd frontend && npm run dev -- --host 127.0.0.1 --port 5173
```

## 验证
```bash
cargo test --manifest-path server/Cargo.toml
cargo test --manifest-path common/cli/Cargo.toml
cd frontend && npm test
cd frontend && npm run test:e2e
bash scripts/validate-rust-vue.sh
```

## 说明
- `common dev` 会先生成 Revi 的 base manifests，再输出 Rust server 与 Vue frontend 的推荐启动命令。
- `common demo` 会刷新 `site/assets/route-manifest.json`、`site/assets/theme-tokens.json` 与 `site/assets/runtime-contract.json`。
- `common docs` 会刷新 `site/guide/docs-index.json`，作为 docs/front-door 的静态索引。
- `revi` 二进制继续承载 review API、workspace/data 配置与 `/workspace` 静态挂载。
- Playwright 与日常开发都统一以 Rust backend 为唯一受支持 API runtime。
