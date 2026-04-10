from __future__ import annotations

import json
import sqlite3
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

_SCHEMA = """
CREATE TABLE IF NOT EXISTS test_results (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id      TEXT    NOT NULL,
    created_at  TEXT    NOT NULL,
    provider    TEXT    NOT NULL,
    model       TEXT    NOT NULL,
    test_type   TEXT    NOT NULL,
    success     INTEGER NOT NULL DEFAULT 0,
    latency_first_token_ms REAL,
    latency_total_ms       REAL,
    tokens_input   INTEGER,
    tokens_output  INTEGER,
    throughput_tps REAL,
    error_type     TEXT,
    error_message  TEXT,
    score          REAL,
    metadata       TEXT
);

CREATE INDEX IF NOT EXISTS idx_results_created  ON test_results(created_at);
CREATE INDEX IF NOT EXISTS idx_results_provider ON test_results(provider, model);
CREATE INDEX IF NOT EXISTS idx_results_type     ON test_results(test_type);
CREATE INDEX IF NOT EXISTS idx_results_run      ON test_results(run_id);
"""


@dataclass
class TestResult:
    run_id: str
    provider: str
    model: str
    test_type: str
    success: bool = False
    latency_first_token_ms: float | None = None
    latency_total_ms: float | None = None
    tokens_input: int | None = None
    tokens_output: int | None = None
    throughput_tps: float | None = None
    error_type: str | None = None
    error_message: str | None = None
    score: float | None = None
    metadata: dict[str, Any] = field(default_factory=dict)
    created_at: str = ""
    id: int | None = None

    def __post_init__(self):
        if not self.created_at:
            self.created_at = datetime.now(timezone.utc).isoformat()


class Database:
    def __init__(self, db_path: str = "./raccoon.db"):
        Path(db_path).parent.mkdir(parents=True, exist_ok=True)
        self.conn = sqlite3.connect(db_path, check_same_thread=False)
        self.conn.row_factory = sqlite3.Row
        self.conn.executescript(_SCHEMA)

    def save_result(self, r: TestResult) -> int:
        cur = self.conn.execute(
            """INSERT INTO test_results
               (run_id, created_at, provider, model, test_type, success,
                latency_first_token_ms, latency_total_ms, tokens_input, tokens_output,
                throughput_tps, error_type, error_message, score, metadata)
               VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
            (r.run_id, r.created_at, r.provider, r.model, r.test_type,
             int(r.success), r.latency_first_token_ms, r.latency_total_ms,
             r.tokens_input, r.tokens_output, r.throughput_tps,
             r.error_type, r.error_message, r.score,
             json.dumps(r.metadata) if r.metadata else None),
        )
        self.conn.commit()
        return cur.lastrowid  # type: ignore

    def save_results(self, results: list[TestResult]) -> None:
        for r in results:
            self.save_result(r)

    def query(
        self,
        provider: str | None = None,
        model: str | None = None,
        test_type: str | None = None,
        since: str | None = None,
        limit: int = 500,
        offset: int = 0,
    ) -> list[dict[str, Any]]:
        clauses: list[str] = []
        params: list[Any] = []
        if provider:
            clauses.append("provider = ?")
            params.append(provider)
        if model:
            clauses.append("model = ?")
            params.append(model)
        if test_type:
            clauses.append("test_type = ?")
            params.append(test_type)
        if since:
            clauses.append("created_at >= ?")
            params.append(since)

        where = (" WHERE " + " AND ".join(clauses)) if clauses else ""
        sql = f"SELECT * FROM test_results{where} ORDER BY created_at DESC LIMIT ? OFFSET ?"
        params.extend([limit, offset])
        rows = self.conn.execute(sql, params).fetchall()
        return [dict(r) for r in rows]

    def summary(self, since: str | None = None) -> dict[str, Any]:
        where = ""
        params: list[Any] = []
        if since:
            where = " WHERE created_at >= ?"
            params = [since]

        row = self.conn.execute(
            f"""SELECT
                COUNT(*) as total,
                SUM(success) as passed,
                AVG(CASE WHEN success=1 THEN latency_total_ms END) as avg_latency,
                COUNT(DISTINCT provider) as providers,
                COUNT(DISTINCT provider || '/' || model) as models,
                MAX(created_at) as last_run
            FROM test_results{where}""",
            params,
        ).fetchone()
        total = row["total"] or 0
        passed = row["passed"] or 0
        return {
            "total": total,
            "passed": passed,
            "failed": total - passed,
            "success_rate": round(passed / total * 100, 1) if total else 0,
            "avg_latency_ms": round(row["avg_latency"] or 0, 1),
            "providers": row["providers"] or 0,
            "models": row["models"] or 0,
            "last_run": row["last_run"],
        }

    def heatmap(self, since: str | None = None) -> list[dict[str, Any]]:
        where = ""
        params: list[Any] = []
        if since:
            where = " WHERE created_at >= ?"
            params = [since]

        rows = self.conn.execute(
            f"""SELECT provider, model, test_type,
                COUNT(*) as total,
                SUM(success) as passed,
                AVG(CASE WHEN success=1 THEN latency_total_ms END) as avg_latency
            FROM test_results{where}
            GROUP BY provider, model, test_type
            ORDER BY provider, model, test_type""",
            params,
        ).fetchall()
        return [dict(r) for r in rows]

    def trends(self, since: str | None = None, bucket: str = "hour") -> list[dict[str, Any]]:
        where = ""
        params: list[Any] = []
        if since:
            where = " WHERE created_at >= ?"
            params = [since]

        # bucket by hour or day
        if bucket == "day":
            time_expr = "substr(created_at, 1, 10)"
        else:
            time_expr = "substr(created_at, 1, 13)"

        rows = self.conn.execute(
            f"""SELECT {time_expr} as bucket, provider,
                COUNT(*) as total,
                SUM(success) as passed,
                AVG(CASE WHEN success=1 THEN latency_total_ms END) as avg_latency
            FROM test_results{where}
            GROUP BY bucket, provider
            ORDER BY bucket""",
            params,
        ).fetchall()
        return [dict(r) for r in rows]

    def provider_stats(self, since: str | None = None) -> list[dict[str, Any]]:
        where = ""
        params: list[Any] = []
        if since:
            where = " WHERE created_at >= ?"
            params = [since]

        rows = self.conn.execute(
            f"""SELECT provider,
                COUNT(*) as total,
                SUM(success) as passed,
                AVG(CASE WHEN success=1 THEN latency_total_ms END) as avg_latency,
                AVG(CASE WHEN success=1 THEN latency_first_token_ms END) as avg_ttfb
            FROM test_results{where}
            GROUP BY provider
            ORDER BY provider""",
            params,
        ).fetchall()
        return [dict(r) for r in rows]

    def recent_failures(self, limit: int = 20, since: str | None = None) -> list[dict[str, Any]]:
        where = "WHERE success = 0"
        params: list[Any] = []
        if since:
            where += " AND created_at >= ?"
            params.append(since)

        params.append(limit)
        rows = self.conn.execute(
            f"SELECT * FROM test_results {where} ORDER BY created_at DESC LIMIT ?",
            params,
        ).fetchall()
        return [dict(r) for r in rows]

    def close(self):
        self.conn.close()
