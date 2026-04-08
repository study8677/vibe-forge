# Hiccup90 NAS Dashboard

> **提示词:** 做一个适合NAS的导航页，要求功能性和自由度媲美 gethomepage/homepage，支持前端直接修改设置，风格美观现代，整体轻量高性能。

## 项目简介

这是一个适合 NAS 和自托管环境的轻量导航页，强调静态部署、快速加载和前端直接编辑配置。它参考了 `homepage` 的自由度，以及 `sun-panel` / `dashdot` 的视觉与交互方向，但保持零依赖实现，方便直接放到 NAS 静态服务中使用。

## 核心功能

- 前端直接编辑标题、主题、分组、服务链接和摘要小组件
- 本地 `localStorage` 持久化，支持 JSON 导入导出
- 支持分组折叠、服务搜索和编辑模式下的拖拽排序
- 默认提供现代化的深浅主题、背景样式和紧凑/舒适布局密度
- 自带核心逻辑单元测试与本地校验脚本

## 技术栈

| 层级 | 技术 |
|------|------|
| 页面结构 | HTML5 |
| 表现层 | CSS3 |
| 交互层 | JavaScript ES Modules |
| 配置存储 | localStorage |
| 测试与校验 | Node.js `node:test` + `node --check` |

## 文件说明

- `index.html` — 应用入口和页面骨架
- `src/app.js` — 状态管理、事件绑定、编辑和拖拽交互
- `src/ui/render.js` — 页面各区域的渲染模板
- `src/core/` — 配置归一化、搜索、存储等核心逻辑
- `src/styles.css` — 主题、布局和视觉样式
- `tests/` — 核心行为测试
- `docs/superpowers/specs/2026-04-08-nas-dashboard-design.md` — 设计说明

## 运行方式

1. 直接用静态服务器打开项目目录，例如：

```bash
python3 -m http.server 4173
```

2. 浏览器访问 `http://127.0.0.1:4173`
3. 如需校验项目，可运行：

```bash
npm run check
```
