# 始皇防蛐蛐 · ShiHuang Guard

> **提示词:** 全栈专家角色，用 Go/Node.js 构建高并发后端。设计插件化架构与实时监控系统。加入"始皇防蛐蛐"模块：正则监听全站关键词，触发自动化预警。界面现代且易扩展。

## 项目简介

全站关键词正则监控与自动预警系统。Go 后端通过插件化架构实现高并发内容扫描，内置"始皇防蛐蛐"模块提供 8 条默认正则规则（广告/短链/刷屏/引战/诈骗/引流/机器人/数据泄露），WebSocket 实时推送预警到 React 前端仪表盘。

## 核心功能

- **插件化架构** — 实现 `Plugin` 接口即可热插拔新监控模块
- **高并发扫描引擎** — goroutine 并行扇出到所有插件，单次扫描 ~45µs
- **始皇防蛐蛐模块** — 8 类正则规则覆盖常见违规场景，支持动态增删
- **实时监控面板** — WebSocket 推送 + React 暗色主题仪表盘
- **正则在线测试** — 前端内置 regex tester，创建规则前即时验证

## 技术栈

| 层级 | 技术 |
|------|------|
| 后端核心 | Go 1.22 (标准库 HTTP + gorilla/websocket) |
| 扫描引擎 | 并发 fan-out，Plugin 接口 + Registry |
| 前端 | React 18 + Vite + Tailwind CSS |
| 实时通信 | WebSocket (hub broadcast) |
| 持久化 | JSON 文件存储 |
| Node.js 工具 | webhook-receiver + benchmark |

## 文件说明

- `cmd/server/main.go` — 服务入口，组装所有组件
- `internal/engine/engine.go` — 核心扫描引擎（并发派发）
- `internal/plugin/types.go` — Plugin 接口定义
- `internal/plugin/registry.go` — 线程安全插件注册中心
- `internal/plugins/cricket/guard.go` — 始皇防蛐蛐模块
- `internal/hub/hub.go` — WebSocket 广播中心
- `internal/handler/handler.go` — REST API + WS 路由
- `internal/store/store.go` — 告警持久化
- `web/src/components/` — React 前端组件（Dashboard/AlertFeed/RuleManager/Scanner/PluginPanel）
- `tools/webhook-receiver.js` — Node.js webhook 网关
- `tools/benchmark.js` — 并发压测工具

## 运行方式

```bash
# 安装依赖 & 编译
go mod tidy
go build -o bin/shihuang-guard ./cmd/server

# 启动后端 (默认 :8080)
PORT=9090 ./bin/shihuang-guard

# 启动前端开发服务器 (另一个终端)
cd web && npm install && npm run dev

# 或一次性构建并运行
make all && PORT=9090 make run

# 压测
node tools/benchmark.js
```

打开浏览器访问 `http://localhost:5173`（开发模式）即可体验实时监控面板。
