# Release Guide

## 目标
描述 Revi 在接入 base 后的 release 入口、产物生成方式与最小校验路径。

## 当前入口
```bash
make release
cargo run --manifest-path common/cli/Cargo.toml -- release
bash scripts/build-release.sh
bash scripts/validate-release.sh
```

## 产物
- Rust 二进制：`release/revi-<platform>`
- 前端构建：`release/frontend-dist/`
- checksum：`release/SHA256SUMS`
- manifest：`release-manifest.json` 与 `release-manifest-<platform>.json`

## 最小校验
```bash
bash scripts/build-release.sh
bash scripts/validate-release.sh
bash scripts/validate-rust-vue.sh
```

## 边界
- release 命名空间与产品产物仍保持 `revi`，不复用 `rustwebappcommon-*` 的产品命名。
- Pages 继续直接发布 `site/`；release script 只负责本地构建与基础 smoke。
- `.github/workflows/release-artifacts.yml` 会对 `linux-x86_64`、`linux-aarch64` 与 `macos-aarch64` 做多平台构建，并在 tag push 时发布 GitHub Release assets。
- 如需更深的多平台签名与升级验证，后续再评估是否上推为 common seam。
