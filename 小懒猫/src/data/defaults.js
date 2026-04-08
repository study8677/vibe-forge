export const LAYER_META = {
  capture: {
    label: "速记层",
    description: "刚进入系统、等待整理的片段"
  },
  profile: {
    label: "事实层",
    description: "稳定偏好、关系和个人资料"
  },
  topic: {
    label: "主题层",
    description: "围绕生活主题整理后的内容"
  },
  archive: {
    label: "档案层",
    description: "阶段总结与长期归档"
  }
};

export const CATEGORY_META = {
  general: { label: "日常", accent: "sand" },
  lifestyle: { label: "生活", accent: "clay" },
  health: { label: "健康", accent: "sage" },
  work: { label: "工作", accent: "slate" },
  learning: { label: "学习", accent: "amber" },
  finance: { label: "财务", accent: "stone" },
  family: { label: "家庭", accent: "rose" },
  travel: { label: "出行", accent: "sea" },
  social: { label: "社交", accent: "moss" }
};

export const TOPIC_SPACES = [
  {
    id: "topic-health",
    name: "健康节律",
    category: "health",
    accent: "sage",
    description: "运动、睡眠、饮食和恢复"
  },
  {
    id: "topic-work",
    name: "工作台",
    category: "work",
    accent: "slate",
    description: "项目推进、会话纪要和任务脉络"
  },
  {
    id: "topic-family",
    name: "家庭关系",
    category: "family",
    accent: "rose",
    description: "家庭安排、纪念日和照料事项"
  },
  {
    id: "topic-learning",
    name: "学习清单",
    category: "learning",
    accent: "amber",
    description: "想学的内容、阅读摘要和练习线索"
  },
  {
    id: "topic-travel",
    name: "出行计划",
    category: "travel",
    accent: "sea",
    description: "路线、偏好、行前准备"
  },
  {
    id: "topic-lifestyle",
    name: "日常偏好",
    category: "lifestyle",
    accent: "clay",
    description: "饮食、咖啡、空间习惯和舒适感"
  }
];

export const CONNECTOR_PRESETS = [
  {
    id: "telegram",
    name: "Telegram",
    family: "IM",
    status: "connected",
    description: "轻量聊天与频道消息接入",
    accent: "sea",
    inboxCount: 1
  },
  {
    id: "feishu",
    name: "飞书",
    family: "Work",
    status: "connected",
    description: "工作消息、待办和文档提醒",
    accent: "slate",
    inboxCount: 0
  },
  {
    id: "discord",
    name: "Discord",
    family: "Community",
    status: "preview",
    description: "社群动态与私信摘要",
    accent: "moss",
    inboxCount: 0
  },
  {
    id: "slack",
    name: "Slack",
    family: "Work",
    status: "preview",
    description: "频道消息整合入口",
    accent: "stone",
    inboxCount: 0
  },
  {
    id: "wechat",
    name: "微信",
    family: "Social",
    status: "planned",
    description: "保留主流个人聊天程序接入位",
    accent: "sage",
    inboxCount: 0
  },
  {
    id: "whatsapp",
    name: "WhatsApp",
    family: "IM",
    status: "planned",
    description: "国际化联系人与群组消息接口",
    accent: "clay",
    inboxCount: 0
  }
];

function isoDate(minutesAgo = 0) {
  return new Date(Date.now() - minutesAgo * 60_000).toISOString();
}

export function createSeedState() {
  return {
    version: 1,
    activeConversationId: "conv-main",
    conversations: [
      {
        id: "conv-main",
        title: "今日管家会话",
        createdAt: isoDate(180),
        updatedAt: isoDate(3),
        messages: [
          {
            id: "msg-welcome",
            role: "assistant",
            content:
              "欢迎来到万事录。我会把聊天、记忆、主题和外部消息整理到同一个生活中枢里。",
            createdAt: isoDate(180),
            memoryIds: []
          },
          {
            id: "msg-tip",
            role: "assistant",
            content:
              "你可以直接说“记住我的偏好”“帮我整理待办”或“回顾最近的健康计划”。",
            createdAt: isoDate(176),
            memoryIds: []
          }
        ]
      }
    ],
    memories: [
      {
        id: "mem-run",
        title: "运动偏好",
        summary: "喜欢晨跑和跑后拉伸",
        content: "每周三次晨跑，跑后需要做 10 分钟拉伸。",
        layer: "profile",
        category: "health",
        tags: ["运动", "偏好", "晨跑"],
        source: "chat",
        importance: 4,
        status: "organized",
        createdAt: isoDate(960),
        updatedAt: isoDate(420),
        lastUsedAt: isoDate(20),
        entityRefs: []
      },
      {
        id: "mem-coffee",
        title: "咖啡偏好",
        summary: "上午更喜欢燕麦拿铁",
        content: "工作日上午喝燕麦拿铁，下午尽量不再摄入咖啡因。",
        layer: "profile",
        category: "lifestyle",
        tags: ["偏好", "咖啡", "燕麦拿铁"],
        source: "chat",
        importance: 3,
        status: "organized",
        createdAt: isoDate(1200),
        updatedAt: isoDate(360),
        lastUsedAt: isoDate(60),
        entityRefs: []
      },
      {
        id: "mem-family",
        title: "家庭固定安排",
        summary: "周日晚和家人视频通话",
        content: "每周日晚上和家里视频通话，提前留出安静时间。",
        layer: "topic",
        category: "family",
        tags: ["家庭", "视频通话", "固定安排"],
        source: "manual",
        importance: 4,
        status: "organized",
        createdAt: isoDate(1600),
        updatedAt: isoDate(520),
        lastUsedAt: isoDate(180),
        entityRefs: []
      },
      {
        id: "mem-trip",
        title: "杭州出行草稿",
        summary: "五一前后有杭州短途出行想法",
        content: "想把杭州周末出行整理成清单：酒店、路线、雨具和咖啡店备选。",
        layer: "capture",
        category: "travel",
        tags: ["出行", "杭州"],
        source: "chat",
        importance: 2,
        status: "pending",
        createdAt: isoDate(90),
        updatedAt: isoDate(90),
        lastUsedAt: isoDate(90),
        entityRefs: []
      },
      {
        id: "mem-review",
        title: "三月生活复盘",
        summary: "作息变稳，但运动频率下降",
        content: "三月整体作息变稳，工作专注提高，但运动频率明显下降，需要四月恢复节律。",
        layer: "archive",
        category: "health",
        tags: ["复盘", "作息", "运动"],
        source: "manual",
        importance: 5,
        status: "archived",
        createdAt: isoDate(4320),
        updatedAt: isoDate(4320),
        lastUsedAt: isoDate(720),
        entityRefs: []
      }
    ],
    topicSpaces: TOPIC_SPACES,
    connectors: CONNECTOR_PRESETS.map((connector) => ({
      ...connector,
      lastSyncedAt: connector.status === "connected" ? isoDate(45) : null
    })),
    inbox: [
      {
        id: "inbox-seed-telegram",
        connectorId: "telegram",
        title: "跑步群提醒",
        snippet: "周六 7:00 河边集合，建议自带水壶。",
        topicHint: "health",
        createdAt: isoDate(40),
        status: "new"
      }
    ],
    ui: {
      searchQuery: "",
      selectedLayer: "all",
      selectedCategory: "all"
    }
  };
}
