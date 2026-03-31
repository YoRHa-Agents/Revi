import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import { state } from '../mock/data.js'

const TEST_ID = 'test/mock-item'

afterEach(() => {
  // clean up test item
  delete state.comments[TEST_ID]
  delete state.archive[TEST_ID]
})

// ── addComment ────────────────────────────────────────────────────────────────

describe('addComment', () => {
  it('adds a comment with auto-id starting with "c" followed by digits', () => {
    state.addComment(TEST_ID, { author: 'Tester', content: 'Hello', reference: { type: 'general', value: null } })
    const comments = state.comments[TEST_ID]
    expect(comments).toHaveLength(1)
    expect(comments[0].id).toMatch(/^c\d+$/)
  })

  it('sets correct defaults: status="open", resolvedAt=null', () => {
    state.addComment(TEST_ID, { author: 'Tester', content: 'Hello', reference: { type: 'general', value: null } })
    const c = state.comments[TEST_ID][0]
    expect(c.status).toBe('open')
    expect(c.resolvedAt).toBeNull()
  })

  it('appends to the right key', () => {
    state.addComment(TEST_ID, { author: 'A', content: 'first', reference: { type: 'general', value: null } })
    state.addComment(TEST_ID, { author: 'B', content: 'second', reference: { type: 'general', value: null } })
    expect(state.comments[TEST_ID]).toHaveLength(2)
  })

  it('creates key if absent (new item ID)', () => {
    expect(state.comments[TEST_ID]).toBeUndefined()
    state.addComment(TEST_ID, { author: 'X', content: 'new', reference: { type: 'general', value: null } })
    expect(Array.isArray(state.comments[TEST_ID])).toBe(true)
    expect(state.comments[TEST_ID]).toHaveLength(1)
  })

  it('preserves content and author', () => {
    state.addComment(TEST_ID, { author: 'AuthorName', content: 'My comment content', reference: { type: 'section', value: '## Intro' } })
    const c = state.comments[TEST_ID][0]
    expect(c.author).toBe('AuthorName')
    expect(c.content).toBe('My comment content')
    expect(c.reference).toEqual({ type: 'section', value: '## Intro' })
  })
})

// ── resolveComment ────────────────────────────────────────────────────────────

describe('resolveComment', () => {
  beforeEach(() => {
    state.addComment(TEST_ID, { author: 'Tester', content: 'To resolve', reference: { type: 'general', value: null } })
  })

  it('sets status="resolved" and resolvedAt non-null on matching id', () => {
    const id = state.comments[TEST_ID][0].id
    state.resolveComment(TEST_ID, id)
    const c = state.comments[TEST_ID][0]
    expect(c.status).toBe('resolved')
    expect(c.resolvedAt).not.toBeNull()
    expect(typeof c.resolvedAt).toBe('string')
  })

  it('is a no-op on missing comment id (does not throw)', () => {
    expect(() => state.resolveComment(TEST_ID, 'nonexistent-id')).not.toThrow()
    // the original comment stays open
    expect(state.comments[TEST_ID][0].status).toBe('open')
  })
})

// ── archiveResolved ───────────────────────────────────────────────────────────

describe('archiveResolved', () => {
  it('moves resolved comments to archive, leaving only open ones', () => {
    state.addComment(TEST_ID, { author: 'A', content: 'open comment', reference: { type: 'general', value: null } })
    state.addComment(TEST_ID, { author: 'B', content: 'resolved comment', reference: { type: 'general', value: null } })
    const resolvedId = state.comments[TEST_ID][1].id
    state.resolveComment(TEST_ID, resolvedId)

    state.archiveResolved(TEST_ID)

    // only the open comment remains
    expect(state.comments[TEST_ID]).toHaveLength(1)
    expect(state.comments[TEST_ID][0].status).toBe('open')

    // archive entry created with the resolved comment
    expect(state.archive[TEST_ID]).toBeDefined()
    expect(state.archive[TEST_ID][0].comments).toHaveLength(1)
    expect(state.archive[TEST_ID][0].comments[0].id).toBe(resolvedId)
  })

  it('is a no-op if no resolved comments', () => {
    state.addComment(TEST_ID, { author: 'A', content: 'open', reference: { type: 'general', value: null } })
    const lengthBefore = state.comments[TEST_ID].length

    state.archiveResolved(TEST_ID)

    expect(state.comments[TEST_ID]).toHaveLength(lengthBefore)
    expect(state.archive[TEST_ID]).toBeUndefined()
  })

  it('archive entry has archivedAt and comments array', () => {
    state.addComment(TEST_ID, { author: 'A', content: 'to archive', reference: { type: 'general', value: null } })
    const id = state.comments[TEST_ID][0].id
    state.resolveComment(TEST_ID, id)
    state.archiveResolved(TEST_ID)

    const entry = state.archive[TEST_ID][0]
    expect(typeof entry.archivedAt).toBe('string')
    expect(Array.isArray(entry.comments)).toBe(true)
  })
})

// ── openCount / resolvedCount ─────────────────────────────────────────────────

describe('openCount / resolvedCount', () => {
  it('returns 0 for unknown item id', () => {
    expect(state.openCount('unknown/item')).toBe(0)
    expect(state.resolvedCount('unknown/item')).toBe(0)
  })

  it('correct values before and after mutations', () => {
    state.addComment(TEST_ID, { author: 'A', content: 'c1', reference: { type: 'general', value: null } })
    state.addComment(TEST_ID, { author: 'B', content: 'c2', reference: { type: 'general', value: null } })

    expect(state.openCount(TEST_ID)).toBe(2)
    expect(state.resolvedCount(TEST_ID)).toBe(0)

    const id = state.comments[TEST_ID][0].id
    state.resolveComment(TEST_ID, id)

    expect(state.openCount(TEST_ID)).toBe(1)
    expect(state.resolvedCount(TEST_ID)).toBe(1)
  })

  it('seeded item "plans/sprint-1-design" has expected counts (read-only)', () => {
    // Seed: c1 open, c2 open, c3 resolved → 2 open, 1 resolved
    expect(state.openCount('plans/sprint-1-design')).toBe(2)
    expect(state.resolvedCount('plans/sprint-1-design')).toBe(1)
  })
})

// ── getItem / getComments ─────────────────────────────────────────────────────

describe('getItem', () => {
  it('returns the correct item for a known id', () => {
    const item = state.getItem('plans/sprint-1-design')
    expect(item).toBeDefined()
    expect(item.id).toBe('plans/sprint-1-design')
    expect(item.type).toBe('plan')
  })

  it('returns undefined for an unknown id', () => {
    expect(state.getItem('unknown/item')).toBeUndefined()
  })
})

describe('getComments', () => {
  it('returns the correct comments array for a known id', () => {
    const comments = state.getComments('plans/sprint-1-design')
    expect(Array.isArray(comments)).toBe(true)
    expect(comments.length).toBeGreaterThan(0)
  })

  it('returns empty array for unknown id', () => {
    const comments = state.getComments('unknown/item')
    expect(Array.isArray(comments)).toBe(true)
    expect(comments).toHaveLength(0)
  })
})

// ── exportForAgent ────────────────────────────────────────────────────────────

describe('exportForAgent', () => {
  it('schema_version is "1.0"', () => {
    const result = state.exportForAgent('plans/sprint-1-design')
    expect(result.schema_version).toBe('1.0')
  })

  it('correct summary counts (total, open, resolved)', () => {
    const result = state.exportForAgent('plans/sprint-1-design')
    // Seed: c1 open, c2 open, c3 resolved → total=3, open=2, resolved=1
    expect(result.summary.total).toBe(3)
    expect(result.summary.open).toBe(2)
    expect(result.summary.resolved).toBe(1)
  })

  it('open_comments contains only open comments', () => {
    const result = state.exportForAgent('plans/sprint-1-design')
    expect(result.open_comments).toHaveLength(2)
    for (const c of result.open_comments) {
      expect(c).toHaveProperty('id')
      expect(c).toHaveProperty('author')
      expect(c).toHaveProperty('content')
      expect(c).toHaveProperty('reference')
      expect(c).toHaveProperty('created_at')
    }
  })

  it('exported_at is present and is a string', () => {
    const result = state.exportForAgent('plans/sprint-1-design')
    expect(typeof result.exported_at).toBe('string')
    expect(result.exported_at.length).toBeGreaterThan(0)
  })
})
