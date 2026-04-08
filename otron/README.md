# OTron 远程连接工具

> **提示词:** 用electron搭建一个远程连接工具，需要拥有管理连接、连接导入导出、文件上传下载文件功能。

## 项目简介

OTron 是一个基于 Electron 的桌面端 SSH 远程连接管理工具，提供终端模拟、连接管理、SFTP 文件传输等核心功能，面向需要管理多台服务器的开发者和运维人员。

## 核心功能

- SSH 终端：基于 xterm.js 的 256 色终端，支持多会话标签页
- 连接管理：新建/编辑/删除连接，支持密码和密钥认证，密码通过系统钥匙串加密
- 导入/导出：连接配置 JSON 格式导入导出，方便迁移
- SFTP 文件管理：浏览远程目录、上传/下载文件、新建文件夹、重命名、删除
- 传输进度：实时显示文件上传下载进度

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Electron |
| 终端模拟 | xterm.js + addon-fit + addon-web-links |
| SSH/SFTP | ssh2 |
| 构建工具 | esbuild |
| UI 主题 | Catppuccin Mocha 暗色主题 |

## 文件说明

- `main.js` — Electron 主进程，处理 SSH/SFTP 连接、IPC 通信、连接存储
- `preload.js` — Context Bridge 预加载脚本，安全暴露 IPC 接口
- `build.js` — esbuild 构建脚本，打包渲染进程 JS
- `src/index.html` — 主界面 HTML
- `src/css/styles.css` — 暗色主题样式
- `src/renderer/app.js` — 渲染进程逻辑（终端、文件管理、连接管理 UI）

## 运行方式

```bash
npm install
npm start
```
