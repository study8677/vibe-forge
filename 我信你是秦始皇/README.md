# 航班价格监控工具

> **提示词:** 输入一个出发地点（如北京首都机场）和一个达到地点（如上海虹桥机场）和一个明确的日期时段（如2026.5.1日 下午1:00-5:00），自动在机票购票引擎中检测飞机票的价格变动，在降价超过一定价格时用邮寄方式推送结果。

## 项目简介

一个 Python CLI 工具，通过 Amadeus 航班搜索 API 持续监控指定航线、日期、时段的机票价格，自动检测降价并通过邮件发送通知。支持中文机场名称输入、灵活的时段过滤和可配置的降价阈值。

## 核心功能

- 支持 44 个国内外机场的中文名称 / IATA 代码自动识别
- 按出发时段（如 13:00-17:00）过滤航班
- 本地 JSON 文件记录价格历史，对比检测降价
- 降价超阈值时自动发送 HTML 格式邮件通知（支持 QQ/163/Gmail）
- 单次查询和持续监控两种运行模式

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | Python 3 |
| 航班数据 | Amadeus Self-Service API (OAuth2) |
| 邮件通知 | smtplib (SMTP SSL/TLS) |
| 配置管理 | python-dotenv (.env) |

## 文件说明

- `main.py` — CLI 入口，参数解析与监控调度循环
- `airports.py` — 中文机场名 <-> IATA 代码映射与模糊匹配
- `flight_api.py` — Amadeus API 客户端（OAuth2 认证 + 航班搜索）
- `tracker.py` — 价格历史记录与降价检测
- `notifier.py` — SMTP 邮件通知（HTML + 纯文本双格式）
- `.env.example` — 环境变量配置模板
- `requirements.txt` — Python 依赖

## 运行方式

```bash
# 1. 安装依赖
pip install -r requirements.txt

# 2. 配置 API 和邮箱
cp .env.example .env
# 编辑 .env 填入 Amadeus API Key 和 SMTP 邮箱配置

# 3. 单次查询
python main.py --from 北京首都机场 --to 上海虹桥机场 --date 2026.5.1 --time 13:00-17:00

# 4. 持续监控（每15分钟检查，降价超100元通知）
python main.py --from 北京首都机场 --to 上海虹桥机场 \
  --date 2026.5.1 --time 13:00-17:00 \
  --threshold 100 --monitor --interval 15

# 5. 查看支持的机场
python main.py --list-airports
```
