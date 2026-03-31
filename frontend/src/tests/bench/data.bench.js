import { bench, describe } from 'vitest'
import { state } from '../../mock/data.js'

describe('mock data ops', () => {
  bench('exportForAgent', () => {
    state.exportForAgent('plans/sprint-1-design')
  })

  bench('addComment x10', () => {
    for (let i = 0; i < 10; i++) {
      state.addComment('bench/test', {
        author: 'bench',
        content: 'test comment',
        reference: { type: 'general', value: null }
      })
    }
    // cleanup
    delete state.comments['bench/test']
  })

  bench('openCount', () => {
    state.openCount('plans/sprint-1-design')
  })
})
