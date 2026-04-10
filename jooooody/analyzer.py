"""评价分析与综合评分引擎"""

from __future__ import annotations

import math
import re
from collections import Counter

import jieba

from models import Product, Review
import config as cfg


class Analyzer:
    """多维度评价分析 + 综合评分 + 排名"""

    def __init__(self):
        self._pos_set = set(cfg.POSITIVE_WORDS)
        self._neg_set = set(cfg.NEGATIVE_WORDS)
        self._severe_set = set(cfg.SEVERE_NEGATIVE)
        self._fake_set = set(cfg.FAKE_REVIEW_SIGNALS)
        # jieba 加载自定义词
        for w in cfg.POSITIVE_WORDS + cfg.NEGATIVE_WORDS:
            jieba.add_word(w)

    # ── 主入口 ────────────────────────────────────────

    def analyze_and_rank(self, products: list[Product]) -> list[Product]:
        """分析所有商品的评价，计算综合评分并排名"""
        if not products:
            return []

        # 1. 分析每个商品的评价
        for p in products:
            self._analyze_product(p)

        # 2. 归一化各指标
        self._normalize_scores(products)

        # 3. 加权综合评分
        for p in products:
            p.composite_score = sum(
                p.score_breakdown.get(k, 0) * v
                for k, v in cfg.SCORE_WEIGHTS.items()
            )

        # 4. 排名
        products.sort(key=lambda p: p.composite_score, reverse=True)
        for i, p in enumerate(products, 1):
            p.rank = i

        # 5. 生成推荐摘要
        for p in products:
            p.summary = self._generate_summary(p)

        return products

    # ── 单商品分析 ────────────────────────────────────

    def _analyze_product(self, product: Product):
        """分析单个商品的所有评价"""
        reviews = product.reviews
        if not reviews:
            product.score_breakdown = {k: 0.0 for k in cfg.SCORE_WEIGHTS}
            return

        # 逐条评价做情感分析
        for r in reviews:
            self._analyze_review(r)

        # 统计指标
        total = len(reviews)
        positive_count = sum(1 for r in reviews if r.is_positive)
        negative_count = sum(1 for r in reviews if r.is_positive is False)
        fake_count = sum(1 for r in reviews if r.is_fake)
        image_count = sum(1 for r in reviews if r.has_image)
        long_reviews = sum(1 for r in reviews if len(r.content) >= 30)

        # 追评分析
        follow_ups = [r for r in reviews if r.follow_up]
        fu_positive = sum(
            1 for r in follow_ups
            if self._sentiment_score(r.follow_up) > 0
        )
        fu_negative = sum(
            1 for r in follow_ups
            if self._sentiment_score(r.follow_up) < 0
        )

        # 严重差评
        severe_count = sum(
            1 for r in reviews
            if any(w in r.content for w in self._severe_set)
            or (r.follow_up and any(w in r.follow_up for w in self._severe_set))
        )

        # 原始分数（未归一化）
        product.score_breakdown = {
            "price_value": product.price,  # 后续归一化时反转
            "sales": product.sales_count,
            "shop_rating": product.shop_rating or product.product_rating or 0,
            "positive_ratio": positive_count / total if total else 0,
            "review_depth": (
                (image_count / total * 0.5 + long_reviews / total * 0.5)
                if total else 0
            ),
            "follow_up_sentiment": (
                fu_positive / len(follow_ups) if follow_ups else 0.5
            ),
            "authenticity": 1 - (fake_count / total) if total else 0.5,
            "negative_severity": 1 - (severe_count / total) if total else 1.0,
        }

    def _analyze_review(self, review: Review):
        """单条评价情感分析"""
        score = self._sentiment_score(review.content)

        # 追评会修正情感判断（追评权重更高，因为更真实）
        if review.follow_up:
            fu_score = self._sentiment_score(review.follow_up)
            score = score * 0.4 + fu_score * 0.6

        review.sentiment_score = score
        review.is_positive = score > 0
        review.is_fake = self._is_fake_review(review)
        review.keywords = self._extract_keywords(review.content)

    def _sentiment_score(self, text: str) -> float:
        """计算文本情感分数 [-1.0, 1.0]"""
        if not text:
            return 0.0

        words = list(jieba.cut(text))

        pos_count = 0
        neg_count = 0
        severe_count = 0

        # 否定词检测
        negation_words = {"不", "没", "无", "非", "别", "未", "莫", "勿", "难"}
        negate = False

        for i, w in enumerate(words):
            if w in negation_words:
                negate = True
                continue

            is_pos = w in self._pos_set
            is_neg = w in self._neg_set

            if negate:
                # 否定反转
                is_pos, is_neg = is_neg, is_pos
                negate = False

            if is_pos:
                pos_count += 1
            if is_neg:
                neg_count += 1
            if w in self._severe_set:
                severe_count += 1

        # 严重差评加倍惩罚
        neg_count += severe_count * 2

        total = pos_count + neg_count
        if total == 0:
            # 没有明确情感词，根据评分和内容长度给默认值
            return 0.1  # 轻微偏正

        return (pos_count - neg_count) / total

    def _is_fake_review(self, review: Review) -> bool:
        """检测疑似刷单/虚假评价"""
        content = review.content

        # 1. 太短且无实质内容
        if len(content) <= 4:
            return True

        # 2. 包含刷单信号词
        for signal in self._fake_set:
            if signal in content:
                return True

        # 3. 纯符号或表情
        clean = re.sub(r'[^\u4e00-\u9fff\w]', '', content)
        if len(clean) < 3:
            return True

        # 4. 模板化评价（以"好评"等开头并且很短）
        if content.startswith(("好评", "满分", "五星", "还行")) and len(content) < 10:
            return True

        return False

    def _extract_keywords(self, text: str, top_n: int = 5) -> list[str]:
        """提取评价关键词"""
        import jieba.analyse
        tags = jieba.analyse.extract_tags(text, topK=top_n, withWeight=False)
        return tags

    # ── 归一化 ────────────────────────────────────────

    def _normalize_scores(self, products: list[Product]):
        """Min-Max 归一化各维度分数到 [0, 1]"""
        keys = list(cfg.SCORE_WEIGHTS.keys())

        for key in keys:
            values = [p.score_breakdown.get(key, 0) for p in products]
            vmin, vmax = min(values), max(values)

            for p in products:
                raw = p.score_breakdown.get(key, 0)
                if vmax == vmin:
                    normalized = 0.5
                else:
                    normalized = (raw - vmin) / (vmax - vmin)

                # 价格反转：越低越好
                if key == "price_value":
                    normalized = 1 - normalized

                p.score_breakdown[key] = round(normalized, 4)

    # ── 推荐摘要 ──────────────────────────────────────

    def _generate_summary(self, product: Product) -> str:
        """基于分析结果生成推荐摘要"""
        parts = []
        bd = product.score_breakdown

        # 价格评价
        if bd.get("price_value", 0) >= 0.7:
            parts.append("价格有竞争力")
        elif bd.get("price_value", 0) <= 0.3:
            parts.append("价格偏高")

        # 销量
        if bd.get("sales", 0) >= 0.7:
            parts.append("销量领先")

        # 好评
        ratio = bd.get("positive_ratio", 0)
        if ratio >= 0.8:
            parts.append(f"好评率极高({ratio:.0%})")
        elif ratio >= 0.6:
            parts.append(f"好评率较高({ratio:.0%})")
        elif ratio < 0.4 and product.reviews:
            parts.append(f"好评率偏低({ratio:.0%})")

        # 评价质量
        if bd.get("review_depth", 0) >= 0.6:
            parts.append("评价详细可信")

        # 追评
        if bd.get("follow_up_sentiment", 0) >= 0.7:
            parts.append("追评反馈良好")
        elif bd.get("follow_up_sentiment", 0) < 0.3 and product.follow_up_count > 0:
            parts.append("追评中差评较多⚠")

        # 真实度
        if bd.get("authenticity", 0) < 0.5:
            parts.append("疑似刷单较多⚠")

        # 严重差评
        if bd.get("negative_severity", 0) < 0.5:
            parts.append("存在严重质量投诉⚠")

        # 高频关键词
        all_keywords = []
        for r in product.reviews:
            all_keywords.extend(r.keywords)
        if all_keywords:
            top_kw = [
                w for w, _ in Counter(all_keywords).most_common(5)
                if len(w) >= 2
            ]
            if top_kw:
                parts.append("高频词: " + "、".join(top_kw[:3]))

        return "；".join(parts) if parts else "数据不足，暂无摘要"

    # ── 公共工具方法 ──────────────────────────────────

    def get_top_concerns(self, products: list[Product], top_n: int = 10) -> list[str]:
        """从所有差评中提取最常见的问题"""
        concerns = Counter()
        for p in products:
            for r in p.reviews:
                if r.is_positive is False:
                    words = list(jieba.cut(r.content))
                    for w in words:
                        if w in self._neg_set or w in self._severe_set:
                            concerns[w] += 1
                    if r.follow_up:
                        for w in jieba.cut(r.follow_up):
                            if w in self._neg_set or w in self._severe_set:
                                concerns[w] += 1
        return [w for w, _ in concerns.most_common(top_n)]

    def get_top_praises(self, products: list[Product], top_n: int = 10) -> list[str]:
        """从好评中提取最常见的优点"""
        praises = Counter()
        for p in products:
            for r in p.reviews:
                if r.is_positive:
                    for w in jieba.cut(r.content):
                        if w in self._pos_set:
                            praises[w] += 1
        return [w for w, _ in praises.most_common(top_n)]
