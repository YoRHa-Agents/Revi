#!/usr/bin/env bash

revi_artifact_prefix() {
  echo "${REVI_ARTIFACT_PREFIX:-revi}"
}

revi_supported_platforms() {
  printf '%s\n' \
    "linux-x86_64" \
    "linux-aarch64" \
    "macos-aarch64"
}

revi_validate_target_platform() {
  local target
  target="$1"
  case "$target" in
    linux-x86_64 | linux-aarch64 | macos-aarch64)
      ;;
    *)
      echo "unsupported target platform: $target" >&2
      return 1
      ;;
  esac
}

revi_asset_name_for_target() {
  local target prefix
  target="$1"
  prefix="$(revi_artifact_prefix)"
  revi_validate_target_platform "$target"
  echo "${prefix}-${target}"
}

revi_manifest_name_for_target() {
  local target
  target="$1"
  revi_validate_target_platform "$target"
  echo "release-manifest-${target}.json"
}

revi_host_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os" in
    Linux)
      case "$arch" in
        x86_64) echo "linux-x86_64" ;;
        aarch64 | arm64) echo "linux-aarch64" ;;
        *) echo "unsupported Linux architecture: $arch" >&2; return 1 ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        arm64 | aarch64) echo "macos-aarch64" ;;
        *) echo "unsupported macOS architecture: $arch" >&2; return 1 ;;
      esac
      ;;
    *)
      echo "unsupported OS: $os" >&2
      return 1
      ;;
  esac
}

revi_release_repo() {
  echo "${REVI_RELEASE_REPO:-YoRHa-Agents/Revi}"
}

revi_release_tag() {
  echo "${REVI_RELEASE_TAG:-}"
}

revi_sha_cmd() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1"
  else
    shasum -a 256 "$1"
  fi
}

revi_write_release_manifest() {
  local release_dir asset_name target_platform
  release_dir="$1"
  asset_name="$2"
  target_platform="$3"

  python3 - <<'PY' "$release_dir" "$asset_name" "$target_platform" "$(revi_release_repo)" "$(revi_release_tag)" "$(revi_artifact_prefix)"
import json
import sys
from pathlib import Path

release_dir = Path(sys.argv[1])
asset_name = sys.argv[2]
platform = sys.argv[3]
repo = sys.argv[4]
tag = sys.argv[5]
prefix = sys.argv[6]

payload = {
    "release_repo": repo,
    "binary_name": "revi",
    "asset_name": asset_name,
    "asset_prefix": prefix,
    "platform": platform,
    "checksum_file": "SHA256SUMS",
    "frontend_bundle_dir": "frontend-dist",
    "release_tag": tag or "latest",
    "validation": {
        "supports_rust_runtime": True,
        "supports_hash_spa": True,
        "supports_frontend_bundle": True,
    },
}

(release_dir / "release-manifest.json").write_text(
    json.dumps(payload, indent=2) + "\n",
    encoding="utf-8",
)
(release_dir / f"release-manifest-{platform}.json").write_text(
    json.dumps(payload, indent=2) + "\n",
    encoding="utf-8",
)
PY
}

revi_validate_release_dir() {
  local release_dir asset_name manifest_name
  release_dir="$1"
  asset_name="$2"
  manifest_name="${3:-release-manifest.json}"

  if [[ ! -f "$release_dir/$asset_name" ]]; then
    echo "missing release asset: $release_dir/$asset_name" >&2
    return 1
  fi
  if [[ ! -f "$release_dir/SHA256SUMS" ]]; then
    echo "missing SHA256SUMS in $release_dir" >&2
    return 1
  fi
  if [[ ! -f "$release_dir/$manifest_name" ]]; then
    echo "missing manifest: $release_dir/$manifest_name" >&2
    return 1
  fi
  if [[ ! -d "$release_dir/frontend-dist" ]]; then
    echo "missing frontend-dist bundle in $release_dir" >&2
    return 1
  fi

  if ! grep -q "  $asset_name\$" "$release_dir/SHA256SUMS"; then
    echo "missing checksum entry for $asset_name" >&2
    return 1
  fi
}
