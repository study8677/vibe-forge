# LWH 日志监控中心

> **提示词:** 做一个日志查看工具。左侧管理服务器连接（单机/集群切换），右侧实时推送日志流（点击服务名快速切日志）。选中日志一键调用 Gemini、Claude 等 AI 接口解析报错（内嵌对话窗）。

## 项目简介

LWH（Log Watcher Hub）是一个全栈实时日志查看与分析工具，通过 SSH 连接远程服务器，实时推送日志流到浏览器，并内嵌 AI 对话窗口对选中的报错日志进行智能解析。面向运维工程师和后端开发者。

## 核心功能

- **服务器连接管理**：支持密码/SSH Key 认证，单机与集群两种视图模式
- **自动服务发现**：连接后自动检测 systemd 服务、Docker 容器、PM2 进程、日志文件
- **实时日志流**：通过 WebSocket 推送，支持按级别（ERROR/WARN/INFO/DEBUG）过滤和关键词搜索
- **AI 报错分析**：选中日志文本一键调用 Claude 或 Gemini 流式解析，内嵌对话窗支持追问
- **暗色主题**：专为日志阅读优化的深色 UI，日志级别颜色编码

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | React 18 + TypeScript |
| 构建工具 | Vite 6 |
| UI 样式 | Tailwind CSS |
| 状态管理 | Zustand |
| 实时通信 | Socket.IO |
| 后端服务 | Express |
| SSH 连接 | ssh2 |
| AI 集成 | Anthropic SDK + Google Generative AI |

## 文件说明

- `server/index.ts` — Express 主服务，REST API + WebSocket
- `server/ssh-manager.ts` — SSH 连接池，服务自动检测
- `server/log-streamer.ts` — 实时日志 tail 流
- `server/ai-service.ts` — Claude & Gemini 流式代理
- `src/components/sidebar/` — 左侧服务器管理面板
- `src/components/log-viewer/` — 右侧日志查看器
- `src/components/ai-chat/` — 底部 AI 分析对话窗

## 运行方式

```bash
# 安装依赖
npm install

# 配置 AI API Key（可选）
cp .env.example .env
# 编辑 .env 填入 ANTHROPIC_API_KEY 和/或 GEMINI_API_KEY

# 启动开发服务器（前端 + 后端同时启动）
npm run dev

# 生产构建
npm run build && npm start
```

打开 `http://localhost:5173`，在左侧添加服务器连接后即可使用。
