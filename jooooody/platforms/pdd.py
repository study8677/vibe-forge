"""拼多多爬虫（移动端 H5）"""

from __future__ import annotations

import asyncio
import json
import math
import re
from typing import Optional

from playwright.async_api import Page, Response
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn

from browser import BrowserManager, random_sleep, smooth_scroll, wait_for_login
from models import Product, Review
from platforms.base import BaseScraper
import config as cfg

console = Console()


class PddScraper(BaseScraper):
    platform_name = "pdd"

    BASE_URL = "https://mobile.yangkeduo.com"
    SEARCH_URL = f"{BASE_URL}/search_result.html"
    LOGIN_URL = f"{BASE_URL}/login.html"

    def __init__(self, bm: BrowserManager):
        super().__init__(bm)
        self._intercepted_reviews: list[dict] = []
        self._intercepted_products: list[dict] = []

    # ── 登录 ──────────────────────────────────────────

    async def ensure_login(self) -> bool:
        self.page = await self.bm.new_page("pdd", mobile=True)

        await self.page.goto(self.BASE_URL, wait_until="domcontentloaded")
        await random_sleep(2, 3)

        logged_in = await self._check_login()
        if logged_in:
            console.print("[green]✓ 拼多多已登录[/green]")
            return True

        await self.page.goto(self.LOGIN_URL, wait_until="domcontentloaded")
        success = await wait_for_login(
            self.page,
            'div[class*="user-avatar"], div[class*="personal"], '
            'a[class*="mine"], div[class*="nickname"]',
            "拼多多",
        )
        if success:
            await self.bm.save_cookies("pdd")
        return success

    async def _check_login(self) -> bool:
        try:
            cookies = await self.page.context.cookies()
            for c in cookies:
                if c["name"] in ("PDDAccessToken", "pdd_user_id") and c["value"]:
                    return True
            # 检查页面上是否有登录态标识
            el = await self.page.query_selector(
                'div[class*="user-avatar"], div[class*="nickname"]'
            )
            if el:
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
        self._intercepted_products.clear()

        # 注册网络拦截
        self.page.on("response", self._on_search_response)

        with Progress(
            SpinnerColumn(),
            TextColumn("[bold magenta]拼多多搜索"),
            BarColumn(),
            TextColumn("{task.completed}/{task.total}"),
            console=console,
        ) as progress:
            task = progress.add_task("搜索中", total=limit)

            try:
                # 构造搜索 URL
                url = f"{self.SEARCH_URL}?search_key={keyword}&search_met=search_hot_word"
                await self.page.goto(url, wait_until="domcontentloaded")
                await random_sleep(2, 3)

                # 处理弹窗
                await self._dismiss_popups()

                # 滚动加载更多商品
                last_count = 0
                stale_rounds = 0

                while len(products) < limit and stale_rounds < 5:
                    # 从 API 拦截获取
                    if self._intercepted_products:
                        for item in self._intercepted_products:
                            p = self._parse_product_data(item)
                            if p and min_price <= p.price <= max_price:
                                # 去重
                                if not any(
                                    ep.product_id == p.product_id for ep in products
                                ):
                                    products.append(p)
                                    progress.update(task, completed=len(products))
                                    if len(products) >= limit:
                                        break
                        self._intercepted_products.clear()

                    # 如果 API 拦截没有数据，尝试 DOM 提取
                    if not products or len(products) == last_count:
                        dom_products = await self._extract_from_dom()
                        for p in dom_products:
                            if min_price <= p.price <= max_price:
                                if not any(
                                    ep.product_id == p.product_id for ep in products
                                ):
                                    products.append(p)
                                    progress.update(task, completed=len(products))
                                    if len(products) >= limit:
                                        break

                    if len(products) == last_count:
                        stale_rounds += 1
                    else:
                        stale_rounds = 0
                    last_count = len(products)

                    # 继续滚动
                    await smooth_scroll(self.page, 1000)
                    await random_sleep(*cfg.SCROLL_PAUSE)

            except Exception as e:
                console.print(f"[yellow]拼多多搜索异常: {e}[/yellow]")
            finally:
                try:
                    self.page.remove_listener("response", self._on_search_response)
                except Exception:
                    pass

        console.print(f"[green]✓ 拼多多共采集 {len(products)} 个商品[/green]")
        return products

    async def _on_search_response(self, response: Response):
        """拦截搜索 API 响应"""
        url = response.url
        if not any(kw in url for kw in (
            "search", "proxy/api", "goods_list", "recommend",
        )):
            return
        if response.status != 200:
            return

        try:
            text = await response.text()
            data = json.loads(text)

            # 搜索结果中的商品列表
            items = (
                data.get("goods_list", [])
                or data.get("data", {}).get("goods_list", [])
                or data.get("data", {}).get("list", [])
                or data.get("items", [])
                or data.get("result", {}).get("goods_list", [])
            )
            if items:
                self._intercepted_products.extend(items)
        except Exception:
            pass

    def _parse_product_data(self, data: dict) -> Optional[Product]:
        """解析 API 返回的商品数据"""
        try:
            goods_id = str(
                data.get("goods_id", "")
                or data.get("goodsId", "")
                or data.get("id", "")
            )
            if not goods_id:
                return None

            title = data.get("goods_name", "") or data.get("goodsName", "") or data.get("title", "")
            price_cents = data.get("min_group_price", 0) or data.get("price", 0) or data.get("group_price", 0)

            # 拼多多价格可能是分为单位
            price = price_cents
            if isinstance(price, int) and price > 10000:
                price = price / 100  # 分转元

            sales_text = str(data.get("sales_tip", "") or data.get("salesTip", ""))
            sales = self._parse_sales(sales_text)

            img = data.get("hd_thumb_url", "") or data.get("thumb_url", "") or data.get("image_url", "")
            if img and not img.startswith("http"):
                img = "https:" + img

            return Product(
                platform="pdd",
                product_id=goods_id,
                title=title,
                price=float(price),
                sales_count=sales,
                sales_text=sales_text,
                shop_name=data.get("mall_name", "") or data.get("shopName", ""),
                url=f"{self.BASE_URL}/goods.html?goods_id={goods_id}",
                image_url=img,
            )
        except (ValueError, TypeError):
            return None

    async def _extract_from_dom(self) -> list[Product]:
        """从 DOM 提取商品"""
        products = []
        try:
            data = await self.page.evaluate("""() => {
                const results = [];
                // PDD 商品卡片
                const cards = document.querySelectorAll(
                    'a[href*="goods.html"], a[href*="goods_id"], '
                    + 'div[class*="goods-item"], div[class*="product-item"]'
                );

                for (const card of cards) {
                    let href = '';
                    if (card.tagName === 'A') {
                        href = card.href || card.getAttribute('href') || '';
                    } else {
                        const link = card.querySelector('a[href*="goods"]');
                        if (link) href = link.href || link.getAttribute('href') || '';
                    }

                    const idMatch = href.match(/goods_id=(\\d+)/);
                    if (!idMatch) continue;

                    const text = card.innerText || '';

                    // 价格
                    let price = 0;
                    const priceMatch = text.match(/[¥￥]\\s*(\\d+\\.?\\d*)/);
                    if (priceMatch) price = parseFloat(priceMatch[1]);

                    // 销量
                    let salesText = '';
                    const salesMatch = text.match(/(\\d[\\d.]*万?\\+?)\\s*件?(已拼|已售|人买|人拼)/);
                    if (salesMatch) salesText = salesMatch[0];
                    if (!salesText) {
                        const sm2 = text.match(/已拼(\\d[\\d.]*万?\\+?)件?/);
                        if (sm2) salesText = sm2[0];
                    }

                    // 标题
                    let title = '';
                    const lines = text.split('\\n').map(l => l.trim()).filter(Boolean);
                    for (const l of lines) {
                        if (l.length > title.length && !l.includes('¥') &&
                            !l.includes('已拼') && !l.includes('万+')) {
                            title = l;
                        }
                    }

                    // 图片
                    const img = card.querySelector('img');
                    const imgSrc = img ? (img.src || '') : '';

                    // 店铺名
                    let shopName = '';
                    const shopEl = card.querySelector(
                        'span[class*="shop"], span[class*="mall"]'
                    );
                    if (shopEl) shopName = shopEl.innerText.trim();

                    results.push({
                        id: idMatch[1],
                        title: title.slice(0, 200),
                        price,
                        salesText,
                        shopName,
                        imgSrc,
                    });
                }
                return results;
            }""")

            seen = set()
            for item in data:
                pid = item["id"]
                if pid in seen:
                    continue
                seen.add(pid)

                products.append(Product(
                    platform="pdd",
                    product_id=pid,
                    title=item["title"],
                    price=item["price"],
                    sales_count=self._parse_sales(item["salesText"]),
                    sales_text=item["salesText"],
                    shop_name=item.get("shopName", ""),
                    url=f"{self.BASE_URL}/goods.html?goods_id={pid}",
                    image_url=item.get("imgSrc", ""),
                ))

        except Exception as e:
            console.print(f"[dim]PDD DOM 解析: {e}[/dim]")

        return products

    # ── 评价采集 ───────────────────────────────────────

    async def get_reviews(self, product: Product, limit: int = 100) -> list[Review]:
        reviews: list[Review] = []
        self._intercepted_reviews.clear()

        try:
            self.page.on("response", self._on_review_response)

            await self.page.goto(product.url, wait_until="domcontentloaded")
            await random_sleep(2, 4)

            # 关闭弹窗
            await self._dismiss_popups()

            # 提取商品元信息
            await self._extract_product_meta(product)

            # 滚动到评价区域 & 点击评价 Tab
            await self._scroll_to_reviews()
            await random_sleep(1, 2)

            # 采集评价
            max_pages = math.ceil(limit / cfg.REVIEWS_PAGE_SIZE) + 1
            page_num = 1

            while len(reviews) < limit and page_num <= max_pages:
                await random_sleep(1, 2)

                # 从 API 拦截获取
                if self._intercepted_reviews:
                    for rd in self._intercepted_reviews:
                        if len(reviews) >= limit:
                            break
                        r = self._parse_review_data(rd)
                        if r.content and not any(
                            e.content == r.content for e in reviews
                        ):
                            reviews.append(r)
                    self._intercepted_reviews.clear()
                else:
                    # DOM 兜底
                    dom_reviews = await self._extract_reviews_from_dom()
                    for r in dom_reviews:
                        if len(reviews) >= limit:
                            break
                        if not any(e.content == r.content for e in reviews):
                            reviews.append(r)

                # 滚动加载更多
                await smooth_scroll(self.page, 800)
                await random_sleep(*cfg.SCROLL_PAUSE)

                # 尝试点击"查看更多评价"
                more = await self.page.query_selector(
                    'div:has-text("查看更多评价"), button:has-text("查看更多"), '
                    'span:has-text("更多评价")'
                )
                if more:
                    try:
                        await more.click()
                        await random_sleep(1, 2)
                    except Exception:
                        pass

                page_num += 1

        except Exception as e:
            console.print(
                f"[yellow]PDD 评价采集异常 [{product.product_id}]: {e}[/yellow]"
            )
        finally:
            try:
                self.page.remove_listener("response", self._on_review_response)
            except Exception:
                pass

        product.reviews = reviews
        return reviews

    async def _on_review_response(self, response: Response):
        """拦截评价 API"""
        url = response.url
        if not any(kw in url for kw in (
            "review", "comment", "evaluation", "rate",
            "proxy/api",
        )):
            return
        if "goods_id" not in url and "goodsId" not in url:
            return
        if response.status != 200:
            return

        try:
            text = await response.text()
            data = json.loads(text)

            comments = (
                data.get("data", {}).get("list", [])
                or data.get("data", {}).get("comments", [])
                or data.get("comments", [])
                or data.get("result", {}).get("list", [])
                or data.get("list", [])
            )
            if comments:
                self._intercepted_reviews.extend(comments)
        except Exception:
            pass

    def _parse_review_data(self, data: dict) -> Review:
        """解析评价数据"""
        content = (
            data.get("comment", "")
            or data.get("content", "")
            or data.get("text", "")
            or ""
        )

        # 追评
        follow_up = ""
        follow_up_days = 0
        append_info = data.get("append", {}) or data.get("appendComment", {})
        if isinstance(append_info, dict) and append_info:
            follow_up = append_info.get("comment", "") or append_info.get("content", "") or ""
            follow_up_days = append_info.get("days", 0) or 0
        elif isinstance(append_info, str):
            follow_up = append_info

        # 图片
        pics = data.get("pictures", []) or data.get("pics", []) or data.get("images", []) or []

        # 评分
        star = data.get("star", 5) or data.get("rating", 5) or 5

        return Review(
            content=content.strip(),
            rating=star,
            date=data.get("time", "") or data.get("date", "") or data.get("created_at", ""),
            user=data.get("nick_name", "") or data.get("user_name", "") or "",
            sku=data.get("spec", "") or data.get("sku", "") or "",
            has_image=len(pics) > 0,
            image_count=len(pics),
            follow_up=follow_up.strip(),
            follow_up_days=follow_up_days,
        )

    async def _extract_reviews_from_dom(self) -> list[Review]:
        """DOM 评价提取"""
        reviews = []
        try:
            data = await self.page.evaluate("""() => {
                const reviews = [];
                const cards = document.querySelectorAll(
                    'div[class*="review"], div[class*="comment"], '
                    + 'div[class*="evaluate"]'
                );

                for (const card of cards) {
                    const contentEl = card.querySelector(
                        'p, span[class*="content"], div[class*="text"]'
                    );
                    if (!contentEl) continue;
                    const text = contentEl.innerText.trim();
                    if (!text || text.length < 2) continue;

                    let followUp = '';
                    const appendEl = card.querySelector(
                        'div[class*="append"], div[class*="追评"]'
                    );
                    if (appendEl) followUp = appendEl.innerText.trim();

                    const dateEl = card.querySelector(
                        'span[class*="time"], span[class*="date"]'
                    );
                    const imgs = card.querySelectorAll('img[class*="pic"], img[class*="photo"]');

                    reviews.push({
                        content: text,
                        date: dateEl ? dateEl.innerText.trim() : '',
                        followUp,
                        imageCount: imgs.length,
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
                ))
        except Exception:
            pass
        return reviews

    async def _scroll_to_reviews(self):
        """滚动到评价区域并点击评价 Tab"""
        try:
            selectors = [
                'div:has-text("商品评价")',
                'span:has-text("评价")',
                'div[class*="review-tab"]',
                'a:has-text("评价")',
            ]
            for sel in selectors:
                try:
                    el = await self.page.wait_for_selector(sel, timeout=3000)
                    if el:
                        await el.scroll_into_view_if_needed()
                        await el.click()
                        return
                except Exception:
                    continue

            # 直接滚动到底部附近
            await smooth_scroll(self.page, 3000, steps=15)
        except Exception:
            pass

    async def _extract_product_meta(self, product: Product):
        """提取商品详情页的元信息"""
        try:
            meta = await self.page.evaluate("""() => {
                const text = document.body.innerText;

                let reviewCount = 0;
                const rcMatch = text.match(/(\\d[\\d.]*万?\\+?)\\s*条?评价/);
                if (rcMatch) {
                    let s = rcMatch[1].replace('+', '');
                    reviewCount = s.includes('万') ? parseFloat(s) * 10000 : parseInt(s);
                }

                let salesText = '';
                const salesMatch = text.match(/已拼(\\d[\\d.]*万?\\+?)件?/);
                if (salesMatch) salesText = salesMatch[0];

                return { reviewCount, salesText };
            }""")

            if meta.get("reviewCount"):
                product.review_count = meta["reviewCount"]
            if meta.get("salesText") and not product.sales_text:
                product.sales_text = meta["salesText"]
                product.sales_count = self._parse_sales(meta["salesText"])
        except Exception:
            pass

    async def _dismiss_popups(self):
        """关闭拼多多的各种弹窗"""
        try:
            popup_selectors = [
                'div[class*="close"]',
                'button[class*="close"]',
                'div[class*="mask"]',
                'div:has-text("我知道了")',
                'span:has-text("关闭")',
                'div:has-text("暂不需要")',
            ]
            for sel in popup_selectors:
                try:
                    el = await self.page.query_selector(sel)
                    if el and await el.is_visible():
                        await el.click()
                        await asyncio.sleep(0.3)
                except Exception:
                    continue
        except Exception:
            pass

    @staticmethod
    def _parse_sales(text: str) -> int:
        if not text:
            return 0
        text = text.replace(",", "").replace("+", "").replace(" ", "")
        m = re.search(r"([\d.]+)\s*万", text)
        if m:
            return int(float(m.group(1)) * 10000)
        m = re.search(r"(\d+)", text)
        return int(m.group(1)) if m else 0
