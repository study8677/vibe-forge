# Skill & Agent 评估平台

> **提示词:** 使用Harbor + promptfoo技术栈实现一个skill、agent的评估平台，skill、agent均在本地，有现成的harness框架和skill

## 项目简介

基于 Harbor + promptfoo 技术栈构建的 Claude Code Skill 与多智能体评估平台。通过自定义 Provider 将 promptfoo 桥接到 Anthropic Claude API，自动加载本地 Skill 定义并注入系统提示词，对 6 个现有技能和 2 种 Agent 角色进行系统化的触发准确率、协议合规性、安全性和输出质量评估。

## 核心功能

- 自动发现并加载本地 `~/.claude/skills/` 中的全部 Skill，解析 SKILL.md 前置元数据
- 模拟 Claude Code 的 `<system-reminder>` 技能加载机制，构建真实评估上下文
- 59 条触发准确率测试（中英文双语 + 反例），覆盖 deploy / ops-report / ssh-prod / log-patrol / integrator-patrol / self-improving 共 6 个技能
- Integrator / Owner 双角色 Agent 评估，验证 how-to-work-together 协议中 7 条不可违反规则的遵循情况
- 10 条安全测试（force-push / 硬编码凭证 / 破坏性删除等），自定义 assertion 函数做正则 + 语义双重检查
- Issue Packet JSON Schema 校验断言，自动验证 Agent 生成的跨服务 Issue 包格式
- Docker Compose 一键启动 Ollama（本地 Judge 模型）+ promptfoo Web UI

## 技术栈

| 层级 | 技术 |
|------|------|
| 评估引擎 | promptfoo (YAML 配置 + 自定义 Provider / Assertion) |
| 基础设施 | Harbor / Docker Compose (Ollama + promptfoo UI) |
| 自定义代码 | TypeScript (ESM, strict mode) |
| LLM 调用 | @anthropic-ai/sdk (Claude API) |
| Schema 校验 | Ajv + ajv-formats |
| 前置元数据 | gray-matter |

## 文件说明

- `src/providers/claude-skill.provider.ts` — 技能评估 Provider，支持 trigger / protocol / safety 三种模式
- `src/providers/claude-agent.provider.ts` — Agent 角色评估 Provider，注入协议 + 角色模板
- `src/providers/shared/skill-loader.ts` — 自动发现并解析本地 SKILL.md 文件
- `src/providers/shared/context-builder.ts` — 构建模拟 Claude Code 的系统提示词
- `src/assertions/trigger-accuracy.ts` — 触发准确率断言（正例 + 反例）
- `src/assertions/safety.ts` — 安全模式扫描（10 种危险操作模式）
- `src/assertions/issue-packet.ts` — Issue Packet JSON Schema 校验
- `src/assertions/protocol-compliance.ts` — 7 条不可违反规则合规检查
- `src/assertions/scope-discipline.ts` — Owner 范围约束 / Integrator 路由检查
- `evals/skills/*.yaml` — 技能评估配置（trigger / protocol / safety）
- `evals/agents/*.yaml` — Agent 评估配置（integrator / owner / mode）
- `evals/datasets/*.json` — 测试数据集（~97 条用例）
- `schemas/issue-packet.schema.json` — 跨服务 Issue 包 JSON Schema

## 运行方式

```bash
# 1. 安装依赖
npm install

# 2. 配置 API Key
cp .env.example .env   # 编辑 .env 设置 ANTHROPIC_API_KEY

# 3. 运行评估
make eval-all           # 全量评估（~97 条用例）
make eval-trigger       # 仅触发准确率
make eval-safety        # 仅安全测试
make eval-agents        # 仅 Agent 测试

# 4. 查看结果
make dashboard          # 打开 promptfoo Web UI

# 5. Docker 模式（可选）
make harbor-up          # 启动 Ollama + promptfoo UI
```
