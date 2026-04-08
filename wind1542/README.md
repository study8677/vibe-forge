# Wind AI 编辑器

> **提示词:** obsidian插件，使用API修改文件，悬浮修改结果显示差异，支持额外一键拒绝/同意，自动时间戳备份可回滚

## 项目简介

一款 Obsidian 插件，接入 AI 大模型 API 对当前笔记进行智能修改。修改结果以悬浮 Diff 弹窗展示，支持行级 + 字符级差异高亮，用户可一键接受或拒绝。每次接受前自动创建时间戳备份，随时可回滚到任意历史版本。

## 核心功能

- **AI 全文/选区修改** — 输入自然语言指令，AI 返回修改后的内容
- **悬浮 Diff 弹窗** — GitHub 风格统一差异视图，行号 + 颜色编码 + 字符级 inline 高亮
- **一键接受/拒绝** — 按钮 + 快捷键（Enter 接受 / Esc 拒绝）
- **自动时间戳备份** — 接受修改前自动备份原文件，存储在 `.wind-backups/` 目录
- **回滚管理器** — 浏览所有备份 → 预览 Diff → 一键恢复或删除，自动裁剪旧备份

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | TypeScript |
| 平台 | Obsidian Plugin API |
| 构建 | esbuild |
| Diff 算法 | Myers diff (行级) + LCS (字符级) |
| API 支持 | OpenAI Compatible / Anthropic Claude |

## 文件说明

- `src/main.ts` — 插件入口，注册命令与 Ribbon 图标，串联各模块
- `src/api-service.ts` — API 通信层，支持 OpenAI / Anthropic 两种格式
- `src/diff-engine.ts` — Myers diff 算法实现，含字符级差异计算
- `src/diff-modal.ts` — 悬浮 Diff 弹窗 UI，Accept / Reject 按钮
- `src/backup-manager.ts` — 时间戳备份管理，自动裁剪
- `src/rollback-modal.ts` — 备份浏览与回滚界面
- `src/instruction-modal.ts` — 修改指令输入弹窗
- `src/settings.ts` — 插件设置面板
- `src/types.ts` — 共享类型定义与默认配置
- `styles.css` — 全部 UI 样式
- `manifest.json` — Obsidian 插件清单

## 运行方式

1. 将 `main.js`、`manifest.json`、`styles.css` 复制到 Obsidian Vault 的 `.obsidian/plugins/wind1542-ai-editor/` 目录
2. 在 Obsidian 设置 → 第三方插件中启用 **Wind AI Editor**
3. 进入插件设置，配置 API Key 和模型
4. 打开任意笔记，点击左侧 Ribbon 魔杖图标或使用命令面板触发 `AI Modify Current File`
