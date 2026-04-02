#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <slug>" >&2
  echo "  slug: kebab-case name, e.g. 'sprint-2-architecture'" >&2
  exit 1
fi

SLUG="$1"
WORKSPACE_DIR="${REVI_WORKSPACE:-${HOME:-$(cd "$(dirname "$0")/.." && pwd)}/.revi/workspace}"
DEST="${WORKSPACE_DIR}/plans/${SLUG}.md"
mkdir -p "$(dirname "$DEST")"

if [[ -f "$DEST" ]]; then
  echo "Error: $DEST already exists" >&2
  exit 1
fi

TITLE=$(echo "$SLUG" | tr '-' ' ' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')

cat > "$DEST" << EOF
# ${TITLE}

## Overview

TODO: Describe the purpose and context of this plan.

## Scope

The key flows this plan covers:
1. **Flow 1** — description
2. **Flow 2** — description

## Data Models

TODO: Describe the key data structures.

## Implementation Steps

- [ ] Step 1
- [ ] Step 2
- [ ] Step 3

## Open Questions

- Question 1?
- Question 2?
EOF

echo "Created: $DEST"
