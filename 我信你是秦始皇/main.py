"""航班价格监控工具 -- 自动检测机票降价并邮件通知

用法示例:
  # 单次查询
  python main.py --from 北京首都机场 --to 上海虹桥机场 --date 2026.5.1 --time 13:00-17:00

  # 持续监控 (每30分钟)
  python main.py --from PEK --to SHA --date 2026-05-01 --time 13:00-17:00 --monitor

  # 自定义阈值和间隔
  python main.py --from 北京 --to 上海 --date 2026.5.1 --threshold 100 --interval 15 --monitor

  # 列出支持的机场
  python main.py --list-airports
"""

from __future__ import annotations

import argparse
import os
import re
import sys
import time
from datetime import datetime

from dotenv import load_dotenv

from airports import resolve_airport, list_airports
from flight_api import AmadeusClient
from notifier import send_notification
from tracker import PriceTracker


def parse_time_range(s: str) -> tuple[int, int, int, int]:
    """Parse 'HH:MM-HH:MM' -> (start_h, start_m, end_h, end_m)."""
    m = re.match(r"(\d{1,2}):(\d{2})\s*-\s*(\d{1,2}):(\d{2})", s)
    if not m:
        raise ValueError(f"时间格式错误: {s} (应为 HH:MM-HH:MM)")
    return int(m[1]), int(m[2]), int(m[3]), int(m[4])


def parse_date(s: str) -> str:
    """Normalize various date formats to YYYY-MM-DD."""
    s = s.strip().replace("日", "").replace("年", "-").replace("月", "-")
    for fmt in ("%Y-%m-%d", "%Y.%m.%d", "%Y/%m/%d"):
        try:
            return datetime.strptime(s, fmt).strftime("%Y-%m-%d")
        except ValueError:
            pass
    for fmt in ("%m-%d", "%m.%d", "%m/%d"):
        try:
            d = datetime.strptime(s, fmt).replace(year=datetime.now().year)
            return d.strftime("%Y-%m-%d")
        except ValueError:
            pass
    raise ValueError(f"日期格式错误: {s} (应为 YYYY-MM-DD 或 YYYY.MM.DD)")


def run_check(args: argparse.Namespace, client: AmadeusClient, tracker: PriceTracker):
    """Execute a single price-check cycle."""
    origin_code = resolve_airport(args.origin)
    dest_code = resolve_airport(args.destination)
    date = parse_date(args.date)
    threshold = float(os.getenv("PRICE_DROP_THRESHOLD", str(args.threshold)))

    print(f"\n{'=' * 60}")
    print(f"[检测] {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"[航线] {args.origin}({origin_code}) -> {args.destination}({dest_code})")
    print(f"[日期] {date}")

    try:
        offers = client.search_flights(
            origin=args.origin,
            destination=args.destination,
            date=date,
            currency=args.currency,
        )
    except Exception as e:
        print(f"[错误] 查询航班失败: {e}")
        return

    # Time-range filter
    if args.time_range:
        sh, sm, eh, em = parse_time_range(args.time_range)
        print(f"[时段] {sh:02d}:{sm:02d} - {eh:02d}:{em:02d}")
        offers = [o for o in offers if o.departure_in_range(sh, sm, eh, em)]

    if not offers:
        print("[结果] 未找到符合条件的航班")
        return

    offers.sort(key=lambda o: o.price)
    print(f"[结果] 找到 {len(offers)} 个航班, 最低价 {offers[0].currency} {offers[0].price:.0f}")
    for i, o in enumerate(offers[:5]):
        print(f"  {i + 1}. {o}")

    # Price-drop detection
    drop = tracker.check_and_update(
        origin=origin_code,
        destination=dest_code,
        date=date,
        offers=offers,
        threshold=threshold,
    )

    if drop:
        print(
            f"\n[降价] 降幅 {drop.drop_amount:.0f}{drop.currency} "
            f"({drop.drop_percent:.1f}%): "
            f"{drop.previous_price:.0f} -> {drop.current_price:.0f}"
        )
        _try_notify(drop, offers, args)
    else:
        _print_trend(tracker, origin_code, dest_code, date, offers[0].price, threshold)


def _try_notify(drop, offers, args):
    """Attempt to send an email notification for a price drop."""
    email = os.getenv("NOTIFY_EMAIL", args.email or "")
    smtp_user = os.getenv("SMTP_USER", "")
    smtp_password = os.getenv("SMTP_PASSWORD", "")

    if not email:
        print("[提示] 未配置邮箱, 跳过通知 (在 .env 中设置 NOTIFY_EMAIL)")
        return
    if not smtp_user or not smtp_password:
        print("[提示] 未配置 SMTP 账号密码, 跳过通知 (在 .env 中设置 SMTP_USER / SMTP_PASSWORD)")
        return

    try:
        send_notification(
            drop=drop,
            all_offers=offers,
            smtp_host=os.getenv("SMTP_HOST", "smtp.qq.com"),
            smtp_port=int(os.getenv("SMTP_PORT", "465")),
            smtp_user=smtp_user,
            smtp_password=smtp_password,
            recipient=email,
            use_ssl=os.getenv("SMTP_USE_SSL", "true").lower() == "true",
        )
    except Exception as e:
        print(f"[错误] 发送邮件失败: {e}")


def _print_trend(tracker, origin, dest, date, current_price, threshold):
    """Print price trend compared to last check."""
    history = tracker.get_history(origin, dest, date)
    if len(history) < 2:
        print(f"[价格] 首次记录, 后续将对比变动 (降价阈值: {threshold:.0f})")
        return
    prev = history[-2]["lowest_price"]
    if current_price > prev:
        print(f"[价格] 较上次涨价 {current_price - prev:.0f}")
    elif current_price == prev:
        print("[价格] 与上次持平")
    else:
        print(f"[价格] 较上次降 {prev - current_price:.0f} (未达阈值 {threshold:.0f})")


def main():
    load_dotenv()

    parser = argparse.ArgumentParser(
        description="航班价格监控 -- 自动检测机票降价并邮件通知",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""\
示例:
  python main.py --from 北京首都机场 --to 上海虹桥机场 --date 2026.5.1 --time 13:00-17:00
  python main.py --from PEK --to SHA --date 2026-05-01 --monitor --interval 15
  python main.py --list-airports""",
    )
    parser.add_argument("--from", dest="origin", help="出发机场 (中文名或IATA代码)")
    parser.add_argument("--to", dest="destination", help="到达机场 (中文名或IATA代码)")
    parser.add_argument("--date", help="出发日期 (YYYY-MM-DD / YYYY.M.D)")
    parser.add_argument("--time", dest="time_range", help="出发时段 (HH:MM-HH:MM)")
    parser.add_argument("--threshold", type=float, default=50, help="降价阈值 (元, 默认50)")
    parser.add_argument("--email", help="通知邮箱 (或在 .env 设置 NOTIFY_EMAIL)")
    parser.add_argument("--currency", default="CNY", help="货币 (默认 CNY)")
    parser.add_argument("--monitor", action="store_true", help="持续监控模式")
    parser.add_argument(
        "--interval",
        type=int,
        default=None,
        help="检查间隔/分钟 (默认读取 .env CHECK_INTERVAL, 否则30)",
    )
    parser.add_argument("--list-airports", action="store_true", help="列出支持的机场")

    args = parser.parse_args()

    if args.list_airports:
        list_airports()
        return

    if not args.origin or not args.destination or not args.date:
        parser.print_help()
        print("\n请提供 --from, --to, --date 三个必要参数")
        sys.exit(1)

    api_key = os.getenv("AMADEUS_API_KEY")
    api_secret = os.getenv("AMADEUS_API_SECRET")
    if not api_key or not api_secret:
        print("[错误] 请在 .env 中配置 AMADEUS_API_KEY 和 AMADEUS_API_SECRET")
        print("       免费注册: https://developers.amadeus.com")
        sys.exit(1)

    client = AmadeusClient(
        api_key=api_key,
        api_secret=api_secret,
        env=os.getenv("AMADEUS_ENV", "test"),
    )
    tracker = PriceTracker()

    interval = args.interval or int(os.getenv("CHECK_INTERVAL", "30"))

    if args.monitor:
        print(f"[启动] 价格监控已启动, 每 {interval} 分钟检查一次")
        print("[提示] Ctrl+C 停止")
        try:
            while True:
                run_check(args, client, tracker)
                print(f"\n[等待] 下次检查: {interval} 分钟后 ...")
                time.sleep(interval * 60)
        except KeyboardInterrupt:
            print("\n[停止] 监控已停止")
    else:
        run_check(args, client, tracker)


if __name__ == "__main__":
    main()
