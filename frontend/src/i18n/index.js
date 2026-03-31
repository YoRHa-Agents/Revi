import { createI18n } from 'vue-i18n'
import zh from './zh.json'
import en from './en.json'

const savedLang = localStorage.getItem('revi-lang') || 'en'

export const i18n = createI18n({
  legacy: false,
  locale: savedLang,
  fallbackLocale: 'en',
  messages: { zh, en },
})

export function setLang(lang) {
  i18n.global.locale.value = lang
  localStorage.setItem('revi-lang', lang)
}
