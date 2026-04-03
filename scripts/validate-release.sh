#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/release-contract.sh"

TARGET_PLATFORM="${REVI_TARGET_PLATFORM:-$(revi_host_platform)}"
ASSET_NAME="$(revi_asset_name_for_target "$TARGET_PLATFORM")"
MANIFEST_NAME="$(revi_manifest_name_for_target "$TARGET_PLATFORM")"

bash "$ROOT_DIR/scripts/build-release.sh"
revi_validate_release_dir "$ROOT_DIR/release" "$ASSET_NAME" "$MANIFEST_NAME"

echo "release validation passed for $ASSET_NAME"
