# Soul - 中国体育赛事聚合系统设计方案

## 1. 系统定位

聚焦**中国队比赛**的体育赛事聚合平台，自动抓取多个免费数据源的赛程、直播链接，融合去重后提供统一视图，并支持用户按运动/球队订阅推送。

---

## 2. 覆盖范围

### 运动项目

| 优先级 | 项目 | 说明 |
|--------|------|------|
| P0 | 足球 | 国足、中超、亚冠、世预赛 |
| P0 | 篮球 | CBA、国家队、篮球世界杯 |
| P0 | 排球 | 中国女排/男排、世联赛 |
| P1 | 乒乓球 | WTT巡回赛、世乒赛 |
| P1 | 羽毛球 | BWF巡回赛、汤尤杯 |
| P2 | 游泳/跳水 | 世锦赛、世界杯 |
| P2 | 田径 | 钻石联赛、世锦赛 |
| P3 | 综合赛事 | 奥运会、亚运会、大运会 |

### 关键筛选逻辑

所有数据以 `china_involved = true` 为核心过滤条件，优先展示中国队参与的赛事。

---

## 3. 数据源分析

### Tier 1 — 结构化/API 源（高可靠）

| 数据源 | 类型 | 地址 | 说明 |
|--------|------|------|------|
| **央视EPG API** | JSON API | `api.cntv.cn/epg/getEpgInfoByChannelNew` | CCTV5/5+ 节目表，JSONP 格式，参数 `c=cctv5&d=YYYYMMDD&t=jsonp`。最核心数据源，覆盖大部分中国队赛事转播 |
| **FIFA 赛历** | 结构化 HTML | `fifa.com/tournaments` | 世预赛、世界杯赛程 |
| **FIBA 赛历** | 结构化 HTML | `fiba.basketball/events` | 篮球世界杯、亚洲杯预选赛 |
| **BWF 赛历** | 结构化 HTML | `bwf.tournamentsoftware.com` | 羽毛球巡回赛，表格结构清晰 |
| **ITTF/WTT** | 结构化 HTML | `worldtabletennis.com` | 乒乓球 WTT 系列赛事 |
| **TheSportsDB** | 免费 API | `thesportsdb.com/api/v1/json` | 辅助元数据（队徽、场馆），覆盖有限 |

### Tier 2 — DOM 抓取源（中可靠）

| 数据源 | 类型 | 说明 |
|--------|------|------|
| **直播吧** (`zhibo8.cc`) | SSR HTML | 全运动聚合赛历，标注免费直播，服务端渲染可直接 HTTP 抓取 |
| **懂球帝** (`dongqiudi.com`) | Mobile API | 足球深度覆盖，API `api.dongqiudi.com` 需 cookie 认证 |
| **CBA 官网** (`cbaleague.com`) | HTML | CBA 赛程与数据 |
| **中国足协** (`thecfa.cn`) | HTML | 国家队、足协杯、中超赛程 |

### Tier 3 — JS 渲染源（高成本）

| 数据源 | 类型 | 说明 |
|--------|------|------|
| **央视频** (`yangshipin.cn`) | JS Render | CCTV 直播流平台，需 Playwright 渲染 |
| **咪咕视频** (`miguvideo.com`) | JS Render | 中国移动体育流平台，持有 CBA/WTT/英超等版权 |
| **新浪体育** (`sports.sina.com.cn`) | Mixed | TV 节目表 `match.sports.sina.com.cn/tvguide/program/tv/1` |

### Tier 4 — 补充源

| 数据源 | 说明 |
|--------|------|
| **TVMao** (`tvmao.com`) | 电视节目表聚合，SSR 易抓取 |
| **epg.pw** | 免费 XMLTV 格式 EPG 数据 |
| **iptv-org/epg** (GitHub) | 开源 EPG 工具，已有 CCTV 频道解析器 |

---

## 4. 系统架构

```
                    ┌─────────────────────────────────────────────┐
                    │              用户触达层                       │
                    │  ┌─────────┐ ┌────────┐ ┌───────┐ ┌─────┐ │
                    │  │ Web前端  │ │Telegram│ │ 微信   │ │Email│ │
                    │  │ (React) │ │  Bot   │ │ 公众号 │ │     │ │
                    │  └────┬────┘ └───┬────┘ └───┬───┘ └──┬──┘ │
                    └───────┼──────────┼──────────┼────────┼────┘
                            │          │          │        │
                    ┌───────▼──────────▼──────────▼────────▼────┐
                    │              API 网关层 (FastAPI)           │
                    │  /events  /subscriptions  /streams  /ws   │
                    └───────────────────┬───────────────────────┘
                                        │
              ┌─────────────────────────┼─────────────────────────┐
              │                         │                         │
    ┌─────────▼─────────┐   ┌──────────▼──────────┐   ┌─────────▼─────────┐
    │    数据融合引擎      │   │   推送调度引擎        │   │   实时得分引擎      │
    │  (Fusion Engine)   │   │ (Notification Sched) │   │  (Live Tracker)   │
    │                    │   │                      │   │                   │
    │  归一化 → 匹配 →    │   │  APScheduler 定时    │   │  比赛进行中         │
    │  决策 → 充实 →      │   │  T-24h / T-1h / T-0 │   │  30-60s 轮询      │
    │  冲突解决 → 变更检测 │   │  多通道分发          │   │  得分变更 → 推送    │
    └─────────┬─────────┘   └──────────────────────┘   └───────────────────┘
              │
    ┌─────────▼──────────────────────────────────────────────┐
    │                   抓取调度层 (Scraper Orchestrator)       │
    │  APScheduler CronTrigger (定期) + DateTrigger (赛前)    │
    │  ┌──────────────────────────────────────────────────┐  │
    │  │              Scraper Plugin Registry              │  │
    │  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐   │  │
    │  │  │央视EPG  │ │直播吧   │ │FIFA    │ │BWF     │   │  │
    │  │  │Scraper │ │Scraper │ │Scraper │ │Scraper │   │  │
    │  │  └────────┘ └────────┘ └────────┘ └────────┘   │  │
    │  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐   │  │
    │  │  │CBA     │ │懂球帝   │ │ITTF    │ │咪咕     │   │  │
    │  │  │Scraper │ │Scraper │ │Scraper │ │Scraper │   │  │
    │  │  └────────┘ └────────┘ └────────┘ └────────┘   │  │
    │  └──────────────────────────────────────────────────┘  │
    └────────────────────────┬───────────────────────────────┘
                             │
    ┌────────────────────────▼───────────────────────────────┐
    │                    数据持久层                            │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
    │  │  PostgreSQL   │  │    Redis      │  │ Playwright   │ │
    │  │  事件/订阅/   │  │  缓存/限流/   │  │  Browser     │ │
    │  │  通知记录      │  │  消息队列     │  │  Pool        │ │
    │  └──────────────┘  └──────────────┘  └──────────────┘ │
    └───────────────────────────────────────────────────────┘
```

---

## 5. 数据模型

### 5.1 核心实体关系

```
sports 1──N teams 1──N team_aliases
  │                │
  │                │
events N──1 competitions
  │
  ├── home_team_id FK→ teams
  ├── away_team_id FK→ teams
  │
  ├──N source_events (原始抓取记录)
  │     └── source_id FK→ sources
  │
  ├──N streams (直播流)
  │     └── source_id FK→ sources
  │
  └──N notifications (推送记录)
        └── subscription_id FK→ subscriptions
              └── user_id FK→ users
```

### 5.2 表结构

#### sports — 运动项目

```sql
CREATE TABLE sports (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name_zh     TEXT NOT NULL,          -- "足球"
    name_en     TEXT NOT NULL,          -- "Football"
    code        TEXT NOT NULL UNIQUE,   -- "football" (slug)
    icon_url    TEXT,
    created_at  TIMESTAMPTZ DEFAULT now()
);
```

#### teams — 球队/代表队

```sql
CREATE TABLE teams (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name_zh       TEXT NOT NULL,          -- "中国男子足球国家队" (规范名)
    name_en       TEXT,                   -- "China PR Men's National Football Team"
    short_name_zh TEXT NOT NULL,          -- "中国男足"
    short_name_en TEXT,                   -- "China"
    fifa_code     TEXT,                   -- "CHN"
    sport_id      UUID NOT NULL REFERENCES sports(id),
    is_national   BOOLEAN DEFAULT false,  -- 国家队标记
    country_code  TEXT,                   -- "CN" ISO 3166-1
    logo_url      TEXT,
    metadata      JSONB DEFAULT '{}',     -- 联合会 ID 等扩展字段
    created_at    TIMESTAMPTZ DEFAULT now()
);
```

#### team_aliases — 队名别名映射

```sql
CREATE TABLE team_aliases (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id     UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    alias       TEXT NOT NULL,             -- "国足"
    source      TEXT DEFAULT 'manual',     -- "manual" | "auto_detected"
    confidence  REAL DEFAULT 1.0,          -- 0-1
    created_at  TIMESTAMPTZ DEFAULT now()
);
CREATE UNIQUE INDEX idx_alias_lower ON team_aliases (lower(alias));
```

#### competitions — 赛事

```sql
CREATE TABLE competitions (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name_zh       TEXT NOT NULL,          -- "2026年FIFA世界杯预选赛亚洲区"
    name_en       TEXT,
    short_name_zh TEXT,                   -- "世预赛"
    sport_id      UUID NOT NULL REFERENCES sports(id),
    level         TEXT,                   -- "international" | "continental" | "domestic"
    season        TEXT,                   -- "2025-2026"
    metadata      JSONB DEFAULT '{}',
    created_at    TIMESTAMPTZ DEFAULT now()
);
```

#### events — 比赛事件（融合后）

```sql
CREATE TABLE events (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sport_id        UUID NOT NULL REFERENCES sports(id),
    competition_id  UUID REFERENCES competitions(id),
    home_team_id    UUID REFERENCES teams(id),
    away_team_id    UUID REFERENCES teams(id),
    event_title     TEXT NOT NULL,                   -- "中国 vs 日本"
    event_date      DATE NOT NULL,
    start_time      TIMESTAMPTZ NOT NULL,
    end_time        TIMESTAMPTZ,
    status          TEXT DEFAULT 'scheduled',        -- scheduled|live|finished|postponed|cancelled
    venue           TEXT,
    score_home      INTEGER,
    score_away      INTEGER,
    importance      TEXT DEFAULT 'medium',           -- high|medium|low
    china_involved  BOOLEAN NOT NULL DEFAULT false,  -- 核心过滤字段
    metadata        JSONB DEFAULT '{}',              -- round, group, stage 等
    merged_from     UUID[],                          -- 关联的 source_event ID
    confidence      REAL DEFAULT 1.0,                -- 融合置信度
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_events_china_date ON events (china_involved, event_date);
CREATE INDEX idx_events_sport_date ON events (sport_id, event_date);
CREATE INDEX idx_events_status ON events (status) WHERE status = 'live';
```

#### sources — 数据源注册

```sql
CREATE TABLE sources (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL UNIQUE,   -- "cntv_epg"
    display_name    TEXT NOT NULL,          -- "央视EPG"
    base_url        TEXT,
    source_type     TEXT NOT NULL,          -- "api" | "html_scrape" | "js_render"
    reliability     REAL DEFAULT 0.5,       -- 0-1, 冲突解决权重
    scrape_config   JSONB DEFAULT '{}',     -- 间隔/选择器/认证配置
    is_active       BOOLEAN DEFAULT true,
    last_success_at TIMESTAMPTZ,
    last_error      TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);
```

#### source_events — 原始抓取记录（融合前）

```sql
CREATE TABLE source_events (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id       UUID NOT NULL REFERENCES sources(id),
    event_id        UUID REFERENCES events(id),       -- 融合后回填
    raw_title       TEXT NOT NULL,                     -- "2026世界杯预选赛 中国-日本"
    raw_data        JSONB NOT NULL,                    -- 原始响应完整保留
    parsed_sport    TEXT,                              -- "football"
    parsed_home     TEXT,                              -- "中国"（原始队名）
    parsed_away     TEXT,                              -- "日本"
    parsed_time     TIMESTAMPTZ,
    parsed_comp     TEXT,                              -- "世预赛"
    match_status    TEXT DEFAULT 'unmatched',          -- unmatched|matched|conflict|manual
    scraped_at      TIMESTAMPTZ DEFAULT now(),
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(source_id, raw_title, parsed_time)          -- 源内去重
);
CREATE INDEX idx_se_match ON source_events (source_id, match_status);
```

#### streams — 直播流

```sql
CREATE TABLE streams (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id    UUID NOT NULL REFERENCES events(id),
    source_id   UUID NOT NULL REFERENCES sources(id),
    platform    TEXT NOT NULL,            -- "cctv5" | "migu" | "yangshipin"
    stream_url  TEXT,                     -- 可能在赛前才获取到
    is_free     BOOLEAN DEFAULT true,
    quality     TEXT,                     -- "4k" | "1080p" | "720p"
    status      TEXT DEFAULT 'upcoming',  -- upcoming|active|ended|unavailable
    verified_at TIMESTAMPTZ,
    metadata    JSONB DEFAULT '{}',
    created_at  TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_streams_event ON streams (event_id, is_free);
```

#### users — 用户

```sql
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name    TEXT,
    email           TEXT UNIQUE,
    wechat_openid   TEXT UNIQUE,
    telegram_id     BIGINT UNIQUE,
    webhook_url     TEXT,
    timezone        TEXT DEFAULT 'Asia/Shanghai',
    is_active       BOOLEAN DEFAULT true,
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);
```

#### subscriptions — 订阅

```sql
CREATE TABLE subscriptions (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       UUID NOT NULL REFERENCES users(id),
    filter_type   TEXT NOT NULL,          -- "sport" | "team" | "competition" | "all_china"
    filter_value  UUID,                   -- sport_id / team_id / competition_id; NULL for all_china
    channels      TEXT[] NOT NULL,        -- {"wechat", "telegram", "webhook", "email"}
    notify_24h    BOOLEAN DEFAULT true,   -- 赛前24小时提醒
    notify_1h     BOOLEAN DEFAULT true,   -- 赛前1小时提醒
    notify_start  BOOLEAN DEFAULT true,   -- 开赛提醒（含直播链接）
    notify_score  BOOLEAN DEFAULT false,  -- 进球/得分提醒
    notify_result BOOLEAN DEFAULT true,   -- 赛后结果
    is_active     BOOLEAN DEFAULT true,
    created_at    TIMESTAMPTZ DEFAULT now(),
    updated_at    TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_sub_user ON subscriptions (user_id, is_active);
```

#### notifications — 推送记录

```sql
CREATE TABLE notifications (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id),
    event_id        UUID NOT NULL REFERENCES events(id),
    channel         TEXT NOT NULL,          -- "wechat" | "telegram" | "webhook" | "email"
    trigger_type    TEXT NOT NULL,          -- "24h" | "1h" | "start" | "score" | "result"
    payload         JSONB NOT NULL,
    status          TEXT DEFAULT 'pending', -- pending|sent|failed|retrying
    sent_at         TIMESTAMPTZ,
    error           TEXT,
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(user_id, event_id, channel, trigger_type)  -- 防重复推送
);
```

---

## 6. 抓取插件系统

### 6.1 目录结构

```
src/scrapers/
├── __init__.py          # 插件注册表 SCRAPER_REGISTRY
├── base.py              # SourceScraper Protocol + BaseSourceScraper
├── cntv_epg.py          # 央视 EPG API [Tier 1, 最优先实现]
├── zhibo8.py            # 直播吧 HTML 抓取
├── dongqiudi.py         # 懂球帝 API
├── fifa_calendar.py     # FIFA 官网
├── fiba_calendar.py     # FIBA 官网
├── bwf_calendar.py      # BWF 赛事
├── ittf_calendar.py     # ITTF/WTT 赛事
├── migu_sports.py       # 咪咕视频 [Playwright]
├── cba_official.py      # CBA 官网
├── cfa_official.py      # 中国足协
└── tvmao.py             # TVMao 电视节目表
```

### 6.2 Protocol 定义

```python
from dataclasses import dataclass
from datetime import datetime
from typing import Protocol

@dataclass
class DateRange:
    start: datetime
    end: datetime

@dataclass
class RawEvent:
    """各抓取器统一输出格式"""
    raw_title: str               # 原文标题
    parsed_sport_hint: str       # "football"
    parsed_home_name: str        # "中国"（原始队名）
    parsed_away_name: str        # "日本"
    parsed_datetime: datetime    # 比赛时间
    parsed_competition_hint: str # "世预赛"
    source_url: str              # 数据来源 URL
    raw_data: dict               # 完整原始数据

@dataclass
class RawStream:
    """直播流信息"""
    event_hint: str              # 用于关联赛事
    platform: str                # "cctv5" | "migu"
    stream_url: str | None
    is_free: bool
    quality: str | None

@dataclass
class HealthStatus:
    is_healthy: bool
    latency_ms: float
    last_error: str | None

class SourceScraper(Protocol):
    """每个数据源必须实现此协议"""

    def source_name(self) -> str:
        """返回源标识符，对应 sources.name"""
        ...

    async def scrape_schedule(self, date_range: DateRange) -> list[RawEvent]:
        """抓取日期范围内的赛事"""
        ...

    async def scrape_streams(self, event_hints: list[str]) -> list[RawStream]:
        """根据已知赛事查找直播链接"""
        ...

    async def health_check(self) -> HealthStatus:
        """检查数据源可达性"""
        ...
```

### 6.3 BaseSourceScraper 提供的公共能力

| 能力 | 实现 |
|------|------|
| HTTP 客户端管理 | 共享 `httpx.AsyncClient` 连接池 |
| 域名级限流 | Redis 令牌桶（每域名独立） |
| 重试 | 指数退避，429/503 特殊处理 |
| 原始响应缓存 | 写入 `source_events.raw_data` |
| 结构化日志 | `structlog` 注入 source 上下文 |
| 错误上报 | Sentry breadcrumb |

### 6.4 抓取频率策略

| 数据类型 | 频率 | 触发方式 |
|----------|------|----------|
| 未来赛程（T+7d） | 每 6 小时 | CronTrigger |
| CCTV5 EPG（T+7d） | 每 2 小时 | CronTrigger |
| 直播链接可用性 | 赛前 T-2h 开始，每 15 分钟 | DateTrigger |
| 比赛中实时比分 | 每 30-60 秒 | 赛事状态变为 live 时启动 |
| 赛后结果 | T+2h | DateTrigger |

---

## 7. 数据融合引擎

### 7.1 六阶段流水线

```
source_events (unmatched)
       │
       ▼
┌─────────────────────────┐
│ Stage 1: 归一化           │  - 解析运动类型
│ (Normalization)         │  - 通过别名表解析队名 → canonical_id
│                         │  - 时区统一至 UTC
│                         │  - 提取赛事名称
└───────────┬─────────────┘
            ▼
┌─────────────────────────┐
│ Stage 2: 候选匹配         │  - 对每个未匹配 source_event，
│ (Candidate Matching)    │    查询: 同运动 + 日期±24h +
│                         │    (队伍交集 OR 赛事+轮次)
│                         │  - 计算匹配分数 0-100
└───────────┬─────────────┘
            ▼
┌─────────────────────────┐
│ Stage 3: 决策             │  - ≥90分: 自动匹配
│ (Decision)              │  - 60-89分: 自动匹配 + 标记审核
│                         │  - <60分: 创建新事件
└───────────┬─────────────┘
            ▼
┌─────────────────────────┐
│ Stage 4: 数据充实         │  - 合并多源数据:
│ (Enrichment)            │    时间 → 取最高可靠度源
│                         │    队名 → 用规范名
│                         │    赛事 → 取最详细源
│                         │    直播流 → 聚合所有源
└───────────┬─────────────┘
            ▼
┌─────────────────────────┐
│ Stage 5: 冲突解决         │  - 多源不一致时:
│ (Conflict Resolution)   │    可靠度加权投票
│                         │    时差<30min → 取高可靠源
│                         │    时差>30min → 标记人工审核
└───────────┬─────────────┘
            ▼
┌─────────────────────────┐
│ Stage 6: 变更检测         │  - Diff 新旧状态
│ (Change Detection)      │  - 发出事件:
│                         │    new_event / time_changed /
│                         │    status_changed / stream_added /
│                         │    score_updated
│                         │  → 触发推送系统
└─────────────────────────┘
```

### 7.2 匹配评分算法

```python
def calculate_match_score(source_event, candidate_event) -> int:
    score = 0

    # 运动类型匹配 (必要条件)
    if sport_matches:           score += 30

    # 日期匹配
    if date_exact_match:        score += 20
    elif date_within_24h:       score += 10

    # 队伍匹配 (通过别名表解析后对比)
    if home_team_canonical_match:  score += 20
    if away_team_canonical_match:  score += 20

    # 赛事名称匹配
    if competition_match:       score += 10

    # 惩罚项
    if has_tbd_team:            score -= 10
    if time_diff_gt_2h:         score -= 15

    return score
```

### 7.3 队名归一化

同一实体在不同源中的命名差异巨大：

| 上下文 | 足球 | 篮球 | 羽毛球 |
|--------|------|------|--------|
| 中文全称 | 中国男子足球国家队 | 中国男子篮球国家队 | 中国国家羽毛球队 |
| 中文简称 | 中国男足 / 国足 | 中国男篮 | 国羽 |
| 英文正式 | China PR | China | China |
| FIFA Code | CHN | CHN | CHN |
| 转播中 | 中国队 / 中国 | 中国 | 中国 |

**解决方案**：`team_aliases` 表 + `rapidfuzz` 模糊匹配

匹配管道：
1. 精确匹配别名表
2. 归一化匹配（去空格、统一大小写、繁转简）
3. 模糊匹配 `token_set_ratio ≥ 85`
4. 低于阈值 → 标记人工审核

---

## 8. 订阅推送系统

### 8.1 推送时机

| 触发器 | 时机 | 推送内容示例 |
|--------|------|------------|
| `24h` | T-24h | "明天 20:00 世预赛 中国 vs 日本 将在 CCTV5 直播" |
| `1h` | T-1h | "1小时后开赛: 世预赛 中国 vs 日本，CCTV5/咪咕视频可看" |
| `start` | T-0 | "比赛开始: 中国 vs 日本 世预赛 [直播链接]" |
| `score` | 比赛中 | "进球！中国 1-0 日本（第23分钟）" |
| `result` | T+15min | "比赛结束: 中国 2-1 日本 世预赛" |

### 8.2 调度机制

```
新赛事创建 / 赛事时间变更
       │
       ▼
APScheduler 创建 DateTrigger 任务:
  ├── notify_24h_{event_id}:  run_date = start_time - 24h
  ├── notify_1h_{event_id}:   run_date = start_time - 1h
  └── notify_start_{event_id}: run_date = start_time

赛事状态变为 live:
  └── 启动 Celery 周期任务: 每 30-60s 轮询比分
       └── 比分变化 → 立即推送 score 通知
       └── 比赛结束 → 推送 result 通知
```

### 8.3 推送分发流程

```
触发器触发
    │
    ▼
加载赛事 + 最新直播流信息
    │
    ▼
查询匹配的订阅
  (按 sport / team / competition / all_china 匹配)
    │
    ▼
对每个 (user, subscription, channel):
    │
    ▼
查询 notifications 表去重
  (防止同一触发器重复推送)
    │
    ▼
按通道格式化消息:
  ├── 微信: 模板消息 (结构化字段)
  ├── Telegram: MarkdownV2 + 内联按钮
  ├── Webhook: JSON payload + HMAC 签名
  └── Email: HTML 模板 (Jinja2)
    │
    ▼
异步分发 → 记录推送结果
    │
    ▼
失败: 最多重试 3 次 (指数退避)
```

### 8.4 推送通道对比

| 通道 | 优势 | 限制 | 接入复杂度 |
|------|------|------|-----------|
| **Telegram Bot** | 无速率限制、富文本、内联按钮直达直播 | 国内访问受限 | 低 |
| **企业微信 Webhook** | 群通知简单直接 | 仅支持群消息 | 低 |
| **微信服务号** | 覆盖广、模板消息 | 需审核模板、日限 10 万条 | 高 |
| **Webhook** | 通用，可对接 Slack/Discord/飞书/钉钉 | 需用户提供 URL | 中 |
| **Email** | 适合日报/周报摘要 | 实时性差 | 低 |

---

## 9. API 设计

### Base URL: `/api/v1`

### 赛事相关

```
GET  /events
     ?sport=football,basketball     -- 运动类型（逗号分隔）
     ?china_only=true               -- 仅中国队 (默认 true)
     ?date_from=2026-04-08
     ?date_to=2026-04-15
     ?status=scheduled,live
     ?page=1&per_page=20
     → 分页赛事列表，内嵌 streams

GET  /events/:id
     → 完整赛事详情 + 所有直播流 + 数据源标注

GET  /events/live
     → 当前正在进行的赛事 (status=live, china_involved=true)
     支持 WebSocket 升级获取实时比分

GET  /events/upcoming?hours=48
     → 未来 N 小时内的赛事

GET  /streams/:event_id
     → 该赛事可用的直播流列表，按清晰度排序
```

### 元数据

```
GET  /sports
     → 运动项目列表 (含赛事数量统计)

GET  /teams?sport=football&national_only=true
     → 球队列表

GET  /teams/:id/events?date_from=...&date_to=...
     → 指定球队的赛事
```

### 订阅

```
POST   /subscriptions
       Body: { filter_type, filter_value, channels, notify_* }

GET    /subscriptions
       → 当前用户的订阅列表

PUT    /subscriptions/:id
       Body: 部分更新

DELETE /subscriptions/:id
```

### WebSocket

```
WS /ws/live/:event_id
   → 推送: 比分更新、状态变更、关键事件
```

### 响应格式

```json
{
  "data": { "..." },
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 142,
    "generated_at": "2026-04-08T12:00:00+08:00"
  },
  "sources": ["cntv_epg", "zhibo8", "fifa_calendar"]
}
```

---

## 10. 技术栈总览

| 层级 | 技术 | 理由 |
|------|------|------|
| HTTP 抓取 | `httpx` (async) | HTTP/2、连接池、异步 |
| JS 渲染 | `Playwright` (async) | Chromium 内核，stealth 插件 |
| HTML 解析 | `selectolax` / `lxml` | selectolax 比 BeautifulSoup 快 20x |
| 文本匹配 | `rapidfuzz` | C 扩展模糊匹配 |
| 定时调度 | `APScheduler` | 支持运行时动态创建 DateTrigger |
| 任务队列 | `Celery` + Redis | 扇出任务、重试语义 |
| 主数据库 | PostgreSQL 16 | JSONB、中文全文检索 (zhparser) |
| 缓存/Broker | Redis 7 | APScheduler 存储、限流状态、流 URL 缓存 |
| API 框架 | FastAPI | 异步、自动 OpenAPI、Pydantic 模型 |
| 微信推送 | `wechatpy` + 企微 Webhook | 服务号模板消息 + 企微群机器人 |
| Telegram | `python-telegram-bot` | Bot API |
| 监控 | Sentry + `structlog` | 错误追踪 + 结构化日志 |
| 部署 | Docker Compose | PostgreSQL, Redis, API, Scheduler, Workers |

---

## 11. 反爬策略

| 数据源 | 防护级别 | 策略 |
|--------|---------|------|
| 央视 API | 无 | 直接 httpx，遵守速率限制 |
| FIFA/FIBA/BWF | 低 | httpx + UA 轮换，2-5s 延迟 |
| 直播吧 | 低-中 | httpx + session cookies |
| 懂球帝 API | 中 | 维护认证 session，轮换 token |
| 咪咕视频 | 高 | Playwright + stealth 插件 |
| 央视频 | 高 | Playwright，或拦截 XHR API |

**通用原则**：
- 同域名最低 2s 请求间隔
- 70%+ 数据源用 `httpx` 即可
- 仅 2-3 个 JS 重度源需要 Playwright
- 429/503 指数退避
- 积极缓存 — 赛程数据通常每天只变一次
- 保留原始响应便于调试和重处理

---

## 12. 部署拓扑

```
Docker Compose

┌─────────────────────────────────────────────────────┐
│                                                     │
│  ┌─────────┐  ┌─────────┐  ┌──────────────────┐   │
│  │ postgres │  │  redis  │  │    api (FastAPI)   │   │
│  │  :5432   │  │  :6379  │  │  :8000 (uvicorn)  │   │
│  └─────────┘  └─────────┘  └──────────────────┘   │
│                                                     │
│  ┌──────────────────┐  ┌──────────────────────┐    │
│  │ scheduler         │  │ worker (Celery)       │    │
│  │ (APScheduler)     │  │ concurrency=4         │    │
│  └──────────────────┘  └──────────────────────┘    │
│                                                     │
│  ┌──────────────────┐  ┌──────────────────────┐    │
│  │ playwright-pool   │  │ frontend (React+Vite) │    │
│  │ (浏览器池)         │  │ nginx :80             │    │
│  └──────────────────┘  └──────────────────────┘    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## 13. 实施路线图

### Phase 1（第 1-2 周）: 基础骨架

- 数据库 schema + Alembic 迁移
- FastAPI 骨架 + 健康检查端点
- **央视 EPG 抓取器**（首个实现，最高价值单源）
- 基础赛事创建（单源，无融合）
- Docker Compose 环境搭建

### Phase 2（第 3-4 周）: 多源 + 融合

- 新增 3-4 个抓取器（直播吧、FIFA、CBA、BWF）
- 队名别名表 + 种子数据
- 融合引擎六阶段实现
- APScheduler 定期抓取集成

### Phase 3（第 5-6 周）: 订阅推送

- 订阅模型 + API
- Telegram Bot（最易入手通道）
- 企微 Webhook（群通知）
- 推送调度（T-24h、T-1h、T-0）

### Phase 4（第 7-8 周）: 前端 + 完善

- React 前端（Vite）
- 赛事日历视图
- 实时赛事仪表盘
- 订阅管理 UI
- 微信服务号集成

### Phase 5（持续迭代）: 扩展

- 更多抓取器
- 实时比分追踪
- 历史数据与统计
- ML 辅助别名自动识别
- 用户反馈驱动数据质量提升

---

## 14. 关键设计决策总结

| 决策点 | 选择 | 理由 |
|--------|------|------|
| 调度器 | APScheduler 而非纯 Celery Beat | 需要运行时动态创建 DateTrigger（赛前 N 小时触发），Celery Beat 静态配置做不到 |
| 抓取器隔离 | 每源独立模块 + Protocol | 解析逻辑最易因网站改版而崩溃，隔离后限制爆炸半径 |
| 融合策略 | 先写 source_events 再融合 | 保留原始数据便于重放、调试、审核 |
| 队名匹配 | 别名表 + rapidfuzz | 精确匹配覆盖 90% case，模糊匹配兜底 |
| 冲突解决 | 可靠度加权 | 各源可靠度不同（央视 EPG > 直播吧 > 第三方），量化后自动决策 |
| 首个推送通道 | Telegram | 零审核、无速率限制、富文本、内联按钮，最快验证推送链路 |
