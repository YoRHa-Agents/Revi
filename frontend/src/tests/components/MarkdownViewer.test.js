import { describe, it, expect, vi } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key }),
}))

import MarkdownViewer from '../../components/viewers/MarkdownViewer.vue'

const stubs = { Teleport: true }

function factory(props = {}) {
  return shallowMount(MarkdownViewer, {
    props: { content: '## Hello\nWorld', ...props },
    global: { stubs },
  })
}

describe('MarkdownViewer', () => {
  it('renders without crashing', () => {
    const wrapper = factory()
    expect(wrapper.find('.md-viewer-wrap').exists()).toBe(true)
  })

  it('renders markdown content as HTML', () => {
    const wrapper = factory({ content: '## Hello\nThis is a paragraph.' })
    const viewer = wrapper.find('.md-viewer')
    expect(viewer.exists()).toBe(true)
    expect(viewer.html()).toContain('<h2')
    expect(viewer.html()).toContain('Hello')
    expect(viewer.html()).toContain('This is a paragraph.')
  })

  it('shows search bar when showSearch is true', () => {
    const wrapper = factory({ showSearch: true })
    expect(wrapper.find('.in-search-bar').exists()).toBe(true)
  })

  it('hides search bar when showSearch is false', () => {
    const wrapper = factory({ showSearch: false })
    expect(wrapper.find('.in-search-bar').exists()).toBe(false)
  })

  it('adds heading anchors with ids', () => {
    const wrapper = factory({ content: '## Data Models' })
    const viewer = wrapper.find('.md-viewer')
    expect(viewer.html()).toContain('id="h-data-models"')
  })

  it('renders code blocks', () => {
    const wrapper = factory({ content: '`inline code`' })
    const viewer = wrapper.find('.md-viewer')
    expect(viewer.html()).toContain('<code>')
    expect(viewer.html()).toContain('inline code')
  })
})
