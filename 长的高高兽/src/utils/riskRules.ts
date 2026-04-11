import type { RiskItem, RiskLevel, ListingData } from '../types';

// ── 风险词库 ──────────────────────────────────────────────

const TRADEMARK_RISK_WORDS = [
  'iphone', 'samsung', 'nike', 'adidas', 'gucci', 'louis vuitton', 'prada',
  'chanel', 'hermes', 'rolex', 'cartier', 'tiffany', 'burberry', 'dior',
  'versace', 'armani', 'balenciaga', 'fendi', 'apple', 'google', 'microsoft',
  'disney', 'marvel', 'dc comics', 'pokemon', 'hello kitty', 'peppa pig',
  'lego', 'barbie', 'hot wheels', 'playstation', 'xbox', 'nintendo',
  'north face', 'patagonia', 'under armour', 'new balance', 'converse',
  'ray-ban', 'oakley', 'pandora', 'swarovski', 'beats', 'bose', 'dyson',
];

const PROHIBITED_TITLE_WORDS = [
  'best seller', 'hot sale', 'free shipping', 'limited time', 'act now',
  'sale', 'discount', 'cheap', 'lowest price', 'clearance', 'promotion',
  'bestseller', 'top rated', '#1', 'number one', 'best quality',
  'cure', 'heal', 'treat', 'prevent disease', 'therapy', 'medical grade',
  'fda approved', 'clinically proven',
  'best', 'perfect', 'amazing', 'incredible', 'unbeatable',
  'guaranteed', 'risk free', 'money back',
];

const PROHIBITED_TITLE_WORDS_CN = [
  '最好', '最佳', '第一', '顶级', '免费', '打折', '促销', '清仓',
  '限时', '秒杀', '爆款', '治疗', '治愈', '医疗级', '包邮',
];

const RESTRICTED_CATEGORIES = [
  'Jewelry', 'Watches', 'Automotive', 'Collectible Coins',
  'Fine Art', 'Grocery & Gourmet Food', 'Health & Personal Care',
  'Industrial & Scientific', 'Music & DVD', 'Sexual Wellness',
  'Sports Collectibles', 'Toys & Games', 'Wine', 'Made in Italy',
  'Pesticides', 'Dietary Supplements', 'OTC Medications',
];

const TITLE_BANNED_CHARS = /[★☆♥♦♣♠●◆■□▲△▼▽◇◈※¤@#$%^&*{}[\]<>~`|\\]/;

// ── 辅助 ──────────────────────────────────────────────────

let _counter = 0;
function rid(): string {
  return `r${Date.now().toString(36)}${(++_counter).toString(36)}`;
}

// ── 规则检测函数 ──────────────────────────────────────────

function checkTitle(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const title = (listing.title ?? '').trim();
  const lower = title.toLowerCase();

  if (!title) {
    risks.push({
      ruleId: rid(), ruleName: '标题缺失', level: 'high',
      message: '商品标题为空',
      suggestion: '请填写符合亚马逊规范的商品标题',
      field: 'title',
    });
    return risks;
  }

  if (title.length > 200) {
    risks.push({
      ruleId: rid(), ruleName: '标题超长', level: 'medium',
      message: `标题 ${title.length} 字符，超过 200 字符上限`,
      suggestion: '精简标题：品牌 + 核心产品词 + 关键属性',
      field: 'title',
    });
  } else if (title.length < 20) {
    risks.push({
      ruleId: rid(), ruleName: '标题过短', level: 'low',
      message: `标题仅 ${title.length} 字符，搜索权重可能偏低`,
      suggestion: '补充关键属性词，如材质、尺寸、颜色、用途',
      field: 'title',
    });
  }

  // 全大写检测
  const words = lower.split(/\s+/);
  const capsWords = title
    .split(/\s+/)
    .filter((w) => w.length > 2 && w === w.toUpperCase() && /[A-Z]/.test(w));
  if (capsWords.length > words.length * 0.5 && words.length > 3) {
    risks.push({
      ruleId: rid(), ruleName: '标题全大写', level: 'medium',
      message: '大量全大写单词，违反亚马逊标题规范',
      suggestion: '使用 Title Case，仅品牌名或缩写全大写',
      field: 'title',
    });
  }

  // 特殊字符
  if (TITLE_BANNED_CHARS.test(title)) {
    risks.push({
      ruleId: rid(), ruleName: '标题含特殊字符', level: 'medium',
      message: '包含亚马逊禁止的特殊符号',
      suggestion: '仅保留字母、数字、连字符和逗号',
      field: 'title',
    });
  }

  // 英文违规词
  for (const word of PROHIBITED_TITLE_WORDS) {
    if (lower.includes(word)) {
      risks.push({
        ruleId: rid(), ruleName: '标题违规词', level: 'high',
        message: `包含违规词 "${word}"，可能导致下架`,
        suggestion: `移除 "${word}"，使用客观产品描述替代`,
        field: 'title',
      });
      break; // 只报第一个高风险，后续中风险继续
    }
  }

  // 中文违规词
  for (const word of PROHIBITED_TITLE_WORDS_CN) {
    if (lower.includes(word)) {
      risks.push({
        ruleId: rid(), ruleName: '标题中文违规词', level: 'high',
        message: `包含中文违规词 "${word}"`,
        suggestion: `移除 "${word}"`,
        field: 'title',
      });
      break;
    }
  }

  // 中文字符（非日本站）
  if (/[\u4e00-\u9fff]/.test(title)) {
    risks.push({
      ruleId: rid(), ruleName: '标题含中文', level: 'medium',
      message: '标题含中文字符，北美/欧洲站违规',
      suggestion: '如非日本站，改为纯英文标题',
      field: 'title',
    });
  }

  // 重复词检测
  const wordFreq: Record<string, number> = {};
  for (const w of words) {
    if (w.length > 3) wordFreq[w] = (wordFreq[w] || 0) + 1;
  }
  const repeated = Object.entries(wordFreq).filter(([, c]) => c >= 3);
  if (repeated.length > 0) {
    risks.push({
      ruleId: rid(), ruleName: '标题关键词堆砌', level: 'medium',
      message: `关键词重复过多: ${repeated.map(([w, c]) => `"${w}"×${c}`).join('、')}`,
      suggestion: '每个关键词出现一次即可，堆砌会被亚马逊降权',
      field: 'title',
    });
  }

  return risks;
}

function checkBrand(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const brand = (listing.brand ?? '').trim().toLowerCase();
  const title = (listing.title ?? '').toLowerCase();

  if (!brand) {
    risks.push({
      ruleId: rid(), ruleName: '品牌缺失', level: 'medium',
      message: '未填写品牌名称',
      suggestion: '填写自有品牌名，或使用 "Generic" / "Unbranded"',
      field: 'brand',
    });
    return risks;
  }

  for (const tm of TRADEMARK_RISK_WORDS) {
    if (brand.includes(tm) || title.includes(`for ${tm}`) === false && title.includes(tm)) {
      // 如果品牌字段直接包含知名商标
      if (brand.includes(tm)) {
        risks.push({
          ruleId: rid(), ruleName: '品牌侵权风险', level: 'high',
          message: `品牌字段含已注册商标 "${tm}"，存在侵权下架风险`,
          suggestion: '确认已获品牌授权书，否则立即更换',
          field: 'brand',
        });
        break;
      }
    }
  }

  // 标题中不含品牌名
  if (brand && brand !== 'generic' && brand !== 'unbranded' && !title.includes(brand)) {
    risks.push({
      ruleId: rid(), ruleName: '标题缺品牌名', level: 'low',
      message: '标题未包含品牌名，不利于品牌搜索曝光',
      suggestion: '在标题开头添加品牌名',
      field: 'title',
    });
  }

  return risks;
}

function checkPrice(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const price = listing.price ?? 0;

  if (price <= 0) {
    risks.push({
      ruleId: rid(), ruleName: '价格异常', level: 'high',
      message: '售价为 0 或负数',
      suggestion: '设置合理售价',
      field: 'price',
    });
    return risks;
  }

  if (price < 3) {
    risks.push({
      ruleId: rid(), ruleName: '价格过低', level: 'high',
      message: `售价 $${price.toFixed(2)}，扣除佣金+FBA费后几乎必亏`,
      suggestion: '建议最低售价 $5 以上，计算盈亏平衡点',
      field: 'price',
    });
  } else if (price < 8) {
    risks.push({
      ruleId: rid(), ruleName: '价格偏低', level: 'medium',
      message: `售价 $${price.toFixed(2)}，利润空间有限`,
      suggestion: '核实 FBA 费用和佣金后确认可盈利',
      field: 'price',
    });
  }

  if (price > 999) {
    risks.push({
      ruleId: rid(), ruleName: '高客单价', level: 'low',
      message: `售价 $${price.toFixed(2)}，退货率和广告成本较高`,
      suggestion: '建议配置退货保障政策和高质量图文',
      field: 'price',
    });
  }

  return risks;
}

function checkCategory(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const category = (listing.category ?? '').trim();

  if (!category) {
    risks.push({
      ruleId: rid(), ruleName: '类目缺失', level: 'medium',
      message: '未指定商品类目',
      suggestion: '选择正确类目，影响搜索排名与佣金比例',
      field: 'category',
    });
    return risks;
  }

  for (const rc of RESTRICTED_CATEGORIES) {
    if (category.toLowerCase().includes(rc.toLowerCase())) {
      risks.push({
        ruleId: rid(), ruleName: '受限类目', level: 'medium',
        message: `"${rc}" 为亚马逊受限类目，需提前申请审批`,
        suggestion: '确认已获得类目销售许可',
        field: 'category',
      });
      break;
    }
  }

  return risks;
}

function checkBulletPoints(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const bullets = (listing.bulletPoints ?? []).filter(Boolean);

  if (bullets.length === 0) {
    risks.push({
      ruleId: rid(), ruleName: '五点描述缺失', level: 'medium',
      message: '未填写 Bullet Points',
      suggestion: '添加 5 条卖点描述，每条 200 字符内',
      field: 'bulletPoints',
    });
    return risks;
  }

  if (bullets.length < 3) {
    risks.push({
      ruleId: rid(), ruleName: '五点描述不足', level: 'low',
      message: `仅填写 ${bullets.length} 条 Bullet Point`,
      suggestion: '建议填满 5 条以提升转化率',
      field: 'bulletPoints',
    });
  }

  bullets.forEach((bp, i) => {
    if (bp.length > 500) {
      risks.push({
        ruleId: rid(), ruleName: `第${i + 1}条过长`, level: 'low',
        message: `第 ${i + 1} 条 ${bp.length} 字符，移动端显示截断`,
        suggestion: '控制在 200 字符以内',
        field: 'bulletPoints',
      });
    }
    if (/<[^>]+>/.test(bp)) {
      risks.push({
        ruleId: rid(), ruleName: `第${i + 1}条含HTML`, level: 'medium',
        message: `第 ${i + 1} 条包含 HTML 标签`,
        suggestion: '移除 HTML，亚马逊不渲染且可能导致显示异常',
        field: 'bulletPoints',
      });
    }
  });

  return risks;
}

function checkSearchTerms(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const terms = (listing.searchTerms ?? '').trim();

  if (!terms) {
    risks.push({
      ruleId: rid(), ruleName: '搜索词缺失', level: 'medium',
      message: '未填写后台搜索关键词',
      suggestion: '添加长尾词提升曝光',
      field: 'searchTerms',
    });
    return risks;
  }

  const byteLen = new TextEncoder().encode(terms).length;
  if (byteLen > 250) {
    risks.push({
      ruleId: rid(), ruleName: '搜索词超限', level: 'medium',
      message: `搜索词 ${byteLen} 字节，超过 250 字节限制`,
      suggestion: '精简至 250 字节，超出部分被忽略',
      field: 'searchTerms',
    });
  }

  if (/B0[A-Z0-9]{8,}/i.test(terms)) {
    risks.push({
      ruleId: rid(), ruleName: '搜索词含ASIN', level: 'high',
      message: '搜索词包含 ASIN 编号，严重违规',
      suggestion: '移除 ASIN，使用自然关键词',
      field: 'searchTerms',
    });
  }

  const lower = terms.toLowerCase();
  for (const tm of TRADEMARK_RISK_WORDS) {
    if (lower.includes(tm)) {
      risks.push({
        ruleId: rid(), ruleName: '搜索词侵权', level: 'high',
        message: `搜索词含品牌词 "${tm}"，可能被投诉`,
        suggestion: `移除 "${tm}"，用通用产品词替代`,
        field: 'searchTerms',
      });
      break;
    }
  }

  return risks;
}

function checkDescription(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const desc = (listing.description ?? '').trim();

  if (!desc) {
    risks.push({
      ruleId: rid(), ruleName: '描述缺失', level: 'low',
      message: '未填写商品描述',
      suggestion: '补充描述有助于 SEO 和转化',
      field: 'description',
    });
    return risks;
  }

  if (/(\b\d{3}[-.]?\d{3}[-.]?\d{4}\b|[\w.-]+@[\w.-]+\.\w{2,}|https?:\/\/(?!www\.amazon\.com))/i.test(desc)) {
    risks.push({
      ruleId: rid(), ruleName: '描述含联系方式', level: 'high',
      message: '描述中包含电话/邮箱/外链',
      suggestion: '移除所有联系方式和非亚马逊链接',
      field: 'description',
    });
  }

  if (/<script|javascript:|onclick|onerror/i.test(desc)) {
    risks.push({
      ruleId: rid(), ruleName: '描述含脚本', level: 'high',
      message: '描述中检测到脚本代码',
      suggestion: '移除所有脚本内容',
      field: 'description',
    });
  }

  return risks;
}

function checkSKU(listing: Partial<ListingData>): RiskItem[] {
  const risks: RiskItem[] = [];
  const sku = (listing.sku ?? '').trim();

  if (!sku) {
    risks.push({
      ruleId: rid(), ruleName: 'SKU缺失', level: 'medium',
      message: '未填写 SKU',
      suggestion: '设置唯一 SKU 便于库存管理',
      field: 'sku',
    });
  } else if (sku.length > 40) {
    risks.push({
      ruleId: rid(), ruleName: 'SKU超长', level: 'low',
      message: `SKU ${sku.length} 字符，超过 40 字符限制`,
      suggestion: '精简 SKU 至 40 字符内',
      field: 'sku',
    });
  }

  return risks;
}

// ── 主入口 ────────────────────────────────────────────────

/** 将一行原始数据解析为带风险评估的 ListingData */
export function analyzeListing(raw: Record<string, string>): ListingData {
  const listing: Partial<ListingData> = {
    id: rid(),
    sku: raw['SKU'] || raw['sku'] || raw['商品SKU'] || '',
    title:
      raw['Title'] || raw['title'] || raw['商品标题'] || raw['Product Name'] || '',
    brand: raw['Brand'] || raw['brand'] || raw['品牌'] || '',
    category:
      raw['Category'] || raw['category'] || raw['类目'] || raw['Product Type'] || '',
    price:
      parseFloat(raw['Price'] || raw['price'] || raw['售价'] || '0') || 0,
    currency: raw['Currency'] || raw['currency'] || 'USD',
    bulletPoints: [
      raw['Bullet Point 1'] || raw['bullet_point_1'] || raw['卖点1'],
      raw['Bullet Point 2'] || raw['bullet_point_2'] || raw['卖点2'],
      raw['Bullet Point 3'] || raw['bullet_point_3'] || raw['卖点3'],
      raw['Bullet Point 4'] || raw['bullet_point_4'] || raw['卖点4'],
      raw['Bullet Point 5'] || raw['bullet_point_5'] || raw['卖点5'],
    ].filter(Boolean) as string[],
    searchTerms:
      raw['Search Terms'] || raw['search_terms'] || raw['搜索词'] || '',
    asin: raw['ASIN'] || raw['asin'] || '',
    imageUrls: (raw['Image URLs'] || raw['image_urls'] || raw['图片链接'] || '')
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean),
    description:
      raw['Description'] || raw['description'] || raw['商品描述'] || '',
    reviewStatus: 'pending' as const,
  };

  const risks: RiskItem[] = [
    ...checkTitle(listing),
    ...checkBrand(listing),
    ...checkPrice(listing),
    ...checkCategory(listing),
    ...checkBulletPoints(listing),
    ...checkSearchTerms(listing),
    ...checkDescription(listing),
    ...checkSKU(listing),
  ];

  const highC = risks.filter((r) => r.level === 'high').length;
  const medC = risks.filter((r) => r.level === 'medium').length;
  const lowC = risks.filter((r) => r.level === 'low').length;
  const score = Math.min(highC * 30 + medC * 15 + lowC * 5, 100);

  let overallRisk: RiskLevel = 'pass';
  if (highC > 0) overallRisk = 'high';
  else if (medC > 0) overallRisk = 'medium';
  else if (lowC > 0) overallRisk = 'low';

  return { ...listing, risks, riskScore: score, overallRisk } as ListingData;
}

// ── 工具函数 ──────────────────────────────────────────────

export function getRiskColor(level: RiskLevel): string {
  return { high: '#ff4d4f', medium: '#faad14', low: '#1677ff', pass: '#52c41a' }[level];
}

export function getRiskLabel(level: RiskLevel): string {
  return { high: '高风险', medium: '中风险', low: '低风险', pass: '通过' }[level];
}

export function getStatusLabel(s: ListingData['reviewStatus']): string {
  return { pending: '待审核', reviewed: '已审核', approved: '已通过', rejected: '已驳回' }[s];
}

export function getStatusColor(s: ListingData['reviewStatus']): string {
  return { pending: 'default', reviewed: 'processing', approved: 'success', rejected: 'error' }[s];
}
