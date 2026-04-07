import { PrismaClient } from '@prisma/client'

const prisma = new PrismaClient()

const notes = [
  {
    title: '欢迎使用 Sora 笔记',
    emoji: '🚀',
    status: 'done',
    pinned: true,
    order: 0,
    content: `# 欢迎使用 Sora 笔记 🎉

这是一款基于 **Next.js** 构建的个人笔记应用，灵感来自 Notion。

## 功能特性

- ✅ **Markdown 渲染** — 支持完整的 GFM 语法
- ✅ **任务看板** — 拖拽管理笔记状态
- ✅ **自动保存** — 编辑即保存，无需手动操作
- ✅ **分栏预览** — 编辑与预览并排显示

## 快速上手

1. 在左侧栏点击 **新建笔记**
2. 输入标题和内容（支持 Markdown）
3. 切换到 **任务看板** 拖拽管理

> 💡 试试用快捷键 \`Tab\` 在编辑器中插入缩进

Happy noting! 📝`,
  },
  {
    title: 'Markdown 语法示例',
    emoji: '📖',
    status: 'backlog',
    order: 1,
    content: `# Markdown 语法参考

## 文本格式

**粗体文本** / *斜体文本* / ~~删除线~~

## 列表

- 无序列表项 1
- 无序列表项 2
  - 嵌套列表

1. 有序列表
2. 第二项

## 任务清单

- [x] 已完成的任务
- [ ] 待完成的任务
- [ ] 另一个待办事项

## 代码

行内代码 \`const x = 42\`

\`\`\`javascript
function fibonacci(n) {
  if (n <= 1) return n
  return fibonacci(n - 1) + fibonacci(n - 2)
}

console.log(fibonacci(10)) // 55
\`\`\`

## 表格

| 功能 | 状态 | 优先级 |
|------|------|--------|
| Markdown 渲染 | ✅ 完成 | 高 |
| 任务看板 | ✅ 完成 | 高 |
| 暗色模式 | 🔜 计划中 | 中 |

## 引用

> 好的工具让你专注于创造，而不是工具本身。

## 链接与图片

[Next.js 官网](https://nextjs.org)

---

*以上是 Sora 笔记支持的 Markdown 语法示例。*`,
  },
  {
    title: '项目架构规划',
    emoji: '🏗️',
    status: 'in_progress',
    order: 0,
    content: `# 项目架构

## 技术栈

- **前端**: Next.js 14 + React 18 + Tailwind CSS
- **后端**: Next.js API Routes
- **数据库**: SQLite + Prisma ORM
- **其他**: react-markdown, @hello-pangea/dnd

## 目录结构

\`\`\`
src/
├── app/           # 页面和API路由
├── components/    # UI组件
└── lib/           # 工具函数和配置
\`\`\`

## 待办

- [ ] 添加标签系统
- [ ] 笔记文件夹层级
- [ ] 导出功能 (PDF/HTML)
- [ ] 快捷键支持`,
  },
  {
    title: '本周学习计划',
    emoji: '🎓',
    status: 'todo',
    order: 0,
    content: `# 本周学习计划

## 周一 — TypeScript 高级类型
- 条件类型
- 映射类型
- 模板字面量类型

## 周二 — React Server Components
- RSC 工作原理
- 服务端 vs 客户端组件
- 数据获取模式

## 周三 — 数据库优化
- 索引策略
- 查询优化
- Prisma 性能技巧

## 周四 — 测试
- Vitest 配置
- 组件测试
- E2E 测试 (Playwright)

## 周五 — 回顾与总结
- 整理笔记
- 写周报`,
  },
  {
    title: 'API 设计备忘',
    emoji: '🔧',
    status: 'todo',
    order: 1,
    content: `# RESTful API 设计原则

## 命名规范

- 使用名词而非动词: \`/users\` 而非 \`/getUsers\`
- 使用复数形式: \`/notes\` 而非 \`/note\`
- 嵌套资源: \`/users/:id/notes\`

## HTTP 方法

| 方法 | 用途 | 幂等 |
|------|------|------|
| GET | 查询 | ✅ |
| POST | 创建 | ❌ |
| PATCH | 部分更新 | ✅ |
| DELETE | 删除 | ✅ |

## 状态码

- \`200\` 成功
- \`201\` 创建成功
- \`400\` 请求错误
- \`404\` 未找到
- \`500\` 服务器错误`,
  },
  {
    title: '读书笔记：原子习惯',
    emoji: '📚',
    status: 'done',
    order: 1,
    content: `# 原子习惯 — 核心笔记

## 四大定律

1. **让它显而易见** — 明确时间和地点
2. **让它有吸引力** — 绑定你喜欢的事
3. **让它简单易行** — 两分钟规则
4. **让它令人愉悦** — 立即奖励

## 关键洞察

> 你不会达到目标的水平，你只会下降到系统的水平。

- 习惯是身份认同的投票
- 1% 的日常改进 → 一年后提升 37 倍
- 环境设计比意志力更重要

## 实践

- 每天写 15 分钟代码（习惯叠加）
- 手机放到另一个房间（环境设计）
- 完成后在日历上画 ✓（追踪记录）`,
  },
]

async function main() {
  console.log('🌱 Seeding database...')

  await prisma.note.deleteMany()

  for (const note of notes) {
    await prisma.note.create({ data: note })
  }

  console.log(`✅ Created ${notes.length} notes`)
}

main()
  .catch(console.error)
  .finally(() => prisma.$disconnect())
