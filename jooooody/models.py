"""数据模型"""

from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Optional
from pathlib import Path


@dataclass
class Review:
    """一条评价"""
    content: str  # 评价正文
    rating: int = 5  # 1-5 星
    date: str = ""  # 评价日期
    user: str = ""  # 用户名（脱敏）
    sku: str = ""  # 购买的 SKU 规格
    has_image: bool = False  # 是否带图
    image_count: int = 0  # 图片数量
    follow_up: str = ""  # 追评内容
    follow_up_days: int = 0  # 追评距初评天数
    # 分析结果
    sentiment_score: float = 0.0  # -1.0 ~ 1.0
    is_positive: Optional[bool] = None
    is_fake: bool = False  # 疑似刷单
    keywords: list[str] = field(default_factory=list)  # 提取的关键词


@dataclass
class Product:
    """一个商品"""
    platform: str  # "taobao" | "pdd"
    product_id: str
    title: str
    price: float
    original_price: float = 0.0
    sales_count: int = 0  # 销量/已拼件数
    sales_text: str = ""  # 原始销量文本 "1万+" 等
    shop_name: str = ""
    shop_rating: float = 0.0  # 店铺评分
    product_rating: float = 0.0  # 商品评分
    review_count: int = 0  # 评价总数
    url: str = ""
    image_url: str = ""
    reviews: list[Review] = field(default_factory=list)
    # 分析结果
    composite_score: float = 0.0
    rank: int = 0
    score_breakdown: dict = field(default_factory=dict)
    summary: str = ""  # AI 生成的推荐摘要

    @property
    def positive_review_ratio(self) -> float:
        if not self.reviews:
            return 0.0
        positive = sum(1 for r in self.reviews if r.is_positive)
        return positive / len(self.reviews)

    @property
    def avg_sentiment(self) -> float:
        if not self.reviews:
            return 0.0
        return sum(r.sentiment_score for r in self.reviews) / len(self.reviews)

    @property
    def follow_up_count(self) -> int:
        return sum(1 for r in self.reviews if r.follow_up)

    @property
    def image_review_ratio(self) -> float:
        if not self.reviews:
            return 0.0
        return sum(1 for r in self.reviews if r.has_image) / len(self.reviews)


@dataclass
class SearchResult:
    """一次搜索的完整结果"""
    keyword: str
    min_price: float
    max_price: float
    products: list[Product] = field(default_factory=list)
    timestamp: str = ""

    def save(self, path: Path):
        path.write_text(
            json.dumps(asdict(self), ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    @classmethod
    def load(cls, path: Path) -> "SearchResult":
        data = json.loads(path.read_text(encoding="utf-8"))
        products = []
        for p in data.pop("products", []):
            reviews = [Review(**r) for r in p.pop("reviews", [])]
            products.append(Product(**p, reviews=reviews))
        return cls(**data, products=products)
