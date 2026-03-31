# Revi — Review & Revise

Revi is an agent-human coworking review and revise tool. Human reviewers browse markdown plans, design images, and interactive HTML prototypes in a split-pane Vue 3 SPA, leaving anchored comments that are pinned to specific sections, quoted text, line numbers, image annotation pins, or prototype step indices. AI agents consume a structured JSON export — available at `/api/export/{item_id}` — that surfaces only open comments with their anchor references, enabling agents to read feedback, take action, and mark items resolved without touching the UI.

---

## Quick Start — pre-built binary (recommended)

No runtime dependencies. Download the binary for your platform from `dist/`, drop it anywhere, and run.

```bash
# macOS (Apple Silicon)
./dist/revi-macos-aarch64

# Linux x86-64 (fully static, works on any distro)
./dist/revi-linux-x86_64
```

The binary auto-creates `~/.revi/workspace/` and `~/.revi/data/` on first run.

**With custom paths:**
```bash
./revi --workspace /my/docs --data /my/data --port 9000
```

**Config file** — create `revi.toml` next to the binary (or at `~/.config/revi/config.toml`):
```toml
workspace = "/my/docs"
data      = "/my/data"
port      = 9000
```

Then start the frontend:
```bash
cd frontend && npm install && npm run dev
# Open http://localhost:5173
```

---

## Quick Start — Python backend (development)

1. **Clone the repo**
   ```bash
   git clone <repo-url> && cd Revi
   ```

2. **Backend (Python ≥ 3.11):**

   ```bash
   cd backend

   # Option A — pip
   pip install -e ".[dev]"

   # Option B — uv (faster)
   uv sync --all-extras
   ```

3. **Start the backend**
   ```bash
   uvicorn backend.main:app --reload --port 8000
   ```

4. **Install frontend dependencies**
   ```bash
   cd frontend && npm install
   ```

5. **Start the frontend — open http://localhost:5173**
   ```bash
   npm run dev
   ```

---

## Architecture

```
┌──────────────┐     REST API      ┌────────────────────────┐
│  Vue 3 SPA   │ ←───────────────→ │  Rust binary (revi)    │
│  (port 5173) │                   │  or FastAPI backend     │
└──────────────┘                   │  (port 8000)            │
                                   ├────────────────────────┤
                                   │  workspace/             │
┌──────────────┐     JSON export   │    plans/*.md           │
│  AI Agent    │ ←───────────────→ │    designs/*.png        │
│  (any LLM)   │                   │    prototypes/*.html    │
└──────────────┘                   ├────────────────────────┤
                                   │  data/                  │
                                   │    comments/*.json      │
                                   │    archive/*.json       │
                                   └────────────────────────┘
```

---

## API Reference

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/reviews` | List all review items with open/resolved counts |
| `GET` | `/api/reviews/{item_id}` | Get detail for one item including content |
| `GET` | `/api/comments/{item_id}` | List all comments for an item |
| `POST` | `/api/comments/{item_id}` | Add a comment to an item |
| `PATCH` | `/api/comments/{item_id}/{comment_id}/resolve` | Mark a comment resolved |
| `GET` | `/api/archive/{item_id}` | List archived batches for an item |
| `POST` | `/api/archive/{item_id}` | Archive all resolved comments into a new batch |
| `GET` | `/api/export/{item_id}` | Agent export — open comments as structured JSON |
| `POST` | `/api/upload` | Upload a file into workspace (multipart `file` field) |
| `GET` | `/api/config` | View current server configuration |
| `PATCH` | `/api/config` | Update config file (takes effect on next restart for path changes) |

`{item_id}` uses the format `{subfolder}/{stem}`, e.g. `plans/sprint-1-design`.

### Config API

```bash
# View current config
curl http://localhost:8000/api/config

# Update config file
curl -X PATCH http://localhost:8000/api/config \
  -H "Content-Type: application/json" \
  -d '{"workspacePath":"/new/docs","port":9000}'
```

---

## Workspace Structure

The backend auto-discovers review items from the `workspace/` directory. Place files in the appropriate subfolder and restart (or use the upload API) — no configuration needed.

```
workspace/
  plans/          # Markdown documents (.md) — rendered with heading anchors
  designs/        # Image files (.png, .jpg, .gif, .webp, .svg) — shown with annotation pins
  prototypes/     # Interactive HTML files (.html) — shown as step-through prototype viewer
```

Items are typed automatically by subfolder: `plans/` → `plan`, `designs/` → `design`, `prototypes/` → `prototype`.

Comments are persisted as JSON files under the data directory:

```
data/
  comments/       # {subfolder}__{stem}.json — active open/resolved comments
  archive/        # {subfolder}__{stem}.json — list of archived batches
```

---

## Development

### Building from source

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run tests
make test-rust

# Build macOS binary
make build-macos        # → dist/revi-macos-aarch64

# Build Linux static binary (requires cargo-zigbuild + zig)
make install-rust       # one-time setup
make build-linux        # → dist/revi-linux-x86_64

# Dev server (uses backend/workspace and backend/data)
make dev-rust
```

### Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_API_BASE` | `http://localhost:8000` | Base URL for the backend |

Set `VITE_API_BASE` in `frontend/.env` or inline before `npm run dev`.

### Frontend scripts

```bash
npm run dev       # Start Vite dev server on port 5173
npm run build     # Production build to frontend/dist/
npm test          # Run Vitest unit tests (one-shot)
npm run bench     # Run Vitest benchmarks
```

### Backend scripts

```bash
# Rust
cargo test                          # Integration tests (53 tests)
cargo bench                         # Criterion benchmarks
./revi --help                       # Show all flags

# Python (reference implementation)
pytest tests/
uvicorn backend.main:app --reload --port 8000
```

---

---

# Revi — 审阅与修订

Revi 是一款 Agent 与人类协作的审阅修订工具。人类审阅者可在分屏式 Vue 3 单页应用中浏览 Markdown 计划文档、设计图和可交互 HTML 原型，并留下锚点评论——这些评论可以绑定到特定章节、引用文本、行号、图片标注引脚或原型步骤索引。AI Agent 通过结构化 JSON 导出接口（`/api/export/{item_id}`）消费数据，该接口仅返回开放状态的评论及其锚点引用，使 Agent 能够读取反馈、采取行动并将条目标记为已解决，无需触碰 UI。

---

## 快速开始 — 预编译二进制（推荐）

无需任何运行时依赖。从 `dist/` 目录下载对应平台的二进制文件，放到任意位置直接运行。

```bash
# macOS（Apple Silicon）
./dist/revi-macos-aarch64

# Linux x86-64（完全静态链接，适用于任意发行版）
./dist/revi-linux-x86_64
```

首次运行会自动创建 `~/.revi/workspace/` 和 `~/.revi/data/` 目录。

**自定义路径：**
```bash
./revi --workspace /my/docs --data /my/data --port 9000
```

**配置文件** — 在二进制文件同级目录创建 `revi.toml`（或 `~/.config/revi/config.toml`）：
```toml
workspace = "/my/docs"
data      = "/my/data"
port      = 9000
```

然后启动前端：
```bash
cd frontend && npm install && npm run dev
# 打开 http://localhost:5173
```

---

## 快速开始 — Python 后端（开发用）

1. **克隆仓库**
   ```bash
   git clone <repo-url> && cd Revi
   ```

2. **后端（Python ≥ 3.11）：**

   ```bash
   cd backend

   # 方案 A — pip
   pip install -e ".[dev]"

   # 方案 B — uv（更快）
   uv sync --all-extras
   ```

3. **启动后端**
   ```bash
   uvicorn backend.main:app --reload --port 8000
   ```

4. **安装前端依赖**
   ```bash
   cd frontend && npm install
   ```

5. **启动前端 — 打开 http://localhost:5173**
   ```bash
   npm run dev
   ```

---

## 架构图

```
┌──────────────┐     REST API      ┌────────────────────────┐
│  Vue 3 SPA   │ ←───────────────→ │  Rust 二进制（revi）    │
│  （端口 5173）│                   │  或 FastAPI 后端        │
└──────────────┘                   │  （端口 8000）          │
                                   ├────────────────────────┤
                                   │  workspace/             │
┌──────────────┐     JSON 导出     │    plans/*.md           │
│  AI Agent    │ ←───────────────→ │    designs/*.png        │
│  （任意 LLM）│                   │    prototypes/*.html    │
└──────────────┘                   ├────────────────────────┤
                                   │  data/                  │
                                   │    comments/*.json      │
                                   │    archive/*.json       │
                                   └────────────────────────┘
```

---

## API 参考

| 方法 | 路径 | 描述 |
|------|------|------|
| `GET` | `/api/reviews` | 列出所有审阅条目（含开放/已解决计数） |
| `GET` | `/api/reviews/{item_id}` | 获取单个条目的详情（含内容） |
| `GET` | `/api/comments/{item_id}` | 列出条目的所有评论 |
| `POST` | `/api/comments/{item_id}` | 为条目添加评论 |
| `PATCH` | `/api/comments/{item_id}/{comment_id}/resolve` | 将评论标记为已解决 |
| `GET` | `/api/archive/{item_id}` | 列出条目的归档批次 |
| `POST` | `/api/archive/{item_id}` | 将所有已解决评论归档为新批次 |
| `GET` | `/api/export/{item_id}` | Agent 导出——以结构化 JSON 返回开放评论 |
| `POST` | `/api/upload` | 上传文件到工作区（multipart `file` 字段） |
| `GET` | `/api/config` | 查看当前服务器配置 |
| `PATCH` | `/api/config` | 更新配置文件（路径变更需重启生效） |

`{item_id}` 格式为 `{子目录}/{文件名（不含扩展名）}`，例如 `plans/sprint-1-design`。

---

## 工作区结构

后端会自动从 `workspace/` 目录发现审阅条目。将文件放入对应子目录后重启服务（或使用上传 API）即可，无需任何配置。

```
workspace/
  plans/          # Markdown 文档（.md）——渲染为带标题锚点的页面
  designs/        # 图片文件（.png、.jpg、.gif、.webp、.svg）——显示为带标注引脚的设计图
  prototypes/     # 可交互 HTML 文件（.html）——显示为步进式原型查看器
```

条目类型由子目录自动确定：`plans/` → `plan`，`designs/` → `design`，`prototypes/` → `prototype`。

评论数据以 JSON 文件形式持久化于数据目录：

```
data/
  comments/       # {子目录}__{文件名}.json——存储活跃的开放/已解决评论
  archive/        # {子目录}__{文件名}.json——存储归档批次列表
```

---

## 开发说明

### 从源码构建

```bash
# 安装 Rust（如未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 运行测试
make test-rust

# 构建 macOS 二进制
make build-macos        # → dist/revi-macos-aarch64

# 构建 Linux 静态二进制（需要 cargo-zigbuild + zig）
make install-rust       # 一次性安装
make build-linux        # → dist/revi-linux-x86_64

# 开发服务器（使用 backend/workspace 和 backend/data）
make dev-rust
```

### 环境变量

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `VITE_API_BASE` | `http://localhost:8000` | 后端的 Base URL |

在 `frontend/.env` 中设置 `VITE_API_BASE`，或在 `npm run dev` 之前以内联方式传入。

### 前端脚本

```bash
npm run dev       # 在 5173 端口启动 Vite 开发服务器
npm run build     # 生产构建，输出至 frontend/dist/
npm test          # 运行 Vitest 单元测试（单次执行）
npm run bench     # 运行 Vitest 基准测试
```

### 后端脚本

```bash
# Rust
cargo test                          # 集成测试（53 个测试用例）
cargo bench                         # Criterion 基准测试
./revi --help                       # 显示所有命令行参数

# Python（参考实现）
pytest tests/
uvicorn backend.main:app --reload --port 8000
```
