"""Airport name to IATA code mapping with fuzzy matching."""

from __future__ import annotations

# Chinese airport names -> IATA codes (multiple aliases per airport)
AIRPORTS: dict[str, str] = {
    # 北京
    "北京首都": "PEK", "首都机场": "PEK", "北京首都机场": "PEK",
    "北京大兴": "PKX", "大兴机场": "PKX", "北京大兴机场": "PKX",
    # 上海
    "上海虹桥": "SHA", "虹桥机场": "SHA", "上海虹桥机场": "SHA",
    "上海浦东": "PVG", "浦东机场": "PVG", "上海浦东机场": "PVG",
    # 广州
    "广州白云": "CAN", "白云机场": "CAN", "广州白云机场": "CAN",
    # 深圳
    "深圳宝安": "SZX", "宝安机场": "SZX", "深圳宝安机场": "SZX",
    # 成都
    "成都天府": "TFU", "天府机场": "TFU", "成都天府机场": "TFU",
    "成都双流": "CTU", "双流机场": "CTU", "成都双流机场": "CTU",
    # 重庆
    "重庆江北": "CKG", "江北机场": "CKG", "重庆江北机场": "CKG",
    # 杭州
    "杭州萧山": "HGH", "萧山机场": "HGH", "杭州萧山机场": "HGH",
    # 武汉
    "武汉天河": "WUH", "天河机场": "WUH", "武汉天河机场": "WUH",
    # 西安
    "西安咸阳": "XIY", "咸阳机场": "XIY", "西安咸阳机场": "XIY",
    # 南京
    "南京禄口": "NKG", "禄口机场": "NKG", "南京禄口机场": "NKG",
    # 昆明
    "昆明长水": "KMG", "长水机场": "KMG", "昆明长水机场": "KMG",
    # 长沙
    "长沙黄花": "CSX", "黄花机场": "CSX", "长沙黄花机场": "CSX",
    # 郑州
    "郑州新郑": "CGO", "新郑机场": "CGO", "郑州新郑机场": "CGO",
    # 厦门
    "厦门高崎": "XMN", "高崎机场": "XMN", "厦门高崎机场": "XMN",
    # 青岛
    "青岛胶东": "TAO", "胶东机场": "TAO", "青岛胶东机场": "TAO",
    # 大连
    "大连周水子": "DLC", "周水子机场": "DLC", "大连周水子机场": "DLC",
    # 天津
    "天津滨海": "TSN", "滨海机场": "TSN", "天津滨海机场": "TSN",
    # 哈尔滨
    "哈尔滨太平": "HRB", "太平机场": "HRB", "哈尔滨太平机场": "HRB",
    # 沈阳
    "沈阳桃仙": "SHE", "桃仙机场": "SHE", "沈阳桃仙机场": "SHE",
    # 三亚
    "三亚凤凰": "SYX", "凤凰机场": "SYX", "三亚凤凰机场": "SYX",
    # 海口
    "海口美兰": "HAK", "美兰机场": "HAK", "海口美兰机场": "HAK",
    # 贵阳
    "贵阳龙洞堡": "KWE", "龙洞堡机场": "KWE", "贵阳龙洞堡机场": "KWE",
    # 南宁
    "南宁吴圩": "NNG", "吴圩机场": "NNG", "南宁吴圩机场": "NNG",
    # 兰州
    "兰州中川": "LHW", "中川机场": "LHW", "兰州中川机场": "LHW",
    # 乌鲁木齐
    "乌鲁木齐地窝堡": "URC", "地窝堡机场": "URC", "乌鲁木齐地窝堡机场": "URC",
    # 拉萨
    "拉萨贡嘎": "LXA", "贡嘎机场": "LXA", "拉萨贡嘎机场": "LXA",
    # 珠海
    "珠海金湾": "ZUH", "金湾机场": "ZUH", "珠海金湾机场": "ZUH",
    # 福州
    "福州长乐": "FOC", "长乐机场": "FOC", "福州长乐机场": "FOC",
    # 合肥
    "合肥新桥": "HFE", "新桥机场": "HFE", "合肥新桥机场": "HFE",
    # 济南
    "济南遥墙": "TNA", "遥墙机场": "TNA", "济南遥墙机场": "TNA",
    # 太原
    "太原武宿": "TYN", "武宿机场": "TYN", "太原武宿机场": "TYN",
    # 南昌
    "南昌昌北": "KHN", "昌北机场": "KHN", "南昌昌北机场": "KHN",
    # 呼和浩特
    "呼和浩特白塔": "HET", "白塔机场": "HET", "呼和浩特白塔机场": "HET",
    # 港澳台
    "香港": "HKG", "香港机场": "HKG", "香港国际机场": "HKG",
    "澳门": "MFM", "澳门机场": "MFM", "澳门国际机场": "MFM",
    "台北桃园": "TPE", "桃园机场": "TPE", "台北桃园机场": "TPE",
    # 国际主要
    "东京成田": "NRT", "成田机场": "NRT",
    "东京羽田": "HND", "羽田机场": "HND",
    "首尔仁川": "ICN", "仁川机场": "ICN",
    "新加坡樟宜": "SIN", "樟宜机场": "SIN",
    "曼谷素万那普": "BKK", "素万那普机场": "BKK",
}

# City-only lookups (default to main airport)
CITY_DEFAULTS: dict[str, str] = {
    "北京": "PEK", "上海": "PVG", "广州": "CAN", "深圳": "SZX",
    "成都": "TFU", "重庆": "CKG", "杭州": "HGH", "武汉": "WUH",
    "西安": "XIY", "南京": "NKG", "昆明": "KMG", "长沙": "CSX",
    "郑州": "CGO", "厦门": "XMN", "青岛": "TAO", "大连": "DLC",
    "天津": "TSN", "哈尔滨": "HRB", "沈阳": "SHE", "三亚": "SYX",
    "海口": "HAK", "贵阳": "KWE", "南宁": "NNG", "兰州": "LHW",
    "乌鲁木齐": "URC", "拉萨": "LXA", "珠海": "ZUH", "福州": "FOC",
    "合肥": "HFE", "济南": "TNA", "太原": "TYN", "南昌": "KHN",
    "呼和浩特": "HET", "香港": "HKG", "澳门": "MFM",
}


def resolve_airport(name: str) -> str:
    """Resolve airport name (Chinese or IATA code) to IATA code.

    Supports exact match, city name default, and substring fuzzy match.
    """
    name = name.strip()

    # Direct IATA code
    if len(name) == 3 and name.isalpha() and name.isupper():
        return name

    if name in AIRPORTS:
        return AIRPORTS[name]

    if name in CITY_DEFAULTS:
        return CITY_DEFAULTS[name]

    # Substring fuzzy match
    for key, code in AIRPORTS.items():
        if name in key or key in name:
            return code

    raise ValueError(f"无法识别机场: {name} -- 请使用 --list-airports 查看支持列表")


def list_airports():
    """Print supported airports grouped by IATA code."""
    code_names: dict[str, list[str]] = {}
    for name, code in AIRPORTS.items():
        if code not in code_names:
            code_names[code] = []
        if len(name) > 2:
            code_names[code].append(name)

    print("支持的机场列表:")
    print(f"  {'IATA':<6}{'名称'}")
    print("  " + "-" * 40)
    for code, names in sorted(code_names.items()):
        display = ", ".join(sorted(names, key=len)[:2])
        print(f"  {code:<6}{display}")
