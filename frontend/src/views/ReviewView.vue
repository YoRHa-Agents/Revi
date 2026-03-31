<template>
  <div class="review-page">
    <div v-if="!item" class="not-found">Item not found.</div>
    <template v-else>
      <div class="review-header">
        <div class="header-info">
          <select
            class="type-badge type-badge-select"
            :class="item.type"
            :value="item.type"
            @change="changeItemType($event.target.value)"
            :title="t('review.changeType')"
          >
            <option value="plan">{{ t('home.type.plan') }}</option>
            <option value="design">{{ t('home.type.design') }}</option>
            <option value="prototype">{{ t('home.type.prototype') }}</option>
          </select>
          <h1 class="review-title">{{ locale === 'zh' ? item.titleZh : item.title }}</h1>
        </div>
        <div class="header-actions">
          <button v-if="item.type === 'plan'" class="action-btn" :class="{ active: showIndex }" @click="showIndex = !showIndex" :title="t('docIndex.title')">
            <span class="action-icon">☰</span>{{ t('docIndex.title') }}
          </button>
          <button v-if="item.type === 'plan'" class="action-btn" :class="{ active: showSearch }" @click="showSearch = !showSearch" :title="t('review.search')">
            <span class="action-icon">⌕</span>{{ t('review.search') }}
          </button>
        </div>
      </div>

      <div class="split-layout">
        <transition name="index-slide">
          <DocIndex v-if="showIndex && item.type === 'plan'" :content="itemDetail?.contentText ?? ''" @hide="showIndex = false" />
        </transition>

        <div class="content-pane">
          <MarkdownViewer
            v-if="item.type === 'plan'"
            :content="itemDetail?.contentText ?? ''"
            v-model:showSearch="showSearch"
            :hoveredAnchor="hoveredAnchor"
            @anchor-select="pendingAnchor = $event"
            @section-change="activeSection = $event"
          />
          <ImageViewer
            v-else-if="item.type === 'design'"
            :item="item"
            :hoveredAnchor="hoveredAnchor"
            @anchor-select="pendingAnchor = $event"
          />
          <PrototypeViewer
            v-else-if="item.type === 'prototype'"
            :item="item"
            :hoveredAnchor="hoveredAnchor"
            @anchor-select="pendingAnchor = $event"
          />
        </div>

        <div class="comment-pane">
          <CommentPanel
            :itemId="item.id"
            :pendingAnchor="pendingAnchor"
            :activeSection="activeSection"
            @anchor-consumed="pendingAnchor = null"
            @anchor-hover="hoveredAnchor = $event"
            @anchor-leave="hoveredAnchor = null"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'
import { api } from '../api/client.js'
import CommentPanel    from '../components/CommentPanel.vue'
import DocIndex        from '../components/DocIndex.vue'
import MarkdownViewer  from '../components/viewers/MarkdownViewer.vue'
import ImageViewer     from '../components/viewers/ImageViewer.vue'
import PrototypeViewer from '../components/viewers/PrototypeViewer.vue'

const route = useRoute()
const { t, locale } = useI18n()

const showIndex    = ref(false)
const showSearch   = ref(false)
const pendingAnchor = ref(null)
const hoveredAnchor = ref(null)
const activeSection = ref(null)
const itemDetail   = ref(null)

const item = computed(() => {
  const id = Array.isArray(route.params.itemId)
    ? route.params.itemId.join('/')
    : route.params.itemId
  return state.getItem(id)
})

// Fetch comments and item detail when item changes
watch(item, async (newItem) => {
  hoveredAnchor.value = null
  pendingAnchor.value = null
  activeSection.value = null
  if (newItem) {
    await state.fetchComments(newItem.id)
    itemDetail.value = await api.getReview(newItem.id)
  }
}, { immediate: true })

onMounted(async () => {
  if (state.items.length === 0) {
    await state.fetchItems()
  }
  // Also fetch comments for current item if we have it
  if (item.value) {
    await state.fetchComments(item.value.id)
    itemDetail.value = await api.getReview(item.value.id)
  }
})

async function changeItemType(newType) {
  if (!item.value || newType === item.value.type) return
  await state.updateItemType(item.value.id, newType)
}
</script>

<style scoped>
.review-page {
  display: flex; flex-direction: column;
  height: 100%; overflow: hidden; padding: 16px 20px 20px;
}
.review-header {
  display: flex; align-items: center; justify-content: space-between;
  padding-bottom: 12px; border-bottom: 1px solid #e5e7eb; flex-shrink: 0;
}
.header-info    { display: flex; align-items: center; gap: 10px; }
.header-actions { display: flex; align-items: center; gap: 8px; }

.type-badge {
  font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;
  padding: 3px 8px; border-radius: 20px;
}
.type-badge.plan      { background: #eff6ff; color: #3b82f6; }
.type-badge.design    { background: #fdf4ff; color: #a855f7; }
.type-badge.prototype { background: #f0fdf4; color: #22c55e; }
.type-badge-select {
  appearance: none; -webkit-appearance: none;
  cursor: pointer; border: none; outline: none;
  font-family: inherit;
}
.review-title { font-size: 17px; font-weight: 700; color: #1a1a1a; }

.action-btn {
  display: flex; align-items: center; gap: 5px;
  padding: 5px 11px; border-radius: 7px; border: 1px solid #e5e7eb;
  background: #fff; color: #6b7280; font-size: 12.5px; font-weight: 500;
  cursor: pointer; transition: all 0.15s;
}
.action-btn:hover  { border-color: #d1d5db; color: #1a1a1a; background: #f9fafb; }
.action-btn.active { background: #eff6ff; border-color: #bfdbfe; color: #3b82f6; }
.action-icon { font-size: 13px; }

.split-layout {
  display: flex; flex: 1; overflow: hidden;
  border: 1px solid #e5e7eb; border-radius: 12px;
  margin-top: 12px; background: #fff;
}
.content-pane { flex: 1; min-width: 0; overflow: hidden; border-right: 1px solid #e5e7eb; }
.comment-pane { width: 360px; flex-shrink: 0; overflow: hidden; display: flex; flex-direction: column; }
.not-found { color: #6b7280; font-size: 14px; padding: 24px; }

.index-slide-enter-active, .index-slide-leave-active { transition: width 0.2s ease, opacity 0.15s ease; overflow: hidden; }
.index-slide-enter-from, .index-slide-leave-to  { width: 0; opacity: 0; }
.index-slide-enter-to,   .index-slide-leave-from { width: 180px; opacity: 1; }
</style>
