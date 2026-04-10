from __future__ import annotations

from datetime import datetime, timedelta, timezone
from pathlib import Path

from fastapi import FastAPI, Query
from fastapi.responses import HTMLResponse

from raccoon.config import AppConfig
from raccoon.database import Database

_TEMPLATES = Path(__file__).parent / "templates"

_RANGE_MAP = {
    "1h": timedelta(hours=1),
    "6h": timedelta(hours=6),
    "24h": timedelta(hours=24),
    "7d": timedelta(days=7),
    "30d": timedelta(days=30),
}


def _since(time_range: str) -> str | None:
    delta = _RANGE_MAP.get(time_range)
    if delta:
        return (datetime.now(timezone.utc) - delta).isoformat()
    return None


def create_app(config: AppConfig) -> FastAPI:
    app = FastAPI(title="Raccoon Dashboard")
    db = Database(config.db_path)

    @app.get("/", response_class=HTMLResponse)
    async def index():
        html = (_TEMPLATES / "index.html").read_text()
        return HTMLResponse(html)

    @app.get("/api/summary")
    async def api_summary(time_range: str = Query("24h")):
        return db.summary(_since(time_range))

    @app.get("/api/results")
    async def api_results(
        provider: str | None = None,
        model: str | None = None,
        test_type: str | None = None,
        time_range: str = Query("24h"),
        limit: int = 200,
        offset: int = 0,
    ):
        return db.query(provider, model, test_type, _since(time_range), limit, offset)

    @app.get("/api/heatmap")
    async def api_heatmap(time_range: str = Query("24h")):
        return db.heatmap(_since(time_range))

    @app.get("/api/trends")
    async def api_trends(time_range: str = Query("24h")):
        bucket = "day" if time_range in ("7d", "30d") else "hour"
        return db.trends(_since(time_range), bucket)

    @app.get("/api/providers")
    async def api_providers(time_range: str = Query("24h")):
        return db.provider_stats(_since(time_range))

    @app.get("/api/failures")
    async def api_failures(time_range: str = Query("24h"), limit: int = 30):
        return db.recent_failures(limit, _since(time_range))

    @app.on_event("shutdown")
    async def shutdown():
        db.close()

    return app
