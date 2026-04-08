import test from "node:test";
import assert from "node:assert/strict";

import {
  buildAssistantReply,
  createMemoryDraft,
  organizeMemoryItem,
  promoteCapturedMemories
} from "../src/engine.js";

test("createMemoryDraft extracts a reusable preference memory", () => {
  const draft = createMemoryDraft("以后记住，我喜欢晨跑和燕麦拿铁。");

  assert.equal(draft.layer, "profile");
  assert.equal(draft.category, "lifestyle");
  assert(draft.tags.includes("偏好"));
});

test("organizeMemoryItem promotes pending capture into topic memory", () => {
  const organized = organizeMemoryItem({
    id: "m1",
    title: "健康安排",
    summary: "准备恢复晨跑",
    content: "下周开始恢复晨跑和力量训练",
    tags: [],
    category: "general",
    layer: "capture",
    source: "chat",
    status: "pending",
    importance: 2,
    createdAt: "2026-04-08T08:00:00.000Z",
    updatedAt: "2026-04-08T08:00:00.000Z"
  });

  assert.equal(organized.layer, "topic");
  assert.equal(organized.category, "health");
  assert.equal(organized.status, "organized");
});

test("buildAssistantReply recalls matching memory context", () => {
  const reply = buildAssistantReply({
    input: "帮我回顾一下我的运动偏好",
    relatedMemories: [
      {
        id: "m1",
        title: "运动偏好",
        summary: "喜欢晨跑和拉伸",
        content: "每周三次晨跑，跑后会拉伸"
      }
    ]
  });

  assert.match(reply, /晨跑/);
  assert.match(reply, /拉伸/);
});

test("promoteCapturedMemories only upgrades pending capture items", () => {
  const result = promoteCapturedMemories([
    {
      id: "a",
      title: "待整理",
      summary: "想把旅行计划收成主题",
      content: "五一出行计划待细化",
      tags: [],
      category: "travel",
      layer: "capture",
      source: "chat",
      status: "pending",
      importance: 1,
      createdAt: "2026-04-08T08:00:00.000Z",
      updatedAt: "2026-04-08T08:00:00.000Z"
    },
    {
      id: "b",
      title: "已整理",
      summary: "保持不变",
      content: "已经归档",
      tags: [],
      category: "archive",
      layer: "archive",
      source: "chat",
      status: "archived",
      importance: 1,
      createdAt: "2026-04-08T08:00:00.000Z",
      updatedAt: "2026-04-08T08:00:00.000Z"
    }
  ]);

  assert.equal(result[0].status, "organized");
  assert.equal(result[0].layer, "topic");
  assert.equal(result[1].status, "archived");
  assert.equal(result[1].layer, "archive");
});
