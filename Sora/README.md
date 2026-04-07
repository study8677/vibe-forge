# Sora 笔记

> **提示词:** 基于Next.js构建个人笔记全栈应用，要求支持markdown渲染，支持用任务看板管理笔记。风格要求与notion一致。

## 项目简介

Sora 笔记是一款 Notion 风格的个人笔记全栈应用，支持 Markdown 编辑与实时预览，内置任务看板用于拖拽管理笔记状态。面向需要轻量级笔记和任务管理工具的个人用户。

## 核心功能

- **Markdown 编辑器** — 分栏实时预览，支持 GFM 语法（表格、任务列表、代码高亮）
- **任务看板** — 四列看板（待整理/待办/进行中/已完成），拖拽切换状态
- **自动保存** — 600ms 防抖自动保存，页面切换即时保存
- **笔记管理** — Emoji 图标、状态标签、置顶、搜索过滤
- **响应式设计** — 桌面端固定侧边栏，移动端抽屉式导航

## 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | Next.js 14 (App Router) |
| 前端 | React 18 + TypeScript |
| 样式 | Tailwind CSS (Notion 色系定制) |
| 数据库 | SQLite + Prisma ORM |
| Markdown | react-markdown + remark-gfm + rehype-highlight |
| 拖拽 | @hello-pangea/dnd |
| 图标 | lucide-react |

## 文件说明

- `src/app/` — 页面路由和 API 路由
- `src/components/Sidebar.tsx` — 侧边栏导航与笔记列表
- `src/components/NoteEditor.tsx` — Markdown 编辑器（分栏预览、自动保存）
- `src/components/KanbanBoard.tsx` — 任务看板（拖拽排序）
- `src/components/MarkdownRenderer.tsx` — Markdown 渲染组件
- `src/lib/prisma.ts` — Prisma 客户端单例
- `prisma/schema.prisma` — 数据库模型定义
- `prisma/seed.ts` — 种子数据（6 篇示范笔记）

## 运行方式

```bash
npm install
npx prisma db push
npm run db:seed    # 可选：导入示范笔记
npm run dev
```

访问 http://localhost:3000 即可使用。
