const CATEGORY_KEYWORDS = {
  health: ["晨跑", "拉伸", "运动", "训练", "睡眠", "饮食", "恢复", "健身", "跑步"],
  work: ["项目", "会议", "客户", "工作", "汇报", "推进", "交付", "排期", "复盘"],
  learning: ["学习", "课程", "阅读", "知识", "练习", "复习", "写作", "研究"],
  finance: ["预算", "花销", "收入", "储蓄", "报销", "投资", "账单"],
  family: ["家人", "父母", "家庭", "孩子", "视频通话", "纪念日"],
  travel: ["出行", "旅行", "酒店", "机票", "路线", "周末", "杭州"],
  social: ["朋友", "聚会", "拜访", "社群", "活动", "联系"],
  lifestyle: ["喜欢", "偏好", "习惯", "咖啡", "燕麦拿铁", "空间", "整理", "舒适"]
};

const PREFERENCE_KEYWORDS = ["喜欢", "偏好", "习惯", "以后记住", "记住我", "常用"];
const ORGANIZE_KEYWORDS = ["整理", "归档", "收纳", "归类"];
const RECALL_KEYWORDS = ["回顾", "总结", "复盘", "回看"];
const ARCHIVE_KEYWORDS = ["档案", "长期保存", "归档总结"];

function nowIso() {
  return new Date().toISOString();
}

function normalizeText(text = "") {
  return String(text).replace(/\s+/g, " ").trim();
}

function deriveTitle(text) {
  const cleaned = normalizeText(text).replace(/[。！？,.!?]/g, "");
  return cleaned.slice(0, 14) || "新的记忆";
}

function deriveSummary(text) {
  return normalizeText(text).slice(0, 40);
}

function containsAny(text, keywords) {
  return keywords.some((keyword) => text.includes(keyword));
}

export function inferCategory(text = "") {
  const source = normalizeText(text);

  if (!source) {
    return "general";
  }

  if (containsAny(source, PREFERENCE_KEYWORDS)) {
    return "lifestyle";
  }

  for (const [category, keywords] of Object.entries(CATEGORY_KEYWORDS)) {
    if (containsAny(source, keywords)) {
      return category;
    }
  }

  return "general";
}

export function inferTags(text = "", category = "general") {
  const tags = new Set();

  Object.entries(CATEGORY_KEYWORDS).forEach(([name, keywords]) => {
    keywords.forEach((keyword) => {
      if (text.includes(keyword)) {
        tags.add(keyword);
      }
    });

    if (name === category && category !== "general") {
      tags.add(category === "lifestyle" ? "偏好" : CATEGORY_KEYWORDS[category][0]);
    }
  });

  if (containsAny(text, PREFERENCE_KEYWORDS)) {
    tags.add("偏好");
  }

  if (containsAny(text, ORGANIZE_KEYWORDS)) {
    tags.add("整理");
  }

  if (containsAny(text, RECALL_KEYWORDS)) {
    tags.add("回顾");
  }

  return [...tags];
}

export function inferLayer(text = "", category = "general") {
  if (containsAny(text, ARCHIVE_KEYWORDS) || containsAny(text, RECALL_KEYWORDS)) {
    return "archive";
  }

  if (containsAny(text, PREFERENCE_KEYWORDS)) {
    return "profile";
  }

  if (category !== "general" && category !== "lifestyle") {
    return "topic";
  }

  return "capture";
}

export function createMemoryDraft(input, createdAt = nowIso()) {
  const content = normalizeText(input);

  if (!content) {
    return null;
  }

  const category = inferCategory(content);
  const layer = inferLayer(content, category);
  const tags = inferTags(content, category);
  const status = layer === "capture" ? "pending" : layer === "archive" ? "archived" : "organized";

  return {
    id: `mem-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`,
    title: deriveTitle(content),
    summary: deriveSummary(content),
    content,
    layer,
    category,
    tags,
    source: "chat",
    importance: containsAny(content, PREFERENCE_KEYWORDS) ? 4 : 2,
    status,
    createdAt,
    updatedAt: createdAt,
    lastUsedAt: createdAt,
    entityRefs: []
  };
}

export function organizeMemoryItem(memory) {
  const content = normalizeText(memory.content ?? "");
  const category = inferCategory(content || memory.summary || memory.title);
  const tags = [...new Set([...(memory.tags ?? []), ...inferTags(content, category)])];
  let layer = memory.layer;
  let status = memory.status;

  if (memory.layer === "capture" && memory.status === "pending") {
    if (containsAny(content, PREFERENCE_KEYWORDS)) {
      layer = "profile";
    } else if (category !== "general") {
      layer = category === "lifestyle" ? "profile" : "topic";
    } else {
      layer = "topic";
    }

    status = layer === "archive" ? "archived" : "organized";
  }

  return {
    ...memory,
    category,
    layer,
    status,
    tags,
    updatedAt: nowIso(),
    lastUsedAt: nowIso()
  };
}

export function promoteCapturedMemories(memories = []) {
  return memories.map((memory) => {
    if (memory.layer === "capture" && memory.status === "pending") {
      return organizeMemoryItem(memory);
    }

    return memory;
  });
}

function summarizeMemories(memories) {
  return memories
    .slice(0, 2)
    .map((memory) => `- ${memory.title}：${memory.summary || memory.content}`)
    .join("\n");
}

export function buildAssistantReply({ input = "", relatedMemories = [] }) {
  const content = normalizeText(input);

  if (containsAny(content, ORGANIZE_KEYWORDS)) {
    return "我已经把待整理内容重新归入合适的主题层，后面你可以继续细化成偏好、项目或长期档案。";
  }

  if (containsAny(content, RECALL_KEYWORDS) && relatedMemories.length) {
    return `我先帮你回顾一下当前最相关的记忆：\n${summarizeMemories(
      relatedMemories
    )}\n\n如果你愿意，我可以继续把这些内容整理成行动清单或阶段复盘。`;
  }

  if (containsAny(content, PREFERENCE_KEYWORDS)) {
    return "我收下了，这条会被记成长期可复用的偏好或事实，后面在相关话题里我会优先调用。";
  }

  if (relatedMemories.length) {
    return `我结合你已有的记录看了一下，这件事和这些线索最相关：\n${summarizeMemories(
      relatedMemories
    )}\n\n我已经把这次聊天也纳入记忆库，后续可以继续展开。`;
  }

  return "我已经记下这件事，并先放进你的万事录。后面你可以让我继续整理、回顾，或者把它挂到某个主题空间。";
}
