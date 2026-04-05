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
    fetchItems: vi.fn(() => Promise.resolve()),
    fetchArchive: vi.fn(() => Promise.resolve()),
  }
  return { state: s }
})

import ArchiveView from '../../views/ArchiveView.vue'
import { state } from '../../store/index.js'

describe('ArchiveView', () => {
  beforeEach(() => {
    state.items = []
    state.archive = {}
    vi.clearAllMocks()
  })

  it('renders the archive page', () => {
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.archive-page').exists()).toBe(true)
  })

  it('shows the page title', () => {
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.page-title').text()).toBe('archive.title')
  })

  it('shows empty message when no archives exist', () => {
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.empty').exists()).toBe(true)
    expect(wrapper.find('.empty').text()).toBe('archive.empty')
  })

  it('renders archive batches when data is present', () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    state.archive = {
      'plans/a': [
        {
          archivedAt: '2026-01-01T00:00:00Z',
          comments: [
            { id: 'c1', author: 'Alice', content: 'Done', status: 'resolved', resolvedAt: '2026-01-01T00:00:00Z', reference: null },
          ],
        },
      ],
    }
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.empty').exists()).toBe(false)
    expect(wrapper.findAll('.batch').length).toBe(1)
    expect(wrapper.findAll('.archived-comment').length).toBe(1)
  })

  it('displays comment author and content in archived comments', () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    state.archive = {
      'plans/a': [
        {
          archivedAt: '2026-01-01T00:00:00Z',
          comments: [
            { id: 'c1', author: 'Bob', content: 'Fixed it', status: 'resolved', resolvedAt: '2026-01-01T00:00:00Z', reference: null },
          ],
        },
      ],
    }
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.c-author').text()).toBe('Bob')
    expect(wrapper.find('.c-content').text()).toBe('Fixed it')
  })

  it('renders reference badge when comment has a reference', () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    state.archive = {
      'plans/a': [
        {
          archivedAt: '2026-01-01T00:00:00Z',
          comments: [
            { id: 'c1', author: 'A', content: 'x', status: 'resolved', resolvedAt: '2026-01-01T00:00:00Z', reference: { type: 'section', value: '## Overview' } },
          ],
        },
      ],
    }
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.c-ref').exists()).toBe(true)
    expect(wrapper.find('.ref-badge').text()).toBe('section')
  })

  it('shows type badge for each item', () => {
    state.items = [{ id: 'plans/a', type: 'plan', title: 'Plan A' }]
    state.archive = {
      'plans/a': [{ archivedAt: '2026-01-01T00:00:00Z', comments: [{ id: 'c1', author: 'A', content: 'x', status: 'resolved', resolvedAt: '2026-01-01T00:00:00Z', reference: null }] }],
    }
    const wrapper = shallowMount(ArchiveView)
    expect(wrapper.find('.type-badge').text()).toBe('home.type.plan')
  })
})
