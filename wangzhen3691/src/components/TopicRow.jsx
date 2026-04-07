import { Pin, Flame, MessageCircle, Eye, Clock } from 'lucide-react'
import Avatar from './Avatar'
import CategoryBadge from './CategoryBadge'

export default function TopicRow({ topic, selected, onClick }) {
  const fmt = (n) => (n >= 1000 ? `${(n / 1000).toFixed(1)}k` : n)

  return (
    <tr
      onClick={() => onClick?.(topic.id)}
      className={`group cursor-pointer border-b border-border/50 transition-colors ${
        selected ? 'bg-bg-selected' : 'hover:bg-bg-hover'
      }`}
    >
      {/* Topic info */}
      <td className="py-2.5 px-4">
        <div className="flex items-start gap-2">
          <Avatar user={topic.op} size={28} />
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-1.5 flex-wrap">
              {topic.pinned && <Pin size={12} className="text-muted shrink-0" />}
              {topic.isHot && <Flame size={12} className="text-danger shrink-0" />}
              <span
                className={`text-sm leading-snug ${
                  topic.isNew ? 'text-accent font-medium' : 'text-primary group-hover:text-accent'
                } transition-colors`}
              >
                {topic.title}
              </span>
            </div>
            <div className="flex items-center gap-1.5 mt-1 flex-wrap">
              <CategoryBadge catId={topic.catId} />
              {topic.tags.map((t) => (
                <span key={t} className="text-[11px] text-muted bg-border/50 px-1.5 py-0.5 rounded">
                  {t}
                </span>
              ))}
            </div>
          </div>
        </div>
      </td>

      {/* Posters */}
      <td className="py-2.5 px-2 hidden lg:table-cell">
        <div className="flex -space-x-1.5">
          {topic.posters.slice(0, 4).map((u, i) => (
            <Avatar key={i} user={u} size={22} />
          ))}
          {topic.posters.length > 4 && (
            <div className="w-[22px] h-[22px] rounded-full bg-border flex items-center justify-center text-[9px] text-muted">
              +{topic.posters.length - 4}
            </div>
          )}
        </div>
      </td>

      {/* Replies */}
      <td className="py-2.5 px-3 text-center hidden md:table-cell">
        <div className="flex items-center justify-center gap-1 text-sm text-muted">
          <MessageCircle size={12} />
          <span className={topic.replies > 50 ? 'text-accent font-medium' : ''}>{fmt(topic.replies)}</span>
        </div>
      </td>

      {/* Views */}
      <td className="py-2.5 px-3 text-center hidden md:table-cell">
        <div className="flex items-center justify-center gap-1 text-sm text-muted">
          <Eye size={12} />
          <span>{fmt(topic.views)}</span>
        </div>
      </td>

      {/* Activity */}
      <td className="py-2.5 px-3 text-right">
        <div className="flex items-center justify-end gap-1 text-xs text-muted">
          <Clock size={11} />
          <span>{topic.activity}</span>
        </div>
      </td>
    </tr>
  )
}
