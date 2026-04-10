# Travelport统一网关

> **提示词:** 统一网关支持Travelport SOAP/XML与JSON多账号凭证按操作路由协议入参/出参标准化

## 项目简介

面向航旅系统集成商的 Travelport API 统一网关，将 UAPI（SOAP/XML）和 Travelport+（JSON/REST）两套协议封装在同一套标准化接口之后。支持多账号凭证管理与按操作类型自动路由，调用方无需关心底层协议差异。

## 核心功能

- **双协议适配** — 同时支持 SOAP/XML（UAPI）和 JSON/REST（Travelport+），可按账号或请求级别切换
- **多账号凭证管理** — YAML 配置多组 PCC / TargetBranch / OAuth2 凭证，含自动 Token 刷新
- **按操作路由** — 基于操作类型 + 条件（国家、航司等）的规则引擎，首条匹配命中
- **入参/出参标准化** — Pydantic 模型统一抽象，屏蔽 XML namespace / JSON schema 差异
- **5 类 Air 操作** — air_search / air_price / air_book / air_ticket / pnr_retrieve 完整实现

## 技术栈

| 层级 | 技术 |
|------|------|
| Web 框架 | FastAPI |
| 数据模型 | Pydantic v2 |
| HTTP 客户端 | httpx (async) |
| XML 处理 | lxml / xml.etree |
| 配置 | PyYAML + 环境变量展开 |
| 测试 | pytest + pytest-asyncio |

## 文件说明

- `config/gateway.yaml` — 多账号凭证 + 路由规则配置
- `src/gateway/models.py` — 标准化请求/响应 Pydantic 模型
- `src/gateway/config.py` — 配置加载与环境变量展开
- `src/gateway/credentials.py` — 多账号凭证管理器 + OAuth2 Token 缓存
- `src/gateway/router.py` — 按操作+条件的路由引擎
- `src/gateway/gateway.py` — 核心编排器（路由→构建→发送→解析）
- `src/gateway/app.py` — FastAPI HTTP 入口
- `src/gateway/protocols/soap.py` — SOAP/XML 适配器（信封构建、Basic Auth、Fault 提取）
- `src/gateway/protocols/rest.py` — JSON/REST 适配器（OAuth2 Bearer、JSON 请求）
- `src/gateway/operations/air.py` — 5 个 Air 操作的 SOAP↔REST 双向转换器

## 运行方式

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install fastapi uvicorn httpx pydantic pyyaml lxml
PYTHONPATH=src uvicorn gateway.app:app --reload
```

访问 `http://localhost:8000/docs` 查看交互式 API 文档。
