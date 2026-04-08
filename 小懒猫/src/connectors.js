import { CONNECTOR_PRESETS } from "./data/defaults.js";

const CONNECTOR_MESSAGES = {
  telegram: [
    {
      title: "跑步群提醒",
      snippet: "周六 7:00 河边集合，建议自带水壶。",
      topicHint: "health"
    },
    {
      title: "旅行频道提醒",
      snippet: "西湖周边周末有短时降雨，记得备一件轻便外套。",
      topicHint: "travel"
    }
  ],
  feishu: [
    {
      title: "项目同步提醒",
      snippet: "明早 10:30 需要同步本周里程碑和风险清单。",
      topicHint: "work"
    },
    {
      title: "文档评论",
      snippet: "设计稿已更新，建议补一版自然风视觉说明。",
      topicHint: "work"
    }
  ],
  discord: [
    {
      title: "学习社群消息",
      snippet: "今晚 8 点有人发起读书会，主题是长期主义与系统化记录。",
      topicHint: "learning"
    }
  ],
  slack: [
    {
      title: "频道摘要",
      snippet: "市场频道新增了一个竞品整理帖，值得后续归档。",
      topicHint: "work"
    }
  ],
  wechat: [
    {
      title: "家人群提醒",
      snippet: "周末家庭聚餐时间还没确定，记得跟进。",
      topicHint: "family"
    }
  ],
  whatsapp: [
    {
      title: "朋友出行消息",
      snippet: "有人在约五一短途出游，想确认你的时间。",
      topicHint: "social"
    }
  ]
};

export function getConnectorPreset(connectorId) {
  return CONNECTOR_PRESETS.find((connector) => connector.id === connectorId);
}

export function simulateConnectorSync(connectorId, now = new Date()) {
  const template = CONNECTOR_MESSAGES[connectorId] ?? [
    {
      title: "新消息摘要",
      snippet: "有一条新的外部消息等待整理。",
      topicHint: "general"
    }
  ];

  const time = now.toISOString();
  const inboxItems = template.map((entry, index) => ({
    id: `inbox-${connectorId}-${now.getTime()}-${index}`,
    connectorId,
    title: entry.title,
    snippet: entry.snippet,
    topicHint: entry.topicHint,
    createdAt: time,
    status: "new"
  }));

  return {
    connectorUpdate: {
      lastSyncedAt: time,
      status: "connected"
    },
    inboxItems
  };
}
