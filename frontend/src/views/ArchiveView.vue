<template>
  <div class="archive-page">
    <h1 class="page-title">{{ t('archive.title') }}</h1>

    <div v-if="!hasAny" class="empty">{{ t('archive.empty') }}</div>

    <div v-for="item in state.items" :key="item.id">
      <div v-if="(state.archive[item.id] || []).length">
        <div class="item-section">
          <div class="item-label">
            <span class="type-badge" :class="item.type">{{ t('home.type.' + item.type) }}</span>
            <span class="item-title">{{ locale === 'zh' ? item.titleZh : item.title }}</span>
          </div>

          <div v-for="(batch, bi) in state.archive[item.id]" :key="bi" class="batch">
            <div class="batch-header">
              <span class="batch-label">{{ t('archive.batch') }} #{{ state.archive[item.id].length - bi }}</span>
              <span class="batch-meta">
                {{ t('archive.archivedAt') }}: {{ formatDate(batch.archivedAt) }}
                · {{ batch.comments.length }} {{ t('archive.comments') }}
              </span>
            </div>
            <div class="batch-comments">
              <div v-for="c in batch.comments" :key="c.id" class="archived-comment">
                <div class="c-meta">
                  <span class="c-author">{{ c.author }}</span>
                  <span class="resolved-badge">✓ resolved</span>
                  <span class="c-time">{{ formatDate(c.resolvedAt) }}</span>
                </div>
                <div v-if="c.reference?.value" class="c-ref">
                  <span class="ref-badge">{{ c.reference.type }}</span>
                  {{ c.reference.value }}
                </div>
                <p class="c-content">{{ locale === 'zh' ? c.contentZh : c.content }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'

const { t, locale } = useI18n()

onMounted(async () => {
  if (!state.items.length) await state.fetchItems()
  await Promise.all(state.items.map(item => state.fetchArchive(item.id)))
})

const hasAny = computed(() =>
  state.items.some(item => (state.archive[item.id] || []).length > 0))

function formatDate(iso) {
  if (!iso) return '—'
  return new Date(iso).toLocaleString()
}
</script>

<style scoped>
.archive-page { height: 100%; overflow-y: auto; padding: 28px 32px; }
.page-title { font-size: 22px; font-weight: 400; margin-bottom: 20px; color: var(--text); letter-spacing: 0.04em; }
.empty { color: var(--text-faint); font-size: 14px; padding: 32px 0; text-align: center; }

.item-section { margin-bottom: 28px; }
.item-label { display: flex; align-items: center; gap: 10px; margin-bottom: 12px; }
.item-title { font-size: 16px; font-weight: 500; color: var(--text); letter-spacing: 0.02em; }

.type-badge {
  font-size: 11px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.08em;
  padding: 2px 8px; border: 1px solid var(--border); color: var(--accent);
}
.type-badge.plan, .type-badge.design, .type-badge.prototype { background: transparent; }

.batch { background: var(--bg-card); border: 1px solid var(--border); overflow: hidden; margin-bottom: 12px; }
.batch-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 16px; background: var(--bg-alt); border-bottom: 1px solid var(--border);
}
.batch-label { font-size: 13px; font-weight: 500; color: var(--text-dim); }
.batch-meta  { font-size: 12px; color: var(--text-faint); }

.batch-comments { padding: 8px 12px; display: flex; flex-direction: column; gap: 8px; }

.archived-comment {
  padding: 10px 12px; border: 1px solid var(--border); border-left: 3px solid var(--green);
  opacity: 0.8; display: flex; flex-direction: column; gap: 5px;
}

.c-meta { display: flex; align-items: center; gap: 8px; }
.c-author { font-size: 13px; font-weight: 600; color: var(--text); }
.c-time   { font-size: 11px; color: var(--text-faint); margin-left: auto; }
.resolved-badge { font-size: 11px; color: var(--green); font-weight: 600; }

.c-ref {
  display: flex; align-items: center; gap: 6px;
  font-size: 12px; color: var(--text-dim); background: var(--bg-alt); padding: 3px 8px;
}
.ref-badge { font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em; font-weight: 600; color: var(--text-faint); }
.c-content { font-size: 14px; color: var(--text-dim); line-height: 1.5; }
</style>
