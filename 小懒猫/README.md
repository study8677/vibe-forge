# 万事录 · 私人管家

一个零依赖的静态 Web 原型，聚焦四件事：

- 聊天入口
- 分层分类持久化记忆库
- 本地整理与索引
- 面向流行聊天程序的连接器抽象

## 已实现

- 管家式聊天界面，支持本地规则驱动回复
- 速记层 / 事实层 / 主题层 / 档案层 四层记忆模型
- localStorage 持久化
- 中文双字切片 + 英文单词的轻量搜索索引
- 主题空间总览
- Telegram / 飞书 / Discord / Slack / 微信 / WhatsApp 连接器展示与模拟同步

## 运行

```bash
npm test
npm run start
```

然后打开 `http://localhost:4173`。

## 目录

- `index.html`：页面入口
- `styles.css`：自然浅色主题与布局
- `src/main.js`：启动应用
- `src/app.js`：UI 渲染与交互
- `src/store.js`：状态管理与持久化
- `src/engine.js`：聊天与记忆整理逻辑
- `src/indexing.js`：索引与搜索
- `src/connectors.js`：连接器模拟
- `src/data/defaults.js`：种子数据与产品字典
- `tests/`：Node 内置测试
