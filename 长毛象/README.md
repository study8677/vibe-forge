# 个人物品管理

> **提示词:** 开发一个个人物品管理应用，nextjs，web、移动端适配。

## 项目简介

基于 Next.js 构建的个人物品管理 Web 应用，帮助用户分类、定位和追踪自己的物品。采用移动端优先的响应式设计，在手机和桌面浏览器上均有良好体验。

## 核心功能

- 物品 CRUD：添加、编辑、删除物品，支持拍照/上传图片（自动压缩）
- 搜索与筛选：按名称搜索，按分类、位置筛选，支持多维排序
- 分类管理：预置 9 个分类，可自定义图标和颜色
- 位置管理：预置 7 个存放位置，支持自定义
- 仪表盘：统计物品总数、总价值、分类概览和最近添加

## 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | Next.js 14 (App Router) |
| 语言 | TypeScript |
| 样式 | Tailwind CSS |
| 状态管理 | React Context + useReducer |
| 数据持久化 | localStorage |

## 文件说明

- `app/page.tsx` — 首页仪表盘，统计概览
- `app/items/page.tsx` — 物品列表，搜索/筛选/排序
- `app/items/new/page.tsx` — 添加新物品表单
- `app/items/[id]/page.tsx` — 物品详情查看与编辑
- `app/categories/page.tsx` — 分类管理
- `app/locations/page.tsx` — 位置管理
- `lib/store.tsx` — 全局状态管理（Context + localStorage）
- `lib/types.ts` — TypeScript 类型定义
- `components/` — 共享 UI 组件（导航、卡片、弹窗、图标等）

## 运行方式

```bash
npm install
npm run dev
```

打开 `http://localhost:3000` 即可体验。
