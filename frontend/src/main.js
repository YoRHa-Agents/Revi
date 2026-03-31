import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { i18n } from './i18n/index.js'
import App from './App.vue'
import HomeView from './views/HomeView.vue'
import ReviewView from './views/ReviewView.vue'
import ArchiveView from './views/ArchiveView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: HomeView },
    { path: '/review/:itemId+', component: ReviewView },
    { path: '/archive', component: ArchiveView },
  ],
})

createApp(App).use(router).use(i18n).mount('#app')
