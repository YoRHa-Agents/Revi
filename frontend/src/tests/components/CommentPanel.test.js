import { describe, it, expect, vi, beforeEach } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key, locale: 'en' }),
}))

vi.mock('../../store/index.js', () => {
  const s = {
    items: [],
    comments: {},
    getComments(id) { return s.comments[id] || [] },
    openCount(id) { return (s.comments[id] || []).filter(c => c.status === 'open').length },
    resolvedCount(id) { return (s.comments[id] || []).filter(c => c.status === 'resolved').length },
    addComment: vi.fn(() => Promise.resolve({ id: 'new', status: 'open' })),
    resolveComment: vi.fn(() => Promise.resolve()),
    archiveResolved: vi.fn(() => Promise.resolve()),
    exportForAgent: vi.fn(() => Promise.resolve({})),
  }
  return { state: s }
})

import CommentPanel from '../../components/CommentPanel.vue'
import { state } from '../../store/index.js'

const stubs = { Transition: true, Teleport: true }

function factory(props = {}) {
  return shallowMount(CommentPanel, {
    props: { itemId: 'plans/test', ...props },
    global: { stubs },
  })
}

describe('CommentPanel', () => {
  beforeEach(() => {
    state.comments = {}
    vi.clearAllMocks()
  })

  it('renders the panel with tabs', () => {
    const wrapper = factory()
    expect(wrapper.find('.panel').exists()).toBe(true)
    expect(wrapper.findAll('.tab').length).toBe(2)
  })

  it('shows empty state when no open comments', () => {
    const wrapper = factory()
    expect(wrapper.find('.empty').exists()).toBe(true)
  })

  it('renders open comment cards', () => {
    state.comments = {
      'plans/test': [
        { id: 'c1', status: 'open', author: 'Alice', content: 'Hello', createdAt: new Date().toISOString(), reference: null },
      ],
    }
    const wrapper = factory()
    expect(wrapper.findAll('.comment-card.open').length).toBe(1)
    expect(wrapper.find('.c-author').text()).toBe('Alice')
  })

  it('toggles add-comment form on button click', async () => {
    const wrapper = factory()
    expect(wrapper.find('.add-form').exists()).toBe(false)
    await wrapper.find('.btn-primary').trigger('click')
    expect(wrapper.find('.add-form').exists()).toBe(true)
    await wrapper.find('.btn-primary').trigger('click')
    expect(wrapper.find('.add-form').exists()).toBe(false)
  })

  it('switches between open and resolved tabs', async () => {
    state.comments = {
      'plans/test': [
        { id: 'c1', status: 'resolved', author: 'Bob', content: 'Done', createdAt: new Date().toISOString(), resolvedAt: new Date().toISOString(), reference: null },
      ],
    }
    const wrapper = factory()
    const tabs = wrapper.findAll('.tab')
    await tabs[1].trigger('click')
    expect(tabs[1].classes()).toContain('active')
    expect(wrapper.findAll('.comment-card.resolved').length).toBe(1)
  })

  it('shows archive button when resolved comments exist', () => {
    state.comments = {
      'plans/test': [
        { id: 'c1', status: 'resolved', author: 'A', content: 'x', createdAt: new Date().toISOString(), resolvedAt: new Date().toISOString(), reference: null },
      ],
    }
    const wrapper = factory()
    expect(wrapper.find('.btn-archive').exists()).toBe(true)
  })

  it('hides archive button when no resolved comments', () => {
    const wrapper = factory()
    expect(wrapper.find('.btn-archive').exists()).toBe(false)
  })

  it('emits anchor-hover on comment mouseenter', async () => {
    const ref = { type: 'section', value: '## Intro' }
    state.comments = {
      'plans/test': [
        { id: 'c1', status: 'open', author: 'A', content: 'x', createdAt: new Date().toISOString(), reference: ref },
      ],
    }
    const wrapper = factory()
    await wrapper.find('.comment-card.open').trigger('mouseenter')
    expect(wrapper.emitted('anchor-hover')).toBeTruthy()
    expect(wrapper.emitted('anchor-hover')[0][0]).toEqual(ref)
  })
})
