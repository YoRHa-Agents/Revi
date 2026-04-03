<template>
  <aside class="sidebar">
    <!-- Header + search -->
    <div class="sidebar-top">
      <span class="section-label">{{ t('sidebar.workspace') }}</span>
      <div class="search-box" :class="{ focused: searchFocused }">
        <span class="search-icon">⌕</span>
        <input
          v-model="query"
          type="text"
          :placeholder="t('sidebar.search')"
          class="search-input"
          @focus="searchFocused = true"
          @blur="searchFocused = false"
        />
        <button v-if="query" class="search-clear" @click="query = ''">×</button>
      </div>
    </div>

    <!-- Search results (flat list) -->
    <template v-if="query.trim()">
      <div class="search-results">
        <div v-if="!filteredItems.length" class="no-results">
          {{ t('sidebar.noResults') }}
        </div>
        <router-link
          v-for="item in filteredItems"
          :key="item.id"
          :to="'/review/' + item.id"
          class="file-row"
          :class="{ active: activeId === item.id }"
        >
          <span class="file-dot" :class="item.type"></span>
          <span class="file-name">{{ locale === 'zh' ? item.titleZh : item.title }}</span>
          <span v-if="state.openCount(item.id) > 0" class="open-badge">
            {{ state.openCount(item.id) }}
          </span>
        </router-link>
      </div>
    </template>

    <!-- File groups by type -->
    <template v-else>
      <div v-for="group in groups" :key="group.type" class="file-group">
        <button class="group-header" @click="toggleGroup(group.type)">
          <span class="chevron" :class="{ open: !collapsed[group.type] }">›</span>
          <span class="group-icon">{{ group.icon }}</span>
          <span class="group-name">{{ group.label }}</span>
          <span class="group-count">{{ group.items.length }}</span>
        </button>

        <transition name="slide">
          <div v-if="!collapsed[group.type]" class="group-items">
            <router-link
              v-for="item in group.items"
              :key="item.id"
              :to="'/review/' + item.id"
              class="file-row"
              :class="{ active: activeId === item.id }"
            >
              <span class="file-dot" :class="item.type"></span>
              <span class="file-name">{{ locale === 'zh' ? item.titleZh : item.title }}</span>
              <span v-if="state.openCount(item.id) > 0" class="open-badge">
                {{ state.openCount(item.id) }}
              </span>
            </router-link>
          </div>
        </transition>
      </div>
    </template>

    <!-- Bottom links -->
    <div class="sidebar-footer">
      <router-link to="/archive" class="footer-link" :class="{ active: route.path === '/archive' }">
        <span class="footer-icon">⬚</span>
        <span>{{ t('nav.archive') }}</span>
      </router-link>
    </div>
  </aside>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { state } from '../store/index.js'

const route = useRoute()
const { t, locale } = useI18n()

const query = ref('')
const searchFocused = ref(false)
const collapsed = ref({ plan: false, design: false, prototype: false })

function toggleGroup(type) {
  collapsed.value[type] = !collapsed.value[type]
}

const activeId = computed(() => {
  const parts = route.params.itemId
  return parts ? (Array.isArray(parts) ? parts.join('/') : parts) : null
})

const filteredItems = computed(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return state.items
  return state.items.filter(item =>
    (item.title || '').toLowerCase().includes(q) ||
    (item.titleZh || '').toLowerCase().includes(q)
  )
})

const groups = computed(() => [
  {
    type: 'plan',
    icon: '📄',
    label: t('home.type.plan') + 's',
    items: state.items.filter(i => i.type === 'plan'),
  },
  {
    type: 'design',
    icon: '🎨',
    label: t('home.type.design') + 's',
    items: state.items.filter(i => i.type === 'design'),
  },
  {
    type: 'prototype',
    icon: '⚡',
    label: t('home.type.prototype') + 's',
    items: state.items.filter(i => i.type === 'prototype'),
  },
].filter(g => g.items.length > 0))
</script>

<style scoped>
.sidebar {
  width: 220px; flex-shrink: 0; background: var(--bg-sidebar);
  border-right: 1px solid var(--border); display: flex; flex-direction: column;
  overflow-y: auto; overflow-x: hidden; user-select: none;
}
.sidebar-top { padding: 12px 10px 8px; display: flex; flex-direction: column; gap: 7px; }
.section-label { font-size: 10px; font-weight: 500; letter-spacing: 0.15em; color: var(--text-faint); text-transform: uppercase; padding: 0 4px; }

.search-box {
  display: flex; align-items: center; gap: 5px;
  background: var(--bg-input); border: 1px solid var(--border);
  padding: 5px 8px; transition: border-color 0.15s, box-shadow 0.15s;
}
.search-box.focused { border-color: var(--accent); box-shadow: 0 0 0 2px var(--accent-med); }
.search-icon { font-size: 14px; color: var(--text-faint); flex-shrink: 0; line-height: 1; }
.search-input { flex: 1; border: none; background: none; outline: none; font-size: 12px; color: var(--text); min-width: 0; font-family: inherit; }
.search-input::placeholder { color: var(--border-light); }
.search-clear { background: none; border: none; cursor: pointer; color: var(--text-faint); font-size: 14px; line-height: 1; padding: 0; transition: color 0.1s; }
.search-clear:hover { color: var(--text); }

.search-results { padding: 4px 0; }
.no-results { font-size: 12px; color: var(--text-faint); text-align: center; padding: 16px 12px; }

.file-group { margin-bottom: 2px; }
.group-header {
  width: 100%; display: flex; align-items: center; gap: 5px;
  padding: 5px 10px 5px 8px; background: none; border: none; cursor: pointer;
  font-size: 12px; color: var(--text-dim); font-weight: 500;
  transition: background 0.1s, color 0.1s; text-align: left;
}
.group-header:hover { background: var(--accent-soft); color: var(--text); }
.chevron { font-size: 14px; color: var(--text-faint); transition: transform 0.15s; display: inline-block; width: 12px; line-height: 1; }
.chevron.open { transform: rotate(90deg); }
.group-icon { font-size: 13px; }
.group-name { flex: 1; }
.group-count { font-size: 10px; background: var(--border-light); color: var(--text-dim); padding: 1px 5px; font-weight: 600; }

.group-items { padding: 1px 0; }
.file-row {
  display: flex; align-items: center; gap: 7px;
  padding: 5px 10px 5px 24px; font-size: 12.5px; color: var(--text-dim);
  text-decoration: none; transition: background 0.1s, color 0.1s; cursor: pointer;
  white-space: nowrap; overflow: hidden; border-left: 2px solid transparent;
}
.search-results .file-row { padding-left: 12px; }
.file-row:hover { background: var(--accent-soft); color: var(--text); }
.file-row.active { background: var(--accent-soft); color: var(--accent); font-weight: 500; border-left-color: var(--accent); }
.file-row.active .file-dot { opacity: 1; }
.file-dot { width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0; opacity: 0.5; background: var(--accent); }
.file-dot.plan, .file-dot.design, .file-dot.prototype { background: var(--accent); }
.file-name { flex: 1; overflow: hidden; text-overflow: ellipsis; }
.open-badge {
  flex-shrink: 0; font-size: 10px; font-weight: 600;
  border: 1px solid var(--accent); color: var(--accent); padding: 1px 5px; line-height: 1.4;
}

.sidebar-footer { margin-top: auto; border-top: 1px solid var(--border); padding: 6px 0; }
.footer-link {
  display: flex; align-items: center; gap: 8px; padding: 7px 14px;
  font-size: 12.5px; color: var(--text-dim); text-decoration: none;
  transition: background 0.1s, color 0.1s;
}
.footer-link:hover { background: var(--accent-soft); color: var(--text); }
.footer-link.active { color: var(--accent); background: var(--accent-soft); font-weight: 500; }
.footer-icon { font-size: 14px; }

.slide-enter-active, .slide-leave-active { transition: opacity 0.15s, transform 0.15s; }
.slide-enter-from, .slide-leave-to { opacity: 0; transform: translateY(-4px); }
</style>
