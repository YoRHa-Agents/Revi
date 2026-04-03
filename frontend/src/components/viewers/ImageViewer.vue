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
                <div class="m-mdp">□ Rust runtime hardening &nbsp; □ Wire frontend &nbsp; □ File upload</div>
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
.design-board { height: 100%; display: flex; flex-direction: column; background: #2A2822; overflow: hidden; }

.board-toolbar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 9px 16px; background: #322F26; border-bottom: 1px solid #454138; flex-shrink: 0;
}
.board-title { display: flex; align-items: center; gap: 8px; }
.board-badge {
  font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em;
  background: var(--accent); color: var(--bg); padding: 2px 8px; border-radius: 3px;
}
.board-name  { font-size: 13px; font-weight: 600; color: var(--border-light); }
.board-meta  { font-size: 11px; color: var(--text-faint); }

.canvas {
  flex: 1; display: flex; gap: 40px; padding: 28px 36px;
  overflow-x: auto; overflow-y: hidden; align-items: flex-start; min-height: 0;
  position: relative;
}

.frame { flex-shrink: 0; transition: all 0.3s ease; z-index: 1; }
.frame.expanded {
  position: fixed; inset: 80px 380px 80px 220px;
  z-index: 100; margin: 0;
  display: flex; flex-direction: column; align-items: center; justify-content: flex-start;
  padding-top: 24px;
}
.frame.expanded .frame-label { color: var(--text-faint); margin-bottom: 12px; }
.frame.expanded .frame-screen { width: 490px !important; height: 300px !important; }
.frame-label { font-size: 10px; color: var(--text-dim); font-family: var(--mono); letter-spacing: 0.5px; margin-bottom: 8px; }
.frame-screen {
  width: 490px; height: 300px; position: relative;
  border-radius: 3px; overflow: hidden;
  box-shadow: 0 4px 24px rgba(0,0,0,0.5), 0 0 0 1px rgba(218,212,187,0.06);
  cursor: pointer; transition: box-shadow 0.2s;
}
.frame-screen:hover { box-shadow: 0 4px 24px rgba(0,0,0,0.5), 0 0 0 2px rgba(218,212,187,0.15); }

.expand-overlay {
  position: fixed; inset: 0; z-index: 99;
  background: rgba(0,0,0,0.6); backdrop-filter: blur(2px);
}

.pin {
  position: absolute; z-index: 10;
  width: 19px; height: 19px; border-radius: 50%;
  background: var(--accent); color: var(--bg); font-size: 9px; font-weight: 800;
  display: flex; align-items: center; justify-content: center;
  box-shadow: 0 0 0 2px var(--bg), 0 2px 8px rgba(0,0,0,0.5);
  cursor: pointer; transition: transform 0.15s, background 0.15s, box-shadow 0.15s;
  user-select: none;
}
.pin:hover, .pin.pin-active {
  background: var(--accent); transform: scale(1.35);
  box-shadow: 0 0 0 2px var(--bg), 0 0 0 4px var(--accent-med), 0 2px 8px rgba(0,0,0,0.5);
}

.legend {
  display: flex; flex-wrap: wrap; gap: 10px 20px;
  padding: 10px 28px 12px; background: #322F26; border-top: 1px solid #454138; flex-shrink: 0;
}
.ann {
  display: flex; align-items: flex-start; gap: 7px;
  cursor: pointer; border-radius: 3px; padding: 4px 6px; transition: background 0.1s;
}
.ann:hover, .ann.ann-active { background: rgba(218,212,187,0.05); }
.ann-pin {
  width: 16px; height: 16px; border-radius: 50%;
  background: var(--accent); color: var(--bg); font-size: 8px; font-weight: 800;
  display: flex; align-items: center; justify-content: center; flex-shrink: 0; transition: background 0.15s;
}
.ann.ann-active .ann-pin { background: var(--accent); }
.ann-title { font-size: 11px; font-weight: 600; color: var(--border-light); }
.ann-desc  { font-size: 10px; color: var(--text-faint); }

.mock-app {
  width: 100%; height: 100%; display: flex; flex-direction: column; background: var(--bg);
  font-size: 8px; font-family: var(--serif);
}
.m-topbar {
  height: 26px; background: var(--bg-card); border-bottom: 1px solid var(--border);
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 10px; flex-shrink: 0; gap: 8px;
}
.m-brand { display: flex; align-items: center; gap: 4px; font-size: 9px; font-weight: 700; color: var(--text); }
.m-logo {
  width: 14px; height: 14px; border-radius: 3px; background: var(--text); color: var(--bg);
  font-size: 8px; font-weight: 900; display: flex; align-items: center; justify-content: center;
}
.m-lang { font-size: 7px; color: var(--text-faint); margin-left: auto; }
.m-actions { display: flex; gap: 3px; }
.m-actbtn { font-size: 7px; padding: 2px 5px; border: 1px solid var(--border); border-radius: 3px; color: var(--text-dim); }
.m-actbtn.active { background: var(--accent-soft); border-color: var(--accent); color: var(--accent); }
.m-body { flex: 1; display: flex; overflow: hidden; }
.m-sidebar { width: 80px; flex-shrink: 0; background: var(--bg-sidebar); border-right: 1px solid var(--border); padding: 5px 0; overflow: hidden; }
.m-slabel { font-size: 6px; font-weight: 700; color: var(--text-faint); letter-spacing: 0.5px; text-transform: uppercase; padding: 0 6px 3px; }
.m-search { margin: 0 4px 4px; padding: 3px 5px; border: 1px solid var(--border); border-radius: 3px; font-size: 7px; color: var(--text-faint); background: var(--bg-input); }
.m-group { display: flex; align-items: center; gap: 2px; padding: 3px 5px; font-size: 7px; color: var(--text-dim); }
.m-gc { margin-left: auto; background: var(--border-light); border-radius: 3px; padding: 0 3px; font-size: 6px; color: var(--text-dim); }
.m-row { display: flex; align-items: center; gap: 3px; padding: 2px 5px 2px 12px; font-size: 7px; color: var(--text-dim); }
.m-row.active { background: var(--accent-soft); color: var(--accent); }
.m-dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; background: var(--accent); }
.m-dot.plan, .m-dot.design, .m-dot.proto { background: var(--accent); }
.m-fn { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.m-rb {
  font-size: 6px; font-weight: 700; background: transparent; color: var(--accent);
  border: 1px solid var(--accent); border-radius: 3px; padding: 0 3px; flex-shrink: 0;
}
.m-main { flex: 1; overflow: hidden; padding: 7px; display: flex; flex-direction: column; gap: 5px; background: var(--bg); }
.m-hero {
  background: var(--bg-card); border: 1px solid var(--border); border-radius: 3px;
  padding: 7px 9px; display: flex; justify-content: space-between; align-items: center; flex-shrink: 0;
}
.m-ht { font-size: 10px; font-weight: 800; color: var(--text); }
.m-hs { font-size: 7px; color: var(--text-dim); margin-top: 2px; }
.m-hbadge {
  width: 26px; height: 26px; border-radius: 3px; background: var(--text); color: var(--bg);
  font-size: 13px; font-weight: 900; display: flex; align-items: center; justify-content: center;
}
.m-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px; flex-shrink: 0; }
.m-stat {
  background: var(--bg-card); border: 1px solid var(--border); border-radius: 3px;
  padding: 4px 5px; display: flex; flex-direction: column; gap: 1px;
}
.m-sn { font-size: 13px; font-weight: 800; color: var(--text); line-height: 1; }
.m-sl { font-size: 6px; color: var(--text-faint); }
.m-stat.red .m-sn { color: var(--accent); }
.m-stat.grn .m-sn { color: var(--green); }
.m-stat.gry .m-sn { color: var(--text-dim); }
.m-cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px; flex-shrink: 0; }
.m-card { background: var(--bg-card); border: 1px solid var(--border); border-radius: 3px; overflow: hidden; }
.m-stripe { height: 3px; background: var(--accent); }
.m-stripe.plan, .m-stripe.design, .m-stripe.proto { background: var(--accent); }
.m-cbody { padding: 5px 6px; }
.m-badge {
  font-size: 6px; font-weight: 700; text-transform: uppercase; padding: 1px 4px; border-radius: 3px;
  border: 1px solid var(--border); color: var(--accent); background: transparent;
}
.m-badge.plan, .m-badge.design, .m-badge.proto { background: transparent; color: var(--accent); }
.m-ctitle { font-size: 8px; font-weight: 700; color: var(--text); margin: 3px 0 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.m-cbtn { font-size: 7px; font-weight: 600; background: var(--text); color: var(--bg); padding: 2px 6px; border-radius: 3px; display: inline-block; }
.m-index { width: 58px; flex-shrink: 0; background: var(--bg-sidebar); border-right: 1px solid var(--border); padding: 5px 0; overflow: hidden; }
.m-ilabel { font-size: 6px; font-weight: 700; color: var(--text-faint); letter-spacing: 0.5px; text-transform: uppercase; padding: 0 5px 3px; }
.m-iitem { font-size: 7px; color: var(--text-dim); padding: 2px 5px; white-space: nowrap; overflow: hidden; }
.m-iitem.l1 { font-weight: 600; color: var(--text); }
.m-iitem.l2 { padding-left: 10px; font-size: 6px; }
.m-iitem.active { color: var(--accent); border-left: 2px solid var(--accent); }
.m-mdc { flex: 1; overflow: hidden; padding: 8px 10px; background: var(--bg-card); }
.m-mdh1 { font-size: 10px; font-weight: 800; color: var(--text); margin-bottom: 6px; padding-bottom: 3px; border-bottom: 1px solid var(--border); }
.m-mdh2 { font-size: 8px; font-weight: 600; color: var(--text); margin: 6px 0 2px; }
.m-mdp  { font-size: 7px; color: var(--text-dim); margin-bottom: 3px; line-height: 1.4; }
.m-code { background: var(--accent-soft); color: var(--accent); padding: 0 2px; border-radius: 2px; font-family: var(--mono); }
.m-mdpre {
  font-size: 7px; background: var(--bg-dark); color: var(--border-light);
  padding: 3px 6px; border-radius: 3px; margin: 2px 0; font-family: var(--mono);
}
.m-cp { width: 126px; flex-shrink: 0; background: var(--bg-card); border-left: 1px solid var(--border); display: flex; flex-direction: column; overflow: hidden; }
.m-cptabs { display: flex; border-bottom: 1px solid var(--border); flex-shrink: 0; }
.m-cptab { flex: 1; padding: 4px 3px; font-size: 6px; color: var(--text-dim); text-align: center; }
.m-cptab.active { color: var(--accent); border-bottom: 1.5px solid var(--accent); font-weight: 600; }
.m-cpcard { padding: 5px 6px; border: 1px solid var(--border-light); margin: 3px; border-radius: 3px; }
.m-cpcard.open { border-left: 2px solid var(--accent); }
.m-cpm { display: flex; justify-content: space-between; font-size: 6px; color: var(--text-faint); margin-bottom: 2px; }
.m-cpm b { color: var(--text); }
.m-cpr { font-size: 6px; background: var(--bg-alt); padding: 1px 3px; border-radius: 2px; color: var(--text-dim); margin-bottom: 2px; }
.m-cpt { font-size: 7px; color: var(--text-dim); line-height: 1.3; }
.m-cpbtn {
  font-size: 6px; color: var(--green); background: transparent; border: 1px solid var(--green);
  padding: 1px 5px; border-radius: 3px; display: inline-block; margin-top: 2px;
}
</style>
