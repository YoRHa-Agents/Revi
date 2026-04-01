<template>
  <div class="app">
    <header class="topbar">
      <div class="topbar-left">
        <router-link to="/" class="brand">
          <span class="brand-logo">R E V I</span>
        </router-link>
      </div>
      <div class="topbar-right">
        <button class="lang-btn" :class="{ active: locale === 'en' }" @click="switchLang('en')">EN</button>
        <span class="lang-sep">/</span>
        <button class="lang-btn" :class="{ active: locale === 'zh' }" @click="switchLang('zh')">中文</button>
      </div>
    </header>

    <div class="body">
      <FileSidebar />
      <div class="content">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script setup>
import { useI18n } from 'vue-i18n'
import { setLang } from './i18n/index.js'
import FileSidebar from './components/FileSidebar.vue'

const { t, locale } = useI18n()

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

.body { flex: 1; display: flex; overflow: hidden; }
.content { flex: 1; overflow: hidden; display: flex; flex-direction: column; min-width: 0; }
</style>
