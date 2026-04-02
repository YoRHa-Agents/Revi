#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <slug>" >&2
  echo "  slug: kebab-case name, e.g. 'dashboard-v2'" >&2
  exit 1
fi

SLUG="$1"
WORKSPACE_DIR="${REVI_WORKSPACE:-${HOME:-$(cd "$(dirname "$0")/.." && pwd)}/.revi/workspace}"
DEST="${WORKSPACE_DIR}/designs/${SLUG}.svg"
mkdir -p "$(dirname "$DEST")"

if [[ -f "$DEST" ]]; then
  echo "Error: $DEST already exists" >&2
  exit 1
fi

TITLE=$(echo "$SLUG" | tr '-' ' ' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')

cat > "$DEST" << EOF
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600" viewBox="0 0 800 600">
  <!-- ${TITLE} Design Placeholder -->
  <rect width="800" height="600" fill="#f8fafc"/>
  <rect x="20" y="20" width="760" height="560" rx="12" fill="white" stroke="#e2e8f0" stroke-width="2"/>

  <!-- Header -->
  <rect x="20" y="20" width="760" height="60" rx="12" fill="#f1f5f9"/>
  <text x="40" y="57" font-family="system-ui,sans-serif" font-size="20" font-weight="700" fill="#1e293b">${TITLE}</text>
  <text x="40" y="80" font-family="system-ui,sans-serif" font-size="13" fill="#64748b">Design placeholder — replace with actual mockup</text>

  <!-- Placeholder content area -->
  <rect x="40" y="100" width="720" height="460" rx="8" fill="#f8fafc" stroke="#e2e8f0" stroke-width="1.5" stroke-dasharray="8 4"/>
  <text x="400" y="320" text-anchor="middle" font-family="system-ui,sans-serif" font-size="16" fill="#94a3b8">📐 Add your design here</text>
  <text x="400" y="345" text-anchor="middle" font-family="system-ui,sans-serif" font-size="13" fill="#cbd5e1">Replace this SVG file with your actual design</text>
</svg>
EOF

echo "Created: $DEST"
