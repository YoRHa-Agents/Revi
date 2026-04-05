import { describe, it, expect, vi, beforeEach } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key, locale: 'en' }),
}))

vi.mock('../../store/index.js', () => {
  const s = {
    items: [],
    comments: {},
    archive: {},
    loading: false,
    error: null,
    config: null,
    configLoading: false,
    workspaceConfigured: null,
    getItem(id) { return s.items.find(i => i.id === id) },
    getComments(id) { return s.comments[id] || [] },
    openCount(id) { return (s.comments[id] || []).filter(c => c.status === 'open').length },
    resolvedCount(id) { return (s.comments[id] || []).filter(c => c.status === 'resolved').length },
    fetchConfig: vi.fn(() => Promise.resolve()),
    fetchItems: vi.fn(() => Promise.resolve()),
    setWorkspacePath: vi.fn(() => Promise.resolve()),
  }
  return { state: s }
})

vi.mock('../../api/client.js', () => ({
  api: {
    getReviews: vi.fn(() => Promise.resolve([])),
    getConfig: vi.fn(() => Promise.resolve({ workspaceConfigured: true })),
    updateConfig: vi.fn(() => Promise.resolve({ workspaceConfigured: true })),
    upload: vi.fn(() => Promise.resolve({})),
  },
  setApiBase: vi.fn(),
  getApiBase: () => 'http://localhost:8000',
}))

import HomeView from '../../views/HomeView.vue'
import { state } from '../../store/index.js'

const stubs = { RouterLink: { template: '<a><slot /></a>' } }

describe('HomeView', () => {
  beforeEach(() => {
    state.items = []
    state.comments = {}
    state.archive = {}
    state.loading = false
    state.error = null
    state.config = null
    state.workspaceConfigured = null
    vi.clearAllMocks()
  })

  it('renders the home container', () => {
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.home').exists()).toBe(true)
  })

  it('shows workspace setup when not configured', () => {
    state.workspaceConfigured = false
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.workspace-setup').exists()).toBe(true)
  })

  it('shows dashboard when workspace is configured', () => {
    state.workspaceConfigured = true
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.hero').exists()).toBe(true)
    expect(wrapper.find('.stats-row').exists()).toBe(true)
  })

  it('renders item cards in the grid', () => {
    state.workspaceConfigured = true
    state.items = [
      { id: 'plans/a', type: 'plan', title: 'Plan A', updatedAt: new Date().toISOString() },
      { id: 'designs/b', type: 'design', title: 'Design B', updatedAt: new Date().toISOString() },
    ]
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.findAll('.item-card').length).toBe(2)
  })

  it('shows loading indicator when loading', () => {
    state.workspaceConfigured = true
    state.loading = true
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.loading').exists()).toBe(true)
  })

  it('shows error message when error is set', () => {
    state.workspaceConfigured = true
    state.error = 'Something went wrong'
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.error').text()).toBe('Something went wrong')
  })

  it('renders correct stat counts', () => {
    state.workspaceConfigured = true
    state.items = [
      { id: 'plans/a', type: 'plan', title: 'A', updatedAt: new Date().toISOString() },
    ]
    state.comments = { 'plans/a': [{ id: 'c1', status: 'open' }, { id: 'c2', status: 'resolved' }] }
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    const nums = wrapper.findAll('.stat-num')
    expect(nums[0].text()).toBe('1')
    expect(nums[1].text()).toBe('1')
    expect(nums[2].text()).toBe('1')
  })

  it('shows workspace path when config is available', () => {
    state.workspaceConfigured = true
    state.config = { workspacePath: '/docs' }
    const wrapper = shallowMount(HomeView, { global: { stubs } })
    expect(wrapper.find('.ws-path').text()).toBe('/docs')
  })
})
