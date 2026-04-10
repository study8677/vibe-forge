"""平台爬虫基类"""

from __future__ import annotations

import abc
from typing import Optional

from browser import BrowserManager
from models import Product, Review


class BaseScraper(abc.ABC):
    """所有平台爬虫的抽象基类"""

    platform_name: str = ""

    def __init__(self, browser_manager: BrowserManager):
        self.bm = browser_manager
        self.page = None

    @abc.abstractmethod
    async def ensure_login(self) -> bool:
        """确保已登录，未登录则引导用户登录"""
        ...

    @abc.abstractmethod
    async def search_products(
        self,
        keyword: str,
        min_price: float,
        max_price: float,
        limit: int = 100,
    ) -> list[Product]:
        """搜索商品，返回 Product 列表（不含评价）"""
        ...

    @abc.abstractmethod
    async def get_reviews(
        self, product: Product, limit: int = 100
    ) -> list[Review]:
        """采集指定商品的评价（含追评）"""
        ...

    async def close(self):
        if self.page:
            await self.page.close()
            self.page = None
