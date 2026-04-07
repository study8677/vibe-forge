export const STATUSES = [
  { id: 'backlog', label: '待整理', color: 'bg-notion-bg-active text-notion-text-secondary' },
  { id: 'todo', label: '待办', color: 'bg-notion-blue-bg text-notion-blue' },
  { id: 'in_progress', label: '进行中', color: 'bg-notion-orange-bg text-notion-orange' },
  { id: 'done', label: '已完成', color: 'bg-notion-green-bg text-notion-green' },
] as const

export type StatusId = (typeof STATUSES)[number]['id']

export function getStatus(id: string) {
  return STATUSES.find((s) => s.id === id) ?? STATUSES[0]
}

export const EMOJI_LIST = [
  '📝', '📓', '📔', '📒', '📕', '📗', '📘', '📙', '📖',
  '💡', '🎯', '🚀', '🔥', '💻', '🎨', '🎵', '🌟', '⭐',
  '✅', '❌', '⚡', '🔧', '🔑', '📌', '📎', '🏷️', '💬',
  '🐛', '🎉', '📊', '📈', '🗂️', '📋', '🔍', '💰', '🎓',
]
