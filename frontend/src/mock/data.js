import { reactive } from 'vue'

const now = () => new Date().toISOString()
const ts = (offset = 0) => new Date(Date.now() - offset * 60000).toISOString()

export const state = reactive({
  items: [
    {
      id: 'plans/sprint-1-design',
      type: 'plan',
      title: 'Sprint 1 — System Design',
      titleZh: 'Sprint 1 — 系统设计',
      description: 'Architecture and data model for the Revi MVP.',
      descriptionZh: 'Revi MVP 的架构与数据模型设计。',
      updatedAt: ts(30),
    },
    {
      id: 'designs/ui-mockup-v1',
      type: 'design',
      title: 'UI Mockup v1',
      titleZh: 'UI 原型图 v1',
      description: 'High-fidelity mockup of the review interface.',
      descriptionZh: '审阅界面的高保真原型图。',
      updatedAt: ts(120),
    },
    {
      id: 'prototypes/review-flow',
      type: 'prototype',
      title: 'Review Flow Prototype',
      titleZh: '审阅流程原型',
      description: 'Interactive HTML prototype of the core review flow.',
      descriptionZh: '核心审阅流程的可交互 HTML 原型。',
      updatedAt: ts(200),
    },
  ],

  comments: {
    'plans/sprint-1-design': [
      {
        id: 'c1',
        author: 'Alice',
        content: 'The storage layer should support both local JSON and SQLite from the start — migration later will be painful.',
        contentZh: '存储层应该从一开始就同时支持本地 JSON 和 SQLite，后期迁移会很麻烦。',
        reference: { type: 'quote', value: 'Comments are stored as JSON files in `data/comments/{item_id}.json`. This keeps everything file-system portable and git-friendly.', section: 'Storage Layer' },
        status: 'open',
        createdAt: ts(180),
        resolvedAt: null,
      },
      {
        id: 'c2',
        author: 'Agent-Planner',
        content: 'The export format is missing a `schema_version` field. Agents need this to parse future breaking changes.',
        contentZh: '导出格式缺少 `schema_version` 字段，Agent 需要此字段来解析未来的破坏性变更。',
        reference: { type: 'quote', value: '"schema_version": "1.0"', section: 'Agent Export Format' },
        status: 'open',
        createdAt: ts(90),
        resolvedAt: null,
      },
      {
        id: 'c3',
        author: 'Bob',
        content: 'Add a `lang` field to each review item so bilingual designs can declare which languages are present.',
        contentZh: '为每个审阅条目添加 `lang` 字段，让双语设计可以声明包含哪些语言。',
        reference: { type: 'quote', value: 'Each review item is auto-discovered from the `workspace/` directory. Items are categorized as `plan`, `design`, or `prototype` based on the subfolder.', section: 'Data Models' },
        status: 'resolved',
        createdAt: ts(240),
        resolvedAt: ts(60),
      },
    ],
    'designs/ui-mockup-v1': [
      {
        id: 'c4',
        author: 'Carol',
        content: 'The comment panel feels too narrow at 35%. Consider making it resizable.',
        contentZh: '评论面板 35% 宽度感觉太窄了，考虑做成可调整大小的。',
        reference: { type: 'annotation', value: '6', label: 'Comment Panel' },
        status: 'open',
        createdAt: ts(150),
        resolvedAt: null,
      },
      {
        id: 'c5',
        author: 'Alice',
        content: 'The "Archive resolved" button should show a count badge, e.g. "Archive (3)".',
        contentZh: '"归档已解决" 按钮应该显示数量标记，例如 "归档 (3)"。',
        reference: { type: 'annotation', value: '3', label: 'Item Cards' },
        status: 'resolved',
        createdAt: ts(300),
        resolvedAt: ts(100),
      },
    ],
    'prototypes/review-flow': [
      {
        id: 'c6',
        author: 'Bob',
        content: 'Step 2 "Open for Review" — the doc index pane should be collapsible on narrow screens.',
        contentZh: '第 2 步"打开审阅"——文档目录面板在窄屏幕上应可折叠。',
        reference: { type: 'step', value: 1, label: 'Open for Review' },
        status: 'open',
        createdAt: ts(60),
        resolvedAt: null,
      },
    ],
  },

  archive: {
    'plans/sprint-1-design': [
      {
        archivedAt: ts(500),
        comments: [
          {
            id: 'a1',
            author: 'Agent-Planner',
            content: 'Initial scope is too broad. Narrow down MVP to comment + export flow only.',
            contentZh: '初始范围太宽泛，将 MVP 缩减到仅评论+导出流程。',
            reference: { type: 'section', value: '## Scope' },
            resolvedAt: ts(520),
          },
        ],
      },
    ],
  },

  // helpers
  getItem(id) {
    return this.items.find(i => i.id === id)
  },
  getComments(itemId) {
    return this.comments[itemId] || []
  },
  openCount(itemId) {
    return (this.comments[itemId] || []).filter(c => c.status === 'open').length
  },
  resolvedCount(itemId) {
    return (this.comments[itemId] || []).filter(c => c.status === 'resolved').length
  },
  addComment(itemId, { author, content, reference }) {
    if (!this.comments[itemId]) this.comments[itemId] = []
    this.comments[itemId].push({
      id: 'c' + Date.now(),
      author,
      content,
      contentZh: content,
      reference,
      status: 'open',
      createdAt: now(),
      resolvedAt: null,
    })
  },
  resolveComment(itemId, commentId) {
    const c = (this.comments[itemId] || []).find(c => c.id === commentId)
    if (c) { c.status = 'resolved'; c.resolvedAt = now() }
  },
  archiveResolved(itemId) {
    const resolved = (this.comments[itemId] || []).filter(c => c.status === 'resolved')
    if (!resolved.length) return
    if (!this.archive[itemId]) this.archive[itemId] = []
    this.archive[itemId].unshift({ archivedAt: now(), comments: resolved.map(c => ({ ...c })) })
    this.comments[itemId] = this.comments[itemId].filter(c => c.status === 'open')
  },
  exportForAgent(itemId) {
    const item = this.getItem(itemId)
    const open = (this.comments[itemId] || []).filter(c => c.status === 'open')
    return {
      schema_version: '1.0',
      item: { id: item.id, type: item.type, title: item.title },
      summary: {
        total: (this.comments[itemId] || []).length,
        open: open.length,
        resolved: this.resolvedCount(itemId),
      },
      open_comments: open.map(({ id, author, content, reference, createdAt }) => ({
        id, author, content, reference, created_at: createdAt,
      })),
      exported_at: now(),
    }
  },
})

export const PLAN_CONTENT = `# Sprint 1 — System Design

## Overview

Revi is an agent-human coworking review service. This document covers the architecture for the MVP.

## Scope

The MVP focuses on two core flows:
1. **Human Review Flow** — browse items, leave comments, resolve feedback
2. **Agent Consume Flow** — fetch structured export, update plan, mark resolved

## Data Models

Each review item is auto-discovered from the \`workspace/\` directory. Items are categorized as \`plan\`, \`design\`, or \`prototype\` based on the subfolder.

### Comment Schema

\`\`\`json
{
  "id": "uuid",
  "item_id": "plans/sprint-1-design",
  "author": "string",
  "content": "string",
  "reference": { "type": "section | line | general", "value": "## Architecture" },
  "status": "open | resolved",
  "created_at": "ISO8601",
  "resolved_at": null
}
\`\`\`

## Storage Layer

Comments are stored as JSON files in \`data/comments/{item_id}.json\`. This keeps everything file-system portable and git-friendly.

## Agent Export Format

The \`/api/export/{item_id}\` endpoint returns:

\`\`\`json
{
  "schema_version": "1.0",
  "item": { "id": "...", "type": "...", "title": "..." },
  "summary": { "total": 5, "open": 3, "resolved": 2 },
  "open_comments": [...],
  "exported_at": "ISO8601"
}
\`\`\`

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | \`/api/reviews\` | List all review items |
| POST | \`/api/comments/{item_id}\` | Add a comment |
| PATCH | \`/api/comments/{item_id}/{comment_id}/resolve\` | Resolve a comment |
| POST | \`/api/archive/{item_id}\` | Archive resolved |
| GET | \`/api/export/{item_id}\` | Agent export |

## Next Steps

- [ ] Keep Rust backend as the single runtime truth
- [ ] Wire frontend to the base-aligned runtime and docs/demo entry points
- [ ] Add file-upload support for designs
`
