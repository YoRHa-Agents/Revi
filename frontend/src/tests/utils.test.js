import { describe, it, expect } from 'vitest'

// relativeTime logic (copied from HomeView, adapted to return raw strings)
function relativeTime(iso, nowMs = Date.now()) {
  const diff = Math.floor((nowMs - new Date(iso)) / 60000)
  if (diff < 2) return 'just now'
  if (diff < 60) return `${diff}m ago`
  return `${Math.floor(diff / 60)}h ago`
}

// typeIcon logic (copied from HomeView)
function typeIcon(type) {
  return { plan: '📄', design: '🎨', prototype: '⚡' }[type] || '📁'
}

// slugify logic (copied from DocIndex.vue)
function slugify(text) {
  return text.toLowerCase().replace(/[^\w\u4e00-\u9fff\s-]/g, '').trim().replace(/\s+/g, '-')
}

// ── relativeTime ──────────────────────────────────────────────────────────────

describe('relativeTime', () => {
  it('returns "just now" for a time less than 2 minutes ago', () => {
    const now = Date.now()
    const oneMinuteAgo = new Date(now - 60 * 1000).toISOString()
    expect(relativeTime(oneMinuteAgo, now)).toBe('just now')
  })

  it('returns "just now" for exactly 0 minutes diff', () => {
    const now = Date.now()
    expect(relativeTime(new Date(now).toISOString(), now)).toBe('just now')
  })

  it('returns "30m ago" for 30 minutes ago', () => {
    const now = Date.now()
    const thirtyMinAgo = new Date(now - 30 * 60 * 1000).toISOString()
    expect(relativeTime(thirtyMinAgo, now)).toBe('30m ago')
  })

  it('returns "3h ago" for 3 hours ago', () => {
    const now = Date.now()
    const threeHoursAgo = new Date(now - 3 * 60 * 60 * 1000).toISOString()
    expect(relativeTime(threeHoursAgo, now)).toBe('3h ago')
  })

  it('returns "59m ago" for 59 minutes ago', () => {
    const now = Date.now()
    const fiftyNineMinAgo = new Date(now - 59 * 60 * 1000).toISOString()
    expect(relativeTime(fiftyNineMinAgo, now)).toBe('59m ago')
  })

  it('returns "1h ago" for 60 minutes ago', () => {
    const now = Date.now()
    const sixtyMinAgo = new Date(now - 60 * 60 * 1000).toISOString()
    expect(relativeTime(sixtyMinAgo, now)).toBe('1h ago')
  })
})

// ── typeIcon ──────────────────────────────────────────────────────────────────

describe('typeIcon', () => {
  it('returns 📄 for "plan"', () => {
    expect(typeIcon('plan')).toBe('📄')
  })

  it('returns 🎨 for "design"', () => {
    expect(typeIcon('design')).toBe('🎨')
  })

  it('returns ⚡ for "prototype"', () => {
    expect(typeIcon('prototype')).toBe('⚡')
  })

  it('returns 📁 for unknown type', () => {
    expect(typeIcon('unknown')).toBe('📁')
    expect(typeIcon('')).toBe('📁')
    expect(typeIcon('report')).toBe('📁')
  })
})

// ── slugify (heading slug generation from DocIndex) ───────────────────────────

describe('slugify', () => {
  it('lowercases text', () => {
    expect(slugify('Architecture')).toBe('architecture')
  })

  it('replaces spaces with hyphens', () => {
    expect(slugify('Data Models')).toBe('data-models')
  })

  it('handles a markdown heading like "## Architecture"', () => {
    // The DocIndex strips the "## " prefix before passing to slugify, so test the text part
    expect(slugify('Architecture')).toBe('architecture')
  })

  it('converts "Storage Layer" to "storage-layer"', () => {
    expect(slugify('Storage Layer')).toBe('storage-layer')
  })

  it('handles multiple spaces', () => {
    expect(slugify('Agent Export Format')).toBe('agent-export-format')
  })

  it('removes special characters except word chars and hyphens', () => {
    // Punctuation like apostrophes/periods should be stripped
    expect(slugify('Hello, World!')).toBe('hello-world')
  })

  it('preserves Chinese characters', () => {
    expect(slugify('系统设计')).toBe('系统设计')
  })

  it('handles mixed Chinese and ASCII', () => {
    const result = slugify('Sprint 1 系统设计')
    expect(result).toBe('sprint-1-系统设计')
  })
})
