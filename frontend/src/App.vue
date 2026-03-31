<template>
  <div class="app">
    <header class="topbar">
      <div class="topbar-left">
        <router-link to="/" class="brand">
          <span class="brand-logo">R</span>
          <div class="brand-text">
            <span class="brand-name">{{ t('app.title') }}</span>
            <span class="brand-sub">{{ t('app.subtitle') }}</span>
          </div>
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
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #f5f5f5; color: #1a1a1a; }

.app { height: 100vh; display: flex; flex-direction: column; overflow: hidden; }

/* Topbar */
.topbar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 20px; height: 48px; background: #fff;
  border-bottom: 1px solid #e5e7eb;
  flex-shrink: 0; z-index: 100;
}
.topbar-left { display: flex; align-items: center; }
.brand { display: flex; align-items: center; gap: 10px; text-decoration: none; }
.brand-logo {
  width: 30px; height: 30px; border-radius: 8px;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: #fff; font-size: 16px; font-weight: 800;
  display: flex; align-items: center; justify-content: center;
  flex-shrink: 0;
}
.brand-text { display: flex; flex-direction: column; line-height: 1.1; }
.brand-name { font-size: 15px; font-weight: 700; color: #1a1a1a; letter-spacing: -0.3px; }
.brand-sub  { font-size: 10px; color: #9ca3af; letter-spacing: 0.2px; }

.topbar-right { display: flex; align-items: center; gap: 4px; }
.lang-btn {
  background: none; border: none; cursor: pointer; font-size: 12px;
  color: #9ca3af; padding: 3px 7px; border-radius: 4px; transition: all 0.15s;
}
.lang-btn:hover { color: #1a1a1a; background: #f3f4f6; }
.lang-btn.active { color: #1a1a1a; font-weight: 700; }
.lang-sep { color: #e5e7eb; font-size: 12px; }

/* Body layout */
.body { flex: 1; display: flex; overflow: hidden; }

/* Content area */
.content { flex: 1; overflow: hidden; display: flex; flex-direction: column; min-width: 0; }
</style>
