# Demo Storyboard

## 页面地图
- `/`
- `/demo/`
- `/guide/`
- `/#/review/:itemId+`
- `/#/archive`

## 页面职责
- `Landing`：解释 Revi 现在如何以 `RustWebAppCommon` 为 base，并保持 review domain 为 app-owned。
- `Demo`：展示静态交互样例与 plan/design/prototype 三类 review 体验。
- `Guide`：面向 human reviewer 的前门说明。
- `Review App`：Hash-router SPA，连接 Rust API 与 anchored comments。
- `Archive`：展示归档后的 review rounds。

## 当前约束
- `site/`、`docs/`、README 与前端 UI 必须使用同一条 Rust + Vue 单轨叙事。
- `common` 负责 docs/demo/release 入口语义，`revi` 负责产品 runtime。
- Review domain 的业务模型保持 `app_owned`，不写回 common。
- Pages 子路径 `/Revi/` 与 hash-router 路径必须持续可用。
