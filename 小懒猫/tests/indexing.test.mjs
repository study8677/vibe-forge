import test from "node:test";
import assert from "node:assert/strict";

import { buildMemoryIndex, searchMemoryIndex, tokenize } from "../src/indexing.js";

test("tokenize handles mixed Chinese and English content", () => {
  const tokens = tokenize("晨跑计划 Morning Coffee");

  assert(tokens.includes("晨跑"));
  assert(tokens.includes("跑计"));
  assert(tokens.includes("morning"));
  assert(tokens.includes("coffee"));
});

test("searchMemoryIndex ranks strong token matches ahead of weak matches", () => {
  const memories = [
    {
      id: "m1",
      title: "运动偏好",
      summary: "喜欢晨跑和拉伸",
      content: "每周三次晨跑，跑后会拉伸",
      tags: ["健康", "运动"],
      category: "health",
      layer: "profile",
      importance: 4,
      updatedAt: "2026-04-08T08:00:00.000Z",
      lastUsedAt: "2026-04-08T09:00:00.000Z"
    },
    {
      id: "m2",
      title: "咖啡偏好",
      summary: "喜欢燕麦拿铁",
      content: "工作日上午会喝燕麦拿铁",
      tags: ["偏好", "饮品"],
      category: "lifestyle",
      layer: "profile",
      importance: 3,
      updatedAt: "2026-04-07T08:00:00.000Z",
      lastUsedAt: "2026-04-07T09:00:00.000Z"
    }
  ];

  const index = buildMemoryIndex(memories);
  const results = searchMemoryIndex({
    index,
    memories,
    query: "晨跑 偏好"
  });

  assert.equal(results[0].id, "m1");
  assert.equal(results.length, 2);
});
