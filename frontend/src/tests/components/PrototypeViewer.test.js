import { describe, it, expect, vi } from 'vitest'
import { shallowMount } from '@vue/test-utils'

import PrototypeViewer from '../../components/viewers/PrototypeViewer.vue'

const item = { id: 'prototypes/flow', type: 'prototype', title: 'Review Flow' }

function factory(props = {}) {
  return shallowMount(PrototypeViewer, {
    props: { item, ...props },
  })
}

describe('PrototypeViewer', () => {
  it('renders the prototype viewer', () => {
    const wrapper = factory()
    expect(wrapper.find('.proto-viewer').exists()).toBe(true)
  })

  it('displays all 5 steps in the navigation', () => {
    const wrapper = factory()
    expect(wrapper.findAll('.step-item').length).toBe(5)
  })

  it('first step is active by default', () => {
    const wrapper = factory()
    const items = wrapper.findAll('.step-item')
    expect(items[0].classes()).toContain('active')
  })

  it('clicking a step changes the active step', async () => {
    const wrapper = factory()
    const items = wrapper.findAll('.step-item')
    await items[2].trigger('click')
    expect(items[2].classes()).toContain('active')
  })

  it('shows step description for the active step', () => {
    const wrapper = factory()
    expect(wrapper.find('.desc-title').text()).toBe('Discover Items')
  })

  it('emits anchor-select when step comment button is clicked', async () => {
    const wrapper = factory()
    await wrapper.findAll('.step-anchor-btn')[1].trigger('click')
    expect(wrapper.emitted('anchor-select')).toBeTruthy()
    const payload = wrapper.emitted('anchor-select')[0][0]
    expect(payload.type).toBe('step')
    expect(payload.value).toBe(1)
  })

  it('shows progress dots matching step count', () => {
    const wrapper = factory()
    expect(wrapper.findAll('.prog-dot').length).toBe(5)
    expect(wrapper.findAll('.prog-dot')[0].classes()).toContain('active')
  })

  it('disables Back button on first step', () => {
    const wrapper = factory()
    const backBtn = wrapper.findAll('.nbtn')[0]
    expect(backBtn.attributes('disabled')).toBeDefined()
  })

  it('highlights step when hoveredAnchor matches', () => {
    const wrapper = factory({ hoveredAnchor: { type: 'step', value: 3 } })
    const items = wrapper.findAll('.step-item')
    expect(items[3].classes()).toContain('hovered')
  })
})
