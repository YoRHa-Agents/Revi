#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/release-contract.sh"

HOST_PLATFORM="$(revi_host_platform)"
TARGET_PLATFORM="${REVI_TARGET_PLATFORM:-$HOST_PLATFORM}"
RELEASE_DIR="${REVI_RELEASE_DIR:-$ROOT_DIR/release}"
FRONTEND_DIR="$ROOT_DIR/frontend"

revi_validate_target_platform "$TARGET_PLATFORM"

asset_name="$(revi_asset_name_for_target "$TARGET_PLATFORM")"

rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR/frontend-dist"

case "$TARGET_PLATFORM" in
  linux-x86_64)
    if [[ "$HOST_PLATFORM" == "linux-x86_64" ]]; then
      cargo build --manifest-path "$ROOT_DIR/server/Cargo.toml" --release --target x86_64-unknown-linux-musl
      cp "$ROOT_DIR/server/target/x86_64-unknown-linux-musl/release/revi" "$RELEASE_DIR/$asset_name"
    else
      echo "cannot build $TARGET_PLATFORM from host $HOST_PLATFORM" >&2
      exit 1
    fi
    ;;
  linux-aarch64)
    if [[ "$HOST_PLATFORM" == linux-* ]]; then
      cargo zigbuild --manifest-path "$ROOT_DIR/server/Cargo.toml" --release --target aarch64-unknown-linux-musl
      cp "$ROOT_DIR/server/target/aarch64-unknown-linux-musl/release/revi" "$RELEASE_DIR/$asset_name"
    else
      echo "linux-aarch64 builds require a Linux host with cargo-zigbuild" >&2
      exit 1
    fi
    ;;
  macos-aarch64)
    if [[ "$HOST_PLATFORM" == macos-aarch64 ]]; then
      cargo build --manifest-path "$ROOT_DIR/server/Cargo.toml" --release --target aarch64-apple-darwin
      cp "$ROOT_DIR/server/target/aarch64-apple-darwin/release/revi" "$RELEASE_DIR/$asset_name"
    else
      echo "macos-aarch64 builds require a macOS arm64 host" >&2
      exit 1
    fi
    ;;
esac

chmod +x "$RELEASE_DIR/$asset_name"

npm --prefix "$FRONTEND_DIR" run build
cp -R "$FRONTEND_DIR/dist/." "$RELEASE_DIR/frontend-dist/"

(
  cd "$RELEASE_DIR"
  revi_sha_cmd "$asset_name"
) > "$RELEASE_DIR/SHA256SUMS"

revi_write_release_manifest "$RELEASE_DIR" "$asset_name" "$TARGET_PLATFORM"
revi_validate_release_dir "$RELEASE_DIR" "$asset_name" "$(revi_manifest_name_for_target "$TARGET_PLATFORM")"

echo "release bundle prepared under $RELEASE_DIR"
