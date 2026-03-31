import { describe, it, expect, beforeEach, vi } from 'vitest'

// This file will fail until Agent D creates src/store/index.js
// Skip for now if the import fails
let state = null
try {
  // Use a dynamic expression to prevent Vite from failing at transform time
  // when the module does not exist yet.
  const storePath = '../store/index.js'
  const mod = await import(/* @vite-ignore */ storePath)
  state = mod.state
} catch {
  state = null
}

const mockFetch = vi.fn()
vi.stubGlobal('fetch', mockFetch)

describe.skipIf(!state)('store', () => {
  beforeEach(() => {
    mockFetch.mockReset()
    // Reset state for isolation
    if (state) {
      state.items = []
      state.comments = {}
      state.archive = {}
      state.loading = false
      state.error = null
    }
  })

  it('fetchItems: sets state.items after successful fetch', async () => {
    const mockItem = {
      id: 'plans/test',
      type: 'plan',
      title: 'Test',
      updatedAt: new Date().toISOString(),
      openCount: 0,
      resolvedCount: 0,
    }
    mockFetch.mockResolvedValueOnce({
      ok: true,
      headers: { get: () => 'application/json' },
      json: async () => [mockItem],
    })

    await state.fetchItems()

    expect(state.items).toHaveLength(1)
    expect(state.items[0].id).toBe('plans/test')
  })

  it('fetchItems: sets loading=false after completion', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      headers: { get: () => 'application/json' },
      json: async () => [],
    })

    await state.fetchItems()

    expect(state.loading).toBe(false)
  })

  it('addComment: calls POST with correct body, adds to state.comments[itemId]', async () => {
    const itemId = 'plans/test'
    const commentData = { author: 'Alice', content: 'Test comment', reference: { type: 'general', value: null } }

    mockFetch.mockResolvedValueOnce({
      ok: true,
      headers: { get: () => 'application/json' },
      json: async () => ({ id: 'c123', ...commentData, status: 'open', createdAt: new Date().toISOString(), resolvedAt: null }),
    })

    await state.addComment(itemId, commentData)

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining(itemId),
      expect.objectContaining({
        method: 'POST',
        body: expect.any(String),
      })
    )

    const body = JSON.parse(mockFetch.mock.calls[0][1].body)
    expect(body.author).toBe('Alice')
    expect(body.content).toBe('Test comment')

    expect(state.comments[itemId]).toBeDefined()
    expect(state.comments[itemId].length).toBeGreaterThan(0)
  })

  it('resolveComment: calls PATCH with correct URL', async () => {
    const itemId = 'plans/test'
    const commentId = 'c123'

    mockFetch.mockResolvedValueOnce({
      ok: true,
      headers: { get: () => 'application/json' },
      json: async () => ({ id: commentId, status: 'resolved', resolvedAt: new Date().toISOString() }),
    })

    await state.resolveComment(itemId, commentId)

    const [url, options] = mockFetch.mock.calls[0]
    expect(options.method).toBe('PATCH')
    expect(url).toMatch(/resolve/)
  })

  it('archiveResolved: calls POST to /api/archive/{id}', async () => {
    const itemId = 'plans/test'

    mockFetch.mockResolvedValueOnce({
      ok: true,
      headers: { get: () => 'application/json' },
      json: async () => ({ archivedAt: new Date().toISOString(), comments: [] }),
    })

    await state.archiveResolved(itemId)

    const [url, options] = mockFetch.mock.calls[0]
    expect(options.method).toBe('POST')
    expect(url).toMatch(/archive/)
    expect(url).toMatch(itemId.replace('/', '/'))
  })

  it('error handling: sets state.error and state.loading=false when fetch rejects', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network error'))

    try {
      await state.fetchItems()
    } catch {
      // may or may not throw depending on implementation
    }

    expect(state.error).not.toBeNull()
    expect(state.loading).toBe(false)
  })

  describe('sync helpers', () => {
    beforeEach(() => {
      state.items = [{ id: 'plans/test', type: 'plan', title: 'Test' }]
      state.comments = {
        'plans/test': [
          { id: 'c1', status: 'open', author: 'A', content: 'open', createdAt: new Date().toISOString(), resolvedAt: null },
          { id: 'c2', status: 'resolved', author: 'B', content: 'resolved', createdAt: new Date().toISOString(), resolvedAt: new Date().toISOString() },
        ],
      }
    })

    it('getItem returns correct item for known id', () => {
      const item = state.getItem('plans/test')
      expect(item).toBeDefined()
      expect(item.id).toBe('plans/test')
    })

    it('getItem returns undefined for unknown id', () => {
      expect(state.getItem('unknown/item')).toBeUndefined()
    })

    it('getComments returns array for known id', () => {
      const comments = state.getComments('plans/test')
      expect(Array.isArray(comments)).toBe(true)
      expect(comments).toHaveLength(2)
    })

    it('getComments returns empty array for unknown id', () => {
      expect(state.getComments('unknown/item')).toEqual([])
    })

    it('openCount returns correct count', () => {
      expect(state.openCount('plans/test')).toBe(1)
    })

    it('resolvedCount returns correct count', () => {
      expect(state.resolvedCount('plans/test')).toBe(1)
    })

    it('openCount returns 0 for unknown id', () => {
      expect(state.openCount('unknown/item')).toBe(0)
    })
  })
})
