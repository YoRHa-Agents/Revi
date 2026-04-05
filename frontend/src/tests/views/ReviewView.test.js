import { describe, it, expect, vi, beforeEach } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key, locale: 'en' }),
}))

vi.mock('vue-router', () => ({
  useRoute: () => ({ params: { itemId: 'plans/test' } }),
}))

vi.mock('../../store/index.js', () => {
  const s = {
    items: [],
    comments: {},
    loading: false,
    error: null,
    getItem(id) { return s.items.find(i => i.id === id) },
    getComments(id) { return s.comments[id] || [] },
    openCount(id) { return (s.comments[id] || []).filter(c => c.status === 'open').length },
    resolvedCount(id) { return (s.comments[id] || []).filter(c => c.status === 'resolved').length },
    fetchItems: vi.fn(() => Promise.resolve()),
    fetchComments: vi.fn(() => Promise.resolve()),
    updateItemType: vi.fn(() => Promise.resolve()),
  }
  return { state: s }
})

vi.mock('../../api/client.js', () => ({
  api: {
    getReview: vi.fn(() => Promise.resolve({ contentText: '# Test' })),
    getReviews: vi.fn(() => Promise.resolve([])),
  },
}))

import ReviewView from '../../views/ReviewView.vue'
import { state } from '../../store/index.js'

const stubs = {
  CommentPanel: true,
  DocIndex: true,
  MarkdownViewer: true,
  ImageViewer: true,
  PrototypeViewer: true,
  Transition: true,
}

describe('ReviewView', () => {
  beforeEach(() => {
    state.items = []
    state.comments = {}
    vi.clearAllMocks()
  })

  it('renders the review page container', () => {
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    expect(wrapper.find('.review-page').exists()).toBe(true)
  })

  it('shows not-found message when item does not exist', () => {
    state.items = []
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    expect(wrapper.find('.not-found').exists()).toBe(true)
    expect(wrapper.find('.not-found').text()).toContain('not found')
  })

  it('renders review header when item exists', () => {
    state.items = [{ id: 'plans/test', type: 'plan', title: 'Test Plan' }]
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    expect(wrapper.find('.review-header').exists()).toBe(true)
    expect(wrapper.find('.review-title').text()).toBe('Test Plan')
  })

  it('renders split layout with content and comment panes', () => {
    state.items = [{ id: 'plans/test', type: 'plan', title: 'Test Plan' }]
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    expect(wrapper.find('.split-layout').exists()).toBe(true)
    expect(wrapper.find('.content-pane').exists()).toBe(true)
    expect(wrapper.find('.comment-pane').exists()).toBe(true)
  })

  it('shows Index and Search buttons for plan type', () => {
    state.items = [{ id: 'plans/test', type: 'plan', title: 'Test Plan' }]
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    const actionBtns = wrapper.findAll('.action-btn')
    expect(actionBtns.length).toBe(2)
  })

  it('hides Index and Search buttons for non-plan types', () => {
    state.items = [{ id: 'plans/test', type: 'design', title: 'Test Design' }]
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    expect(wrapper.findAll('.action-btn').length).toBe(0)
  })

  it('shows type badge matching item type', () => {
    state.items = [{ id: 'plans/test', type: 'plan', title: 'Test Plan' }]
    const wrapper = shallowMount(ReviewView, { global: { stubs } })
    const badge = wrapper.find('.type-badge')
    expect(badge.exists()).toBe(true)
  })
})
