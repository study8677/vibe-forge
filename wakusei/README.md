# 惑星 AI 终端

> **提示词:** 用 Next.js 搭个粗野主义 AI 聊天页面，TypeScript 全栈。左边填 API 地址和 Key，存 localStorage。主区域聊天，走后端代理流式输出。历史对话存 IndexedDB，能点开回看。样式要硬边框、纯直角、黑黄白配色。

## 项目简介

一个粗野主义（Brutalist）风格的 AI 聊天终端。支持接入任意 OpenAI 兼容 API，通过后端代理实现流式输出，所有对话历史存储在浏览器 IndexedDB 中，无需后端数据库。

## 核心功能

- **API 自由配置** — 填入任意 OpenAI 兼容的 API 地址和 Key，存 localStorage
- **流式输出** — 后端 Edge Runtime 代理转发 SSE，前端逐 token 渲染
- **对话持久化** — IndexedDB 存储所有历史对话，点击即可回看
- **粗野主义 UI** — 硬边框、纯直角、黑黄白三色、等宽字体、终端交互风格
- **响应式布局** — 桌面端侧边栏常驻，移动端可折叠

## 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | Next.js 16 (App Router) |
| 语言 | TypeScript |
| 样式 | Tailwind CSS v4 |
| 存储 | localStorage + IndexedDB |
| API | Edge Runtime Route Handler (SSE proxy) |
| 字体 | JetBrains Mono |

## 文件说明

- `src/app/page.tsx` — 主页面，组合 Sidebar 和 ChatArea
- `src/app/api/chat/route.ts` — 后端流式代理，转发请求到用户配置的 API
- `src/components/Sidebar.tsx` — 左侧栏：API 配置 + 历史对话列表
- `src/components/ChatArea.tsx` — 主聊天区：消息展示 + 流式输出 + 输入框
- `src/lib/db.ts` — IndexedDB 封装层
- `src/app/globals.css` — 粗野主义全局样式

## 运行方式

```bash
npm install
npm run dev
# 访问 http://localhost:3000
```

在左侧栏填入 API 地址（如 `https://api.openai.com`）和 API Key，即可开始对话。
