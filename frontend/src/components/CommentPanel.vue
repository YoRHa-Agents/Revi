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
.panel { display: flex; flex-direction: column; height: 100%; overflow: hidden; background: #fff; }

.tabs { display: flex; border-bottom: 1px solid #e5e7eb; flex-shrink: 0; }
.tab {
  flex: 1; padding: 12px; font-size: 13px; font-weight: 500;
  background: none; border: none; cursor: pointer; color: #6b7280;
  border-bottom: 2px solid transparent; transition: all 0.15s;
}
.tab.active { color: #1a1a1a; border-bottom-color: #3b82f6; }
.tab:hover  { color: #1a1a1a; background: #f9fafb; }

.actions-bar { display: flex; gap: 8px; padding: 10px 12px; border-bottom: 1px solid #f3f4f6; flex-shrink: 0; }
.btn-primary {
  font-size: 13px; font-weight: 500; padding: 6px 12px; border-radius: 6px;
  background: #3b82f6; color: #fff; border: none; cursor: pointer; transition: background 0.15s;
}
.btn-primary:hover { background: #2563eb; }
.btn-archive {
  font-size: 12px; padding: 6px 10px; border-radius: 6px;
  background: #f3f4f6; color: #6b7280; border: 1px solid #e5e7eb; cursor: pointer; transition: all 0.15s;
}
.btn-archive:hover { background: #fef3c7; color: #d97706; border-color: #fde68a; }

/* Form */
.add-form {
  padding: 12px; border-bottom: 1px solid #f3f4f6;
  display: flex; flex-direction: column; gap: 8px;
  background: #f9fafb; flex-shrink: 0; overflow: hidden;
}
.form-slide-enter-active, .form-slide-leave-active { transition: max-height 0.22s ease, opacity 0.15s; overflow: hidden; }
.form-slide-enter-from, .form-slide-leave-to { max-height: 0; opacity: 0; }
.form-slide-enter-to, .form-slide-leave-from { max-height: 500px; opacity: 1; }

/* Anchor preview */
.anchor-preview {
  border-radius: 0 6px 6px 0; padding: 8px 10px;
  display: flex; flex-direction: column; gap: 4px; border-left: 3px solid;
}
.anchor-quote      { background: #eff6ff; border-color: #3b82f6; }
.anchor-annotation { background: #fdf4ff; border-color: #a855f7; }
.anchor-step       { background: #f0fdf4; border-color: #22c55e; }
.anchor-section    { background: #fefce8; border-color: #eab308; }

.anchor-label {
  display: flex; align-items: center; gap: 5px;
  font-size: 11px; font-weight: 600; color: #374151;
}
.anchor-quote .anchor-label      { color: #1d4ed8; }
.anchor-annotation .anchor-label { color: #7e22ce; }
.anchor-step .anchor-label       { color: #15803d; }
.anchor-section .anchor-label    { color: #a16207; }
.anchor-icon { font-size: 13px; }
.anchor-remove {
  margin-left: auto; background: none; border: none; cursor: pointer;
  font-size: 15px; line-height: 1; color: #9ca3af; padding: 0; transition: color 0.1s;
}
.anchor-remove:hover { color: #ef4444; }
.anchor-text {
  font-size: 12px; font-style: italic; line-height: 1.5; margin: 0;
  display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
}

.field { display: flex; flex-direction: column; gap: 3px; }
.field label { font-size: 12px; font-weight: 500; color: #6b7280; }
.field input, .field textarea {
  padding: 7px 10px; border: 1px solid #e5e7eb; border-radius: 6px;
  font-size: 13px; resize: vertical; font-family: inherit; transition: border-color 0.15s;
}
.field input:focus, .field textarea:focus { outline: none; border-color: #93c5fd; }
.btn-submit {
  align-self: flex-end; padding: 6px 14px; background: #10b981; color: #fff;
  border: none; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; transition: background 0.15s;
}
.btn-submit:hover:not(:disabled) { background: #059669; }
.btn-submit:disabled { opacity: 0.4; cursor: not-allowed; }

/* Comments */
.comments { flex: 1; overflow-y: auto; padding: 8px 12px; display: flex; flex-direction: column; gap: 8px; }
.empty { text-align: center; color: #9ca3af; font-size: 13px; padding: 24px 0; }

.comment-card {
  background: #fff; border: 1px solid #e5e7eb; border-radius: 8px;
  padding: 12px; display: flex; flex-direction: column; gap: 6px;
  transition: border-color 0.15s, box-shadow 0.2s, background 0.2s;
  cursor: default;
}
.comment-card.open     { border-left: 3px solid #ef4444; }
.comment-card.resolved { border-left: 3px solid #10b981; opacity: 0.75; }
.comment-card:hover    { border-color: #d1d5db; }

/* Section match: comment is relevant to current scroll position */
.comment-card.section-match {
  background: #f8faff;
  box-shadow: 0 0 0 2px #bfdbfe;
  opacity: 1 !important;
}

.c-meta { display: flex; align-items: center; gap: 8px; }
.c-author { font-size: 13px; font-weight: 600; color: #1a1a1a; }
.c-time   { font-size: 11px; color: #9ca3af; margin-left: auto; }
.resolved-badge { font-size: 11px; color: #10b981; font-weight: 600; }

/* Quote reference */
:deep(.c-quote) {
  margin: 0; padding: 6px 10px;
  background: #f0f9ff; border-left: 3px solid #38bdf8;
  border-radius: 0 5px 5px 0;
  font-size: 12.5px; font-style: italic; color: #0c4a6e; line-height: 1.5;
}
:deep(.c-quote-section) {
  display: block; font-size: 10px; font-weight: 700; font-style: normal;
  text-transform: uppercase; letter-spacing: 0.4px;
  color: #38bdf8; margin-bottom: 3px;
}

/* Other ref types */
:deep(.c-ref) {
  display: flex; align-items: center; gap: 6px; flex-wrap: wrap;
  font-size: 12px; color: #6b7280; background: #f9fafb;
  padding: 3px 8px; border-radius: 4px;
}
:deep(.ref-badge) {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.5px; font-weight: 600; color: #9ca3af;
}
:deep(.ref-label) { color: #374151; font-weight: 500; }

.c-content { font-size: 14px; color: #374151; line-height: 1.5; }

.btn-resolve {
  align-self: flex-end; font-size: 12px; padding: 4px 10px;
  border-radius: 5px; border: 1px solid #d1fae5; background: #ecfdf5;
  color: #059669; cursor: pointer; font-weight: 500; transition: all 0.15s;
}
.btn-resolve:hover { background: #d1fae5; border-color: #6ee7b7; }

.export-block { border-top: 1px solid #e5e7eb; padding: 10px 12px; flex-shrink: 0; }
.export-block summary { font-size: 12px; font-weight: 500; color: #6b7280; cursor: pointer; user-select: none; }
.export-block summary:hover { color: #1a1a1a; }
.export-json {
  margin-top: 8px; background: #1e1e2e; color: #cdd6f4;
  border-radius: 6px; padding: 12px; font-size: 11px;
  overflow-x: auto; max-height: 280px; overflow-y: auto;
  font-family: 'SF Mono', Consolas, monospace;
}
</style>
