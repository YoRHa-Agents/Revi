# Demo Storyboard

## 页面地图
- `/`
- `/demo/`
- `/guide/`
- `/#/review/:itemId+`
- `/#/archive`

## 页面职责
- `Landing`：解释 Revi 现在如何以 `RustWebAppCommon` 为 base，并保持 review domain 为 app-owned。
- `Demo`：展示静态交互样例与 plan/design/prototype 三类 review 体验。包含 workspace setup 流程预览。
- `Guide`：面向 human reviewer 的前门说明，包括无 workspace 启动与远程连接。
- `Review App`：Hash-router SPA，连接 Rust API 与 anchored comments。启动时检查 workspace 配置状态，未配置则展示 workspace 选择器。
- `Archive`：展示归档后的 review rounds。

## 当前约束
- `site/`、`docs/`、README 与前端 UI 必须使用同一条 Rust + Vue 单轨叙事。
- `common` 负责 docs/demo/release 入口语义，`revi` 负责产品 runtime。
- Review domain 的业务模型保持 `app_owned`，不写回 common。
- Pages 子路径 `/Revi/` 与 hash-router 路径必须持续可用。
- Revi 支持无 workspace 启动：前端根据 `GET /api/config` 的 `workspaceConfigured` 字段决定展示 workspace 选择器还是 dashboard。
- Workspace 可通过 web UI 设置本地路径或连接远程服务器，`PATCH /api/config` 热更新无需重启。
