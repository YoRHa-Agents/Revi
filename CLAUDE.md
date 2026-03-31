# Revi — Agent Guide

Revi is an agent-human review tool. Humans annotate plans/designs/prototypes; agents read structured JSON exports.

## Start servers

**Rust binary (recommended — zero dependencies):**
```bash
# Zero config: uses ~/.revi/workspace and ~/.revi/data
./dist/revi-macos-aarch64     # macOS Apple Silicon
./dist/revi-linux-x86_64      # Linux x86-64

# Custom paths
./revi --workspace backend/workspace --data backend/data --port 8000

# Or create revi.toml next to the binary:
# workspace = "backend/workspace"
# data      = "backend/data"
# port      = 8000
```

**Python backend (dev/reference):**
```bash
cd backend && uvicorn main:app --reload --port 8000
```

**Frontend (either backend):**
```bash
cd frontend && npm run dev
```

## Most-used curl examples

```bash
# List all review items
curl http://localhost:8000/api/reviews

# Get agent export for an item
curl http://localhost:8000/api/export/plans/sprint-1-design

# Add a comment
curl -X POST http://localhost:8000/api/comments/plans/sprint-1-design \
  -H "Content-Type: application/json" \
  -d '{"author":"Agent","content":"Needs more detail","reference":{"type":"section","value":"## Overview"}}'

# Resolve a comment (replace COMMENT_ID)
curl -X PATCH http://localhost:8000/api/comments/plans/sprint-1-design/COMMENT_ID/resolve

# Archive all resolved comments
curl -X POST http://localhost:8000/api/archive/plans/sprint-1-design

# View current server config
curl http://localhost:8000/api/config

# Update config file (restart required for path changes)
curl -X PATCH http://localhost:8000/api/config \
  -H "Content-Type: application/json" \
  -d '{"workspacePath":"/new/docs","port":9000}'
```

## item_id format

`{subfolder}/{stem}` — e.g. `plans/sprint-1-design`, `designs/ui-mockup-v1`

## Reference types

| type | value | use case |
|------|-------|----------|
| section | "## Heading" | markdown heading anchor |
| quote | "quoted text" | inline text selection |
| line | 42 | line number |
| annotation | "3" | image annotation pin number |
| step | 1 | prototype step index (0-based) |
| general | null | no specific anchor |

See `docs/agent-guide.md` for full reference.
