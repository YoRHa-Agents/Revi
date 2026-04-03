<template>
  <div class="home">
    <!-- Workspace Setup (shown when not configured) -->
    <div v-if="state.workspaceConfigured === false" class="workspace-setup">
      <div class="setup-card">
        <div class="setup-icon">R</div>
        <h1 class="setup-title">{{ t('workspace.welcome') }}</h1>
        <p class="setup-desc">{{ t('workspace.description') }}</p>

        <div v-if="state.error" class="error">{{ state.error }}</div>

        <!-- Local workspace path -->
        <div class="setup-section">
          <label class="setup-label">{{ t('workspace.localPath') }}</label>
          <div class="input-row">
            <input
              v-model="workspacePath"
              type="text"
              class="setup-input"
              :placeholder="t('workspace.pathPlaceholder')"
              @keyup.enter="applyWorkspace"
            />
            <button
              class="setup-btn primary"
              :disabled="!workspacePath.trim() || state.configLoading"
              @click="applyWorkspace"
            >
              {{ state.configLoading ? '...' : t('workspace.open') }}
            </button>
          </div>
        </div>

        <!-- Remote server -->
        <div class="setup-section">
          <label class="setup-label">{{ t('workspace.remoteServer') }}</label>
          <div class="input-row">
            <input
              v-model="remoteUrl"
              type="text"
              class="setup-input"
              :placeholder="t('workspace.remotePlaceholder')"
              @keyup.enter="connectRemote"
            />
            <button
              class="setup-btn"
              :disabled="!remoteUrl.trim() || connecting"
              @click="connectRemote"
            >
              {{ connecting ? '...' : t('workspace.connect') }}
            </button>
          </div>
        </div>

        <div class="setup-hint">{{ t('workspace.hint') }}</div>
      </div>
    </div>

    <!-- Normal dashboard (shown when workspace configured) -->
    <template v-else-if="state.workspaceConfigured !== null">
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

      <!-- Workspace path indicator -->
      <div v-if="state.config" class="workspace-indicator">
        <span class="ws-label">{{ t('workspace.current') }}:</span>
        <span class="ws-path">{{ state.config.workspacePath }}</span>
        <button class="ws-change" @click="state.workspaceConfigured = false">
          {{ t('workspace.change') }}
        </button>
      </div>

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

                <router-link :to="'/review/' + item.id" class="open-btn">
                  {{ t('dashboard.open') }}
                  <span class="btn-arrow">→</span>
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>


<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'
import { api, setApiBase, getApiBase } from '../api/client.js'

const { t, locale } = useI18n()

const workspacePath = ref('')
const remoteUrl = ref('')
const connecting = ref(false)

onMounted(async () => {
  await state.fetchConfig()
  if (state.workspaceConfigured) {
    await state.fetchItems()
  }
})

async function applyWorkspace() {
  if (!workspacePath.value.trim()) return
  state.error = null
  try {
    await state.setWorkspacePath(workspacePath.value.trim())
  } catch {
    // error already set in store
  }
}

async function connectRemote() {
  if (!remoteUrl.value.trim()) return
  connecting.value = true
  state.error = null
  const url = remoteUrl.value.trim().replace(/\/+$/, '')
  try {
    const res = await fetch(url + '/api/config')
    if (!res.ok) throw new Error(`Server responded ${res.status}`)
    const cfg = await res.json()
    setApiBase(url)
    state.config = cfg
    state.workspaceConfigured = cfg.workspaceConfigured
    if (cfg.workspaceConfigured) {
      await state.fetchItems()
    }
  } catch (e) {
    state.error = t('workspace.connectError') + ': ' + e.message
  } finally {
    connecting.value = false
  }
}

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
.home { height: 100%; overflow-y: auto; padding: 28px 32px; display: flex; flex-direction: column; gap: 28px; }

/* Workspace Setup */
.workspace-setup {
  display: flex; align-items: center; justify-content: center;
  flex: 1; min-height: 400px;
}
.setup-card {
  background: var(--bg-card); border: 1px solid var(--border);
  padding: 40px 48px; max-width: 560px; width: 100%;
  display: flex; flex-direction: column; align-items: center; gap: 20px;
}
.setup-icon {
  width: 64px; height: 64px; background: var(--text); color: var(--bg);
  display: flex; align-items: center; justify-content: center;
  font-size: 32px; font-weight: 400; letter-spacing: 0.1em;
}
.setup-title {
  font-size: 22px; font-weight: 400; color: var(--text);
  letter-spacing: 0.04em; text-align: center;
}
.setup-desc {
  font-size: 14px; color: var(--text-dim); line-height: 1.6;
  text-align: center; max-width: 400px;
}
.setup-section { width: 100%; display: flex; flex-direction: column; gap: 6px; }
.setup-label {
  font-size: 11px; font-weight: 500; color: var(--text-faint);
  text-transform: uppercase; letter-spacing: 0.12em;
}
.input-row { display: flex; gap: 8px; }
.setup-input {
  flex: 1; padding: 9px 12px; border: 1px solid var(--border);
  background: var(--bg-input, var(--bg)); color: var(--text);
  font-size: 13px; font-family: inherit; outline: none;
  transition: border-color 0.15s;
}
.setup-input:focus { border-color: var(--accent); }
.setup-input::placeholder { color: var(--text-faint); }
.setup-btn {
  padding: 9px 18px; border: 1px solid var(--border);
  background: transparent; color: var(--text-dim); font-size: 13px;
  font-weight: 500; cursor: pointer; transition: all 0.15s;
  letter-spacing: 0.04em; white-space: nowrap;
}
.setup-btn:hover { border-color: var(--accent); color: var(--accent); }
.setup-btn:disabled { opacity: 0.4; cursor: default; }
.setup-btn.primary {
  background: var(--text); color: var(--bg); border-color: var(--text);
}
.setup-btn.primary:hover { background: var(--accent); border-color: var(--accent); }
.setup-hint {
  font-size: 12px; color: var(--text-faint); text-align: center;
  line-height: 1.5; margin-top: 4px;
}

/* Workspace indicator */
.workspace-indicator {
  display: flex; align-items: center; gap: 8px;
  padding: 8px 14px; background: var(--bg-card); border: 1px solid var(--border);
  font-size: 12px;
}
.ws-label { color: var(--text-faint); font-weight: 500; letter-spacing: 0.06em; text-transform: uppercase; }
.ws-path { color: var(--text-dim); font-family: monospace; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.ws-change {
  background: none; border: 1px solid var(--border); color: var(--text-faint);
  padding: 3px 10px; font-size: 11px; cursor: pointer; transition: all 0.15s;
}
.ws-change:hover { border-color: var(--accent); color: var(--accent); }

.hero {
  display: flex; align-items: center; justify-content: space-between;
  background: var(--bg-card); border: 1px solid var(--border); padding: 28px 32px;
}
.hero-title { font-size: 24px; font-weight: 400; color: var(--text); letter-spacing: 0.04em; margin-bottom: 6px; }
.hero-tagline { font-size: 14px; color: var(--text-dim); line-height: 1.5; max-width: 360px; }
.hero-badge {
  width: 56px; height: 56px; background: var(--text); display: flex;
  align-items: center; justify-content: center; flex-shrink: 0;
}
.badge-r { font-size: 28px; font-weight: 400; color: var(--bg); letter-spacing: 0.1em; }

.stats-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.stat-card {
  background: var(--bg-card); border: 1px solid var(--border); padding: 16px 20px;
  display: flex; flex-direction: column; gap: 4px; transition: border-color 0.15s;
}
.stat-card:hover { border-color: var(--accent); }
.stat-num { font-size: 28px; font-weight: 400; color: var(--text); line-height: 1; }
.stat-label { font-size: 12px; color: var(--text-faint); font-weight: 500; letter-spacing: 0.04em; }
.stat-card.accent-red  .stat-num { color: var(--accent); }
.stat-card.accent-green .stat-num { color: var(--green); }
.stat-card.accent-gray  .stat-num { color: var(--text-dim); }

.loading { font-size: 14px; color: var(--text-dim); padding: 8px 0; }
.error   { font-size: 14px; color: var(--accent); padding: 8px 12px; background: var(--accent-soft); }

.section-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px; }
.section-header .section-title { margin-bottom: 0; }

.btn-upload {
  font-size: 13px; font-weight: 500; padding: 6px 14px;
  background: transparent; color: var(--text-dim); border: 1px solid var(--border); cursor: pointer;
  transition: all 0.15s; letter-spacing: 0.04em;
}
.btn-upload:hover { border-color: var(--accent); color: var(--accent); }

.upload-actions { display: flex; gap: 8px; }
.upload-btn {
  display: flex; align-items: center; gap: 5px;
  padding: 6px 14px; border: 1px solid var(--border);
  background: transparent; color: var(--text-dim); font-size: 13px; cursor: pointer;
  transition: all 0.15s; letter-spacing: 0.04em;
}
.upload-btn:hover { border-color: var(--accent); color: var(--accent); }
.upload-modal-overlay {
  position: fixed; inset: 0; background: rgba(0,0,0,0.35);
  display: flex; align-items: center; justify-content: center; z-index: 100;
}
.upload-modal {
  background: var(--bg-card); border: 1px solid var(--border); padding: 24px; min-width: 420px;
  max-width: 560px; box-shadow: 0 8px 32px rgba(0,0,0,0.15);
}
.modal-title { font-size: 15px; font-weight: 500; margin: 0 0 16px; color: var(--text); letter-spacing: 0.04em; }
.file-list { display: flex; flex-direction: column; gap: 8px; max-height: 320px; overflow-y: auto; }
.file-row { display: flex; align-items: center; gap: 8px; }
.file-name { flex: 1; font-size: 13px; color: var(--text-dim); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.type-select { padding: 4px 8px; border: 1px solid var(--border); font-size: 12px; color: var(--text-dim); background: transparent; }
.remove-btn { background: none; border: none; color: var(--text-faint); cursor: pointer; font-size: 14px; padding: 2px 4px; }
.remove-btn:hover { color: var(--accent); }
.modal-footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; }
.cancel-btn {
  padding: 7px 16px; border: 1px solid var(--border);
  background: transparent; color: var(--text-dim); font-size: 13px; cursor: pointer;
}
.confirm-btn {
  padding: 7px 16px; border: 1px solid var(--accent);
  background: var(--accent); color: var(--bg); font-size: 13px; font-weight: 500; cursor: pointer;
}
.confirm-btn:disabled { opacity: 0.5; cursor: default; }

.section-title {
  font-size: 12px; font-weight: 500; color: var(--text-faint);
  text-transform: uppercase; letter-spacing: 0.15em; margin-bottom: 14px;
}

.item-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 14px; }

.item-card {
  background: var(--bg-card); border: 1px solid var(--border);
  overflow: hidden; display: flex; flex-direction: column;
  transition: border-color 0.15s, transform 0.15s;
}
.item-card:hover { border-color: var(--accent); transform: translateY(-2px); }

.card-stripe { height: 3px; background: var(--accent); }

.card-body { padding: 18px 20px; display: flex; flex-direction: column; gap: 10px; flex: 1; }

.card-top { display: flex; align-items: center; justify-content: space-between; }
.type-badge {
  display: flex; align-items: center; gap: 5px;
  font-size: 11px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.08em;
  padding: 2px 8px; border: 1px solid var(--border); color: var(--accent);
}
.type-badge.plan, .type-badge.design, .type-badge.prototype { background: transparent; color: var(--accent); }
.type-icon { font-size: 12px; }
.updated-at { font-size: 11px; color: var(--text-faint); }

.card-title { font-size: 15px; font-weight: 500; color: var(--text); line-height: 1.4; letter-spacing: 0.02em; }
.card-desc  { font-size: 13px; color: var(--text-dim); line-height: 1.5; flex: 1; }

.card-footer {
  display: flex; align-items: center; justify-content: space-between;
  padding-top: 10px; border-top: 1px solid var(--border-light); margin-top: 4px;
}

.comment-status { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.status-pill {
  display: flex; align-items: center; gap: 4px;
  font-size: 11px; font-weight: 500; padding: 2px 8px;
  border: 1px solid var(--border); background: transparent;
}
.status-pill.open     { border-color: var(--accent); color: var(--accent); }
.status-pill.resolved { border-color: var(--green); color: var(--green); }
.status-pill.empty    { border-color: var(--border); color: var(--text-faint); }
.status-dot { width: 5px; height: 5px; border-radius: 50%; background: var(--accent); animation: pulse 2s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }

.open-btn {
  display: flex; align-items: center; gap: 5px;
  padding: 7px 14px; background: var(--text); color: var(--bg);
  font-size: 13px; font-weight: 500; text-decoration: none;
  transition: background 0.15s, transform 0.1s; flex-shrink: 0;
  letter-spacing: 0.04em;
}
.open-btn:hover { background: var(--accent); }
.open-btn:active { transform: scale(0.97); }
.btn-arrow { font-size: 14px; transition: transform 0.15s; }
.open-btn:hover .btn-arrow { transform: translateX(3px); }
</style>
