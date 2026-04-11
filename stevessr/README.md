# Stevessr — 纯 Rust 重写的 Discourse 论坛后端

> **提示词:** 将 Discourse 完全转换为纯 Rust 后端项目，要求功能 1:1 还原，同时为插件提供工业级完备接口

## 项目简介

Stevessr 是 Discourse（Ruby on Rails 论坛平台）的纯 Rust 后端重写，目标是功能 1:1 还原并提供工业级插件系统。项目采用 Axum + SQLx + PostgreSQL + Redis 技术栈，包含 345 个 Rust 源文件、24K+ 行代码、95+ 数据库表、200+ API 端点和 120+ 插件钩子点。

## 核心功能

- **完整论坛功能**：用户（信任等级 0-4）、话题、帖子、分类、标签、群组、通知、私信、搜索、审核、徽章、投票、聊天
- **工业级插件系统**：120+ Before/After 钩子、25+ 扩展注册点、50+ 异步事件、原生 + WASM 双运行时
- **全套 API**：200+ REST 端点，镜像 Discourse 控制器结构
- **后台任务**：Redis 驱动的异步任务队列，10+ 内置任务类型
- **Markdown 渲染管线**：@提及、#话题标签、:emoji:、引用、剧透、详情折叠

## 技术栈

| 层级 | 技术 |
|------|------|
| Web 框架 | Axum 0.8 |
| 数据库 | PostgreSQL (SQLx 0.8) |
| 缓存/队列 | Redis |
| 全文搜索 | Tantivy |
| 异步运行时 | Tokio |
| 插件运行时 | libloading (原生) + wasmtime (WASM) |
| 认证 | Argon2 + JWT + OAuth2 + SSO |
| 邮件 | Lettre (发送) + mail-parser (接收) |
| Markdown | pulldown-cmark + ammonia |

## 工作区结构

| Crate | 文件数 | 说明 |
|-------|--------|------|
| `stevessr-core` | 14 | 领域类型、ID、枚举、Trait、错误层级 |
| `stevessr-db` | 103 | 95+ 表模型、CRUD、查询构建器 |
| `stevessr-services` | 106 | 21 个服务域的完整业务逻辑 |
| `stevessr-plugin-api` | 13 | 插件接口（钩子、事件、扩展、存储、FFI） |
| `stevessr-plugin-host` | 13 | 插件加载、原生 + WASM 运行时、钩子分发 |
| `stevessr-api` | 54 | 200+ Axum 路由、提取器、序列化器、WebSocket |
| `stevessr-markdown` | 15 | Markdown 渲染管线 |
| `stevessr-jobs` | 15 | Redis 后台任务队列 |
| `stevessr-cache` | 7 | Redis 缓存层 |
| `stevessr-server` | 1 | 主二进制入口 |

## 运行方式

```bash
# 前置依赖：Rust 1.85+, PostgreSQL, Redis

# 配置数据库
cp .env.example .env
# 编辑 .env 设置 DATABASE_URL 和 REDIS_URL

# 编译检查
cargo check --workspace

# 运行服务器
cargo run -p stevessr-server

# 管理 CLI
cargo run -p stevessr-cli -- migrate
cargo run -p stevessr-cli -- create-admin -u admin -e admin@example.com -p yourpassword
```
