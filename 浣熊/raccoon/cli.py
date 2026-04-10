from __future__ import annotations

import asyncio
import shutil
from pathlib import Path
from typing import Optional

import typer
from rich.console import Console
from rich.panel import Panel

from raccoon import __version__

app = typer.Typer(
    name="raccoon",
    help="浣熊 — LLM API Testing Platform / 大模型API测试平台",
    no_args_is_help=True,
)
console = Console()

_CONFIG_OPTION = typer.Option(None, "--config", "-c", help="Path to config.yaml")
_TIME_OPTION = typer.Option(None, "--time", "-t", help="Time range: 1h, 6h, 24h, 7d, 30d")


def _banner():
    console.print(Panel.fit(
        "[bold cyan]浣熊 Raccoon[/bold cyan] — LLM API Test Platform\n"
        f"[dim]v{__version__}[/dim]",
        border_style="cyan",
    ))


@app.command()
def run(
    provider: Optional[str] = typer.Option(None, "--provider", "-p", help="Test only this provider"),
    model: Optional[str] = typer.Option(None, "--model", "-m", help="Test only this model"),
    test: Optional[str] = typer.Option(None, "--test", "-T", help="Run only this test type"),
    concurrency: int = typer.Option(0, "--concurrency", "-n", help="Max parallel calls (0=use config)"),
    config: Optional[str] = _CONFIG_OPTION,
):
    """Run tests now."""
    from raccoon.config import load_config
    from raccoon.runner import RunConfig, print_summary, run_tests

    _banner()
    cfg = load_config(config)
    if not cfg.providers:
        console.print("[red]No providers configured. Run 'raccoon config --init' first.[/red]")
        raise typer.Exit(1)

    rc = RunConfig(
        provider_filter=provider,
        model_filter=model,
        test_filter=test,
        concurrency=concurrency or cfg.concurrency,
    )
    results = asyncio.run(run_tests(cfg, rc))
    print_summary(results)


@app.command()
def schedule(
    interval: Optional[int] = typer.Option(None, "--interval", "-i", help="Minutes between runs"),
    config: Optional[str] = _CONFIG_OPTION,
):
    """Start periodic test scheduler."""
    from raccoon.config import load_config
    from raccoon.scheduler import start_scheduler

    _banner()
    cfg = load_config(config)
    if not cfg.providers:
        console.print("[red]No providers configured.[/red]")
        raise typer.Exit(1)

    start_scheduler(cfg, interval)


@app.command()
def report(
    time_range: Optional[str] = _TIME_OPTION,
    format: str = typer.Option("table", "--format", "-f", help="Output format: table, json"),
    config: Optional[str] = _CONFIG_OPTION,
):
    """Show test results report."""
    from raccoon.config import load_config
    from raccoon.database import Database
    from raccoon.reporter import (
        report_failures,
        report_heatmap,
        report_providers,
        report_results_json,
        report_summary,
    )

    cfg = load_config(config)
    db = Database(cfg.db_path)

    if format == "json":
        console.print(report_results_json(db, time_range))
    else:
        _banner()
        report_summary(db, time_range)
        console.print()
        report_heatmap(db, time_range)
        console.print()
        report_providers(db, time_range)
        console.print()
        report_failures(db, time_range=time_range)

    db.close()


@app.command()
def dashboard(
    port: int = typer.Option(8077, "--port", "-p", help="Server port"),
    host: str = typer.Option("0.0.0.0", "--host", "-H", help="Server host"),
    config: Optional[str] = _CONFIG_OPTION,
):
    """Start the web dashboard."""
    import uvicorn

    from raccoon.config import load_config
    from raccoon.dashboard.app import create_app

    _banner()
    cfg = load_config(config)
    app = create_app(cfg)

    console.print(f"[bold green]Dashboard → http://localhost:{port}[/bold green]\n")
    uvicorn.run(app, host=host, port=port, log_level="warning")


@app.command()
def providers(config: Optional[str] = _CONFIG_OPTION):
    """List configured providers and models."""
    from rich.table import Table

    from raccoon.config import load_config

    cfg = load_config(config)
    if not cfg.providers:
        console.print("[yellow]No providers configured.[/yellow]")
        return

    table = Table(title="Configured Providers", show_lines=True)
    table.add_column("Provider", style="cyan")
    table.add_column("Type", style="dim")
    table.add_column("Base URL")
    table.add_column("Models", style="green")
    table.add_column("API Key", style="dim")

    for p in cfg.providers:
        key_status = "[green]set[/green]" if p.api_key else "[red]missing[/red]"
        table.add_row(
            p.name,
            p.type,
            p.base_url or "(default)",
            ", ".join(p.models),
            key_status,
        )
    console.print(table)


@app.command("config")
def config_cmd(
    init: bool = typer.Option(False, "--init", help="Create config.yaml from template"),
):
    """Show or initialize configuration."""
    from raccoon.config import load_config

    if init:
        src = Path(__file__).parent.parent / "config.example.yaml"
        dst = Path.cwd() / "config.yaml"
        if dst.exists():
            console.print("[yellow]config.yaml already exists. Skipping.[/yellow]")
        elif src.exists():
            shutil.copy(src, dst)
            console.print(f"[green]Created {dst}[/green]")
            console.print("[dim]Edit config.yaml and set your API keys.[/dim]")
        else:
            console.print("[red]Example config not found. Create config.yaml manually.[/red]")
        return

    cfg = load_config()
    console.print(f"[bold]Database:[/bold] {cfg.db_path}")
    console.print(f"[bold]Schedule:[/bold] every {cfg.schedule_interval} min")
    console.print(f"[bold]Concurrency:[/bold] {cfg.concurrency}")
    console.print(f"[bold]Providers:[/bold] {len(cfg.providers)}")
    for p in cfg.providers:
        console.print(f"  - {p.name}: {', '.join(p.models)}")
    tests_on = [k for k, v in cfg.tests.items() if v.enabled]
    console.print(f"[bold]Tests:[/bold] {', '.join(tests_on)}")


@app.command()
def version():
    """Show version."""
    console.print(f"raccoon v{__version__}")


if __name__ == "__main__":
    app()
