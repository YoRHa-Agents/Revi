<template>
  <div class="md-viewer-wrap">
    <div v-if="showSearch" class="in-search-bar">
      <span class="search-icon-sm">⌕</span>
      <input ref="searchInputEl" v-model="query" type="text" class="search-in-input"
        placeholder="Search in document..." @keydown.enter="next" @keydown.esc="closeSearch" />
      <span class="match-info" :class="{ nomatch: query && !matches.length }">{{ matchLabel }}</span>
      <button class="nav-btn" @click="prev" :disabled="!matches.length">↑</button>
      <button class="nav-btn" @click="next" :disabled="!matches.length">↓</button>
      <button class="close-btn" @click="closeSearch">×</button>
    </div>

    <div class="md-viewer" ref="contentEl" v-html="rendered"
      @mouseup="onMouseUp"
      @scroll="onScrolled"
    ></div>

    <Teleport to="body">
      <div v-if="tooltipVisible" class="sel-tooltip" :style="tooltipStyle" @mousedown.prevent>
        <button class="sel-btn" @click="emitAnchor">
          <span class="sel-icon">💬</span> Comment on selection
        </button>
      </div>
    </Teleport>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { marked, Renderer } from 'marked'

const props = defineProps({
  content:       { type: String,  required: true },
  showSearch:    { type: Boolean, default: false },
  hoveredAnchor: { type: Object,  default: null  },
})
const emit = defineEmits(['update:showSearch', 'anchor-select', 'section-change'])

const contentEl     = ref(null)
const searchInputEl = ref(null)
const query         = ref('')
const matchIndex    = ref(0)
const matches       = ref([])

const tooltipVisible = ref(false)
const tooltipStyle   = ref({})
const selText        = ref('')
const selSection     = ref(null)

// ── Renderer ─────────────────────────────────────────────────
const renderer = new Renderer()
renderer.heading = (text, level) => {
  const id = 'h-' + text.toLowerCase().replace(/[^\w\u4e00-\u9fff\s-]/g, '').trim().replace(/\s+/g, '-')
  return `<h${level} id="${id}">${text}</h${level}>\n`
}
marked.use({ renderer })
const rendered = computed(() => marked.parse(props.content))

// ── Search ────────────────────────────────────────────────────
const matchLabel = computed(() => {
  if (!query.value) return ''
  return matches.value.length ? `${matchIndex.value + 1} / ${matches.value.length}` : 'No matches'
})

function applyHighlights() {
  const el = contentEl.value
  if (!el) return
  el.querySelectorAll('mark.sh').forEach(m => m.replaceWith(m.textContent))
  el.normalize()
  matches.value = []
  matchIndex.value = 0

  const q = query.value.trim()
  if (!q || !props.showSearch) return

  const escaped = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const regex = new RegExp(escaped, 'gi')
  const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT, null)
  const textNodes = []
  let node
  while ((node = walker.nextNode())) textNodes.push(node)

  const newMarks = []
  textNodes.forEach(textNode => {
    const text = textNode.textContent
    regex.lastIndex = 0
    if (!regex.test(text)) return
    regex.lastIndex = 0
    const parent = textNode.parentNode
    if (!parent) return
    const frag = document.createDocumentFragment()
    let lastIdx = 0, match
    regex.lastIndex = 0
    while ((match = regex.exec(text)) !== null) {
      if (match.index > lastIdx) frag.appendChild(document.createTextNode(text.slice(lastIdx, match.index)))
      const mark = document.createElement('mark')
      mark.className = 'sh'
      mark.textContent = match[0]
      frag.appendChild(mark)
      newMarks.push(mark)
      lastIdx = match.index + match[0].length
    }
    if (lastIdx < text.length) frag.appendChild(document.createTextNode(text.slice(lastIdx)))
    parent.replaceChild(frag, textNode)
  })

  matches.value = newMarks
  if (newMarks.length) { newMarks[0].classList.add('sh-active'); newMarks[0].scrollIntoView({ behavior: 'smooth', block: 'nearest' }) }
}

function setActive(i) {
  matches.value.forEach((m, idx) => m.classList.toggle('sh-active', idx === i))
  matches.value[i]?.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
}
function next() { if (!matches.value.length) return; matchIndex.value = (matchIndex.value + 1) % matches.value.length; setActive(matchIndex.value) }
function prev() { if (!matches.value.length) return; matchIndex.value = (matchIndex.value - 1 + matches.value.length) % matches.value.length; setActive(matchIndex.value) }
function closeSearch() { emit('update:showSearch', false) }

watch(query, () => nextTick(applyHighlights))
watch(() => props.showSearch, async (val) => {
  if (val) { await nextTick(); searchInputEl.value?.focus() }
  else { query.value = ''; nextTick(applyHighlights) }
})

// ── Hover highlight ───────────────────────────────────────────
function applyHoverHighlight(anchor) {
  const el = contentEl.value
  if (!el) return
  el.querySelectorAll('mark.hover-hl').forEach(m => m.replaceWith(m.textContent))
  el.normalize()
  if (!anchor) return

  if (anchor.type === 'quote' && anchor.value) {
    const escaped = String(anchor.value).replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const regex = new RegExp(escaped.slice(0, 80), 'i') // cap length for safety
    const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT, null)
    let node
    while ((node = walker.nextNode())) {
      if (node.parentNode?.closest('mark')) continue
      const text = node.textContent
      const match = regex.exec(text)
      if (!match) continue
      const parent = node.parentNode
      const frag = document.createDocumentFragment()
      frag.appendChild(document.createTextNode(text.slice(0, match.index)))
      const mark = document.createElement('mark')
      mark.className = 'hover-hl'
      mark.textContent = match[0]
      frag.appendChild(mark)
      frag.appendChild(document.createTextNode(text.slice(match.index + match[0].length)))
      parent.replaceChild(frag, node)
      mark.scrollIntoView({ behavior: 'smooth', block: 'center' })
      break
    }
  } else if (anchor.type === 'section' && anchor.value) {
    const sectionText = String(anchor.value).replace(/^#+\s*/, '').toLowerCase()
    for (const h of el.querySelectorAll('h1,h2,h3')) {
      if (h.textContent.trim().toLowerCase() === sectionText) {
        h.scrollIntoView({ behavior: 'smooth', block: 'center' })
        h.classList.add('heading-flash')
        setTimeout(() => h.classList.remove('heading-flash'), 1800)
        break
      }
    }
  }
}

watch(() => props.hoveredAnchor, (anchor) => nextTick(() => applyHoverHighlight(anchor)))

// ── Scroll → active section ───────────────────────────────────
let lastSection = null
function onScrolled() {
  const el = contentEl.value
  if (!el) return
  const headings = [...el.querySelectorAll('h1,h2,h3')]
  let current = null
  for (const h of headings) {
    if (h.offsetTop <= el.scrollTop + 80) current = h.textContent.trim()
    else break
  }
  if (current !== lastSection) {
    lastSection = current
    emit('section-change', current)
  }
}

// ── Selection / quote ─────────────────────────────────────────
function getNearestSection(anchorNode) {
  if (!contentEl.value) return null
  let nearest = null
  for (const h of contentEl.value.querySelectorAll('h1,h2,h3')) {
    if (h.compareDocumentPosition(anchorNode) & Node.DOCUMENT_POSITION_FOLLOWING) nearest = h.textContent.trim()
  }
  return nearest
}

function onMouseUp() {
  setTimeout(() => {
    const sel = window.getSelection()
    const text = sel?.toString().trim()
    if (!text || !contentEl.value?.contains(sel.anchorNode)) { tooltipVisible.value = false; return }
    const rect = sel.getRangeAt(0).getBoundingClientRect()
    selText.value    = text
    selSection.value = getNearestSection(sel.anchorNode)
    tooltipStyle.value = { left: `${rect.left + rect.width / 2}px`, top: `${rect.top - 44}px` }
    tooltipVisible.value = true
  }, 10)
}

function emitAnchor() {
  emit('anchor-select', { type: 'quote', value: selText.value, section: selSection.value })
  tooltipVisible.value = false
  window.getSelection()?.removeAllRanges()
}

function onSelectionChange() {
  const sel = window.getSelection()
  if (!sel || sel.isCollapsed) tooltipVisible.value = false
}

onMounted(()   => document.addEventListener('selectionchange', onSelectionChange))
onUnmounted(() => document.removeEventListener('selectionchange', onSelectionChange))
</script>

<style scoped>
.md-viewer-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; }

.in-search-bar {
  display: flex; align-items: center; gap: 6px;
  padding: 7px 12px; background: var(--bg-alt); border-bottom: 1px solid var(--border); flex-shrink: 0;
}
.search-icon-sm { font-size: 15px; color: var(--text-faint); flex-shrink: 0; }
.search-in-input {
  flex: 1; min-width: 0; border: 1px solid var(--border);
  padding: 5px 9px; font-size: 13px; font-family: inherit; outline: none;
  transition: border-color 0.15s; background: transparent; color: var(--text);
}
.search-in-input:focus { border-color: var(--accent); }
.match-info { font-size: 12px; color: var(--text-faint); min-width: 72px; text-align: center; flex-shrink: 0; }
.match-info.nomatch { color: var(--accent); }
.nav-btn {
  width: 26px; height: 26px; flex-shrink: 0; border: 1px solid var(--border);
  background: var(--bg-card); color: var(--text-dim); font-size: 12px; cursor: pointer;
  display: flex; align-items: center; justify-content: center; transition: all 0.1s;
}
.nav-btn:hover:not(:disabled) { background: var(--accent-soft); color: var(--text); border-color: var(--accent); }
.nav-btn:disabled { opacity: 0.35; cursor: not-allowed; }
.close-btn {
  width: 26px; height: 26px; flex-shrink: 0; border: none;
  background: none; color: var(--text-faint); font-size: 17px; line-height: 1;
  cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 0.1s;
}
.close-btn:hover { background: var(--accent-soft); color: var(--accent); }

.md-viewer { flex: 1; padding: 24px 32px; line-height: 1.7; font-size: 15px; color: var(--text); overflow-y: auto; }
</style>

<style>
.md-viewer h1 { font-size: 22px; font-weight: 500; margin: 0 0 20px; padding-bottom: 12px; border-bottom: 1px solid var(--border); color: var(--text); letter-spacing: 0.04em; }
.md-viewer h2 { font-size: 17px; font-weight: 500; margin: 28px 0 10px; color: var(--text); transition: background 0.3s; letter-spacing: 0.03em; }
.md-viewer h3 { font-size: 15px; font-weight: 500; margin: 20px 0 8px; color: var(--text-dim); }
.md-viewer p  { margin: 0 0 12px; color: var(--text-dim); }
.md-viewer ul, .md-viewer ol { margin: 0 0 12px 20px; color: var(--text-dim); }
.md-viewer li { margin-bottom: 4px; }
.md-viewer code { background: var(--accent-soft); padding: 2px 5px; border-radius: 2px; font-size: 13px; font-family: var(--mono); color: var(--accent); }
.md-viewer pre { background: var(--bg-dark); color: var(--border-light); border: 1px solid #454138; border-radius: 3px; padding: 16px; overflow-x: auto; margin: 12px 0; }
.md-viewer pre code { background: none; color: inherit; padding: 0; font-size: 13px; }
.md-viewer table { width: 100%; border-collapse: collapse; margin: 12px 0; font-size: 14px; }
.md-viewer th, .md-viewer td { border: 1px solid var(--border); padding: 8px 12px; text-align: left; }
.md-viewer th { background: var(--bg-alt); font-weight: 600; }
.md-viewer blockquote { border-left: 3px solid var(--border); padding-left: 16px; color: var(--text-dim); margin: 0 0 12px; }

/* Search highlights */
mark.sh { background: var(--amber-soft); color: var(--text); border-radius: 2px; padding: 0 1px; }
mark.sh.sh-active { background: var(--amber); color: var(--bg); }

/* Hover highlight (from comment hover) */
mark.hover-hl {
  background: var(--accent-med); color: var(--text); border-radius: 3px; padding: 1px 2px;
  outline: 2px solid var(--accent); outline-offset: 1px;
  animation: hl-pulse 0.4s ease;
}
@keyframes hl-pulse { from { background: var(--accent); } to { background: var(--accent-med); } }

/* Section heading flash */
.heading-flash {
  background: var(--amber-soft) !important;
  border-radius: 4px;
  padding-left: 6px;
  transition: background 0.4s;
}

/* Selection tooltip */
.sel-tooltip {
  position: fixed; transform: translateX(-50%); z-index: 9999;
  pointer-events: auto; filter: drop-shadow(0 4px 12px rgba(0,0,0,0.25));
}
.sel-tooltip::after {
  content: ''; position: absolute; top: 100%; left: 50%; transform: translateX(-50%);
  border: 5px solid transparent; border-top-color: var(--text);
}
.sel-btn {
  display: flex; align-items: center; gap: 6px;
  background: var(--text); color: var(--bg); border: none; border-radius: 3px;
  padding: 7px 13px; font-size: 13px; font-weight: 500;
  font-family: var(--serif);
  cursor: pointer; white-space: nowrap; transition: background 0.15s;
}
.sel-btn:hover { background: var(--accent); }
.sel-icon { font-size: 14px; }
</style>
