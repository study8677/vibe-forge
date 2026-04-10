#!/usr/bin/env python3
"""
电商比价分析工具 — 自动搜索淘宝 / 拼多多，采集评价，综合推荐最优商品

用法:
    python main.py --keyword "无线鼠标" --max-price 100
    python main.py -k "蓝牙耳机" --min-price 50 --max-price 300
    python main.py -k "洗面奶" --max-price 80 --platform taobao
    python main.py -k "充电宝" --max-price 150 --products 50 --reviews 50
"""

from __future__ import annotations

import argparse
import asyncio
import sys
from datetime import datetime
from pathlib import Path

from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn

# 确保项目根目录在 path 中
sys.path.insert(0, str(Path(__file__).parent))

from browser import BrowserManager
from platforms.taobao import TaobaoScraper
from platforms.pdd import PddScraper
from analyzer import Analyzer
from reporter import Reporter
from models import Product, SearchResult
import config as cfg

console = Console()


async def main():
    args = parse_args()

    keyword = args.keyword
    min_price = args.min_price
    max_price = args.max_price
    max_products = args.products
    max_reviews = args.reviews
    platforms = args.platform
    top_n = args.top

    # 打印搜索头
    Reporter.print_header(keyword, min_price, max_price)

    # 初始化浏览器
    bm = BrowserManager()
    await bm.start()

    all_products: list[Product] = []

    try:
        # ── 1. 搜索商品 ──────────────────────────────

        if "taobao" in platforms:
            console.print("[bold orange1]▸ 启动淘宝搜索...[/bold orange1]")
            tb = TaobaoScraper(bm)
            try:
                await tb.ensure_login()
                tb_products = await tb.search_products(
                    keyword, min_price, max_price, max_products
                )
                all_products.extend(tb_products)
                Reporter.print_search_progress("taobao", len(tb_products))
            except Exception as e:
                console.print(f"[red]淘宝搜索失败: {e}[/red]")

        if "pdd" in platforms:
            console.print("[bold red]▸ 启动拼多多搜索...[/bold red]")
            pdd = PddScraper(bm)
            try:
                await pdd.ensure_login()
                pdd_products = await pdd.search_products(
                    keyword, min_price, max_price, max_products
                )
                all_products.extend(pdd_products)
                Reporter.print_search_progress("pdd", len(pdd_products))
            except Exception as e:
                console.print(f"[red]拼多多搜索失败: {e}[/red]")

        if not all_products:
            console.print("[bold red]未找到任何商品，请检查关键字和价格区间[/bold red]")
            return

        console.print(f"\n[bold]共采集 {len(all_products)} 个商品，开始采集评价...[/bold]\n")

        # ── 2. 采集评价 ──────────────────────────────

        with Progress(
            SpinnerColumn(),
            TextColumn("[bold]采集评价"),
            BarColumn(),
            TextColumn("{task.completed}/{task.total}"),
            console=console,
        ) as progress:
            review_task = progress.add_task("评价采集", total=len(all_products))

            for i, product in enumerate(all_products):
                Reporter.print_review_progress(
                    i + 1, len(all_products), product.title
                )
                try:
                    # 根据平台选择对应的爬虫
                    if product.platform == "taobao":
                        scraper = tb
                    else:
                        scraper = pdd

                    await scraper.get_reviews(product, max_reviews)

                except Exception as e:
                    console.print(
                        f"  [yellow]⚠ 评价采集失败: {product.title[:30]}... ({e})[/yellow]"
                    )

                progress.update(review_task, completed=i + 1)

        total_reviews = sum(len(p.reviews) for p in all_products)
        total_follow_ups = sum(p.follow_up_count for p in all_products)
        console.print(
            f"\n[bold green]✓ 评价采集完成: {total_reviews} 条评价, "
            f"{total_follow_ups} 条追评[/bold green]\n"
        )

        # ── 3. 分析与评分 ────────────────────────────

        console.print("[bold]▸ 分析评价数据...[/bold]")
        analyzer = Analyzer()
        ranked_products = analyzer.analyze_and_rank(all_products)

        top_concerns = analyzer.get_top_concerns(ranked_products)
        top_praises = analyzer.get_top_praises(ranked_products)

        # ── 4. 输出结果 ──────────────────────────────

        Reporter.print_ranking(ranked_products, top_n=top_n)
        Reporter.print_top_recommendations(ranked_products, top_n=5)
        Reporter.print_overall_insights(ranked_products, top_concerns, top_praises)

        # ── 5. 导出文件 ──────────────────────────────

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        safe_keyword = keyword.replace(" ", "_")[:20]

        csv_path = cfg.RESULTS_DIR / f"{safe_keyword}_{timestamp}.csv"
        json_path = cfg.RESULTS_DIR / f"{safe_keyword}_{timestamp}.json"

        result = SearchResult(
            keyword=keyword,
            min_price=min_price,
            max_price=max_price,
            products=ranked_products,
            timestamp=timestamp,
        )

        Reporter.export_csv(ranked_products, csv_path)
        Reporter.export_json(result, json_path)

        console.print(f"\n[bold green]🎉 分析完成！结果已保存到 {cfg.RESULTS_DIR}[/bold green]\n")

    except KeyboardInterrupt:
        console.print("\n[yellow]用户中断，正在保存...[/yellow]")
    finally:
        await bm.stop()


def parse_args():
    parser = argparse.ArgumentParser(
        description="电商比价分析工具 — 自动搜索淘宝/拼多多，采集评价，综合推荐最优商品",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  python main.py -k "无线鼠标" --max-price 100
  python main.py -k "蓝牙耳机" --min-price 50 --max-price 300 --platform taobao pdd
  python main.py -k "洗面奶" --max-price 80 --products 50 --reviews 50
        """,
    )
    parser.add_argument(
        "-k", "--keyword", required=True, help="搜索关键字"
    )
    parser.add_argument(
        "--min-price", type=float, default=0, help="最低价格 (默认: 0)"
    )
    parser.add_argument(
        "--max-price", type=float, required=True, help="最高价格"
    )
    parser.add_argument(
        "--platform",
        nargs="+",
        choices=["taobao", "pdd"],
        default=["taobao", "pdd"],
        help="搜索平台 (默认: taobao pdd)",
    )
    parser.add_argument(
        "--products",
        type=int,
        default=cfg.MAX_PRODUCTS,
        help=f"每个平台最多采集商品数 (默认: {cfg.MAX_PRODUCTS})",
    )
    parser.add_argument(
        "--reviews",
        type=int,
        default=cfg.MAX_REVIEWS,
        help=f"每个商品最多采集评价数 (默认: {cfg.MAX_REVIEWS})",
    )
    parser.add_argument(
        "--top",
        type=int,
        default=20,
        help="排名表格显示的商品数 (默认: 20)",
    )
    parser.add_argument(
        "--headless",
        action="store_true",
        help="无头模式（需要已保存的登录 Cookie）",
    )

    args = parser.parse_args()

    if args.headless:
        cfg.HEADLESS = True

    return args


if __name__ == "__main__":
    asyncio.run(main())
