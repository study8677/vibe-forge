# Vibe Forge

Vibe Coding 项目合集 — 每个目录均由 **50 字以内的提示词** 一键生成。

## 项目列表

| 目录 | 项目名称 | 提示词 | 模型 | 技术栈 |
|------|---------|--------|------|--------|
| `wangzhen3691` | LINUX.DO Clone | 角色为全栈专家，用 React 复刻 LINUX.DO。加入 CLI 交互模式，支持命令行导航论坛。代码追求极致轻量与高内聚，视觉风格硬核。 | Claude Opus | React + Tailwind + xterm.js |
| `TechnologyStar` | 始皇顶级音乐播放器 | 见下方 | Claude Opus | 纯 HTML/CSS/JS |
| `Soul` | 中国体育赛事聚合系统 | 设计中国体育赛事聚合系统方案，聚焦中国队比赛，支持多源免费直播数据抓取与融合，提供订阅推送机制 | Claude Opus | Python + FastAPI + PostgreSQL + Redis |
| `wangzhen3691_2` | 始皇防蛐蛐 · ShiHuang Guard | 全栈专家角色，用 Go/Node.js 构建高并发后端。设计插件化架构与实时监控系统。加入"始皇防蛐蛐"模块：正则监听全站关键词，触发自动化预警。界面现代且易扩展。 | Claude Opus 4.6 | Go + React + WebSocket + Tailwind CSS |
| `Sora` | Sora 笔记 | 基于Next.js构建个人笔记全栈应用，要求支持markdown渲染，支持用任务看板管理笔记。风格要求与notion一致。 | Claude Opus 4.6 | Next.js 14 + React 18 + Tailwind CSS + Prisma + SQLite |
| `不为人知的鹅妈妈童谣` | GPU 视频 AI 流水线工具 | 构建一个视频字幕擦除，TTS生成合并到视频，语音自动对口型，ai换脸的工具，接入GPU | gpt-5.4 | Python + ffmpeg + TOML + unittest |
| `mjjs` | MJJS AI 代码助手 | 写一个vscode插件，能右键解释代码，能补全的，接入自定义大模型 | Claude Opus 4.6 | TypeScript + VS Code Extension API + OpenAI Compatible API |
| `再靠近一点就要融化` | Research Agent | 见下方 | gpt-5.4 | Python 3.14 + OpenAI Responses API + arXiv API + unittest + ruff |
| `wind1542` | Wind AI 编辑器 | obsidian插件，使用API修改文件，悬浮修改结果显示差异，支持额外一键拒绝/同意，自动时间戳备份可回滚 | Claude Opus 4.6 | TypeScript + Obsidian API + esbuild |
| `Hiccup90` | Hiccup90 NAS Dashboard | 见下方 | gpt-5.4 | HTML + CSS + JavaScript ES Modules + localStorage + Node test |
| `floorflour` | 游戏文本编辑器 | 游戏文本编辑器。文本的修正历史记录。项目化的存储读取。自定义格式，将文本映射或读取自具体程序文件 | Claude Opus 4.6 | Python + PySide6 + SQLite |
| `zed-lines-history` | Zed Lines History | 见下方 | gpt-5.4 | Rust workspace + Cargo + Git CLI |
| `yeqi-night-companion` | 夜气 | 做会读空气的深夜陪伴App：克制暧昧，识别情绪，主动陪伴，生成高保真交互原型。 | gpt-5.4 | Vanilla HTML/CSS/JavaScript + Node test |
| `crocodile_qu` | Crocodile OAuth Hub | 见下方 | gpt-5.4 | Next.js 16 + Auth.js + Prisma + MySQL + Playwright |

### TechnologyStar 提示词

> 帮我制作一个顶级音乐播放器：
> 1. 样式高级，可切换风格
> 2. 有始皇跳舞动态效果（支持切换拉丁舞，街舞，中国特色民族舞）
> 3. 多种功能支持
>
> 名称：始皇顶级音乐播放器（neo music helper）

### 再靠近一点就要融化 提示词

> 生成一个科研 Agent：输入研究目标，arxiv检索、论文排序阅读、想法生成、代码生成与执行（反馈循环至无bug）、结果评估并反馈，多次循环。

### Hiccup90 提示词

> 做一个适合NAS的导航页
> 1、功能性和自由度要媲美gethomepage/homepage
> 2、需要在前端页面直接修改设置，例如:sun-panel
> 3、导航页要美观、现代化，例如 sun-pannel或者dashdot
> 4、轻量化，高性能。

### zed-lines-history 提示词

> 做一个zed编辑器的插件
>
> 功能类似vscode里面的gitlens插件
>
> 主要有一个功能，就是 连续按行追踪代码能力
>
> 功能描述：有一个单独开启的lines history面板，我把鼠标聚焦在某一行代码，面板里展示当前和上一版的对比的记录，注意，在这个对比里我可以把鼠标就可以聚焦当前版，也可以继续聚焦在上一版的具体某一行，继续看上一版和上上一版的对比，以此类推，仍然可以聚焦上上版的代码，实现连续追踪

### crocodile_qu 提示词

> 做第三方登录集成，包括腾讯阿里字节github outlook微博微信qq等，有E2E测试，使用mysql

## 说明

- 每个目录以 **L 站佬的名字** 命名，代表该项目的贡献者
- 所有项目均为 AI 辅助 Vibe Coding 生成，提示词不超过 50 字
- 打开对应目录中的 `index.html` 或按目录内 README 说明运行即可体验

## 参与方式

1. 前往 [Linux.do 活动帖](https://linux.do/t/topic/1918242) 了解详情并参与讨论
2. Fork 本仓库
3. 用你的 ID 创建一个新目录
4. 用不超过 50 字的提示词生成你的项目
5. 提交 PR
