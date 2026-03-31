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
.page-title { font-size: 22px; font-weight: 700; margin-bottom: 20px; color: #1a1a1a; }
.empty { color: #9ca3af; font-size: 14px; padding: 32px 0; text-align: center; }

.item-section { margin-bottom: 28px; }
.item-label { display: flex; align-items: center; gap: 10px; margin-bottom: 12px; }
.item-title { font-size: 16px; font-weight: 600; color: #1a1a1a; }

.type-badge {
  font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;
  padding: 3px 8px; border-radius: 20px;
}
.type-badge.plan      { background: #eff6ff; color: #3b82f6; }
.type-badge.design    { background: #fdf4ff; color: #a855f7; }
.type-badge.prototype { background: #f0fdf4; color: #22c55e; }

.batch {
  background: #fff; border: 1px solid #e5e7eb; border-radius: 10px;
  overflow: hidden; margin-bottom: 12px;
}
.batch-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 16px; background: #f9fafb; border-bottom: 1px solid #e5e7eb;
}
.batch-label { font-size: 13px; font-weight: 600; color: #374151; }
.batch-meta  { font-size: 12px; color: #9ca3af; }

.batch-comments { padding: 8px 12px; display: flex; flex-direction: column; gap: 8px; }

.archived-comment {
  padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 7px;
  border-left: 3px solid #d1fae5; opacity: 0.8;
  display: flex; flex-direction: column; gap: 5px;
}

.c-meta { display: flex; align-items: center; gap: 8px; }
.c-author { font-size: 13px; font-weight: 600; color: #1a1a1a; }
.c-time   { font-size: 11px; color: #9ca3af; margin-left: auto; }
.resolved-badge { font-size: 11px; color: #10b981; font-weight: 600; }

.c-ref {
  display: flex; align-items: center; gap: 6px;
  font-size: 12px; color: #6b7280; background: #f9fafb;
  padding: 3px 8px; border-radius: 4px;
}
.ref-badge {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.5px;
  font-weight: 600; color: #9ca3af;
}
.c-content { font-size: 14px; color: #374151; line-height: 1.5; }
</style>
