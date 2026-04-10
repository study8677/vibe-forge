from __future__ import annotations

import asyncio
import uuid
from dataclasses import dataclass

from rich.console import Console
from rich.live import Live
from rich.table import Table

from raccoon.benchmarks import ALL_BENCHMARKS, Benchmark
from raccoon.config import AppConfig
from raccoon.database import Database, TestResult
from raccoon.providers import create_provider

console = Console()


@dataclass
class RunConfig:
    provider_filter: str | None = None
    model_filter: str | None = None
    test_filter: str | None = None
    concurrency: int = 5


async def run_tests(config: AppConfig, run_config: RunConfig | None = None) -> list[TestResult]:
    """Execute all configured tests and return results."""
    rc = run_config or RunConfig(concurrency=config.concurrency)
    run_id = uuid.uuid4().hex[:12]
    db = Database(config.db_path)

    # Build test matrix
    tasks: list[tuple] = []  # (provider_config, model, benchmark)
    for pconf in config.providers:
        if rc.provider_filter and pconf.name != rc.provider_filter:
            continue
        for model in pconf.models:
            if rc.model_filter and model != rc.model_filter:
                continue
            for bname, benchmark in ALL_BENCHMARKS.items():
                if rc.test_filter and bname != rc.test_filter:
                    continue
                tconf = config.tests.get(bname)
                if tconf and not tconf.enabled:
                    continue
                tasks.append((pconf, model, benchmark))

    if not tasks:
        console.print("[yellow]No tests to run. Check your config and filters.[/yellow]")
        return []

    total = len(tasks)
    console.print(f"[bold]Running {total} tests (run_id={run_id})...[/bold]\n")

    # Run with concurrency control
    semaphore = asyncio.Semaphore(rc.concurrency)
    results: list[TestResult] = []
    completed = 0

    async def _run_one(pconf, model, benchmark: Benchmark) -> TestResult:
        nonlocal completed
        timeout = config.tests.get(benchmark.name)
        tout = timeout.timeout if timeout else 30
        provider = create_provider(pconf, model)
        try:
            async with semaphore:
                result = await benchmark.run(provider, run_id, tout)
        finally:
            await provider.close()
        completed += 1
        # Print progress
        status = "[green]PASS[/green]" if result.success else "[red]FAIL[/red]"
        latency = f"{result.latency_total_ms:.0f}ms" if result.latency_total_ms else "-"
        console.print(
            f"  [{completed}/{total}] {pconf.name}/{model} | "
            f"{benchmark.name} | {status} | {latency}"
        )
        return result

    coros = [_run_one(p, m, b) for p, m, b in tasks]
    results = await asyncio.gather(*coros)

    # Save all results
    db.save_results(results)
    db.close()

    return results


def print_summary(results: list[TestResult]) -> None:
    """Print a summary table of test results."""
    if not results:
        return

    table = Table(title="Test Results Summary", show_lines=True)
    table.add_column("Provider", style="cyan")
    table.add_column("Model", style="blue")
    table.add_column("Test", style="magenta")
    table.add_column("Status", justify="center")
    table.add_column("Latency", justify="right", style="yellow")
    table.add_column("TTFB", justify="right", style="yellow")
    table.add_column("TPS", justify="right")
    table.add_column("Score", justify="right")

    for r in sorted(results, key=lambda x: (x.provider, x.model, x.test_type)):
        status = "[green]PASS[/green]" if r.success else "[red]FAIL[/red]"
        latency = f"{r.latency_total_ms:.0f}ms" if r.latency_total_ms else "-"
        ttfb = f"{r.latency_first_token_ms:.0f}ms" if r.latency_first_token_ms else "-"
        tps = f"{r.throughput_tps:.1f}" if r.throughput_tps else "-"
        score = f"{r.score:.2f}" if r.score is not None else "-"
        table.add_row(r.provider, r.model, r.test_type, status, latency, ttfb, tps, score)

    console.print()
    console.print(table)

    passed = sum(1 for r in results if r.success)
    total = len(results)
    avg_latency = sum(r.latency_total_ms or 0 for r in results if r.success) / max(passed, 1)
    rate = passed / total * 100 if total else 0

    console.print(
        f"\n[bold]Summary: {passed}/{total} passed ({rate:.1f}%) | "
        f"Avg latency: {avg_latency:.0f}ms[/bold]\n"
    )

    # Print failures
    failures = [r for r in results if not r.success]
    if failures:
        console.print("[bold red]Failures:[/bold red]")
        for r in failures:
            console.print(
                f"  [red]✗[/red] {r.provider}/{r.model} | {r.test_type} | "
                f"{r.error_type}: {r.error_message[:120] if r.error_message else 'N/A'}"
            )
        console.print()
