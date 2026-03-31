<template>
  <div class="design-board">
    <div class="board-toolbar">
      <div class="board-title">
        <span class="board-badge">Design</span>
        <span class="board-name">{{ item.title }}</span>
      </div>
      <span class="board-meta">2 screens · 6 annotations · double-click frame to expand</span>
    </div>

    <div class="canvas">
      <!-- Frame 1 -->
      <div class="frame" :class="{ expanded: expandedFrame === 1 }">
        <div class="frame-label">Frame 1 / Dashboard</div>
        <div class="frame-screen" @dblclick="toggleExpand(1)"
          :style="expandedFrame === 1 ? { transform: `scale(${expandedScale})`, transformOrigin: 'top center' } : {}"
        >
          <div class="mock-app">
            <div class="m-topbar">
              <div class="m-brand"><span class="m-logo">R</span> Revi</div>
              <div class="m-lang">EN / 中文</div>
            </div>
            <div class="m-body">
              <div class="m-sidebar">
                <div class="m-slabel">WORKSPACE</div>
                <div class="m-search">⌕ Search...</div>
                <div class="m-group">› 📄 Plans <span class="m-gc">1</span></div>
                <div class="m-row active"><span class="m-dot plan"></span><span class="m-fn">Sprint 1 — Design</span><span class="m-rb">2</span></div>
                <div class="m-group">› 🎨 Designs <span class="m-gc">1</span></div>
                <div class="m-row"><span class="m-dot design"></span><span class="m-fn">UI Mockup v1</span><span class="m-rb">1</span></div>
                <div class="m-group">› ⚡ Prototypes <span class="m-gc">1</span></div>
                <div class="m-row"><span class="m-dot proto"></span><span class="m-fn">Review Flow</span></div>
              </div>
              <div class="m-main">
                <div class="m-hero">
                  <div><div class="m-ht">Welcome to Revi</div><div class="m-hs">Your agent-human coworking space</div></div>
                  <div class="m-hbadge">R</div>
                </div>
                <div class="m-stats">
                  <div class="m-stat"><span class="m-sn">3</span><span class="m-sl">Items</span></div>
                  <div class="m-stat red"><span class="m-sn">3</span><span class="m-sl">Open</span></div>
                  <div class="m-stat grn"><span class="m-sn">2</span><span class="m-sl">Resolved</span></div>
                  <div class="m-stat gry"><span class="m-sn">1</span><span class="m-sl">Archived</span></div>
                </div>
                <div class="m-cards">
                  <div class="m-card"><div class="m-stripe plan"></div><div class="m-cbody"><span class="m-badge plan">Plan</span><div class="m-ctitle">Sprint 1</div><div class="m-cbtn">Open →</div></div></div>
                  <div class="m-card"><div class="m-stripe design"></div><div class="m-cbody"><span class="m-badge design">Design</span><div class="m-ctitle">UI Mockup v1</div><div class="m-cbtn">Open →</div></div></div>
                  <div class="m-card"><div class="m-stripe proto"></div><div class="m-cbody"><span class="m-badge proto">Prototype</span><div class="m-ctitle">Review Flow</div><div class="m-cbtn">Open →</div></div></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Annotation pins (clickable) -->
          <div
            v-for="p in frame1Pins" :key="p.n"
            class="pin" :class="{ 'pin-active': isHovered(p.n) }"
            :style="{ left: p.x + 'px', top: p.y + 'px' }"
            @click.stop="emitAnnotation(p)"
            :title="'Click to comment on: ' + annotations[p.n - 1]?.title"
          >{{ p.n }}</div>
        </div>
      </div>

      <!-- Frame 2 -->
      <div class="frame" :class="{ expanded: expandedFrame === 2 }">
        <div class="frame-label">Frame 2 / Review View (Plan)</div>
        <div class="frame-screen" @dblclick="toggleExpand(2)"
          :style="expandedFrame === 2 ? { transform: `scale(${expandedScale})`, transformOrigin: 'top center' } : {}"
        >
          <div class="mock-app">
            <div class="m-topbar">
              <div class="m-brand"><span class="m-logo">R</span> Revi</div>
              <div class="m-actions">
                <div class="m-actbtn active">☰ Index</div>
                <div class="m-actbtn">⌕ Search</div>
              </div>
              <div class="m-lang">EN / 中文</div>
            </div>
            <div class="m-body">
              <div class="m-sidebar" style="width:66px">
                <div class="m-slabel">WORKSPACE</div>
                <div class="m-row active"><span class="m-dot plan"></span><span class="m-fn">Sprint 1</span><span class="m-rb">2</span></div>
                <div class="m-row"><span class="m-dot design"></span><span class="m-fn">UI Mockup</span></div>
                <div class="m-row"><span class="m-dot proto"></span><span class="m-fn">Review Flow</span></div>
              </div>
              <div class="m-index">
                <div class="m-ilabel">INDEX</div>
                <div class="m-iitem l1">Overview</div>
                <div class="m-iitem l2">Scope</div>
                <div class="m-iitem l1">Data Models</div>
                <div class="m-iitem l2">Comment Schema</div>
                <div class="m-iitem l1 active">Storage</div>
                <div class="m-iitem l1">Agent Export</div>
                <div class="m-iitem l1">API Endpoints</div>
              </div>
              <div class="m-mdc">
                <div class="m-mdh1">Sprint 1 — System Design</div>
                <div class="m-mdh2">## Storage Layer</div>
                <div class="m-mdp">Comments stored in <span class="m-code">data/comments/</span>. File-system portable.</div>
                <div class="m-mdh2">## Agent Export Format</div>
                <div class="m-mdpre">GET /api/export/{id}</div>
                <div class="m-mdp">Returns structured JSON with open_comments array for AI agents.</div>
                <div class="m-mdh2">## Next Steps</div>
                <div class="m-mdp">□ FastAPI backend &nbsp; □ Wire frontend &nbsp; □ File upload</div>
              </div>
              <div class="m-cp">
                <div class="m-cptabs">
                  <span class="m-cptab active">Open (2)</span>
                  <span class="m-cptab">Resolved (1)</span>
                </div>
                <div class="m-cpcard open">
                  <div class="m-cpm"><b>Alice</b><span>3h ago</span></div>
                  <div class="m-cpr">section · ## Storage Layer</div>
                  <div class="m-cpt">Should support SQLite from start.</div>
                  <div class="m-cpbtn">✓ Resolve</div>
                </div>
                <div class="m-cpcard open">
                  <div class="m-cpm"><b>Agent-Planner</b><span>1h ago</span></div>
                  <div class="m-cpt">Missing schema_version field.</div>
                  <div class="m-cpbtn">✓ Resolve</div>
                </div>
              </div>
            </div>
          </div>

          <div
            v-for="p in frame2Pins" :key="p.n"
            class="pin" :class="{ 'pin-active': isHovered(p.n) }"
            :style="{ left: p.x + 'px', top: p.y + 'px' }"
            @click.stop="emitAnnotation(p)"
            :title="'Click to comment on: ' + annotations[p.n - 1]?.title"
          >{{ p.n }}</div>
        </div>
      </div>
    </div>

    <!-- Expand overlay (click to close) -->
    <div v-if="expandedFrame" class="expand-overlay" @click="expandedFrame = null" @dblclick.stop></div>

    <div class="legend">
      <div
        v-for="a in annotations" :key="a.n"
        class="ann" :class="{ 'ann-active': isHovered(a.n) }"
        @mouseenter="hoveredPin = a.n"
        @mouseleave="hoveredPin = null"
        @click="emitAnnotation({ n: a.n, label: a.title })"
      >
        <span class="ann-pin">{{ a.n }}</span>
        <div>
          <div class="ann-title">{{ a.title }}</div>
          <div class="ann-desc">{{ a.desc }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick } from 'vue'

const props = defineProps({
  item:          { type: Object, required: true },
  hoveredAnchor: { type: Object, default: null },
})
const emit = defineEmits(['anchor-select'])

const expandedFrame = ref(null)
const hoveredPin    = ref(null)
const expandedScale = ref(1)

function toggleExpand(n) {
  if (expandedFrame.value === n) {
    expandedFrame.value = null
    expandedScale.value = 1
  } else {
    expandedFrame.value = n
    nextTick(() => {
      // Available area matches inset: 80px 380px 80px 220px
      const availW = window.innerWidth - 220 - 380
      const availH = window.innerHeight - 80 - 80
      expandedScale.value = Math.min(availW / 490, availH / 300) * 0.9
    })
  }
}

const annotations = [
  { n: 1, title: 'File Sidebar',     desc: 'Grouped by type · open-comment badges · search' },
  { n: 2, title: 'Dashboard Hero',   desc: 'Stats overview: items, open, resolved, archived' },
  { n: 3, title: 'Item Cards',       desc: 'Type stripe · status pills · one-click open' },
  { n: 4, title: 'Doc Index Panel',  desc: 'Collapsible TOC · smooth scroll to heading' },
  { n: 5, title: 'Markdown Viewer',  desc: 'Rendered with heading anchors · in-page search' },
  { n: 6, title: 'Comment Panel',    desc: 'Open / Resolved tabs · resolve · archive · export' },
]

const frame1Pins = [
  { n: 1, x: 58, y: 145 },
  { n: 2, x: 240, y: 60 },
  { n: 3, x: 390, y: 145 },
]
const frame2Pins = [
  { n: 4, x: 112, y: 60 },
  { n: 5, x: 264, y: 60 },
  { n: 6, x: 426, y: 60 },
]

function isHovered(n) {
  if (hoveredPin.value === n) return true
  return props.hoveredAnchor?.type === 'annotation' && String(props.hoveredAnchor.value) === String(n)
}

function emitAnnotation(p) {
  const ann = annotations.find(a => a.n === p.n)
  emit('anchor-select', { type: 'annotation', value: String(p.n), label: ann?.title || p.label })
}
</script>

<style scoped>
.design-board { height: 100%; display: flex; flex-direction: column; background: #0f0f17; overflow: hidden; }

.board-toolbar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 9px 16px; background: #17172a; border-bottom: 1px solid #2a2a40; flex-shrink: 0;
}
.board-title { display: flex; align-items: center; gap: 8px; }
.board-badge { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.4px; background: #a855f7; color: #fff; padding: 2px 8px; border-radius: 10px; }
.board-name  { font-size: 13px; font-weight: 600; color: #e2e8f0; }
.board-meta  { font-size: 11px; color: #4b5563; }

.canvas {
  flex: 1; display: flex; gap: 40px; padding: 28px 36px;
  overflow-x: auto; overflow-y: hidden; align-items: flex-start; min-height: 0;
  position: relative;
}

/* Frame */
.frame { flex-shrink: 0; transition: all 0.3s ease; z-index: 1; }
.frame.expanded {
  position: fixed; inset: 80px 380px 80px 220px;
  z-index: 100; margin: 0;
  display: flex; flex-direction: column; align-items: center; justify-content: flex-start;
  padding-top: 24px;
}
.frame.expanded .frame-label { color: #9ca3af; margin-bottom: 12px; }
.frame.expanded .frame-screen {
  width: 490px !important; height: 300px !important;
}
.frame-label { font-size: 10px; color: #6b7280; font-family: 'SF Mono', Consolas, monospace; letter-spacing: 0.5px; margin-bottom: 8px; }
.frame-screen {
  width: 490px; height: 300px; position: relative;
  border-radius: 8px; overflow: hidden;
  box-shadow: 0 4px 24px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.06);
  cursor: pointer; transition: box-shadow 0.2s;
}
.frame-screen:hover { box-shadow: 0 4px 24px rgba(0,0,0,0.5), 0 0 0 2px rgba(255,255,255,0.15); }

/* Expand overlay */
.expand-overlay {
  position: fixed; inset: 0; z-index: 99;
  background: rgba(0,0,0,0.6); backdrop-filter: blur(2px);
}

/* Annotation pins */
.pin {
  position: absolute; z-index: 10;
  width: 19px; height: 19px; border-radius: 50%;
  background: #f59e0b; color: #fff; font-size: 9px; font-weight: 800;
  display: flex; align-items: center; justify-content: center;
  box-shadow: 0 0 0 2px #fff, 0 2px 8px rgba(0,0,0,0.5);
  cursor: pointer; transition: transform 0.15s, background 0.15s, box-shadow 0.15s;
  user-select: none;
}
.pin:hover, .pin.pin-active {
  background: #ef4444; transform: scale(1.35);
  box-shadow: 0 0 0 2px #fff, 0 0 0 4px rgba(239,68,68,0.3), 0 2px 8px rgba(0,0,0,0.5);
}

/* Legend */
.legend {
  display: flex; flex-wrap: wrap; gap: 10px 20px;
  padding: 10px 28px 12px; background: #17172a; border-top: 1px solid #2a2a40; flex-shrink: 0;
}
.ann {
  display: flex; align-items: flex-start; gap: 7px;
  cursor: pointer; border-radius: 6px; padding: 4px 6px; transition: background 0.1s;
}
.ann:hover, .ann.ann-active { background: rgba(255,255,255,0.05); }
.ann-pin {
  width: 16px; height: 16px; border-radius: 50%;
  background: #f59e0b; color: #fff; font-size: 8px; font-weight: 800;
  display: flex; align-items: center; justify-content: center; flex-shrink: 0; transition: background 0.15s;
}
.ann.ann-active .ann-pin { background: #ef4444; }
.ann-title { font-size: 11px; font-weight: 600; color: #d1d5db; }
.ann-desc  { font-size: 10px; color: #6b7280; }

/* ── Mock UI ── */
.mock-app {
  width: 100%; height: 100%; display: flex; flex-direction: column; background: #f5f5f5;
  font-size: 8px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}
.m-topbar {
  height: 26px; background: #fff; border-bottom: 1px solid #e5e7eb;
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 10px; flex-shrink: 0; gap: 8px;
}
.m-brand { display: flex; align-items: center; gap: 4px; font-size: 9px; font-weight: 700; color: #1a1a1a; }
.m-logo { width: 14px; height: 14px; border-radius: 3px; background: linear-gradient(135deg, #3b82f6, #8b5cf6); color: #fff; font-size: 8px; font-weight: 900; display: flex; align-items: center; justify-content: center; }
.m-lang { font-size: 7px; color: #9ca3af; margin-left: auto; }
.m-actions { display: flex; gap: 3px; }
.m-actbtn { font-size: 7px; padding: 2px 5px; border: 1px solid #e5e7eb; border-radius: 4px; color: #6b7280; }
.m-actbtn.active { background: #eff6ff; border-color: #bfdbfe; color: #3b82f6; }
.m-body { flex: 1; display: flex; overflow: hidden; }
.m-sidebar { width: 80px; flex-shrink: 0; background: #fafafa; border-right: 1px solid #e5e7eb; padding: 5px 0; overflow: hidden; }
.m-slabel { font-size: 6px; font-weight: 700; color: #9ca3af; letter-spacing: 0.5px; text-transform: uppercase; padding: 0 6px 3px; }
.m-search { margin: 0 4px 4px; padding: 3px 5px; border: 1px solid #e5e7eb; border-radius: 3px; font-size: 7px; color: #9ca3af; }
.m-group { display: flex; align-items: center; gap: 2px; padding: 3px 5px; font-size: 7px; color: #6b7280; }
.m-gc { margin-left: auto; background: #e5e7eb; border-radius: 6px; padding: 0 3px; font-size: 6px; }
.m-row { display: flex; align-items: center; gap: 3px; padding: 2px 5px 2px 12px; font-size: 7px; color: #374151; }
.m-row.active { background: #e8f0fe; color: #1d4ed8; }
.m-dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.m-dot.plan { background: #3b82f6; } .m-dot.design { background: #a855f7; } .m-dot.proto { background: #22c55e; }
.m-fn { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.m-rb { font-size: 6px; font-weight: 700; background: #fee2e2; color: #dc2626; border-radius: 6px; padding: 0 3px; flex-shrink: 0; }
.m-main { flex: 1; overflow: hidden; padding: 7px; display: flex; flex-direction: column; gap: 5px; background: #f5f5f5; }
.m-hero { background: linear-gradient(135deg, #eff6ff, #faf5ff); border: 1px solid #e0e7ff; border-radius: 5px; padding: 7px 9px; display: flex; justify-content: space-between; align-items: center; flex-shrink: 0; }
.m-ht { font-size: 10px; font-weight: 800; color: #1a1a1a; }
.m-hs { font-size: 7px; color: #6b7280; margin-top: 2px; }
.m-hbadge { width: 26px; height: 26px; border-radius: 6px; background: linear-gradient(135deg, #3b82f6, #8b5cf6); color: #fff; font-size: 13px; font-weight: 900; display: flex; align-items: center; justify-content: center; }
.m-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px; flex-shrink: 0; }
.m-stat { background: #fff; border: 1px solid #e5e7eb; border-radius: 5px; padding: 4px 5px; display: flex; flex-direction: column; gap: 1px; }
.m-sn { font-size: 13px; font-weight: 800; color: #1a1a1a; line-height: 1; }
.m-sl { font-size: 6px; color: #9ca3af; }
.m-stat.red .m-sn { color: #dc2626; } .m-stat.grn .m-sn { color: #16a34a; } .m-stat.gry .m-sn { color: #6b7280; }
.m-cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px; flex-shrink: 0; }
.m-card { background: #fff; border: 1px solid #e5e7eb; border-radius: 5px; overflow: hidden; }
.m-stripe { height: 3px; }
.m-stripe.plan { background: linear-gradient(90deg, #3b82f6, #60a5fa); }
.m-stripe.design { background: linear-gradient(90deg, #a855f7, #c084fc); }
.m-stripe.proto { background: linear-gradient(90deg, #22c55e, #4ade80); }
.m-cbody { padding: 5px 6px; }
.m-badge { font-size: 6px; font-weight: 700; text-transform: uppercase; padding: 1px 4px; border-radius: 6px; }
.m-badge.plan { background: #eff6ff; color: #3b82f6; } .m-badge.design { background: #fdf4ff; color: #a855f7; } .m-badge.proto { background: #f0fdf4; color: #22c55e; }
.m-ctitle { font-size: 8px; font-weight: 700; color: #1a1a1a; margin: 3px 0 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.m-cbtn { font-size: 7px; font-weight: 600; background: #1a1a1a; color: #fff; padding: 2px 6px; border-radius: 3px; display: inline-block; }
.m-index { width: 58px; flex-shrink: 0; background: #fafafa; border-right: 1px solid #e5e7eb; padding: 5px 0; overflow: hidden; }
.m-ilabel { font-size: 6px; font-weight: 700; color: #9ca3af; letter-spacing: 0.5px; text-transform: uppercase; padding: 0 5px 3px; }
.m-iitem { font-size: 7px; color: #6b7280; padding: 2px 5px; white-space: nowrap; overflow: hidden; }
.m-iitem.l1 { font-weight: 600; color: #374151; } .m-iitem.l2 { padding-left: 10px; font-size: 6px; } .m-iitem.active { color: #3b82f6; border-left: 2px solid #3b82f6; }
.m-mdc { flex: 1; overflow: hidden; padding: 8px 10px; background: #fff; }
.m-mdh1 { font-size: 10px; font-weight: 800; color: #1a1a1a; margin-bottom: 6px; padding-bottom: 3px; border-bottom: 1px solid #e5e7eb; }
.m-mdh2 { font-size: 8px; font-weight: 600; color: #1a1a1a; margin: 6px 0 2px; }
.m-mdp  { font-size: 7px; color: #374151; margin-bottom: 3px; line-height: 1.4; }
.m-code { background: #f3f4f6; color: #d63384; padding: 0 2px; border-radius: 2px; font-family: monospace; }
.m-mdpre { font-size: 7px; background: #1e1e2e; color: #cdd6f4; padding: 3px 6px; border-radius: 3px; margin: 2px 0; font-family: monospace; }
.m-cp { width: 126px; flex-shrink: 0; background: #fff; border-left: 1px solid #e5e7eb; display: flex; flex-direction: column; overflow: hidden; }
.m-cptabs { display: flex; border-bottom: 1px solid #e5e7eb; flex-shrink: 0; }
.m-cptab { flex: 1; padding: 4px 3px; font-size: 6px; color: #6b7280; text-align: center; }
.m-cptab.active { color: #1a1a1a; border-bottom: 1.5px solid #3b82f6; font-weight: 600; }
.m-cpcard { padding: 5px 6px; border-bottom: 1px solid #f3f4f6; margin: 3px; border-radius: 3px; }
.m-cpcard.open { border-left: 2px solid #ef4444; }
.m-cpm { display: flex; justify-content: space-between; font-size: 6px; color: #9ca3af; margin-bottom: 2px; }
.m-cpm b { color: #1a1a1a; }
.m-cpr { font-size: 6px; background: #f9fafb; padding: 1px 3px; border-radius: 2px; color: #6b7280; margin-bottom: 2px; }
.m-cpt { font-size: 7px; color: #374151; line-height: 1.3; }
.m-cpbtn { font-size: 6px; color: #059669; background: #ecfdf5; padding: 1px 5px; border-radius: 3px; display: inline-block; margin-top: 2px; }
</style>
