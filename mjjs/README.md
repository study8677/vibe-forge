# MJJS AI Assistant — VS Code 插件

接入自定义大模型的 VS Code AI 代码助手，支持右键解释代码和智能代码补全。

## 功能

### 右键菜单（选中代码后右键）
- **解释代码** — 简要解释选中代码的功能
- **详细解释代码** — 逐行分析，包含算法、设计模式说明
- **优化代码** — 分析性能和可读性，给出优化建议
- **添加注释** — 自动为代码添加中文注释
- **查找潜在问题** — 检查 Bug、安全隐患、边界条件

### 代码补全
- 输入代码时自动触发 AI 行内补全
- 按 `Tab` 接受补全建议
- 快捷键 `Cmd+Shift+Space` (Mac) / `Ctrl+Shift+Space` 手动触发

### 快捷键
| 功能 | Mac | Windows/Linux |
|------|-----|---------------|
| 解释代码 | `Cmd+Shift+E` | `Ctrl+Shift+E` |
| 触发补全 | `Cmd+Shift+Space` | `Ctrl+Shift+Space` |

## 配置

打开 VS Code 设置，搜索 `mjjs`：

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| `mjjs.apiEndpoint` | API 地址（兼容 OpenAI 格式） | `https://api.openai.com/v1/chat/completions` |
| `mjjs.apiKey` | API 密钥 | — |
| `mjjs.modelName` | 模型名称 | `gpt-3.5-turbo` |
| `mjjs.maxTokens` | 最大生成 token 数 | `2048` |
| `mjjs.temperature` | 生成温度 (0-1) | `0.3` |
| `mjjs.completionEnabled` | 启用代码补全 | `true` |
| `mjjs.completionDebounceMs` | 补全触发延迟(ms) | `500` |
| `mjjs.completionMaxLines` | 补全上下文行数 | `50` |
| `mjjs.systemPrompt` | 系统提示词 | 专业编程助手 |
| `mjjs.completionSystemPrompt` | 补全提示词 | 代码补全引擎 |

## 支持的大模型

兼容 OpenAI Chat Completions API 格式的所有模型：

- **OpenAI**: GPT-4, GPT-3.5-Turbo 等
- **DeepSeek**: `https://api.deepseek.com/v1/chat/completions`
- **通义千问**: `https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions`
- **智谱 GLM**: `https://open.bigmodel.cn/api/paas/v4/chat/completions`
- **Ollama (本地)**: `http://localhost:11434/v1/chat/completions`
- 其他兼容 OpenAI 格式的 API

## 安装

### 从 VSIX 安装
```bash
# 编译打包
npm install
npm run compile
npx @vscode/vsce package

# 安装
code --install-extension mjjs-ai-assistant-0.1.0.vsix
```

### 开发调试
```bash
npm install
npm run watch
# 按 F5 启动调试
```

## 项目结构

```
mjjs/
├── src/
│   ├── extension.ts          # 插件入口
│   ├── llmClient.ts          # LLM API 客户端（支持流式/非流式）
│   ├── explainCommands.ts    # 右键菜单命令
│   ├── completionProvider.ts # 代码补全 Provider
│   └── resultPanel.ts        # 结果展示 Webview 面板
├── package.json              # 插件清单
└── tsconfig.json             # TypeScript 配置
```
