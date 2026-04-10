from __future__ import annotations

import asyncio
import signal
import sys

from apscheduler.schedulers.asyncio import AsyncIOScheduler
from rich.console import Console

from raccoon.config import AppConfig
from raccoon.runner import RunConfig, print_summary, run_tests

console = Console()


async def _job(config: AppConfig) -> None:
    console.print("\n[bold]═══ Scheduled test run ═══[/bold]")
    results = await run_tests(config)
    print_summary(results)


def start_scheduler(config: AppConfig, interval: int | None = None) -> None:
    """Start the periodic test scheduler."""
    minutes = interval or config.schedule_interval

    scheduler = AsyncIOScheduler()
    scheduler.add_job(
        _job,
        "interval",
        minutes=minutes,
        args=[config],
        id="raccoon_tests",
        name="LLM API Tests",
        max_instances=1,
    )

    console.print(f"[bold green]Scheduler started — running every {minutes} minutes[/bold green]")
    console.print("[dim]Press Ctrl+C to stop[/dim]\n")

    loop = asyncio.new_event_loop()

    def _shutdown(*_):
        console.print("\n[yellow]Shutting down scheduler...[/yellow]")
        scheduler.shutdown(wait=False)
        loop.stop()

    signal.signal(signal.SIGINT, _shutdown)
    signal.signal(signal.SIGTERM, _shutdown)

    scheduler.start()

    # Run first test immediately
    loop.run_until_complete(_job(config))

    try:
        loop.run_forever()
    finally:
        loop.close()
