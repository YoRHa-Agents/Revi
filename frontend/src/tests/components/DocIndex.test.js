import { describe, it, expect, vi } from 'vitest'
import { shallowMount } from '@vue/test-utils'

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key) => key }),
}))

import DocIndex from '../../components/DocIndex.vue'

const MD = `# Title
## Overview
Some text.
### Details
More text.
## Summary
`

function factory(content = MD) {
  return shallowMount(DocIndex, {
    props: { content },
  })
}

describe('DocIndex', () => {
  it('renders the index panel', () => {
    const wrapper = factory()
    expect(wrapper.find('.doc-index').exists()).toBe(true)
  })

  it('extracts headings from markdown content', () => {
    const wrapper = factory()
    const items = wrapper.findAll('.heading-item')
    expect(items.length).toBe(4)
    expect(items[0].text()).toBe('Title')
    expect(items[1].text()).toBe('Overview')
    expect(items[2].text()).toBe('Details')
    expect(items[3].text()).toBe('Summary')
  })

  it('applies correct level classes', () => {
    const wrapper = factory()
    const items = wrapper.findAll('.heading-item')
    expect(items[0].classes()).toContain('level-1')
    expect(items[1].classes()).toContain('level-2')
    expect(items[2].classes()).toContain('level-3')
    expect(items[3].classes()).toContain('level-2')
  })

  it('emits hide event on hide button click', async () => {
    const wrapper = factory()
    await wrapper.find('.hide-btn').trigger('click')
    expect(wrapper.emitted('hide')).toBeTruthy()
  })

  it('renders no headings for content without markdown headings', () => {
    const wrapper = factory('Just plain text here.')
    expect(wrapper.findAll('.heading-item').length).toBe(0)
  })

  it('generates correct href for each heading', () => {
    const wrapper = factory('## Data Models\n## API Endpoints')
    const items = wrapper.findAll('.heading-item')
    expect(items[0].attributes('href')).toBe('#h-data-models')
    expect(items[1].attributes('href')).toBe('#h-api-endpoints')
  })
})
