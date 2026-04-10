# 浣熊 LLM API 测试平台

> **提示词:** 开发个大模型API测试平台，定期测不同厂商、模型的调用成功率、agent能力等性能指标，有面板多维度分类展示，可cli

## 项目简介

浣熊 (Raccoon) 是一个大模型 API 测试平台，可定期对不同厂商和模型进行多维度性能基准测试，并通过 CLI 报表和 Web 面板直观展示结果。面向需要横向对比 LLM 服务质量的开发者和团队。

## 核心功能

- **5 种测试用例** — 基础调用、流式输出、工具调用 (Function Calling)、多步 Agent 链、JSON 结构化输出
- **7 家厂商开箱即用** — OpenAI / Anthropic / DeepSeek / 通义千问 / Moonshot / 智谱 / Google Gemini，任何 OpenAI 兼容 API 只需加 `base_url` 即可接入
- **定时调度** — APScheduler 自动周期执行，持续监控 API 可用性
- **CLI 报表** — Rich 表格展示成功率热力图、厂商对比、失败明细，支持 JSON 导出
- **Web 面板** — 深色主题仪表盘，Chart.js 可视化趋势图、延迟对比、成功率矩阵，60 秒自动刷新

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | Python 3.10+ |
| CLI | Typer + Rich |
| Web | FastAPI + Jinja2 + Chart.js |
| 数据库 | SQLite |
| 调度 | APScheduler |
| LLM SDK | openai + anthropic |

## 文件说明

- `pyproject.toml` — 项目配置与依赖
- `config.example.yaml` — 配置模板（厂商、模型、调度间隔）
- `raccoon/cli.py` — Typer CLI 入口
- `raccoon/providers/` — LLM 厂商适配层（OpenAI 兼容 + Anthropic）
- `raccoon/benchmarks/` — 5 种测试用例实现
- `raccoon/runner.py` — 异步并发测试引擎
- `raccoon/scheduler.py` — 定时调度器
- `raccoon/database.py` — SQLite 存储与多维查询
- `raccoon/reporter.py` — CLI 报表格式化
- `raccoon/dashboard/` — FastAPI Web 面板 + 前端模板

## 运行方式

```bash
# 安装
pip install -e .

# 初始化配置
raccoon config --init
# 编辑 config.yaml 填入 API Key

# 运行测试
raccoon run

# 查看报表
raccoon report -t 24h

# 启动 Web 面板
raccoon dashboard

# 定时调度
raccoon schedule -i 30
```
