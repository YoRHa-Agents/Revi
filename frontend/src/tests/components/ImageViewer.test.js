import { describe, it, expect, vi } from 'vitest'
import { shallowMount } from '@vue/test-utils'

import ImageViewer from '../../components/viewers/ImageViewer.vue'

const item = { id: 'designs/mock', type: 'design', title: 'UI Mockup v1' }

function factory(props = {}) {
  return shallowMount(ImageViewer, {
    props: { item, ...props },
  })
}

describe('ImageViewer', () => {
  it('renders the design board', () => {
    const wrapper = factory()
    expect(wrapper.find('.design-board').exists()).toBe(true)
  })

  it('displays the item title', () => {
    const wrapper = factory()
    expect(wrapper.find('.board-name').text()).toBe('UI Mockup v1')
  })

  it('renders annotation pins on frames', () => {
    const wrapper = factory()
    const pins = wrapper.findAll('.pin')
    expect(pins.length).toBe(6)
  })

  it('emits anchor-select when a pin is clicked', async () => {
    const wrapper = factory()
    await wrapper.findAll('.pin')[0].trigger('click')
    expect(wrapper.emitted('anchor-select')).toBeTruthy()
    const payload = wrapper.emitted('anchor-select')[0][0]
    expect(payload.type).toBe('annotation')
    expect(payload.value).toBe('1')
  })

  it('renders legend with annotations', () => {
    const wrapper = factory()
    const annItems = wrapper.findAll('.ann')
    expect(annItems.length).toBe(6)
    expect(wrapper.find('.ann-title').text()).toBe('File Sidebar')
  })

  it('emits anchor-select when legend item is clicked', async () => {
    const wrapper = factory()
    await wrapper.findAll('.ann')[2].trigger('click')
    expect(wrapper.emitted('anchor-select')).toBeTruthy()
    const payload = wrapper.emitted('anchor-select')[0][0]
    expect(payload.type).toBe('annotation')
    expect(payload.value).toBe('3')
  })

  it('highlights pin when hoveredAnchor matches', () => {
    const wrapper = factory({ hoveredAnchor: { type: 'annotation', value: '2' } })
    const pins = wrapper.findAll('.pin')
    expect(pins[1].classes()).toContain('pin-active')
  })
})
