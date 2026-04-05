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

const app = createApp(App)
app.config.errorHandler = (err, instance, info) => {
  console.error('[Vue Error]', err, info)
}
app.use(router).use(i18n).mount('#app')
