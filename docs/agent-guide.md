# Revi — Agent API Reference

This document is the complete API reference for AI agents integrating with Revi. It covers all endpoints, request/response schemas, error codes, reference types, and recommended polling strategies.

The supported runtime behind these endpoints is the Rust `revi` server. Any remaining Python assets in this repository are historical reference or tooling inputs, not the product API runtime.

---

## Base URL

```
http://localhost:8000
```

All endpoints are prefixed with `/api/`.

---

## item_id Format

Every endpoint that targets a specific review item uses an `item_id` path parameter.

**Format:** `{subfolder}/{stem}`

- `subfolder` is one of `plans`, `designs`, or `prototypes`
- `stem` is the filename without extension

**Examples:**

| File in workspace | item_id |
|-------------------|---------|
| `workspace/plans/sprint-1-design.md` | `plans/sprint-1-design` |
| `workspace/designs/ui-mockup-v1.png` | `designs/ui-mockup-v1` |
| `workspace/prototypes/review-flow.html` | `prototypes/review-flow` |

---

## Endpoints

### GET /api/reviews

List all review items discovered in the workspace.

**Response — 200 OK**

```json
[
  {
    "id": "plans/sprint-1-design",
    "type": "plan",
    "title": "Sprint 1 — System Design",
    "titleZh": "Sprint 1 — 系统设计",
    "description": "Architecture and data model for the Revi MVP.",
    "updatedAt": "2026-03-28T09:30:00Z",
    "openCount": 2,
    "resolvedCount": 1
  }
]
```

**Response fields**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Item identifier in `{subfolder}/{stem}` format |
| `type` | string | `"plan"` \| `"design"` \| `"prototype"` |
| `title` | string | Display title (English) |
| `titleZh` | string \| null | Display title (Chinese), if present |
| `description` | string \| null | Short description |
| `updatedAt` | ISO 8601 string | File modification timestamp |
| `openCount` | integer | Number of open (unresolved) comments |
| `resolvedCount` | integer | Number of resolved comments |

---

### GET /api/reviews/{item_id}

Get full detail for one review item, including a URL or text content for rendering.

**Path parameters**

| Parameter | Description |
|-----------|-------------|
| `item_id` | e.g. `plans/sprint-1-design` |

**Response — 200 OK**

```json
{
  "id": "plans/sprint-1-design",
  "type": "plan",
  "title": "Sprint 1 — System Design",
  "titleZh": null,
  "description": null,
  "updatedAt": "2026-03-28T09:30:00Z",
  "openCount": 2,
  "resolvedCount": 1,
  "contentUrl": "/api/workspace/plans/sprint-1-design.md",
  "contentText": "# Sprint 1 — System Design\n\n..."
}
```

**Additional fields (extends ReviewItem)**

| Field | Type | Description |
|-------|------|-------------|
| `contentUrl` | string \| null | URL to fetch the raw file (for designs/prototypes) |
| `contentText` | string \| null | Raw text content (for plans/markdown) |

**Errors**

| Code | Condition |
|------|-----------|
| 404 | `item_id` not found in workspace |

---

### GET /api/comments/{item_id}

List all comments (open and resolved) for an item.

**Response — 200 OK**

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "itemId": "plans/sprint-1-design",
    "author": "Agent-Planner",
    "content": "The export format is missing a schema_version field.",
    "reference": {
      "type": "quote",
      "value": "\"schema_version\": \"1.0\"",
      "section": "Agent Export Format",
      "label": null
    },
    "status": "open",
    "createdAt": "2026-03-28T08:00:00Z",
    "resolvedAt": null
  }
]
```

**Comment fields**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string (UUID) | yes | Unique comment identifier |
| `itemId` | string | yes | The item this comment belongs to |
| `author` | string | yes | Name of the commenter (human or agent) |
| `content` | string | yes | Comment body text |
| `reference` | Reference \| null | no | Anchor pointing to the specific location |
| `status` | string | yes | `"open"` \| `"resolved"` |
| `createdAt` | ISO 8601 string | yes | When the comment was created |
| `resolvedAt` | ISO 8601 string \| null | no | When the comment was resolved, or null |

**Errors**

| Code | Condition |
|------|-----------|
| 404 | `item_id` not found |

---

### POST /api/comments/{item_id}

Add a new comment to an item.

**Request body**

```json
{
  "author": "Agent",
  "content": "This section needs more detail on error handling.",
  "reference": {
    "type": "section",
    "value": "## API Endpoints"
  }
}
```

**Request fields**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `author` | string | yes | Name of the commenter |
| `content` | string | yes | Comment body text |
| `reference` | Reference \| null | no | Anchor for the comment (see Reference Types below) |

**Response — 201 Created**

Returns the created Comment object (same schema as GET /api/comments response items).

**Errors**

| Code | Condition |
|------|-----------|
| 400 | Missing required fields (`author` or `content`) |
| 404 | `item_id` not found |
| 422 | Invalid reference type or malformed request body |

---

### PATCH /api/comments/{item_id}/{comment_id}/resolve

Mark a comment as resolved. Idempotent — resolving an already-resolved comment is a no-op.

**Path parameters**

| Parameter | Description |
|-----------|-------------|
| `item_id` | e.g. `plans/sprint-1-design` |
| `comment_id` | UUID of the comment to resolve |

**Response — 200 OK**

Returns the updated Comment object with `status: "resolved"` and `resolvedAt` set.

**Errors**

| Code | Condition |
|------|-----------|
| 404 | `item_id` or `comment_id` not found |

---

### GET /api/archive/{item_id}

List all archived batches for an item, newest first.

**Response — 200 OK**

```json
[
  {
    "archivedAt": "2026-03-27T15:00:00Z",
    "comments": [
      {
        "id": "a1b2c3d4-...",
        "itemId": "plans/sprint-1-design",
        "author": "Agent-Planner",
        "content": "Initial scope is too broad.",
        "reference": { "type": "section", "value": "## Scope" },
        "status": "resolved",
        "createdAt": "2026-03-27T10:00:00Z",
        "resolvedAt": "2026-03-27T14:55:00Z"
      }
    ]
  }
]
```

**ArchiveBatch fields**

| Field | Type | Description |
|-------|------|-------------|
| `archivedAt` | ISO 8601 string | When this batch was created |
| `comments` | Comment[] | Resolved comments included in this batch |

---

### POST /api/archive/{item_id}

Archive all currently resolved comments for an item into a new timestamped batch. After archiving, resolved comments are removed from the active comment list.

**Response — 200 OK**

Returns the newly created ArchiveBatch object.

**Errors**

| Code | Condition |
|------|-----------|
| 404 | `item_id` not found |
| 400 | No resolved comments to archive |

---

### GET /api/export/{item_id}

Agent export endpoint. Returns only open (unresolved) comments for the item, together with item metadata and a schema version identifier.

**Response — 200 OK**

```json
{
  "schemaVersion": "1.0",
  "item": {
    "id": "plans/sprint-1-design",
    "type": "plan",
    "title": "Sprint 1 — System Design"
  },
  "summary": {
    "total": 3,
    "open": 2,
    "resolved": 1
  },
  "openComments": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "author": "Alice",
      "content": "The storage layer should support SQLite from the start.",
      "reference": {
        "type": "quote",
        "value": "Comments are stored as JSON files",
        "section": "Storage Layer",
        "label": null
      },
      "createdAt": "2026-03-28T08:00:00Z"
    }
  ],
  "exportedAt": "2026-03-28T10:00:00Z"
}
```

**ExportResponse fields**

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | string | Always `"1.0"` — check this before parsing |
| `item` | ExportItem | Minimal item metadata |
| `summary` | ExportSummary | Comment counts |
| `openComments` | ExportComment[] | Only open comments; empty array when all resolved |
| `exportedAt` | ISO 8601 string | Server time when this export was generated |

**ExportItem fields**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Item identifier |
| `type` | string | `"plan"` \| `"design"` \| `"prototype"` |
| `title` | string | Item title |

**ExportSummary fields**

| Field | Type | Description |
|-------|------|-------------|
| `total` | integer | Total comments (open + resolved) |
| `open` | integer | Open comment count |
| `resolved` | integer | Resolved comment count |

**ExportComment fields**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (UUID) | Comment identifier (use this for resolve/archive calls) |
| `author` | string | Commenter name |
| `content` | string | Comment body |
| `reference` | Reference \| null | Anchor pointing to the relevant location |
| `createdAt` | ISO 8601 string | When the comment was created |

**Errors**

| Code | Condition |
|------|-----------|
| 404 | `item_id` not found |

---

### POST /api/upload

Upload a file into the workspace. The Rust server infers the target subfolder from
the file extension, or you can override it with a multipart `type` field.

**Request**

`multipart/form-data` with a `file` field. Optional `type` values:
`plan`, `design`, `prototype`.

```bash
curl -X POST http://localhost:8000/api/upload \
  -F "file=@sprint-2-plan.md"
```

```bash
curl -X POST http://localhost:8000/api/upload \
  -F "file=@wireframe.bin" \
  -F "type=design"
```

**Response — 201 Created**

```json
{
  "itemId": "plans/sprint-2-plan",
  "filename": "sprint-2-plan.md",
  "url": "/workspace/plans/sprint-2-plan.md"
}
```

**Errors**

| Code | Condition |
|------|-----------|
| 400 | Invalid `type` or unsupported file type |
| 400 | Missing file field |

---

## Reference Types

A `Reference` object anchors a comment to a specific location within the review item. All fields except `type` are optional depending on the reference type.

**Reference fields**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | yes | One of the types listed below |
| `value` | string \| integer \| null | depends | The anchor value (see table) |
| `section` | string \| null | no | For `quote` type: the heading the quote appears under |
| `label` | string \| null | no | Human-readable label for the anchor |

**Reference type table**

| type | value type | value example | use case |
|------|-----------|---------------|----------|
| `section` | string | `"## Overview"` | Markdown heading anchor — links to a specific `##` heading |
| `quote` | string | `"Comments are stored as JSON files"` | Inline text selection — the exact text that was selected |
| `line` | integer | `42` | Line number in the document |
| `annotation` | string | `"3"` | Image annotation pin number (stringified integer) |
| `step` | integer | `1` | Prototype step index, 0-based (step 1 = second step) |
| `general` | null | `null` | No specific anchor — comment applies to the whole item |

**Example Reference objects**

```json
{ "type": "section", "value": "## Architecture" }
{ "type": "quote", "value": "file-system portable", "section": "Storage Layer" }
{ "type": "line", "value": 42 }
{ "type": "annotation", "value": "3", "label": "Comment Panel" }
{ "type": "step", "value": 1, "label": "Open for Review" }
{ "type": "general", "value": null }
```

---

## Schema Version Compatibility

The export endpoint always includes `"schemaVersion": "1.0"`. Agents **must** check this field before parsing the response body:

```python
export = requests.get("http://localhost:8000/api/export/plans/sprint-1-design").json()
assert export["schemaVersion"] == "1.0", f"Unexpected schema version: {export['schemaVersion']}"
```

Future breaking changes to the export schema will increment the version number. Non-breaking additions (new optional fields) will not change the version.

---

## Agent Export Workflow

The recommended workflow for an agent processing feedback on a review item:

### 1. Fetch the export

```bash
curl http://localhost:8000/api/export/plans/sprint-1-design
```

Check `openComments` array. If empty, the item has no pending feedback — skip it.

### 2. Process each comment

Read `content` and `reference` to understand what the human is asking. Use `reference.type` and `reference.value` to locate the relevant part of the document.

### 3. Resolve each comment after acting on it

```bash
curl -X PATCH \
  http://localhost:8000/api/comments/plans/sprint-1-design/COMMENT_ID/resolve
```

### 4. Archive resolved comments (optional but recommended)

Once all open comments for a review round are resolved, archive them to keep the workspace clean:

```bash
curl -X POST http://localhost:8000/api/archive/plans/sprint-1-design
```

### 5. Verify the loop is complete

```bash
curl http://localhost:8000/api/export/plans/sprint-1-design
# openComments should now be []
```

### Polling strategy

`GET /api/export/{item_id}` returns only open comments. An agent can poll this endpoint to detect new feedback:

```python
import time, requests

def poll_for_feedback(item_id: str, interval: int = 30):
    url = f"http://localhost:8000/api/export/{item_id}"
    while True:
        data = requests.get(url).json()
        if data["openComments"]:
            process_comments(data["openComments"])
        time.sleep(interval)
```

A reasonable poll interval is 15–60 seconds depending on expected review latency.

---

## camelCase Note

All JSON responses use **camelCase** field names (generated by Pydantic's `alias_generator=to_camel`). When sending request bodies, both camelCase and snake_case are accepted. Examples:

| Model field | JSON key in response |
|-------------|---------------------|
| `item_id` | `itemId` |
| `created_at` | `createdAt` |
| `resolved_at` | `resolvedAt` |
| `open_count` | `openCount` |
| `schema_version` | `schemaVersion` |
| `open_comments` | `openComments` |
| `exported_at` | `exportedAt` |
| `archived_at` | `archivedAt` |
| `content_url` | `contentUrl` |
| `content_text` | `contentText` |
