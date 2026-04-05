<template>
  <div class="doc-index">
    <div class="index-header">
      <span class="index-title">{{ t('docIndex.title') }}</span>
      <button class="hide-btn" @click="emit('hide')" :title="t('docIndex.hide')">←</button>
    </div>
    <nav class="index-nav" role="navigation" aria-label="Document outline">
      <a
        v-for="h in headings"
        :key="h.id"
        :class="['heading-item', 'level-' + h.level]"
        :href="'#' + h.id"
        @click.prevent="scrollTo(h.id)"
      >
        {{ h.text }}
      </a>
    </nav>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  content: { type: String, required: true },
  scrollContainer: { type: Object, default: null },
})
const emit = defineEmits(['hide'])
const { t } = useI18n()

function slugify(text) {
  return text.toLowerCase().replace(/[^\w\u4e00-\u9fff\s-]/g, '').trim().replace(/\s+/g, '-')
}

const headings = computed(() => {
  const lines = props.content.split('\n')
  const result = []
  for (const line of lines) {
    const m = line.match(/^(#{1,3})\s+(.+)/)
    if (m) {
      const text = m[2].trim()
      result.push({ level: m[1].length, text, id: 'h-' + slugify(text) })
    }
  }
  return result
})

function scrollTo(id) {
  const el = document.getElementById(id)
  if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
}
</script>

<style scoped>
.doc-index {
  width: 180px; flex-shrink: 0; border-right: 1px solid var(--border);
  display: flex; flex-direction: column; background: var(--bg-sidebar); overflow: hidden;
}
.index-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 12px 8px; border-bottom: 1px solid var(--border); flex-shrink: 0;
}
.index-title { font-size: 10px; font-weight: 500; letter-spacing: 0.15em; text-transform: uppercase; color: var(--text-faint); }
.hide-btn { background: none; border: none; cursor: pointer; color: var(--text-faint); font-size: 14px; padding: 2px 4px; transition: all 0.1s; line-height: 1; }
.hide-btn:hover { background: var(--accent-soft); color: var(--text); }

.index-nav { flex: 1; overflow-y: auto; padding: 8px 0; display: flex; flex-direction: column; }

.heading-item {
  display: block; font-size: 12px; color: var(--text-dim); text-decoration: none;
  padding: 4px 12px; line-height: 1.4; transition: background 0.1s, color 0.1s;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis; cursor: pointer;
  border-left: 2px solid transparent;
}
.heading-item:hover { background: var(--accent-soft); color: var(--text); border-left-color: var(--accent); }
.heading-item.level-1 { font-weight: 500; color: var(--text); padding-left: 12px; }
.heading-item.level-2 { padding-left: 20px; }
.heading-item.level-3 { padding-left: 28px; font-size: 11px; color: var(--text-faint); }

@media (max-width: 767px) {
  .doc-index { width: 100%; border-right: none; border-bottom: 1px solid var(--border); max-height: 200px; }
}
</style>
