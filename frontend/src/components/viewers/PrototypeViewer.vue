<template>
  <div class="proto-viewer">
    <!-- Left: step nav -->
    <div class="steps-nav">
      <div class="nav-header">
        <div class="nav-title">Review Flow</div>
        <div class="nav-sub">Interactive prototype · {{ steps.length }} steps</div>
      </div>

      <div class="steps-list">
        <button
          v-for="(step, i) in steps"
          :key="i"
          class="step-item"
          :class="{ active: current === i, done: current > i, hovered: isHoveredStep(i) }"
          @click="current = i"
        >
          <div class="step-circle">
            <span v-if="current > i" class="step-check">✓</span>
            <span v-else>{{ i + 1 }}</span>
          </div>
          <div class="step-info">
            <div class="step-name">{{ step.title }}</div>
            <div class="step-role">{{ step.role }}</div>
          </div>
          <button class="step-anchor-btn" @click.stop="emitStepAnchor(i)" title="Comment on this step">💬</button>
        </button>
      </div>

      <div class="nav-btns">
        <button class="nbtn" @click="current = Math.max(0, current - 1)" :disabled="current === 0">← Back</button>
        <button class="nbtn next" @click="current = Math.min(steps.length - 1, current + 1)" :disabled="current === steps.length - 1">Next →</button>
      </div>
    </div>

    <!-- Right: step content -->
    <div class="step-content">
      <!-- Mini UI visual -->
      <div class="step-visual"
        @mousedown="onPanStart"
        @mousemove="onPanMove"
        @mouseup="onPanEnd"
        @mouseleave="onPanEnd"
      >

        <!-- Zoom controls -->
        <div class="zoom-bar">
          <button class="zoom-btn" @click="zoomOut" :disabled="zoom <= ZOOM_MIN">−</button>
          <span class="zoom-label" @click="zoomReset" title="Reset zoom">{{ Math.round(zoom * 100) }}%</span>
          <button class="zoom-btn" @click="zoomIn"  :disabled="zoom >= ZOOM_MAX">+</button>
        </div>

        <!-- Step 0: Discover -->
        <div v-if="current === 0" class="visual" :style="visualStyle">
          <div class="vis-app">
            <div class="v-topbar"><div class="v-brand"><span class="v-logo">R</span> Revi</div><div class="v-lang">EN / 中文</div></div>
            <div class="v-body">
              <div class="v-sidebar">
                <div class="v-slabel">WORKSPACE</div>
                <div class="v-row active"><span class="v-dot plan"></span><span class="v-fn">Sprint 1 — Design</span><span class="v-rb">2</span></div>
                <div class="v-row"><span class="v-dot design"></span><span class="v-fn">UI Mockup v1</span><span class="v-rb">1</span></div>
                <div class="v-row"><span class="v-dot proto"></span><span class="v-fn">Review Flow Prototype</span><span class="v-rb">1</span></div>
              </div>
              <div class="v-main">
                <div class="v-hero"><div class="v-ht">Welcome to Revi</div><div class="v-hb">R</div></div>
                <div class="v-stats">
                  <div class="v-stat"><span class="v-sn">3</span><span class="v-sl">Items</span></div>
                  <div class="v-stat red"><span class="v-sn">4</span><span class="v-sl">Open</span></div>
                  <div class="v-stat grn"><span class="v-sn">2</span><span class="v-sl">Resolved</span></div>
                </div>
                <div class="v-cards">
                  <div class="v-card"><div class="v-cstripe plan"></div><div class="v-cbody"><span class="v-cbadge plan">PLAN</span><div class="v-ctitle">Sprint 1</div><div class="v-copen">2 open</div><div class="v-cta highlight">Open →</div></div></div>
                  <div class="v-card"><div class="v-cstripe design"></div><div class="v-cbody"><span class="v-cbadge design">DESIGN</span><div class="v-ctitle">UI Mockup v1</div><div class="v-copen">1 open</div><div class="v-cta">Open →</div></div></div>
                  <div class="v-card"><div class="v-cstripe proto"></div><div class="v-cbody"><span class="v-cbadge proto">PROTO</span><div class="v-ctitle">Review Flow</div><div class="v-copen">1 open</div><div class="v-cta">Open →</div></div></div>
                </div>
              </div>
            </div>
          </div>
          <div class="v-cursor" style="right:62px;bottom:64px">👆</div>
        </div>

        <!-- Step 1: Review -->
        <div v-else-if="current === 1" class="visual" :style="visualStyle">
          <div class="vis-app">
            <div class="v-topbar"><div class="v-brand"><span class="v-logo">R</span> Revi</div><div class="v-tbtns"><div class="v-tbbtn act">☰</div><div class="v-tbbtn">⌕</div></div><div class="v-lang">EN / 中文</div></div>
            <div class="v-body">
              <div class="v-sidebar" style="width:72px">
                <div class="v-row active"><span class="v-dot plan"></span><span class="v-fn">Sprint 1</span><span class="v-rb">2</span></div>
                <div class="v-row"><span class="v-dot design"></span><span class="v-fn">UI Mockup</span></div>
              </div>
              <div class="v-idx">
                <div class="v-idxl">INDEX</div>
                <div class="v-idxi l1">Overview</div>
                <div class="v-idxi l2">Scope</div>
                <div class="v-idxi l1">Data Models</div>
                <div class="v-idxi l1 active">Storage</div>
                <div class="v-idxi l1">API</div>
              </div>
              <div class="v-mdc">
                <div class="v-mdh1">Sprint 1 — System Design</div>
                <div class="v-mdh2">## Overview</div>
                <div class="v-mdp">Revi is an agent-human coworking review service. Architecture for MVP.</div>
                <div class="v-mdh2">## Data Models</div>
                <div class="v-mdp">Each review item is auto-discovered from the workspace/ directory.</div>
              </div>
              <div class="v-cp">
                <div class="v-cptabs"><span class="v-cpt act">Open (2)</span><span class="v-cpt">Resolved (1)</span></div>
                <div class="v-cpcard"><div class="v-cpm"><b>Alice</b></div><div class="v-cpt2">Should support SQLite from start.</div></div>
                <div class="v-cpcard"><div class="v-cpm"><b>Agent-Planner</b></div><div class="v-cpt2">Missing schema_version field.</div></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: Comment -->
        <div v-else-if="current === 2" class="visual" :style="visualStyle">
          <div class="vis-app">
            <div class="v-topbar"><div class="v-brand"><span class="v-logo">R</span> Sprint 1 — System Design</div></div>
            <div class="v-body">
              <div class="v-mdc" style="flex:1">
                <div class="v-mdh1">Sprint 1 — System Design</div>
                <div class="v-mdh2">## Agent Export Format</div>
                <div class="v-mdp">Returns structured JSON with open_comments array.</div>
                <div class="v-mdh2 hl">## API Endpoints</div>
                <div class="v-mdp hl">GET /api/export/{id} · POST /api/comments/{id}</div>
              </div>
              <div class="v-cp">
                <div class="v-cptabs"><span class="v-cpt act">Open (2)</span><span class="v-cpt">Resolved (1)</span></div>
                <div class="v-cta-btn">+ Add Comment</div>
                <div class="v-form">
                  <div class="v-field"><div class="v-flabel">Author</div><div class="v-finput filled">Bob</div></div>
                  <div class="v-field"><div class="v-flabel">Reference</div><div class="v-finput filled">## API Endpoints</div></div>
                  <div class="v-field"><div class="v-flabel">Comment</div><div class="v-ftextarea">PATCH endpoint for resolving comments is missing from the spec.</div></div>
                  <div class="v-submit">Submit</div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Resolve -->
        <div v-else-if="current === 3" class="visual" :style="visualStyle">
          <div class="vis-app">
            <div class="v-topbar"><div class="v-brand"><span class="v-logo">R</span> Sprint 1 — System Design</div></div>
            <div class="v-body">
              <div class="v-mdc" style="flex:1">
                <div class="v-mdh1">Sprint 1 — System Design</div>
                <div class="v-mdh2">## Data Models</div>
                <div class="v-mdp">Comment schema with id, author, content, reference, status fields.</div>
              </div>
              <div class="v-cp">
                <div class="v-cptabs"><span class="v-cpt">Open (2)</span><span class="v-cpt act">Resolved (2)</span></div>
                <div class="v-archive-btn">Archive resolved (2)</div>
                <div class="v-cpcard resolved"><div class="v-cpm"><b>Bob</b><span class="v-green">✓ resolved</span></div><div class="v-cpr">section · ## Data Models</div><div class="v-cpt2">Add lang field for bilingual items.</div></div>
                <div class="v-cpcard resolved"><div class="v-cpm"><b>Alice</b><span class="v-green">✓ resolved</span></div><div class="v-cpr">section · ## Storage Layer</div><div class="v-cpt2">Confirmed SQLite on roadmap.</div></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Export -->
        <div v-else-if="current === 4" class="visual" :style="visualStyle">
          <div class="vis-app">
            <div class="v-topbar"><div class="v-brand"><span class="v-logo">R</span> Sprint 1 — System Design</div></div>
            <div class="v-body">
              <div class="v-mdc" style="flex:1">
                <div class="v-mdh1">Sprint 1 — System Design</div>
                <div class="v-mdp">Architecture and data model for the Revi MVP.</div>
              </div>
              <div class="v-cp">
                <div class="v-cptabs"><span class="v-cpt act">Open (2)</span><span class="v-cpt">Resolved (0)</span></div>
                <div class="v-export-block">
                  <div class="v-exp-label">▾ Agent Export Preview</div>
                  <div class="v-exp-json">{{ exportJson }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Step description -->
      <div class="step-desc">
        <div class="step-progress">
          <div class="prog-dots">
            <div v-for="(_, i) in steps" :key="i" class="prog-dot" :class="{ active: i === current, done: i < current }"></div>
          </div>
          <span class="prog-label">{{ current + 1 }} / {{ steps.length }}</span>
        </div>
        <h2 class="desc-title">{{ steps[current].title }}</h2>
        <p class="desc-body">{{ steps[current].description }}</p>
        <ul class="desc-points">
          <li v-for="p in steps[current].points" :key="p">{{ p }}</li>
        </ul>
        <div class="actor-badge">
          <span class="actor-icon">{{ steps[current].actorIcon }}</span>
          {{ steps[current].actor }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
const props = defineProps({
  item:          { type: Object, required: true },
  hoveredAnchor: { type: Object, default: null },
})
const emit = defineEmits(['anchor-select'])

const current = ref(0)
const zoom    = ref(1)
const panX    = ref(0)
const panY    = ref(0)
const isDragging = ref(false)
let dragStart = { x: 0, y: 0, panX: 0, panY: 0 }

const ZOOM_STEP = 0.25
const ZOOM_MIN  = 0.5
const ZOOM_MAX  = 3

function zoomIn()    { zoom.value = Math.min(ZOOM_MAX, +(zoom.value + ZOOM_STEP).toFixed(2)) }
function zoomOut()   { zoom.value = Math.max(ZOOM_MIN, +(zoom.value - ZOOM_STEP).toFixed(2)) }
function zoomReset() { zoom.value = 1; panX.value = 0; panY.value = 0 }

watch(current, () => { zoom.value = 1; panX.value = 0; panY.value = 0 })

function onPanStart(e) {
  if (e.button !== 0 || e.target.closest('.zoom-bar')) return
  isDragging.value = true
  dragStart = { x: e.clientX, y: e.clientY, panX: panX.value, panY: panY.value }
}
function onPanMove(e) {
  if (!isDragging.value) return
  panX.value = dragStart.panX + (e.clientX - dragStart.x)
  panY.value = dragStart.panY + (e.clientY - dragStart.y)
}
function onPanEnd() { isDragging.value = false }

const visualStyle = computed(() => ({
  transform: `translate(${panX.value}px, ${panY.value}px) scale(${zoom.value})`,
  transition: isDragging.value ? 'none' : 'transform 0.15s ease',
  cursor: isDragging.value ? 'grabbing' : 'grab',
}))

function emitStepAnchor(i) {
  emit('anchor-select', { type: 'step', value: i, label: steps[i].title })
}
function isHoveredStep(i) {
  return props.hoveredAnchor?.type === 'step' && Number(props.hoveredAnchor?.value) === i
}

const exportJson = `{
  "schema_version": "1.0",
  "item": {
    "id": "plans/sprint-1-design",
    "type": "plan",
    "title": "Sprint 1 — System Design"
  },
  "summary": { "open": 2, "resolved": 0 },
  "open_comments": [
    {
      "id": "c1",
      "author": "Alice",
      "reference": { "type": "section",
        "value": "## Storage Layer" },
      "content": "Should support SQLite."
    },
    {
      "id": "c2",
      "author": "Agent-Planner",
      "content": "Missing schema_version."
    }
  ],
  "exported_at": "2026-03-28T10:00:00Z"
}`

const steps = [
  {
    title: 'Discover Items',
    role: 'Browse workspace',
    actor: 'Human Reviewer',
    actorIcon: '👤',
    description: 'Open the dashboard to see all review items — plans, designs, and prototypes — grouped in the sidebar. Open comment counts are shown as red badges.',
    points: [
      'Items auto-grouped by type in sidebar',
      'Dashboard shows aggregate stats (open, resolved, archived)',
      'Click any item card or sidebar entry to open it',
    ],
  },
  {
    title: 'Open for Review',
    role: 'Split view',
    actor: 'Human Reviewer',
    actorIcon: '👤',
    description: 'The review view has a three-pane layout: collapsible document index on the left, markdown/design/prototype content in the center, and the comment panel on the right.',
    points: [
      'Index panel shows heading hierarchy (h1–h3)',
      'Click any heading to smooth-scroll to it',
      '⌕ Search button enables in-page full-text search',
    ],
  },
  {
    title: 'Add a Comment',
    role: 'Leave feedback',
    actor: 'Human or Agent',
    actorIcon: '💬',
    description: 'Click "+ Add Comment" to open the form. Optionally reference a specific section heading — this will show as a tag on the comment card so reviewers know what part you\'re addressing.',
    points: [
      'Reference field: e.g. "## API Endpoints" links to the section',
      'Author field identifies who left the comment',
      'Comments appear instantly (in-memory state)',
    ],
  },
  {
    title: 'Resolve & Archive',
    role: 'Close feedback loop',
    actor: 'Human Reviewer',
    actorIcon: '✅',
    description: 'Once a comment is addressed, mark it resolved to move it to the Resolved tab. When a review round is done, archive all resolved comments in one click — keeping the workspace clean.',
    points: [
      'Resolved tab shows all addressed feedback',
      '"Archive resolved" bundles them into a timestamped batch',
      'Archive history is preserved and browsable',
    ],
  },
  {
    title: 'Export to Agent',
    role: 'Agent handoff',
    actor: 'AI Agent',
    actorIcon: '🤖',
    description: 'Expand "Agent Export Preview" to see the structured JSON that AI agents receive. The export includes only open comments, schema version, and item metadata — everything the agent needs to act on feedback.',
    points: [
      'schema_version enables forward-compatible parsing',
      'Only open comments are exported (resolved = done)',
      'reference field tells the agent which section to update',
    ],
  },
]
</script>

<style scoped>
.proto-viewer {
  height: 100%; display: flex; overflow: hidden;
  background: #fff; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

/* Left nav */
.steps-nav {
  width: 200px; flex-shrink: 0;
  background: #fafafa; border-right: 1px solid #e5e7eb;
  display: flex; flex-direction: column; overflow: hidden;
}
.nav-header { padding: 16px 14px 12px; border-bottom: 1px solid #e5e7eb; flex-shrink: 0; }
.nav-title { font-size: 13px; font-weight: 700; color: #1a1a1a; }
.nav-sub   { font-size: 11px; color: #9ca3af; margin-top: 2px; }

.steps-list { flex: 1; overflow-y: auto; padding: 10px 0; display: flex; flex-direction: column; gap: 2px; }
.step-item {
  display: flex; align-items: center; gap: 9px;
  padding: 8px 12px; background: none; border: none; cursor: pointer;
  text-align: left; transition: background 0.1s; border-radius: 0;
}
.step-item:hover  { background: #ededf0; }
.step-item.active { background: #e8f0fe; }
.step-item.done   { opacity: 0.6; }
.step-item.hovered { background: #eff6ff; box-shadow: inset 0 0 0 2px #bfdbfe; }
.step-item.hovered .step-name { color: #1d4ed8; }

.step-circle {
  width: 22px; height: 22px; border-radius: 50%; flex-shrink: 0;
  display: flex; align-items: center; justify-content: center;
  font-size: 11px; font-weight: 700;
  background: #e5e7eb; color: #6b7280;
  transition: background 0.15s, color 0.15s;
}
.step-item.active .step-circle { background: #3b82f6; color: #fff; }
.step-item.done   .step-circle { background: #10b981; color: #fff; }
.step-check { font-size: 10px; }
.step-anchor-btn {
  flex-shrink: 0; width: 20px; height: 20px; border: none; background: none;
  cursor: pointer; font-size: 12px; border-radius: 4px; padding: 0;
  opacity: 0; transition: opacity 0.15s; display: flex; align-items: center; justify-content: center;
}
.step-item:hover .step-anchor-btn { opacity: 1; }
.step-anchor-btn:hover { background: #dbeafe; }
.step-info { flex: 1; min-width: 0; }
.step-name { font-size: 12px; font-weight: 600; color: #1a1a1a; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.step-item.active .step-name { color: #1d4ed8; }
.step-role { font-size: 11px; color: #9ca3af; margin-top: 1px; }

.nav-btns {
  display: flex; gap: 6px; padding: 10px 12px;
  border-top: 1px solid #e5e7eb; flex-shrink: 0;
}
.nbtn {
  flex: 1; padding: 7px 0; border-radius: 7px; border: 1px solid #e5e7eb;
  background: #fff; color: #6b7280; font-size: 12px; font-weight: 500;
  cursor: pointer; transition: all 0.15s;
}
.nbtn:hover:not(:disabled) { background: #f3f4f6; color: #1a1a1a; border-color: #d1d5db; }
.nbtn:disabled { opacity: 0.35; cursor: not-allowed; }
.nbtn.next { background: #3b82f6; color: #fff; border-color: #3b82f6; }
.nbtn.next:hover:not(:disabled) { background: #2563eb; border-color: #2563eb; }

/* Right content */
.step-content {
  flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0;
}

/* Visual area */
.step-visual {
  flex: 1; min-height: 0; overflow: hidden;
  background: #f8fafc; border-bottom: 1px solid #e5e7eb;
  display: flex; align-items: center; justify-content: center; padding: 20px;
  position: relative;
}
.visual {
  width: 100%; max-width: 560px; height: 100%; max-height: 300px; position: relative;
  transform-origin: center center; user-select: none;
}

/* Zoom bar */
.zoom-bar {
  position: absolute; top: 10px; right: 10px; z-index: 20;
  display: flex; align-items: center; gap: 2px;
  background: rgba(255,255,255,0.95); border: 1px solid #e5e7eb; border-radius: 7px;
  padding: 3px 5px; box-shadow: 0 1px 4px rgba(0,0,0,0.08);
}
.zoom-btn {
  width: 22px; height: 22px; border: none; background: none; cursor: pointer;
  font-size: 15px; font-weight: 700; color: #374151; border-radius: 4px; padding: 0;
  display: flex; align-items: center; justify-content: center; line-height: 1; transition: background 0.1s;
}
.zoom-btn:hover:not(:disabled) { background: #f3f4f6; }
.zoom-btn:disabled { opacity: 0.3; cursor: not-allowed; }
.zoom-label {
  min-width: 38px; text-align: center; font-size: 11px; font-weight: 600;
  color: #6b7280; cursor: pointer; padding: 2px 3px; border-radius: 3px;
  transition: background 0.1s;
}
.zoom-label:hover { background: #f3f4f6; color: #1a1a1a; }
.v-cursor { position: absolute; font-size: 18px; pointer-events: none; z-index: 10; animation: bounce 1.2s infinite; }
@keyframes bounce { 0%,100% { transform: translateY(0); } 50% { transform: translateY(-4px); } }

/* Mini app shell */
.vis-app {
  width: 100%; height: 100%;
  display: flex; flex-direction: column; background: #f5f5f5;
  border-radius: 8px; overflow: hidden;
  box-shadow: 0 4px 20px rgba(0,0,0,0.1), 0 0 0 1px rgba(0,0,0,0.06);
  font-size: 8px; font-family: inherit;
}
.v-topbar {
  height: 26px; background: #fff; border-bottom: 1px solid #e5e7eb;
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 10px; flex-shrink: 0; gap: 8px;
}
.v-brand { display: flex; align-items: center; gap: 4px; font-size: 8px; font-weight: 700; color: #1a1a1a; }
.v-logo {
  width: 14px; height: 14px; border-radius: 3px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: #fff; font-size: 8px; font-weight: 900;
  display: flex; align-items: center; justify-content: center;
}
.v-lang { font-size: 7px; color: #9ca3af; margin-left: auto; }
.v-tbtns { display: flex; gap: 3px; }
.v-tbbtn { font-size: 7px; padding: 2px 5px; border: 1px solid #e5e7eb; border-radius: 3px; color: #6b7280; }
.v-tbbtn.act { background: #eff6ff; border-color: #bfdbfe; color: #3b82f6; }

.v-body { flex: 1; display: flex; overflow: hidden; }

/* Sidebar */
.v-sidebar {
  width: 88px; flex-shrink: 0; background: #fafafa; border-right: 1px solid #e5e7eb; padding: 5px 0;
}
.v-slabel { font-size: 6px; font-weight: 700; color: #9ca3af; letter-spacing: 0.4px; text-transform: uppercase; padding: 0 5px 3px; }
.v-row { display: flex; align-items: center; gap: 3px; padding: 2px 5px 2px 10px; font-size: 7px; color: #374151; }
.v-row.active { background: #e8f0fe; color: #1d4ed8; }
.v-dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.v-dot.plan   { background: #3b82f6; }
.v-dot.design { background: #a855f7; }
.v-dot.proto  { background: #22c55e; }
.v-fn  { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.v-rb  { font-size: 6px; font-weight: 700; background: #fee2e2; color: #dc2626; border-radius: 6px; padding: 0 3px; flex-shrink: 0; }

/* Home main */
.v-main { flex: 1; overflow: hidden; padding: 7px; display: flex; flex-direction: column; gap: 5px; background: #f5f5f5; }
.v-hero { background: linear-gradient(135deg, #eff6ff, #faf5ff); border: 1px solid #e0e7ff; border-radius: 5px; padding: 6px 8px; display: flex; justify-content: space-between; align-items: center; flex-shrink: 0; }
.v-ht { font-size: 10px; font-weight: 800; color: #1a1a1a; }
.v-hb { width: 24px; height: 24px; border-radius: 5px; background: linear-gradient(135deg, #3b82f6, #8b5cf6); color: #fff; font-size: 12px; font-weight: 900; display: flex; align-items: center; justify-content: center; }
.v-stats { display: flex; gap: 4px; flex-shrink: 0; }
.v-stat { flex: 1; background: #fff; border: 1px solid #e5e7eb; border-radius: 4px; padding: 3px 4px; display: flex; flex-direction: column; }
.v-sn { font-size: 11px; font-weight: 800; color: #1a1a1a; line-height: 1; }
.v-sl { font-size: 6px; color: #9ca3af; }
.v-stat.red .v-sn { color: #dc2626; }
.v-stat.grn .v-sn { color: #16a34a; }
.v-cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px; flex: 1; }
.v-card { background: #fff; border: 1px solid #e5e7eb; border-radius: 5px; overflow: hidden; display: flex; flex-direction: column; }
.v-cstripe { height: 3px; }
.v-cstripe.plan   { background: linear-gradient(90deg, #3b82f6, #60a5fa); }
.v-cstripe.design { background: linear-gradient(90deg, #a855f7, #c084fc); }
.v-cstripe.proto  { background: linear-gradient(90deg, #22c55e, #4ade80); }
.v-cbody { padding: 5px 6px; display: flex; flex-direction: column; gap: 2px; flex: 1; }
.v-cbadge { font-size: 6px; font-weight: 700; text-transform: uppercase; padding: 1px 4px; border-radius: 6px; }
.v-cbadge.plan   { background: #eff6ff; color: #3b82f6; }
.v-cbadge.design { background: #fdf4ff; color: #a855f7; }
.v-cbadge.proto  { background: #f0fdf4; color: #22c55e; }
.v-ctitle { font-size: 8px; font-weight: 700; color: #1a1a1a; }
.v-copen  { font-size: 6px; color: #dc2626; background: #fee2e2; padding: 1px 4px; border-radius: 6px; display: inline-block; }
.v-cta { font-size: 7px; font-weight: 600; background: #e5e7eb; color: #374151; padding: 2px 5px; border-radius: 3px; display: inline-block; margin-top: auto; }
.v-cta.highlight { background: #1a1a1a; color: #fff; }

/* Doc index */
.v-idx { width: 56px; flex-shrink: 0; background: #fafafa; border-right: 1px solid #e5e7eb; padding: 5px 0; }
.v-idxl { font-size: 6px; font-weight: 700; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.4px; padding: 0 5px 3px; }
.v-idxi { font-size: 7px; color: #6b7280; padding: 2px 5px; white-space: nowrap; overflow: hidden; }
.v-idxi.l1 { font-weight: 600; color: #374151; }
.v-idxi.l2 { padding-left: 10px; font-size: 6px; }
.v-idxi.active { color: #3b82f6; border-left: 2px solid #3b82f6; }

/* Markdown content */
.v-mdc { overflow: hidden; padding: 8px 10px; background: #fff; }
.v-mdh1 { font-size: 9px; font-weight: 800; color: #1a1a1a; margin-bottom: 5px; padding-bottom: 3px; border-bottom: 1px solid #e5e7eb; }
.v-mdh2 { font-size: 7px; font-weight: 600; color: #1a1a1a; margin: 5px 0 2px; }
.v-mdh2.hl { color: #3b82f6; background: #eff6ff; padding: 2px 4px; border-radius: 2px; margin: 4px 0 2px; }
.v-mdp { font-size: 7px; color: #374151; margin-bottom: 3px; line-height: 1.4; }
.v-mdp.hl { background: #fef3c7; color: #92400e; padding: 2px 4px; border-radius: 2px; }

/* Comment panel */
.v-cp { width: 140px; flex-shrink: 0; background: #fff; border-left: 1px solid #e5e7eb; display: flex; flex-direction: column; overflow: hidden; }
.v-cptabs { display: flex; border-bottom: 1px solid #e5e7eb; flex-shrink: 0; }
.v-cpt { flex: 1; padding: 4px 2px; font-size: 6px; color: #6b7280; text-align: center; }
.v-cpt.act { color: #1a1a1a; border-bottom: 1.5px solid #3b82f6; font-weight: 600; }
.v-cta-btn { margin: 5px 6px; padding: 4px 8px; background: #3b82f6; color: #fff; border-radius: 4px; font-size: 7px; font-weight: 600; text-align: center; flex-shrink: 0; }
.v-cpcard { padding: 5px 6px; margin: 3px; border-radius: 3px; border-left: 2px solid #ef4444; background: #fff; border: 1px solid #f3f4f6; border-left: 2px solid #ef4444; flex-shrink: 0; }
.v-cpcard.resolved { border-left-color: #10b981; opacity: 0.8; }
.v-cpm { display: flex; justify-content: space-between; font-size: 6px; color: #9ca3af; margin-bottom: 2px; }
.v-cpm b { color: #1a1a1a; }
.v-cpr { font-size: 6px; background: #f9fafb; padding: 1px 3px; border-radius: 2px; color: #6b7280; margin-bottom: 2px; }
.v-cpt2 { font-size: 7px; color: #374151; line-height: 1.3; }
.v-green { color: #10b981; font-weight: 600; }

/* Comment form */
.v-form { padding: 5px 6px; display: flex; flex-direction: column; gap: 4px; background: #f9fafb; flex-shrink: 0; }
.v-field { display: flex; flex-direction: column; gap: 1px; }
.v-flabel { font-size: 6px; font-weight: 600; color: #6b7280; }
.v-finput { font-size: 7px; border: 1px solid #e5e7eb; border-radius: 3px; padding: 2px 4px; background: #fff; }
.v-finput.filled { border-color: #93c5fd; color: #1d4ed8; }
.v-ftextarea { font-size: 7px; border: 1px solid #93c5fd; border-radius: 3px; padding: 3px 4px; background: #fff; color: #374151; line-height: 1.4; min-height: 28px; }
.v-submit { align-self: flex-end; font-size: 7px; font-weight: 600; background: #10b981; color: #fff; padding: 2px 8px; border-radius: 3px; }

/* Archive / resolved */
.v-archive-btn { margin: 5px 6px; padding: 3px 6px; background: #fef3c7; color: #d97706; border: 1px solid #fde68a; border-radius: 4px; font-size: 7px; text-align: center; flex-shrink: 0; }

/* Export block */
.v-export-block { margin: 5px 6px; flex: 1; overflow: hidden; display: flex; flex-direction: column; }
.v-exp-label { font-size: 7px; font-weight: 600; color: #6b7280; padding-bottom: 4px; }
.v-exp-json {
  flex: 1; background: #1e1e2e; color: #cdd6f4;
  border-radius: 4px; padding: 6px; font-size: 6.5px;
  font-family: 'SF Mono', Consolas, monospace; line-height: 1.4;
  overflow: hidden; white-space: pre;
}

/* Description panel */
.step-desc {
  padding: 16px 20px; flex-shrink: 0; border-top: 0;
  background: #fff; display: flex; flex-direction: column; gap: 8px;
  max-height: 200px; overflow-y: auto;
}
.step-progress { display: flex; align-items: center; gap: 8px; }
.prog-dots { display: flex; gap: 5px; }
.prog-dot {
  width: 7px; height: 7px; border-radius: 50%; background: #e5e7eb;
  transition: background 0.2s;
}
.prog-dot.active { background: #3b82f6; }
.prog-dot.done   { background: #10b981; }
.prog-label { font-size: 11px; color: #9ca3af; }

.desc-title { font-size: 16px; font-weight: 700; color: #1a1a1a; }
.desc-body  { font-size: 13px; color: #374151; line-height: 1.6; }
.desc-points {
  display: flex; flex-direction: column; gap: 3px;
  padding-left: 14px; margin: 0;
}
.desc-points li { font-size: 12px; color: #6b7280; line-height: 1.5; }

.actor-badge {
  display: inline-flex; align-items: center; gap: 6px;
  font-size: 12px; font-weight: 500; color: #374151;
  background: #f3f4f6; padding: 4px 10px; border-radius: 20px;
  align-self: flex-start;
}
.actor-icon { font-size: 14px; }
</style>
