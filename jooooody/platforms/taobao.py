"""淘宝 / 天猫爬虫"""

from __future__ import annotations

import asyncio
import json
import math
import re
import random
from typing import Optional
from urllib.parse import quote, urlencode

from playwright.async_api import Page, Response
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn

from browser import BrowserManager, random_sleep, smooth_scroll, wait_for_login
from models import Product, Review
from platforms.base import BaseScraper
import config as cfg

console = Console()


class TaobaoScraper(BaseScraper):
    platform_name = "taobao"

    SEARCH_URL = "https://s.taobao.com/search"
    MOBILE_SEARCH_URL = "https://s.m.taobao.com/h5"
    LOGIN_URL = "https://login.taobao.com"
    HOME_URL = "https://www.taobao.com"

    def __init__(self, bm: BrowserManager):
        super().__init__(bm)
        self._intercepted_reviews: list[dict] = []

    # ── 登录 ──────────────────────────────────────────

    async def ensure_login(self) -> bool:
        self.page = await self.bm.new_page("taobao")

        await self.page.goto(self.HOME_URL, wait_until="domcontentloaded")
        await random_sleep(2, 3)

        # 检测是否已登录：页面上有用户昵称说明已登录
        logged_in = await self._check_login()
        if logged_in:
            console.print("[green]✓ 淘宝已登录[/green]")
            return True

        # 跳转到登录页
        await self.page.goto(self.LOGIN_URL, wait_until="domcontentloaded")
        success = await wait_for_login(
            self.page,
            # 登录成功后会跳转回首页，检测用户信息元素
            'a[class*="nickname"], span[class*="nickname"], '
            'a[class*="userName"], div[class*="member"]',
            "淘宝",
        )
        if success:
            await self.bm.save_cookies("taobao")
        return success

    async def _check_login(self) -> bool:
        """检测当前页面是否已登录"""
        try:
            # 多种登录标识
            selectors = [
                'a[class*="nickname"]',
                'span[class*="nickname"]',
                'a[class*="userName"]',
                'div[class*="member"]',
                'a[href*="member"]',
            ]
            for sel in selectors:
                el = await self.page.query_selector(sel)
                if el:
                    text = await el.inner_text()
                    if text.strip() and "登录" not in text:
                        return True

            # 也可以检查 Cookie 中是否有登录标识
            cookies = await self.page.context.cookies()
            for c in cookies:
                if c["name"] in ("_nk_", "snk", "unb") and c["value"]:
                    return True
        except Exception:
            pass
        return False

    # ── 搜索 ──────────────────────────────────────────

    async def search_products(
        self,
        keyword: str,
        min_price: float,
        max_price: float,
        limit: int = 100,
    ) -> list[Product]:
        if not self.page:
            await self.ensure_login()

        products: list[Product] = []
        page_num = 1
        pages_needed = math.ceil(limit / 44)  # 淘宝每页约 44 个商品

        with Progress(
            SpinnerColumn(),
            TextColumn("[bold blue]淘宝搜索"),
            BarColumn(),
            TextColumn("{task.completed}/{task.total}"),
            console=console,
        ) as progress:
            task = progress.add_task("搜索中", total=limit)

            while len(products) < limit and page_num <= pages_needed + 2:
                params = {
                    "q": keyword,
                    "s": (page_num - 1) * 44,
                    "filter": f"reserve_price[{min_price},{max_price}]",
                    "sort": "sale-desc",  # 按销量排序
                }
                url = f"{self.SEARCH_URL}?{urlencode(params)}"

                try:
                    await self.page.goto(url, wait_until="domcontentloaded")
                    await random_sleep(*cfg.PAGE_PAUSE)

                    # 处理可能的滑块验证
                    await self._handle_captcha()

                    # 滚动加载
                    for _ in range(5):
                        await smooth_scroll(self.page, 600)

                    # 提取商品数据
                    page_products = await self._extract_products_from_page()

                    for p in page_products:
                        if len(products) >= limit:
                            break
                        if min_price <= p.price <= max_price:
                            products.append(p)
                            progress.update(task, completed=len(products))

                except Exception as e:
                    console.print(f"[yellow]淘宝第 {page_num} 页采集异常: {e}[/yellow]")

                page_num += 1
                await random_sleep(*cfg.PAGE_PAUSE)

        console.print(f"[green]✓ 淘宝共采集 {len(products)} 个商品[/green]")
        return products

    async def _extract_products_from_page(self) -> list[Product]:
        """从搜索结果页提取商品列表"""
        products = []

        # 策略 1: 尝试从页面嵌入的 JSON 数据提取
        json_products = await self._extract_from_page_data()
        if json_products:
            return json_products

        # 策略 2: DOM 解析
        return await self._extract_from_dom()

    async def _extract_from_page_data(self) -> list[Product]:
        """从页面 JS 变量 / script 标签中提取结构化数据"""
        products = []
        try:
            # 淘宝搜索结果页会在 window 或 script 中嵌入商品数据
            data = await self.page.evaluate("""() => {
                // 尝试多种数据源
                const scripts = document.querySelectorAll('script');
                for (const s of scripts) {
                    const text = s.textContent || '';
                    // 搜索结果数据
                    const match = text.match(/g_page_config\\s*=\\s*({.+?});/s)
                        || text.match(/"itemsArray"\\s*:\\s*(\\[.+?\\])/s)
                        || text.match(/"auctions"\\s*:\\s*(\\[.+?\\])/s);
                    if (match) return match[1] || match[0];
                }
                // 新版淘宝可能用 __INITIAL_DATA__
                if (window.__INITIAL_DATA__) {
                    return JSON.stringify(window.__INITIAL_DATA__);
                }
                if (window.g_page_config) {
                    return JSON.stringify(window.g_page_config);
                }
                return null;
            }""")

            if not data:
                return []

            if isinstance(data, str):
                data = json.loads(data)

            # 解析不同格式的数据
            items = self._find_items_in_data(data)

            for item in items:
                try:
                    nid = str(item.get("nid", "") or item.get("item_id", "") or item.get("itemId", ""))
                    if not nid:
                        continue

                    title = item.get("raw_title", "") or item.get("title", "")
                    price = float(item.get("view_price", 0) or item.get("price", 0))
                    sales_text = item.get("view_sales", "") or item.get("sales", "")
                    sales = self._parse_sales(sales_text)
                    shop = item.get("nick", "") or item.get("shopName", "")

                    detail_url = item.get("detail_url", "")
                    if detail_url and not detail_url.startswith("http"):
                        detail_url = "https:" + detail_url

                    pic_url = item.get("pic_url", "")
                    if pic_url and not pic_url.startswith("http"):
                        pic_url = "https:" + pic_url

                    products.append(Product(
                        platform="taobao",
                        product_id=nid,
                        title=title,
                        price=price,
                        sales_count=sales,
                        sales_text=sales_text,
                        shop_name=shop,
                        url=detail_url or f"https://item.taobao.com/item.htm?id={nid}",
                        image_url=pic_url,
                    ))
                except (ValueError, TypeError):
                    continue

        except Exception as e:
            console.print(f"[dim]JSON 解析: {e}[/dim]")

        return products

    def _find_items_in_data(self, data, depth=0) -> list[dict]:
        """递归搜索数据中的商品列表"""
        if depth > 8:
            return []

        if isinstance(data, list):
            # 检查是否是商品列表
            if data and isinstance(data[0], dict) and (
                "nid" in data[0] or "item_id" in data[0] or "itemId" in data[0]
            ):
                return data
            # 递归搜索列表中的每个元素
            for item in data:
                result = self._find_items_in_data(item, depth + 1)
                if result:
                    return result

        if isinstance(data, dict):
            # 常见的商品列表键名
            for key in (
                "auctions", "itemsArray", "items", "resultList",
                "itemList", "list", "data", "content", "resultContent",
            ):
                if key in data:
                    result = self._find_items_in_data(data[key], depth + 1)
                    if result:
                        return result
            # 递归搜索所有值
            for v in data.values():
                if isinstance(v, (dict, list)):
                    result = self._find_items_in_data(v, depth + 1)
                    if result:
                        return result

        return []

    async def _extract_from_dom(self) -> list[Product]:
        """从 DOM 中提取商品（兜底策略）"""
        products = []
        try:
            # 查找所有指向商品详情的链接
            items = await self.page.evaluate("""() => {
                const results = [];
                // 查找所有商品卡片链接
                const links = document.querySelectorAll(
                    'a[href*="item.taobao.com/item.htm"], '
                    + 'a[href*="detail.tmall.com/item.htm"], '
                    + 'a[href*="detail.tmall.com/item_o.htm"]'
                );

                for (const link of links) {
                    const href = link.href || link.getAttribute('href') || '';
                    const idMatch = href.match(/[?&]id=(\\d+)/);
                    if (!idMatch) continue;

                    const card = link.closest('div[class*="Card"], div[class*="card"], div[class*="item"], li');
                    if (!card) continue;

                    const text = card.innerText || '';
                    const lines = text.split('\\n').map(l => l.trim()).filter(Boolean);

                    // 提取价格
                    let price = 0;
                    const priceMatch = text.match(/¥\\s*(\\d+\\.?\\d*)/);
                    if (priceMatch) price = parseFloat(priceMatch[1]);

                    // 提取销量
                    let salesText = '';
                    const salesMatch = text.match(/(\\d[\\d.]*万?\\+?)\\s*人?(付款|已售|已买|收货|人购买)/);
                    if (salesMatch) salesText = salesMatch[0];

                    // 标题：通常是最长的文本行
                    let title = '';
                    for (const line of lines) {
                        if (line.length > title.length && !line.includes('¥') && !line.includes('付款')) {
                            title = line;
                        }
                    }

                    // 图片
                    const img = card.querySelector('img');
                    const imgSrc = img ? (img.src || img.getAttribute('data-src') || '') : '';

                    results.push({
                        id: idMatch[1],
                        title: title.slice(0, 200),
                        price,
                        salesText,
                        href,
                        imgSrc,
                    });
                }
                return results;
            }""")

            seen_ids = set()
            for item in items:
                pid = item["id"]
                if pid in seen_ids:
                    continue
                seen_ids.add(pid)

                url = item["href"]
                if url and not url.startswith("http"):
                    url = "https:" + url

                products.append(Product(
                    platform="taobao",
                    product_id=pid,
                    title=item["title"],
                    price=item["price"],
                    sales_count=self._parse_sales(item["salesText"]),
                    sales_text=item["salesText"],
                    url=url or f"https://item.taobao.com/item.htm?id={pid}",
                    image_url=item["imgSrc"],
                ))

        except Exception as e:
            console.print(f"[yellow]DOM 解析失败: {e}[/yellow]")

        return products

    # ── 评价采集 ───────────────────────────────────────

    async def get_reviews(self, product: Product, limit: int = 100) -> list[Review]:
        reviews: list[Review] = []
        self._intercepted_reviews.clear()

        try:
            # 注册网络拦截，捕获评价 API 响应
            self.page.on("response", self._on_review_response)

            # 打开商品详情页
            await self.page.goto(product.url, wait_until="domcontentloaded")
            await random_sleep(2, 4)

            # 处理验证码
            await self._handle_captcha()

            # 提取商品评分和评价数
            await self._extract_product_meta(product)

            # 点击评价 Tab
            await self._click_review_tab()
            await random_sleep(1, 2)

            # 翻页采集评价
            page_num = 1
            max_pages = math.ceil(limit / cfg.REVIEWS_PAGE_SIZE) + 1

            while len(reviews) < limit and page_num <= max_pages:
                # 等待评价加载
                await random_sleep(1, 2)

                # 优先从 API 拦截中获取
                if self._intercepted_reviews:
                    for rd in self._intercepted_reviews:
                        if len(reviews) >= limit:
                            break
                        reviews.append(self._parse_review_data(rd))
                    self._intercepted_reviews.clear()
                else:
                    # 从 DOM 提取
                    dom_reviews = await self._extract_reviews_from_dom()
                    for r in dom_reviews:
                        if len(reviews) >= limit:
                            break
                        # 去重
                        if not any(
                            existing.content == r.content and existing.date == r.date
                            for existing in reviews
                        ):
                            reviews.append(r)

                # 翻页
                has_next = await self._click_next_review_page()
                if not has_next:
                    break
                page_num += 1
                await random_sleep(*cfg.PAGE_PAUSE)

        except Exception as e:
            console.print(
                f"[yellow]评价采集异常 [{product.product_id}]: {e}[/yellow]"
            )
        finally:
            try:
                self.page.remove_listener("response", self._on_review_response)
            except Exception:
                pass

        product.reviews = reviews
        return reviews

    async def _on_review_response(self, response: Response):
        """拦截评价 API 响应"""
        url = response.url
        if not any(kw in url for kw in (
            "rate.taobao.com", "rate.tmall.com",
            "feedRateList", "list_detail_rate",
            "reviewList", "rateList",
        )):
            return

        try:
            text = await response.text()
            # JSONP 格式处理
            text = re.sub(r'^[^({]*[({]', '{', text.rstrip().rstrip(')').rstrip(';'))
            data = json.loads(text)

            # 提取评价列表
            comment_list = (
                data.get("comments", [])
                or data.get("rateList", [])
                or data.get("rateDetail", {}).get("rateList", [])
                or data.get("data", {}).get("comments", [])
                or data.get("data", {}).get("rateList", [])
            )
            self._intercepted_reviews.extend(comment_list)
        except Exception:
            pass

    def _parse_review_data(self, data: dict) -> Review:
        """解析 API 返回的单条评价"""
        content = data.get("content", "") or data.get("rateContent", "") or ""

        # 追评
        follow_up = ""
        follow_up_days = 0
        append = data.get("appendComment", {}) or data.get("append", {})
        if append:
            follow_up = append.get("content", "") or append.get("commentContent", "") or ""
            follow_up_days = append.get("dayAfterConfirm", 0) or 0

        # 图片
        photos = data.get("photos", []) or data.get("pics", []) or []
        has_image = len(photos) > 0

        # 评分
        rate = data.get("rate", 5)
        if isinstance(rate, str):
            rate = {"good": 5, "neutral": 3, "bad": 1}.get(rate, 3)

        return Review(
            content=content.strip(),
            rating=rate,
            date=data.get("date", "") or data.get("rateDate", "") or "",
            user=data.get("displayUserNick", "") or data.get("nick", "") or "",
            sku=data.get("auctionSku", "") or data.get("sku", "") or "",
            has_image=has_image,
            image_count=len(photos),
            follow_up=follow_up.strip(),
            follow_up_days=follow_up_days,
        )

    async def _extract_reviews_from_dom(self) -> list[Review]:
        """从 DOM 提取评价（兜底）"""
        reviews = []
        try:
            data = await self.page.evaluate("""() => {
                const reviews = [];
                const cards = document.querySelectorAll(
                    'div[class*="rate-item"], div[class*="review-item"], '
                    + 'div[class*="comment-item"], div[class*="Comment"], '
                    + 'div[class*="rateContent"]'
                );

                for (const card of cards) {
                    const content = card.querySelector(
                        'div[class*="content"], span[class*="content"], p[class*="content"]'
                    );
                    if (!content) continue;

                    const text = content.innerText.trim();
                    if (!text) continue;

                    // 追评
                    let followUp = '';
                    const append = card.querySelector(
                        'div[class*="append"], div[class*="追评"], div[class*="follow"]'
                    );
                    if (append) followUp = append.innerText.trim();

                    // 日期
                    const dateEl = card.querySelector(
                        'span[class*="date"], span[class*="time"], div[class*="date"]'
                    );
                    const date = dateEl ? dateEl.innerText.trim() : '';

                    // 图片
                    const imgs = card.querySelectorAll(
                        'img[class*="photo"], img[class*="pic"], img[class*="review"]'
                    );

                    // SKU
                    const skuEl = card.querySelector(
                        'span[class*="sku"], div[class*="sku"], span[class*="spec"]'
                    );

                    reviews.push({
                        content: text,
                        date,
                        followUp,
                        imageCount: imgs.length,
                        sku: skuEl ? skuEl.innerText.trim() : '',
                    });
                }
                return reviews;
            }""")

            for item in data:
                reviews.append(Review(
                    content=item["content"],
                    date=item.get("date", ""),
                    follow_up=item.get("followUp", ""),
                    has_image=item.get("imageCount", 0) > 0,
                    image_count=item.get("imageCount", 0),
                    sku=item.get("sku", ""),
                ))

        except Exception:
            pass

        return reviews

    async def _click_review_tab(self):
        """点击评价 Tab"""
        try:
            # 尝试多种选择器
            tab_selectors = [
                'text="累计评价"', 'text="评价"', 'text="宝贝评价"',
                'a[href*="rate"]', 'div[class*="tab"][class*="rate"]',
                'li:has-text("评价")', 'span:has-text("评价")',
            ]
            for sel in tab_selectors:
                try:
                    el = await self.page.wait_for_selector(sel, timeout=3000)
                    if el:
                        await el.click()
                        await random_sleep(1, 2)
                        return
                except Exception:
                    continue

            # 如果找不到 Tab，尝试滚动到评价区域
            await self.page.evaluate("""() => {
                const el = document.querySelector(
                    'div[class*="rate"], div[class*="review"], div[id*="rate"]'
                );
                if (el) el.scrollIntoView({ behavior: 'smooth' });
            }""")
        except Exception:
            pass

    async def _click_next_review_page(self) -> bool:
        """点击评价下一页，返回是否成功"""
        try:
            selectors = [
                'a:has-text("下一页")',
                'button:has-text("下一页")',
                'a[class*="next"]',
                'button[class*="next"]',
                'li[class*="next"] a',
            ]
            for sel in selectors:
                try:
                    el = await self.page.query_selector(sel)
                    if el:
                        disabled = await el.get_attribute("disabled")
                        cls = await el.get_attribute("class") or ""
                        if disabled or "disabled" in cls:
                            return False
                        await el.click()
                        return True
                except Exception:
                    continue
        except Exception:
            pass
        return False

    async def _extract_product_meta(self, product: Product):
        """提取商品详情页的评分和评价数等信息"""
        try:
            meta = await self.page.evaluate("""() => {
                const text = document.body.innerText;
                let reviewCount = 0;
                const rcMatch = text.match(/累计评价[\\s:：]*(\\d[\\d万+]*)/);
                if (rcMatch) {
                    let s = rcMatch[1].replace('+', '');
                    reviewCount = s.includes('万') ? parseFloat(s) * 10000 : parseInt(s);
                }

                let rating = 0;
                const ratingMatch = text.match(/(\\d\\.\\d)\\s*分/);
                if (ratingMatch) rating = parseFloat(ratingMatch[1]);

                let shopRating = 0;
                const shopMatch = text.match(/店铺评分[\\s:：]*(\\d\\.\\d)/);
                if (shopMatch) shopRating = parseFloat(shopMatch[1]);

                return { reviewCount, rating, shopRating };
            }""")

            if meta.get("reviewCount"):
                product.review_count = meta["reviewCount"]
            if meta.get("rating"):
                product.product_rating = meta["rating"]
            if meta.get("shopRating"):
                product.shop_rating = meta["shopRating"]

        except Exception:
            pass

    async def _handle_captcha(self):
        """检测并处理滑块验证"""
        try:
            captcha = await self.page.query_selector(
                'div[class*="captcha"], div[id*="nocaptcha"], '
                'iframe[src*="captcha"], div[class*="baxia"]'
            )
            if captcha:
                console.print(
                    "[bold yellow]⚠ 检测到验证码，请在浏览器中手动完成验证[/bold yellow]"
                )
                # 等待验证码消失
                for _ in range(120):  # 最多 2 分钟
                    await asyncio.sleep(1)
                    still_there = await self.page.query_selector(
                        'div[class*="captcha"], div[id*="nocaptcha"], '
                        'iframe[src*="captcha"], div[class*="baxia"]'
                    )
                    if not still_there:
                        console.print("[green]✓ 验证完成[/green]")
                        return
                console.print("[red]验证码处理超时[/red]")
        except Exception:
            pass

    # ── 工具 ──────────────────────────────────────────

    @staticmethod
    def _parse_sales(text: str) -> int:
        """解析销量文本，如 '1.5万+人付款' -> 15000"""
        if not text:
            return 0
        text = text.replace(",", "").replace("+", "").replace(" ", "")
        m = re.search(r"([\d.]+)\s*万", text)
        if m:
            return int(float(m.group(1)) * 10000)
        m = re.search(r"(\d+)", text)
        return int(m.group(1)) if m else 0
