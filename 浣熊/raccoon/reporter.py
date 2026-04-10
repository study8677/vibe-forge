from __future__ import annotations

import json
from datetime import datetime, timedelta, timezone

from rich.console import Console
from rich.panel import Panel
from rich.table import Table

from raccoon.database import Database

console = Console()

_RANGE_MAP = {
    "1h": timedelta(hours=1),
    "6h": timedelta(hours=6),
    "24h": timedelta(hours=24),
    "7d": timedelta(days=7),
    "30d": timedelta(days=30),
}


def _since(time_range: str | None) -> str | None:
    if not time_range:
        return None
    delta = _RANGE_MAP.get(time_range)
    if delta:
        return (datetime.now(timezone.utc) - delta).isoformat()
    return None


def report_summary(db: Database, time_range: str | None = None) -> None:
    since = _since(time_range)
    s = db.summary(since)

    grid = Table.grid(padding=(0, 3))
    grid.add_column(justify="right", style="bold")
    grid.add_column()
    grid.add_row("Total Tests", str(s["total"]))
    grid.add_row("Passed", f"[green]{s['passed']}[/green]")
    grid.add_row("Failed", f"[red]{s['failed']}[/red]")
    grid.add_row("Success Rate", f"{s['success_rate']}%")
    grid.add_row("Avg Latency", f"{s['avg_latency_ms']:.0f}ms")
    grid.add_row("Providers", str(s["providers"]))
    grid.add_row("Models", str(s["models"]))
    grid.add_row("Last Run", s["last_run"] or "never")

    tr = f" ({time_range})" if time_range else ""
    console.print(Panel(grid, title=f"Overview{tr}", border_style="blue"))


def report_heatmap(db: Database, time_range: str | None = None) -> None:
    since = _since(time_range)
    data = db.heatmap(since)
    if not data:
        console.print("[yellow]No data yet.[/yellow]")
        return

    # Collect test types and provider/models
    test_types = sorted({r["test_type"] for r in data})
    pm_keys = sorted({(r["provider"], r["model"]) for r in data})

    lookup = {}
    for r in data:
        key = (r["provider"], r["model"], r["test_type"])
        total, passed = r["total"], r["passed"]
        lookup[key] = (round(passed / total * 100, 1) if total else 0, total)

    table = Table(title="Success Rate Heatmap", show_lines=True)
    table.add_column("Provider / Model", style="cyan")
    for tt in test_types:
        table.add_column(tt, justify="center")

    for prov, model in pm_keys:
        cells = []
        for tt in test_types:
            rate, count = lookup.get((prov, model, tt), (0, 0))
            if count == 0:
                cells.append("[dim]-[/dim]")
            elif rate >= 90:
                cells.append(f"[green]{rate}%[/green]")
            elif rate >= 70:
                cells.append(f"[yellow]{rate}%[/yellow]")
            else:
                cells.append(f"[red]{rate}%[/red]")
        table.add_row(f"{prov}/{model}", *cells)

    console.print(table)


def report_providers(db: Database, time_range: str | None = None) -> None:
    since = _since(time_range)
    stats = db.provider_stats(since)
    if not stats:
        console.print("[yellow]No data yet.[/yellow]")
        return

    table = Table(title="Provider Comparison", show_lines=True)
    table.add_column("Provider", style="cyan")
    table.add_column("Tests", justify="right")
    table.add_column("Passed", justify="right", style="green")
    table.add_column("Rate", justify="right")
    table.add_column("Avg Latency", justify="right", style="yellow")
    table.add_column("Avg TTFB", justify="right", style="yellow")

    for s in stats:
        total = s["total"]
        passed = s["passed"] or 0
        rate = round(passed / total * 100, 1) if total else 0
        rate_style = "green" if rate >= 90 else ("yellow" if rate >= 70 else "red")
        latency = f"{s['avg_latency']:.0f}ms" if s["avg_latency"] else "-"
        ttfb = f"{s['avg_ttfb']:.0f}ms" if s["avg_ttfb"] else "-"
        table.add_row(
            s["provider"], str(total), str(passed),
            f"[{rate_style}]{rate}%[/{rate_style}]",
            latency, ttfb,
        )

    console.print(table)


def report_failures(db: Database, limit: int = 20, time_range: str | None = None) -> None:
    since = _since(time_range)
    failures = db.recent_failures(limit, since)
    if not failures:
        console.print("[green]No recent failures![/green]")
        return

    table = Table(title="Recent Failures", show_lines=True)
    table.add_column("Time", style="dim")
    table.add_column("Provider", style="cyan")
    table.add_column("Model", style="blue")
    table.add_column("Test", style="magenta")
    table.add_column("Error Type", style="red")
    table.add_column("Message")

    for f in failures:
        t = f["created_at"][:19].replace("T", " ")
        msg = (f["error_message"] or "")[:80]
        table.add_row(t, f["provider"], f["model"], f["test_type"], f["error_type"] or "-", msg)

    console.print(table)


def report_results_json(db: Database, time_range: str | None = None, limit: int = 100) -> str:
    since = _since(time_range)
    rows = db.query(since=since, limit=limit)
    return json.dumps(rows, indent=2, ensure_ascii=False)
