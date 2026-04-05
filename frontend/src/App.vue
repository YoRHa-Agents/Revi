<template>
  <div class="app">
    <a href="#main-content" class="skip-link">Skip to main content</a>
    <header class="topbar" role="banner">
      <div class="topbar-left">
        <button class="hamburger" @click="sidebarOpen = !sidebarOpen" aria-label="Menu">
          <span></span><span></span><span></span>
        </button>
        <router-link to="/" class="brand">
          <span class="brand-logo">R E V I</span>
        </router-link>
      </div>
      <div class="topbar-right">
        <button class="lang-btn" :class="{ active: locale === 'en' }" :aria-pressed="locale === 'en'" @click="switchLang('en')">EN</button>
        <span class="lang-sep">/</span>
        <button class="lang-btn" :class="{ active: locale === 'zh' }" :aria-pressed="locale === 'zh'" @click="switchLang('zh')">中文</button>
      </div>
    </header>

    <div class="body">
      <div v-if="sidebarOpen" class="sidebar-backdrop" @click="sidebarOpen = false"></div>
      <FileSidebar :class="{ open: sidebarOpen }" />
      <div class="content" id="main-content" role="main">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { setLang } from './i18n/index.js'
import FileSidebar from './components/FileSidebar.vue'

const route = useRoute()
const { t, locale } = useI18n()
const sidebarOpen = ref(false)

watch(() => route.path, () => { sidebarOpen.value = false })

function switchLang(lang) {
  setLang(lang)
}
</script>

<style>
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: var(--serif); background: var(--bg); color: var(--text); }

.app { height: 100vh; display: flex; flex-direction: column; overflow: hidden; }

.topbar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 20px; height: 44px; background: var(--bg-alt);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0; z-index: 100;
}
.topbar-left { display: flex; align-items: center; }
.brand { display: flex; align-items: center; text-decoration: none; }
.brand-logo {
  font-family: var(--serif);
  font-size: 14px; font-weight: 400;
  letter-spacing: 0.25em;
  color: var(--text);
}

.topbar-right { display: flex; align-items: center; gap: 4px; }
.lang-btn {
  background: none; border: 1px solid transparent; cursor: pointer; font-size: 12px;
  font-family: var(--serif);
  color: var(--text-faint); padding: 3px 7px; transition: all 0.15s;
  letter-spacing: 0.04em;
}
.lang-btn:hover { color: var(--text); border-color: var(--border); }
.lang-btn.active { color: var(--text); font-weight: 600; }
.lang-sep { color: var(--border-light); font-size: 12px; }

.body { flex: 1; display: flex; overflow: hidden; position: relative; }
.content { flex: 1; overflow: hidden; display: flex; flex-direction: column; min-width: 0; }

.hamburger {
  display: none; background: none; border: none; cursor: pointer;
  flex-direction: column; gap: 4px; padding: 4px; margin-right: 10px;
}
.hamburger span {
  display: block; width: 18px; height: 2px; background: var(--text);
  transition: transform 0.2s;
}
.sidebar-backdrop { display: none; }

@media (max-width: 767px) {
  .hamburger { display: flex; }
  .sidebar-backdrop {
    display: block; position: fixed; inset: 0; top: 44px;
    background: rgba(0,0,0,0.3); z-index: 49;
    animation: fade-in 0.2s ease;
  }
}
@keyframes fade-in { from { opacity: 0; } to { opacity: 1; } }

.skip-link {
  position: absolute; left: -9999px; top: auto;
  width: 1px; height: 1px; overflow: hidden;
  z-index: 999;
}
.skip-link:focus {
  position: fixed; top: 0; left: 0;
  width: auto; height: auto;
  padding: 8px 16px; background: var(--accent);
  color: var(--bg); font-size: 14px; z-index: 999;
}

:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}
button:focus-visible, a:focus-visible, input:focus-visible, select:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}
</style>
