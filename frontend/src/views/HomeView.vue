<template>
  <div class="home">
    <!-- Hero -->
    <div class="hero">
      <div class="hero-text">
        <h1 class="hero-title">{{ t('dashboard.welcome') }}</h1>
        <p class="hero-tagline">{{ t('dashboard.tagline') }}</p>
      </div>
      <div class="hero-badge">
        <span class="badge-r">R</span>
      </div>
    </div>

    <!-- Stats row -->
    <div class="stats-row">
      <div class="stat-card">
        <span class="stat-num">{{ state.items.length }}</span>
        <span class="stat-label">{{ t('dashboard.totalItems') }}</span>
      </div>
      <div class="stat-card accent-red">
        <span class="stat-num">{{ totalOpen }}</span>
        <span class="stat-label">{{ t('dashboard.openComments') }}</span>
      </div>
      <div class="stat-card accent-green">
        <span class="stat-num">{{ totalResolved }}</span>
        <span class="stat-label">{{ t('dashboard.resolvedComments') }}</span>
      </div>
      <div class="stat-card accent-gray">
        <span class="stat-num">{{ totalArchived }}</span>
        <span class="stat-label">{{ t('nav.archive') }}</span>
      </div>
    </div>

    <!-- Loading / Error -->
    <div v-if="state.loading" class="loading">Loading...</div>
    <div v-if="state.error" class="error">{{ state.error }}</div>

    <!-- All items -->
    <div class="section">
      <div class="section-header">
        <h2 class="section-title">{{ t('dashboard.allItems') }}</h2>
        <div class="upload-actions">
          <button class="upload-btn" @click="$refs.fileInput.click()">
            <span>📄</span> {{ t('home.openFile') }}
          </button>
          <button class="upload-btn" @click="$refs.folderInput.click()">
            <span>📁</span> {{ t('home.openFolder') }}
          </button>
          <input ref="fileInput" type="file" multiple style="display:none"
            accept=".md,.html,.htm,.png,.jpg,.jpeg,.svg,.webp,.pdf"
            @change="onFilesSelected($event.target.files); $refs.fileInput.value=''" />
          <input ref="folderInput" type="file" webkitdirectory style="display:none"
            @change="onFilesSelected($event.target.files); $refs.folderInput.value=''" />
        </div>

        <div v-if="pendingFiles.length" class="upload-modal-overlay" @click.self="pendingFiles = []">
          <div class="upload-modal">
            <h3 class="modal-title">{{ t('home.uploadFiles') }}</h3>
            <div class="file-list">
              <div v-for="(pf, i) in pendingFiles" :key="i" class="file-row">
                <span class="file-name">{{ pf.file.name }}</span>
                <select v-model="pf.type" class="type-select">
                  <option value="auto">{{ t('home.typeAuto') }} ({{ pf.detected }})</option>
                  <option value="plan">{{ t('home.typePlan') }}</option>
                  <option value="design">{{ t('home.typeDesign') }}</option>
                  <option value="prototype">{{ t('home.typePrototype') }}</option>
                </select>
                <button class="remove-btn" @click="pendingFiles.splice(i, 1)">✕</button>
              </div>
            </div>
            <div class="modal-footer">
              <button class="cancel-btn" @click="pendingFiles = []">{{ t('review.cancel') }}</button>
              <button class="confirm-btn" :disabled="uploading" @click="uploadPending">
                {{ uploading ? '...' : t('home.uploadConfirm') }}
              </button>
            </div>
          </div>
        </div>
      </div>
      <div class="item-grid">
        <div v-for="item in state.items" :key="item.id" class="item-card">
          <!-- Card color stripe -->
          <div class="card-stripe" :class="item.type"></div>

          <div class="card-body">
            <div class="card-top">
              <span class="type-badge" :class="item.type">
                <span class="type-icon">{{ typeIcon(item.type) }}</span>
                {{ t('home.type.' + item.type) }}
              </span>
              <span class="updated-at">{{ relativeTime(item.updatedAt) }}</span>
            </div>

            <h3 class="card-title">{{ locale === 'zh' ? item.titleZh : item.title }}</h3>
            <p class="card-desc">{{ locale === 'zh' ? item.descriptionZh : item.description }}</p>

            <div class="card-footer">
              <!-- Comment status -->
              <div class="comment-status">
                <span v-if="state.openCount(item.id) > 0" class="status-pill open">
                  <span class="status-dot"></span>
                  {{ state.openCount(item.id) }} {{ t('home.openComments') }}
                </span>
                <span v-if="state.resolvedCount(item.id) > 0" class="status-pill resolved">
                  ✓ {{ state.resolvedCount(item.id) }}
                </span>
                <span v-if="!state.openCount(item.id) && !state.resolvedCount(item.id)" class="status-pill empty">
                  {{ t('home.noComments') }}
                </span>
              </div>

              <!-- Open button -->
              <router-link :to="'/review/' + item.id" class="open-btn">
                {{ t('dashboard.open') }}
                <span class="btn-arrow">→</span>
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>


<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'
import { api } from '../api/client.js'

const { t, locale } = useI18n()

onMounted(() => state.fetchItems())

const pendingFiles = ref([])
const uploading = ref(false)

const EXT_TYPE = {
  '.md': 'plan', '.html': 'prototype', '.htm': 'prototype',
  '.png': 'design', '.jpg': 'design', '.jpeg': 'design',
  '.svg': 'design', '.webp': 'design', '.pdf': 'design',
}

function detectType(filename) {
  const lastDot = filename.lastIndexOf('.')
  if (lastDot === -1) return null
  const ext = filename.slice(lastDot).toLowerCase()
  return EXT_TYPE[ext] || null
}

function onFilesSelected(fileList) {
  const supported = ['md', 'html', 'htm', 'png', 'jpg', 'jpeg', 'svg', 'webp', 'pdf']
  for (const file of fileList) {
    const lastDot = file.name.lastIndexOf('.')
    const ext = lastDot !== -1 ? file.name.slice(lastDot + 1).toLowerCase() : ''
    if (!supported.includes(ext)) continue
    const detected = detectType(file.name) || 'plan'
    pendingFiles.value.push({ file, detected, type: 'auto' })
  }
}

async function uploadPending() {
  if (!pendingFiles.value.length) return
  uploading.value = true
  try {
    for (const pf of pendingFiles.value) {
      const fd = new FormData()
      fd.append('file', pf.file)
      if (pf.type !== 'auto') fd.append('type', pf.type)
      await api.upload(fd)
    }
    pendingFiles.value = []
    await state.fetchItems()
  } catch (e) {
    state.error = e.message
  } finally {
    uploading.value = false
  }
}

const totalOpen     = computed(() => state.items.reduce((s, i) => s + state.openCount(i.id), 0))
const totalResolved = computed(() => state.items.reduce((s, i) => s + state.resolvedCount(i.id), 0))
const totalArchived = computed(() =>
  state.items.reduce((s, i) => s + (state.archive[i.id] || []).length, 0))

function typeIcon(type) {
  return { plan: '📄', design: '🎨', prototype: '⚡' }[type] || '📁'
}

function relativeTime(iso) {
  const diff = Math.floor((Date.now() - new Date(iso)) / 60000)
  if (diff < 2) return t('time.justNow')
  if (diff < 60) return t('time.minutesAgo', { n: diff })
  return t('time.hoursAgo', { n: Math.floor(diff / 60) })
}
</script>

<style scoped>
.home {
  height: 100%;
  overflow-y: auto;
  padding: 28px 32px;
  display: flex;
  flex-direction: column;
  gap: 28px;
}

/* Hero */
.hero {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: linear-gradient(135deg, #eff6ff 0%, #faf5ff 100%);
  border: 1px solid #e0e7ff;
  border-radius: 16px;
  padding: 28px 32px;
}
.hero-title {
  font-size: 24px;
  font-weight: 800;
  color: #1a1a1a;
  letter-spacing: -0.5px;
  margin-bottom: 6px;
}
.hero-tagline {
  font-size: 14px;
  color: #6b7280;
  line-height: 1.5;
  max-width: 360px;
}
.hero-badge {
  width: 64px;
  height: 64px;
  border-radius: 18px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.badge-r {
  font-size: 32px;
  font-weight: 900;
  color: #fff;
}

/* Stats */
.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}
.stat-card {
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  padding: 16px 20px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  transition: box-shadow 0.15s;
}
.stat-card:hover { box-shadow: 0 2px 12px rgba(0,0,0,0.06); }
.stat-num {
  font-size: 28px;
  font-weight: 800;
  color: #1a1a1a;
  line-height: 1;
}
.stat-label {
  font-size: 12px;
  color: #9ca3af;
  font-weight: 500;
}
.stat-card.accent-red  .stat-num { color: #dc2626; }
.stat-card.accent-green .stat-num { color: #16a34a; }
.stat-card.accent-gray  .stat-num { color: #6b7280; }

/* Loading / Error */
.loading { font-size: 14px; color: #6b7280; padding: 8px 0; }
.error   { font-size: 14px; color: #dc2626; padding: 8px 12px; background: #fee2e2; border-radius: 8px; }

/* Section header with upload */
.section-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px; }
.section-header .section-title { margin-bottom: 0; }

.btn-upload {
  font-size: 13px; font-weight: 600; padding: 6px 14px; border-radius: 8px;
  background: #f3f4f6; color: #374151; border: 1px solid #e5e7eb; cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}
.btn-upload:hover { background: #e5e7eb; border-color: #d1d5db; }

.upload-actions { display: flex; gap: 8px; }
.upload-btn {
  display: flex; align-items: center; gap: 5px;
  padding: 6px 14px; border-radius: 7px; border: 1px solid #e5e7eb;
  background: #fff; color: #6b7280; font-size: 13px; cursor: pointer;
  transition: all 0.15s;
}
.upload-btn:hover { border-color: #d1d5db; background: #f9fafb; color: #1a1a1a; }
.upload-modal-overlay {
  position: fixed; inset: 0; background: rgba(0,0,0,0.35);
  display: flex; align-items: center; justify-content: center; z-index: 100;
}
.upload-modal {
  background: #fff; border-radius: 12px; padding: 24px; min-width: 420px;
  max-width: 560px; box-shadow: 0 8px 32px rgba(0,0,0,0.15);
}
.modal-title { font-size: 15px; font-weight: 700; margin: 0 0 16px; color: #1a1a1a; }
.file-list { display: flex; flex-direction: column; gap: 8px; max-height: 320px; overflow-y: auto; }
.file-row { display: flex; align-items: center; gap: 8px; }
.file-name { flex: 1; font-size: 13px; color: #374151; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.type-select { padding: 4px 8px; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 12px; color: #374151; }
.remove-btn { background: none; border: none; color: #9ca3af; cursor: pointer; font-size: 14px; padding: 2px 4px; }
.remove-btn:hover { color: #ef4444; }
.modal-footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; }
.cancel-btn {
  padding: 7px 16px; border-radius: 7px; border: 1px solid #e5e7eb;
  background: #fff; color: #6b7280; font-size: 13px; cursor: pointer;
}
.confirm-btn {
  padding: 7px 16px; border-radius: 7px; border: none;
  background: #3b82f6; color: #fff; font-size: 13px; font-weight: 600; cursor: pointer;
}
.confirm-btn:disabled { opacity: 0.5; cursor: default; }

/* Section */
.section-title {
  font-size: 14px;
  font-weight: 700;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 14px;
}

/* Item grid */
.item-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 14px;
}

.item-card {
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 14px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: box-shadow 0.15s, border-color 0.15s, transform 0.15s;
}
.item-card:hover {
  box-shadow: 0 6px 24px rgba(0,0,0,0.08);
  border-color: #d1d5db;
  transform: translateY(-2px);
}

.card-stripe { height: 4px; }
.card-stripe.plan      { background: linear-gradient(90deg, #3b82f6, #60a5fa); }
.card-stripe.design    { background: linear-gradient(90deg, #a855f7, #c084fc); }
.card-stripe.prototype { background: linear-gradient(90deg, #22c55e, #4ade80); }

.card-body { padding: 18px 20px; display: flex; flex-direction: column; gap: 10px; flex: 1; }

.card-top { display: flex; align-items: center; justify-content: space-between; }
.type-badge {
  display: flex; align-items: center; gap: 5px;
  font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px;
  padding: 3px 9px; border-radius: 20px;
}
.type-badge.plan      { background: #eff6ff; color: #3b82f6; }
.type-badge.design    { background: #fdf4ff; color: #a855f7; }
.type-badge.prototype { background: #f0fdf4; color: #22c55e; }
.type-icon { font-size: 12px; }
.updated-at { font-size: 11px; color: #9ca3af; }

.card-title { font-size: 15px; font-weight: 700; color: #1a1a1a; line-height: 1.4; }
.card-desc  { font-size: 13px; color: #6b7280; line-height: 1.5; flex: 1; }

.card-footer {
  display: flex; align-items: center; justify-content: space-between;
  padding-top: 10px; border-top: 1px solid #f3f4f6; margin-top: 4px;
}

.comment-status { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.status-pill {
  display: flex; align-items: center; gap: 4px;
  font-size: 11px; font-weight: 500; padding: 3px 8px; border-radius: 20px;
}
.status-pill.open     { background: #fee2e2; color: #dc2626; }
.status-pill.resolved { background: #dcfce7; color: #16a34a; }
.status-pill.empty    { background: #f3f4f6; color: #9ca3af; }
.status-dot {
  width: 5px; height: 5px; border-radius: 50%;
  background: #dc2626; animation: pulse 2s infinite;
}
@keyframes pulse {
  0%, 100% { opacity: 1; } 50% { opacity: 0.4; }
}

.open-btn {
  display: flex; align-items: center; gap: 5px;
  padding: 7px 14px; border-radius: 8px;
  background: #1a1a1a; color: #fff;
  font-size: 13px; font-weight: 600;
  text-decoration: none;
  transition: background 0.15s, transform 0.1s;
  flex-shrink: 0;
}
.open-btn:hover { background: #374151; }
.open-btn:active { transform: scale(0.97); }
.btn-arrow { font-size: 14px; transition: transform 0.15s; }
.open-btn:hover .btn-arrow { transform: translateX(3px); }
</style>
