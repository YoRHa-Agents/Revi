#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <slug>" >&2
  echo "  slug: kebab-case name, e.g. 'onboarding-flow'" >&2
  exit 1
fi

SLUG="$1"
WORKSPACE_DIR="${REVI_WORKSPACE:-${HOME:-$(cd "$(dirname "$0")/.." && pwd)}/.revi/workspace}"
DEST="${WORKSPACE_DIR}/prototypes/${SLUG}.html"
mkdir -p "$(dirname "$DEST")"

if [[ -f "$DEST" ]]; then
  echo "Error: $DEST already exists" >&2
  exit 1
fi

TITLE=$(echo "$SLUG" | tr '-' ' ' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')

cat > "$DEST" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>PLACEHOLDER_TITLE</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: system-ui, -apple-system, sans-serif; background: #f8fafc; color: #1e293b; }
    .app { max-width: 900px; margin: 0 auto; padding: 32px 24px; }
    h1 { font-size: 24px; font-weight: 800; margin-bottom: 8px; color: #0f172a; }
    .subtitle { font-size: 14px; color: #64748b; margin-bottom: 32px; }
    .step-nav { display: flex; gap: 8px; margin-bottom: 24px; flex-wrap: wrap; }
    .step-btn {
      padding: 8px 16px; border-radius: 8px; border: 1px solid #e2e8f0;
      background: white; cursor: pointer; font-size: 13px; font-weight: 500;
      color: #64748b; transition: all 0.15s;
    }
    .step-btn.active { background: #3b82f6; color: white; border-color: #3b82f6; }
    .step-btn:hover:not(.active) { border-color: #93c5fd; color: #3b82f6; }
    .step-content { background: white; border: 1px solid #e2e8f0; border-radius: 12px; padding: 32px; min-height: 300px; }
    .step-panel { display: none; }
    .step-panel.active { display: block; }
    .step-title { font-size: 18px; font-weight: 700; margin-bottom: 12px; }
    .step-desc { font-size: 14px; color: #64748b; line-height: 1.6; }
    .placeholder-box {
      margin-top: 24px; padding: 32px; border-radius: 8px;
      background: #f8fafc; border: 2px dashed #cbd5e1; text-align: center;
    }
    .placeholder-box p { color: #94a3b8; font-size: 14px; }
  </style>
</head>
<body>
<div class="app">
  <h1>PLACEHOLDER_TITLE</h1>
  <p class="subtitle">Interactive prototype — click steps to navigate</p>

  <div class="step-nav">
    <button class="step-btn active" onclick="showStep(0)">Step 1: Overview</button>
    <button class="step-btn" onclick="showStep(1)">Step 2: Flow</button>
    <button class="step-btn" onclick="showStep(2)">Step 3: Result</button>
  </div>

  <div class="step-content">
    <div class="step-panel active" id="step-0">
      <h2 class="step-title">Step 1: Overview</h2>
      <p class="step-desc">Describe what happens in this step. Add interactive elements, screenshots, or wireframes as needed.</p>
      <div class="placeholder-box">
        <p>🖼️ Add your step 1 content here</p>
        <p>Replace these panels with actual prototype content</p>
      </div>
    </div>

    <div class="step-panel" id="step-1">
      <h2 class="step-title">Step 2: Flow</h2>
      <p class="step-desc">Describe the user flow in this step.</p>
      <div class="placeholder-box">
        <p>🖼️ Add your step 2 content here</p>
      </div>
    </div>

    <div class="step-panel" id="step-2">
      <h2 class="step-title">Step 3: Result</h2>
      <p class="step-desc">Show the final state or outcome.</p>
      <div class="placeholder-box">
        <p>🖼️ Add your step 3 content here</p>
      </div>
    </div>
  </div>
</div>

<script>
  function showStep(n) {
    document.querySelectorAll('.step-panel').forEach((p, i) => {
      p.classList.toggle('active', i === n)
    })
    document.querySelectorAll('.step-btn').forEach((b, i) => {
      b.classList.toggle('active', i === n)
    })
  }
</script>
</body>
</html>
HTMLEOF

# Replace the placeholder title with the actual title
sed -i.bak "s/PLACEHOLDER_TITLE/${TITLE}/g" "$DEST"
rm "${DEST}.bak"

echo "Created: $DEST"
