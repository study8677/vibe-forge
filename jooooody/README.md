# 电商比价分析工具

> **提示词:** 输入关键字和价格，自动在拼多多淘宝上搜索，浏览前100个商品及对应100条最新评价和所有追评，综合出最优的

## 项目简介

自动化电商比价工具，输入关键字和价格区间后，自动在淘宝和拼多多上搜索商品，采集前 100 个商品的最新评价与追评，通过多维度情感分析和综合评分，帮你找到性价比最高的商品。面向有选择困难症的精打细算型消费者。

## 核心功能

- 双平台自动搜索（淘宝 + 拼多多），支持价格区间过滤和销量排序
- 每个商品自动采集 100 条最新评价及所有追评，API 拦截 + DOM 解析双策略
- 中文情感分析（jieba 分词 + 情感词典），含否定词反转和严重差评识别
- 刷单/虚假评价检测，过滤水军干扰
- 7 维加权综合评分（价格、销量、好评率、评价质量、追评情感、真实度、差评严重度）
- Rich 终端表格 + 详细推荐卡片 + CSV/JSON 导出

## 技术栈

| 层级 | 技术 |
|------|------|
| 浏览器自动化 | Playwright（含反检测、Cookie 持久化） |
| 中文 NLP | jieba 分词 + 自定义情感词典 |
| 终端 UI | Rich（表格、进度条、面板） |
| 数据处理 | Pandas |
| 语言 | Python 3.10+ / asyncio |

## 文件说明

- `main.py` — CLI 入口，参数解析与流程编排
- `config.py` — 全局配置（采集参数、评分权重、情感词典）
- `models.py` — 数据模型（Product、Review、SearchResult）
- `browser.py` — Playwright 浏览器管理、Cookie 持久化、反检测 JS
- `analyzer.py` — 评价分析引擎（情感分析、刷评检测、综合评分排名）
- `reporter.py` — Rich 终端输出 + CSV/JSON 导出
- `platforms/base.py` — 爬虫抽象基类
- `platforms/taobao.py` — 淘宝/天猫爬虫（搜索 + 评价 + 追评）
- `platforms/pdd.py` — 拼多多爬虫（移动端 H5）

## 运行方式

```bash
pip install -r requirements.txt
playwright install chromium

# 搜索两个平台，价格 0~100
python main.py -k "无线鼠标" --max-price 100

# 指定平台和价格区间
python main.py -k "蓝牙耳机" --min-price 50 --max-price 300 --platform taobao

# 控制采集量
python main.py -k "洗面奶" --max-price 80 --products 50 --reviews 50
```

首次运行会弹出浏览器窗口，手动登录淘宝/拼多多后 Cookie 自动保存，后续运行无需重复登录。
