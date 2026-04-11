"""Email notification for flight price drops."""

from __future__ import annotations

import smtplib
from datetime import datetime
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

from flight_api import FlightOffer
from tracker import PriceDrop


def _build_email(drop: PriceDrop, all_offers: list[FlightOffer]) -> MIMEMultipart:
    msg = MIMEMultipart("alternative")
    msg["Subject"] = (
        f"[降价提醒] {drop.origin}->{drop.destination} "
        f"{drop.date} 降 {drop.drop_amount:.0f}{drop.currency}"
    )

    # --- plain text ---
    lines = [
        "机票降价提醒",
        "",
        f"航线: {drop.origin} -> {drop.destination}",
        f"日期: {drop.date}",
        f"原价: {drop.currency} {drop.previous_price:.0f}",
        f"现价: {drop.currency} {drop.current_price:.0f}",
        f"降幅: {drop.drop_amount:.0f} ({drop.drop_percent:.1f}%)",
        "",
        "最低价航班:",
        f"  {drop.flight}",
        "",
        "其他航班 (价格从低到高):",
    ]
    for offer in sorted(all_offers, key=lambda o: o.price)[:10]:
        lines.append(f"  {offer}")
    lines.append("")
    lines.append(f"检测时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    text_body = "\n".join(lines)

    # --- HTML ---
    rows = ""
    for offer in sorted(all_offers, key=lambda o: o.price)[:10]:
        dep = offer.format_time(offer.departure_time)
        arr = offer.format_time(offer.arrival_time)
        stops = "直飞" if offer.stops == 0 else f"{offer.stops}转"
        rows += (
            f"<tr>"
            f"<td style='padding:6px;border:1px solid #ddd'>{offer.airline} {offer.flight_number}</td>"
            f"<td style='padding:6px;border:1px solid #ddd'>{dep}-{arr}</td>"
            f"<td style='padding:6px;border:1px solid #ddd'>{stops}</td>"
            f"<td style='padding:6px;border:1px solid #ddd'>{offer.currency} {offer.price:.0f}</td>"
            f"</tr>"
        )

    html_body = f"""\
<html><body style="font-family:Arial,sans-serif;max-width:620px;margin:0 auto">
<h2 style="color:#d32f2f">机票降价提醒</h2>
<table style="border-collapse:collapse;width:100%;margin:12px 0">
  <tr><td style="padding:8px;border:1px solid #ddd;background:#f5f5f5"><b>航线</b></td>
      <td style="padding:8px;border:1px solid #ddd">{drop.origin} -> {drop.destination}</td></tr>
  <tr><td style="padding:8px;border:1px solid #ddd;background:#f5f5f5"><b>日期</b></td>
      <td style="padding:8px;border:1px solid #ddd">{drop.date}</td></tr>
  <tr><td style="padding:8px;border:1px solid #ddd;background:#f5f5f5"><b>原价</b></td>
      <td style="padding:8px;border:1px solid #ddd;text-decoration:line-through;color:#999">
        {drop.currency} {drop.previous_price:.0f}</td></tr>
  <tr><td style="padding:8px;border:1px solid #ddd;background:#f5f5f5"><b>现价</b></td>
      <td style="padding:8px;border:1px solid #ddd;color:#d32f2f;font-size:1.2em;font-weight:bold">
        {drop.currency} {drop.current_price:.0f}</td></tr>
  <tr><td style="padding:8px;border:1px solid #ddd;background:#f5f5f5"><b>降幅</b></td>
      <td style="padding:8px;border:1px solid #ddd;color:#2e7d32">
        {drop.drop_amount:.0f} ({drop.drop_percent:.1f}%)</td></tr>
</table>

<h3>最低价航班</h3>
<p style="padding:10px;background:#e8f5e9;border-radius:4px;font-family:monospace">
  {drop.flight}
</p>

<h3>可选航班 (价格从低到高)</h3>
<table style="border-collapse:collapse;width:100%">
  <tr style="background:#f5f5f5">
    <th style="padding:6px;border:1px solid #ddd">航班</th>
    <th style="padding:6px;border:1px solid #ddd">时间</th>
    <th style="padding:6px;border:1px solid #ddd">中转</th>
    <th style="padding:6px;border:1px solid #ddd">价格</th>
  </tr>
  {rows}
</table>

<p style="color:#999;font-size:0.85em;margin-top:20px">
  检测时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}<br>
  此邮件由航班价格监控系统自动发送
</p>
</body></html>"""

    msg.attach(MIMEText(text_body, "plain", "utf-8"))
    msg.attach(MIMEText(html_body, "html", "utf-8"))
    return msg


def send_notification(
    drop: PriceDrop,
    all_offers: list[FlightOffer],
    smtp_host: str,
    smtp_port: int,
    smtp_user: str,
    smtp_password: str,
    recipient: str,
    use_ssl: bool = True,
):
    """Send a price-drop notification email via SMTP."""
    msg = _build_email(drop, all_offers)
    msg["From"] = smtp_user
    msg["To"] = recipient

    if use_ssl:
        with smtplib.SMTP_SSL(smtp_host, smtp_port, timeout=15) as srv:
            srv.login(smtp_user, smtp_password)
            srv.send_message(msg)
    else:
        with smtplib.SMTP(smtp_host, smtp_port, timeout=15) as srv:
            srv.starttls()
            srv.login(smtp_user, smtp_password)
            srv.send_message(msg)

    print(f"[通知] 降价提醒已发送至 {recipient}")
