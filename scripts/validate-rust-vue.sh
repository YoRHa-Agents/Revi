#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo test --manifest-path "$ROOT_DIR/server/Cargo.toml"
cargo test --manifest-path "$ROOT_DIR/common/core/Cargo.toml"
cargo test --manifest-path "$ROOT_DIR/common/adapters/Cargo.toml"
cargo test --manifest-path "$ROOT_DIR/common/cli/Cargo.toml"
npm --prefix "$ROOT_DIR/frontend" test

echo "rust + vue validation passed"
