"""报告输出 — 终端表格 + CSV/JSON 导出"""

from __future__ import annotations

import csv
import json
from dataclasses import asdict
from datetime import datetime
from pathlib import Path

from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.text import Text

from models import Product, SearchResult
import config as cfg

console = Console()


class Reporter:
    """格式化输出搜索结果和分析报告"""

    # ── 终端输出 ──────────────────────────────────────

    @staticmethod
    def print_header(keyword: str, min_price: float, max_price: float):
        console.print()
        console.print(Panel.fit(
            f"[bold]🔍 搜索关键字: [cyan]{keyword}[/cyan]  "
            f"💰 价格区间: [green]¥{min_price:.0f} ~ ¥{max_price:.0f}[/green][/bold]",
            title="[bold]电商比价分析工具[/bold]",
            border_style="bright_blue",
        ))
        console.print()

    @staticmethod
    def print_search_progress(platform: str, count: int):
        icon = "🟠" if platform == "taobao" else "🔴"
        name = "淘宝" if platform == "taobao" else "拼多多"
        console.print(f"  {icon} {name}: 采集到 [bold]{count}[/bold] 个商品")

    @staticmethod
    def print_review_progress(current: int, total: int, product_title: str):
        short_title = product_title[:30] + "..." if len(product_title) > 30 else product_title
        console.print(
            f"  📝 [{current}/{total}] 采集评价: [dim]{short_title}[/dim]",
            highlight=False,
        )

    @staticmethod
    def print_ranking(products: list[Product], top_n: int = 20):
        """打印排名表格"""
        console.print()
        console.print(
            Panel.fit(
                "[bold]综合排名 TOP {}[/bold]".format(min(top_n, len(products))),
                border_style="bright_green",
            )
        )

        table = Table(show_header=True, header_style="bold cyan", show_lines=True)
        table.add_column("#", style="bold", width=4, justify="center")
        table.add_column("平台", width=6, justify="center")
        table.add_column("商品名称", width=36, no_wrap=False)
        table.add_column("价格", width=8, justify="right", style="green")
        table.add_column("销量", width=10, justify="right")
        table.add_column("评价数", width=7, justify="right")
        table.add_column("好评率", width=7, justify="center")
        table.add_column("追评", width=5, justify="center")
        table.add_column("综合分", width=8, justify="center", style="bold yellow")
        table.add_column("摘要", width=40, no_wrap=False)

        for p in products[:top_n]:
            platform_label = (
                "[orange1]淘宝[/orange1]"
                if p.platform == "taobao"
                else "[red]拼多多[/red]"
            )

            # 好评率颜色
            pr = p.positive_review_ratio
            if pr >= 0.8:
                pr_style = f"[green]{pr:.0%}[/green]"
            elif pr >= 0.5:
                pr_style = f"[yellow]{pr:.0%}[/yellow]"
            else:
                pr_style = f"[red]{pr:.0%}[/red]"
            if not p.reviews:
                pr_style = "[dim]-[/dim]"

            # 综合分颜色
            score = p.composite_score
            if score >= 0.7:
                score_style = f"[bold green]{score:.2f}[/bold green]"
            elif score >= 0.4:
                score_style = f"[yellow]{score:.2f}[/yellow]"
            else:
                score_style = f"[red]{score:.2f}[/red]"

            sales_display = _format_sales(p.sales_count)

            table.add_row(
                str(p.rank),
                platform_label,
                p.title[:60],
                f"¥{p.price:.1f}",
                sales_display,
                str(len(p.reviews)) if p.reviews else str(p.review_count or "-"),
                pr_style,
                str(p.follow_up_count) if p.reviews else "-",
                score_style,
                p.summary[:60] if p.summary else "-",
            )

        console.print(table)

    @staticmethod
    def print_top_recommendations(products: list[Product], top_n: int = 5):
        """详细打印 Top N 推荐"""
        console.print()
        console.print(
            Panel.fit(
                f"[bold]🏆 最优推荐 TOP {min(top_n, len(products))}[/bold]",
                border_style="bright_yellow",
            )
        )

        for p in products[:top_n]:
            platform = "淘宝" if p.platform == "taobao" else "拼多多"
            bd = p.score_breakdown

            # 评分雷达
            radar = (
                f"  价格竞争力 {'█' * int(bd.get('price_value', 0) * 10):<10} {bd.get('price_value', 0):.0%}\n"
                f"  销量热度   {'█' * int(bd.get('sales', 0) * 10):<10} {bd.get('sales', 0):.0%}\n"
                f"  好评率     {'█' * int(bd.get('positive_ratio', 0) * 10):<10} {bd.get('positive_ratio', 0):.0%}\n"
                f"  评价质量   {'█' * int(bd.get('review_depth', 0) * 10):<10} {bd.get('review_depth', 0):.0%}\n"
                f"  追评反馈   {'█' * int(bd.get('follow_up_sentiment', 0) * 10):<10} {bd.get('follow_up_sentiment', 0):.0%}\n"
                f"  评价真实度 {'█' * int(bd.get('authenticity', 0) * 10):<10} {bd.get('authenticity', 0):.0%}\n"
                f"  安全无害   {'█' * int(bd.get('negative_severity', 0) * 10):<10} {bd.get('negative_severity', 0):.0%}"
            )

            # 精选好评 & 差评
            good_reviews = [
                r for r in p.reviews
                if r.is_positive and len(r.content) >= 20
            ][:3]
            bad_reviews = [
                r for r in p.reviews
                if r.is_positive is False and len(r.content) >= 10
            ][:2]
            follow_ups = [
                r for r in p.reviews if r.follow_up and len(r.follow_up) >= 10
            ][:2]

            review_text = ""
            if good_reviews:
                review_text += "[green]精选好评:[/green]\n"
                for r in good_reviews:
                    review_text += f'  ✓ "{r.content[:80]}"\n'
            if bad_reviews:
                review_text += "[red]精选差评:[/red]\n"
                for r in bad_reviews:
                    review_text += f'  ✗ "{r.content[:80]}"\n'
            if follow_ups:
                review_text += "[yellow]精选追评:[/yellow]\n"
                for r in follow_ups:
                    review_text += f'  → [{r.follow_up_days}天后] "{r.follow_up[:80]}"\n'

            content = (
                f"[bold]#{p.rank} [{platform}] {p.title[:50]}[/bold]\n"
                f"价格: [green]¥{p.price:.1f}[/green]  "
                f"销量: {_format_sales(p.sales_count)}  "
                f"店铺: {p.shop_name or '未知'}\n"
                f"综合评分: [bold yellow]{p.composite_score:.3f}[/bold yellow]  "
                f"评价数: {len(p.reviews)}  "
                f"追评数: {p.follow_up_count}\n\n"
                f"[dim]{radar}[/dim]\n\n"
                f"{p.summary}\n\n"
                f"{review_text}\n"
                f"[dim]链接: {p.url}[/dim]"
            )

            console.print(Panel(
                content,
                border_style="yellow" if p.rank <= 3 else "dim",
                width=100,
            ))

    @staticmethod
    def print_overall_insights(
        products: list[Product],
        top_concerns: list[str],
        top_praises: list[str],
    ):
        """打印整体洞察"""
        console.print()

        total = len(products)
        tb_count = sum(1 for p in products if p.platform == "taobao")
        pdd_count = total - tb_count
        avg_price = sum(p.price for p in products) / total if total else 0
        total_reviews = sum(len(p.reviews) for p in products)

        insights = (
            f"[bold]📊 整体洞察[/bold]\n\n"
            f"  采集商品: {total} 个 (淘宝 {tb_count} / 拼多多 {pdd_count})\n"
            f"  采集评价: {total_reviews} 条\n"
            f"  平均价格: ¥{avg_price:.1f}\n\n"
        )

        if top_praises:
            insights += "  [green]常见好评词:[/green] " + "、".join(top_praises[:8]) + "\n"
        if top_concerns:
            insights += "  [red]常见差评词:[/red] " + "、".join(top_concerns[:8]) + "\n"

        console.print(Panel(insights, border_style="bright_blue", width=100))

    # ── 导出 ──────────────────────────────────────────

    @staticmethod
    def export_csv(products: list[Product], path: Path):
        """导出排名结果到 CSV"""
        with open(path, "w", newline="", encoding="utf-8-sig") as f:
            writer = csv.writer(f)
            writer.writerow([
                "排名", "平台", "商品名称", "价格", "销量", "店铺",
                "评价数", "好评率", "追评数", "综合评分", "摘要", "链接",
            ])
            for p in products:
                writer.writerow([
                    p.rank,
                    "淘宝" if p.platform == "taobao" else "拼多多",
                    p.title,
                    f"{p.price:.2f}",
                    p.sales_count,
                    p.shop_name,
                    len(p.reviews),
                    f"{p.positive_review_ratio:.2%}" if p.reviews else "-",
                    p.follow_up_count,
                    f"{p.composite_score:.4f}",
                    p.summary,
                    p.url,
                ])
        console.print(f"[green]✓ CSV 已导出: {path}[/green]")

    @staticmethod
    def export_json(result: SearchResult, path: Path):
        """导出完整数据到 JSON"""
        result.save(path)
        console.print(f"[green]✓ JSON 已导出: {path}[/green]")


# ── 辅助函数 ──────────────────────────────────────────

def _format_sales(count: int) -> str:
    if count >= 10000:
        return f"{count / 10000:.1f}万"
    return str(count)
