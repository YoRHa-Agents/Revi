# Sprint 1 — System Design

## Overview

Revi is an agent-human coworking review service. This document covers the
architecture for the Rust + Vue MVP runtime.

## Scope

The MVP focuses on two core flows:
1. **Human Review Flow** — browse items, leave comments, resolve feedback.
2. **Agent Consume Flow** — fetch structured export, update plan, mark resolved.

## Data Models

Each review item is auto-discovered from the `workspace/` directory. Items are
categorized as `plan`, `design`, or `prototype` based on the subfolder.

### Comment Schema

```json
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
```

## Storage Layer

Comments are stored as JSON files in `data/comments/{item_id}.json`. This keeps
everything file-system portable and git-friendly.

## Agent Export Format

The `/api/export/{item_id}` endpoint returns:

```json
{
  "schema_version": "1.0",
  "item": { "id": "...", "type": "...", "title": "..." },
  "summary": { "total": 5, "open": 3, "resolved": 2 },
  "open_comments": [...],
  "exported_at": "ISO8601"
}
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/reviews` | List all review items |
| POST | `/api/comments/{item_id}` | Add a comment |
| PATCH | `/api/comments/{item_id}/{comment_id}/resolve` | Resolve a comment |
| POST | `/api/archive/{item_id}` | Archive resolved |
| GET | `/api/export/{item_id}` | Agent export |

## Next Steps

- [ ] Keep Rust backend as the single runtime truth
- [ ] Wire frontend to the real API
- [ ] Add file-upload support for designs
