<template>
  <div class="panel">
    <div class="tabs">
      <button class="tab" :class="{ active: tab === 'open' }" @click="tab = 'open'">
        {{ t('review.openTab', { n: openComments.length }) }}
      </button>
      <button class="tab" :class="{ active: tab === 'resolved' }" @click="tab = 'resolved'">
        {{ t('review.resolvedTab', { n: resolvedComments.length }) }}
      </button>
    </div>

    <div class="actions-bar">
      <button class="btn-primary" @click="toggleForm">
        {{ showForm ? t('review.cancel') : t('review.addComment') }}
      </button>
      <button v-if="resolvedComments.length" class="btn-archive" @click="archiveResolved">
        {{ t('review.archiveResolved', { n: resolvedComments.length }) }}
      </button>
    </div>

    <transition name="form-slide">
      <div v-if="showForm" class="add-form" ref="formEl">
        <!-- Anchor preview -->
        <div v-if="form.anchor" class="anchor-preview" :class="'anchor-' + form.anchor.type">
          <div class="anchor-label">
            <span class="anchor-icon">{{ anchorIcon(form.anchor.type) }}</span>
            <span>{{ anchorLabel(form.anchor) }}</span>
            <button class="anchor-remove" @click="form.anchor = null">×</button>
          </div>
          <p v-if="form.anchor.type === 'quote'" class="anchor-text">{{ form.anchor.value }}</p>
          <p v-else-if="form.anchor.label" class="anchor-text">{{ form.anchor.label }}</p>
        </div>

        <div class="field">
          <label>{{ t('review.author') }}</label>
          <input ref="authorInputEl" v-model="form.author" type="text" placeholder="Your name" />
        </div>
        <div class="field">
          <label>{{ t('review.content') }}</label>
          <textarea v-model="form.content" rows="3" :placeholder="t('review.content') + '...'"></textarea>
        </div>
        <div v-if="!form.anchor" class="field">
          <label>{{ t('review.reference') }}</label>
          <input v-model="form.reference" type="text" :placeholder="t('review.referencePlaceholder')" />
        </div>
        <button class="btn-submit" @click="submitComment" :disabled="!form.author || !form.content">
          {{ t('review.submit') }}
        </button>
      </div>
    </transition>

    <div class="comments" ref="commentsEl">
      <template v-if="tab === 'open'">
        <div v-if="!openComments.length" class="empty">{{ t('review.noOpen') }}</div>
        <div
          v-for="c in openComments" :key="c.id"
          class="comment-card open"
          :class="{ 'section-match': isActiveSection(c) }"
          :ref="el => { if (el) cardRefs[c.id] = el }"
          @mouseenter="emit('anchor-hover', c.reference)"
          @mouseleave="emit('anchor-leave')"
        >
          <div class="c-meta">
            <span class="c-author">{{ c.author }}</span>
            <span class="c-time">{{ relativeTime(c.createdAt) }}</span>
          </div>
          <component :is="'div'" v-bind="refAttrs(c.reference)">
            <ReferenceTag :reference="c.reference" :t="t" />
          </component>
          <p class="c-content">{{ locale === 'zh' ? c.contentZh : c.content }}</p>
          <button class="btn-resolve" @click="resolve(c.id)">✓ {{ t('review.resolve') }}</button>
        </div>
      </template>

      <template v-if="tab === 'resolved'">
        <div v-if="!resolvedComments.length" class="empty">{{ t('review.noResolved') }}</div>
        <div
          v-for="c in resolvedComments" :key="c.id"
          class="comment-card resolved"
          :class="{ 'section-match': isActiveSection(c) }"
          :ref="el => { if (el) cardRefs[c.id] = el }"
          @mouseenter="emit('anchor-hover', c.reference)"
          @mouseleave="emit('anchor-leave')"
        >
          <div class="c-meta">
            <span class="c-author">{{ c.author }}</span>
            <span class="resolved-badge">✓</span>
            <span class="c-time">{{ relativeTime(c.createdAt) }}</span>
          </div>
          <component :is="'div'" v-bind="refAttrs(c.reference)">
            <ReferenceTag :reference="c.reference" :t="t" />
          </component>
          <p class="c-content">{{ locale === 'zh' ? c.contentZh : c.content }}</p>
        </div>
      </template>
    </div>

    <details class="export-block" @toggle="refreshExport">
      <summary>{{ t('review.agentExport') }}</summary>
      <pre class="export-json">{{ exportJson }}</pre>
    </details>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, defineComponent, h } from 'vue'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'

// ── Inline sub-component for reference rendering ──────────────
const ReferenceTag = defineComponent({
  props: { reference: Object, t: Function },
  setup(props) {
    return () => {
      const ref_ = props.reference
      if (!ref_ || ref_.type === 'general' || !ref_.value) return null

      if (ref_.type === 'quote') {
        return h('blockquote', { class: 'c-quote' }, [
          ref_.section ? h('span', { class: 'c-quote-section' }, ref_.section) : null,
          ref_.value,
        ])
      }
      if (ref_.type === 'annotation') {
        return h('div', { class: 'c-ref' }, [
          h('span', { class: 'ref-badge' }, props.t('review.annotation')),
          ` #${ref_.value}`,
          ref_.label ? h('span', { class: 'ref-label' }, ` — ${ref_.label}`) : null,
        ])
      }
      if (ref_.type === 'step') {
        return h('div', { class: 'c-ref' }, [
          h('span', { class: 'ref-badge' }, props.t('review.step')),
          ` ${props.t('review.stepN', { n: Number(ref_.value) + 1 })}`,
          ref_.label ? h('span', { class: 'ref-label' }, ` — ${ref_.label}`) : null,
        ])
      }
      // section / general text ref
      return h('div', { class: 'c-ref' }, [
        h('span', { class: 'ref-badge' }, props.t('review.' + ref_.type)),
        ' ' + ref_.value,
      ])
    }
  },
})

// ── Component setup ───────────────────────────────────────────
const props = defineProps({
  itemId:        { type: String, required: true },
  pendingAnchor: { type: Object, default: null },
  activeSection: { type: String, default: null },
})
const emit = defineEmits(['anchor-consumed', 'anchor-hover', 'anchor-leave'])
const { t, locale } = useI18n()

const tab      = ref('open')
const showForm = ref(false)
const formEl   = ref(null)
const commentsEl    = ref(null)
const authorInputEl = ref(null)
const cardRefs = {}

const form = ref({ author: '', content: '', reference: '', anchor: null })

const openComments     = computed(() => state.getComments(props.itemId).filter(c => c.status === 'open'))
const resolvedComments = computed(() => state.getComments(props.itemId).filter(c => c.status === 'resolved'))
const exportJson       = ref('')

async function refreshExport() {
  try {
    const data = await state.exportForAgent(props.itemId)
    exportJson.value = JSON.stringify(data, null, 2)
  } catch {
    exportJson.value = ''
  }
}

// Open form pre-filled when anchor arrives from viewer
watch(() => props.pendingAnchor, async (anchor) => {
  if (!anchor) return
  form.value.anchor = anchor
  showForm.value    = true
  tab.value         = 'open'
  emit('anchor-consumed')
  await nextTick()
  authorInputEl.value?.focus()
  formEl.value?.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
})

// Scroll comment panel to first matching comment when section changes
watch(() => props.activeSection, async (section) => {
  if (!section) return
  await nextTick()
  const allComments = [...openComments.value, ...resolvedComments.value]
  const match = allComments.find(c => isActiveSection(c))
  if (match && cardRefs[match.id]) {
    cardRefs[match.id].scrollIntoView({ behavior: 'smooth', block: 'nearest' })
  }
})

function isActiveSection(c) {
  if (!props.activeSection || !c.reference) return false
  const ref = c.reference
  if (ref.type === 'quote')    return ref.section === props.activeSection
  if (ref.type === 'section')  return ref.value?.replace(/^#+\s*/, '') === props.activeSection
  return false
}

function refAttrs() { return {} } // needed to avoid template warning with dynamic component

function anchorIcon(type) {
  return { quote: '❝', annotation: '📍', step: '▸', section: '§' }[type] || '🔗'
}
function anchorLabel(anchor) {
  if (anchor.type === 'quote')      return 'Quoting' + (anchor.section ? ' · ' + anchor.section : '')
  if (anchor.type === 'annotation') return 'Annotation #' + anchor.value
  if (anchor.type === 'step')       return 'Step ' + (Number(anchor.value) + 1)
  if (anchor.type === 'section')    return 'Section · ' + anchor.value
  return 'Reference'
}

function toggleForm() {
  showForm.value = !showForm.value
  if (!showForm.value) { form.value.anchor = null }
  else nextTick(() => authorInputEl.value?.focus())
}

function relativeTime(iso) {
  const diff = Math.floor((Date.now() - new Date(iso)) / 60000)
  if (diff < 2)  return t('time.justNow')
  if (diff < 60) return t('time.minutesAgo', { n: diff })
  return t('time.hoursAgo', { n: Math.floor(diff / 60) })
}

async function submitComment() {
  if (!form.value.author || !form.value.content) return
  const reference = form.value.anchor
    ? { ...form.value.anchor }
    : form.value.reference
      ? { type: 'section', value: form.value.reference }
      : { type: 'general', value: null }

  await state.addComment(props.itemId, { author: form.value.author, content: form.value.content, reference })
  form.value = { author: '', content: '', reference: '', anchor: null }
  showForm.value = false
  await refreshExport()
}

async function resolve(commentId) {
  await state.resolveComment(props.itemId, commentId)
  if (!openComments.value.length) tab.value = 'resolved'
  await refreshExport()
}

async function archiveResolved() {
  await state.archiveResolved(props.itemId)
  tab.value = 'open'
  await refreshExport()
}
</script>

<style scoped>
.panel { display: flex; flex-direction: column; height: 100%; overflow: hidden; background: var(--bg-card); }

.tabs { display: flex; border-bottom: 1px solid var(--border); flex-shrink: 0; }
.tab {
  flex: 1; padding: 12px; font-size: 13px; font-weight: 500;
  background: none; border: none; cursor: pointer; color: var(--text-dim);
  border-bottom: 2px solid transparent; transition: all 0.15s;
}
.tab.active { color: var(--text); border-bottom-color: var(--accent); }
.tab:hover  { color: var(--text); background: var(--accent-soft); }

.actions-bar { display: flex; gap: 8px; padding: 10px 12px; border-bottom: 1px solid var(--border-light); flex-shrink: 0; }
.btn-primary {
  font-size: 13px; font-weight: 500; padding: 6px 12px;
  background: transparent; color: var(--accent); border: 1px solid var(--accent); cursor: pointer; transition: all 0.15s;
  letter-spacing: 0.04em;
}
.btn-primary:hover { background: var(--accent); color: var(--bg); }
.btn-archive {
  font-size: 12px; padding: 6px 10px;
  background: transparent; color: var(--amber); border: 1px solid var(--border); cursor: pointer; transition: all 0.15s;
}
.btn-archive:hover { background: var(--amber-soft); color: var(--amber); border-color: var(--amber); }

.add-form {
  padding: 12px; border-bottom: 1px solid var(--border-light);
  display: flex; flex-direction: column; gap: 8px;
  background: var(--bg-alt); flex-shrink: 0; overflow: hidden;
}
.form-slide-enter-active, .form-slide-leave-active { transition: max-height 0.22s ease, opacity 0.15s; overflow: hidden; }
.form-slide-enter-from, .form-slide-leave-to { max-height: 0; opacity: 0; }
.form-slide-enter-to, .form-slide-leave-from { max-height: 500px; opacity: 1; }

.anchor-preview {
  padding: 8px 10px; display: flex; flex-direction: column; gap: 4px; border-left: 3px solid;
}
.anchor-quote      { background: var(--accent-soft); border-color: var(--accent); }
.anchor-annotation { background: var(--accent-soft); border-color: var(--accent); }
.anchor-step       { background: var(--green-soft); border-color: var(--green); }
.anchor-section    { background: var(--amber-soft); border-color: var(--amber); }

.anchor-label { display: flex; align-items: center; gap: 5px; font-size: 11px; font-weight: 600; color: var(--text-dim); }
.anchor-quote .anchor-label      { color: var(--accent); }
.anchor-annotation .anchor-label { color: var(--accent); }
.anchor-step .anchor-label       { color: var(--green); }
.anchor-section .anchor-label    { color: var(--amber); }
.anchor-icon { font-size: 13px; }
.anchor-remove {
  margin-left: auto; background: none; border: none; cursor: pointer;
  font-size: 15px; line-height: 1; color: var(--text-faint); padding: 0; transition: color 0.1s;
}
.anchor-remove:hover { color: var(--accent); }
.anchor-text {
  font-size: 12px; font-style: italic; line-height: 1.5; margin: 0;
  display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
}

.field { display: flex; flex-direction: column; gap: 3px; }
.field label { font-size: 12px; font-weight: 500; color: var(--text-dim); letter-spacing: 0.06em; text-transform: uppercase; font-size: 10px; }
.field input, .field textarea {
  padding: 7px 0; border: none; border-bottom: 1px solid var(--border);
  background: transparent; font-size: 13px; resize: vertical;
  font-family: var(--serif); transition: border-color 0.15s;
}
.field input:focus, .field textarea:focus { outline: none; border-bottom-color: var(--accent); }
.btn-submit {
  align-self: flex-end; padding: 6px 14px; background: var(--text); color: var(--bg);
  border: 1px solid var(--text); font-size: 13px; font-weight: 500; cursor: pointer;
  transition: all 0.15s; letter-spacing: 0.04em;
}
.btn-submit:hover:not(:disabled) { background: var(--accent); border-color: var(--accent); }
.btn-submit:disabled { opacity: 0.4; cursor: not-allowed; }

.comments { flex: 1; overflow-y: auto; padding: 8px 12px; display: flex; flex-direction: column; gap: 8px; }
.empty { text-align: center; color: var(--text-faint); font-size: 13px; padding: 24px 0; }

.comment-card {
  background: var(--bg); border: 1px solid var(--border-light);
  padding: 12px; display: flex; flex-direction: column; gap: 6px;
  transition: border-color 0.15s, box-shadow 0.2s, background 0.2s; cursor: default;
  border-left: 2px solid var(--accent);
}
.comment-card.open     { border-left: 2px solid var(--accent); }
.comment-card.resolved { border-left: 2px solid var(--green); opacity: 0.65; }
.comment-card:hover    { border-color: var(--accent); }

.comment-card.section-match {
  background: var(--accent-soft);
  box-shadow: 0 0 0 2px var(--accent-med);
  opacity: 1 !important;
}

.c-meta { display: flex; align-items: center; gap: 8px; }
.c-author { font-size: 13px; font-weight: 600; color: var(--text); }
.c-time   { font-size: 11px; color: var(--text-faint); margin-left: auto; }
.resolved-badge { font-size: 11px; color: var(--green); font-weight: 600; }

:deep(.c-quote) {
  margin: 0; padding: 6px 10px;
  background: var(--accent-soft); border-left: 3px solid var(--accent);
  font-size: 12.5px; font-style: italic; color: var(--text-dim); line-height: 1.5;
}
:deep(.c-quote-section) {
  display: block; font-size: 10px; font-weight: 600; font-style: normal;
  text-transform: uppercase; letter-spacing: 0.08em; color: var(--accent); margin-bottom: 3px;
}
:deep(.c-ref) {
  display: flex; align-items: center; gap: 6px; flex-wrap: wrap;
  font-size: 12px; color: var(--text-dim); background: var(--bg-alt); padding: 3px 8px;
}
:deep(.ref-badge) { font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em; font-weight: 600; color: var(--text-faint); }
:deep(.ref-label) { color: var(--text); font-weight: 500; }

.c-content { font-size: 14px; color: var(--text-dim); line-height: 1.5; }

.btn-resolve {
  align-self: flex-end; font-size: 12px; padding: 4px 10px;
  border: 1px solid var(--green); background: transparent;
  color: var(--green); cursor: pointer; font-weight: 500; transition: all 0.15s;
}
.btn-resolve:hover { background: var(--green-soft); border-color: var(--green); }

.export-block { border-top: 1px solid var(--border); padding: 10px 12px; flex-shrink: 0; }
.export-block summary { font-size: 12px; font-weight: 500; color: var(--text-dim); cursor: pointer; user-select: none; letter-spacing: 0.04em; }
.export-block summary:hover { color: var(--text); }
.export-json {
  margin-top: 8px; background: var(--bg-dark); color: var(--border-light);
  padding: 12px; font-size: 11px; border: 1px solid #454138;
  overflow-x: auto; max-height: 280px; overflow-y: auto;
  font-family: var(--mono);
}
</style>
