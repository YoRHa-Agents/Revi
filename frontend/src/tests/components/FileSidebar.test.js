import { describe, it, expect, vi, beforeEach } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key, locale: 'en' }),
}))

vi.mock('vue-router', () => ({
  useRoute: () => ({ params: {}, path: '/' }),
}))

vi.mock('../../store/index.js', () => {
  const s = {
    items: [],
    comments: {},
    openCount(id) { return (s.comments[id] || []).filter(c => c.status === 'open').length },
    resolvedCount(id) { return (s.comments[id] || []).filter(c => c.status === 'resolved').length },
  }
  return { state: s }
})

import FileSidebar from '../../components/FileSidebar.vue'
import { state } from '../../store/index.js'

const stubs = { RouterLink: { template: '<a><slot /></a>' }, Transition: true }

describe('FileSidebar', () => {
  beforeEach(() => {
    state.items = []
    state.comments = {}
  })

  it('renders the sidebar element', () => {
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.find('.sidebar').exists()).toBe(true)
  })

  it('renders a search input', () => {
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.find('.search-input').exists()).toBe(true)
  })

  it('groups items by type', () => {
    state.items = [
      { id: 'plans/a', type: 'plan', title: 'Plan A' },
      { id: 'designs/b', type: 'design', title: 'Design B' },
    ]
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.findAll('.file-group').length).toBe(2)
  })

  it('shows no-results message when search yields nothing', async () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    await wrapper.find('.search-input').setValue('zzzzz')
    expect(wrapper.find('.no-results').exists()).toBe(true)
  })

  it('filters items matching the search query', async () => {
    state.items = [
      { id: 'plans/a', type: 'plan', title: 'Alpha Plan' },
      { id: 'plans/b', type: 'plan', title: 'Beta Plan' },
    ]
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    await wrapper.find('.search-input').setValue('Alpha')
    expect(wrapper.findAll('.search-results .file-row').length).toBe(1)
  })

  it('toggles group collapse on header click', async () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.find('.group-items').exists()).toBe(true)
    await wrapper.find('.group-header').trigger('click')
    expect(wrapper.find('.group-items').exists()).toBe(false)
  })

  it('renders archive link in footer', () => {
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.find('.sidebar-footer').exists()).toBe(true)
  })

  it('shows open-comment badge when item has open comments', () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    state.comments = { 'plans/a': [{ id: 'c1', status: 'open' }] }
    const wrapper = shallowMount(FileSidebar, { global: { stubs } })
    expect(wrapper.find('.open-badge').exists()).toBe(true)
    expect(wrapper.find('.open-badge').text()).toBe('1')
  })
})
