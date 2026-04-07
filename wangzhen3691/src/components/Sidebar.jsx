import {
  MessageSquare, Mail, Shield, Info, UserPlus, Users, Award, Trophy,
  Filter, Globe, Send, Radio, ChevronDown, ChevronRight, Rss, Terminal as TermIcon,
} from 'lucide-react'
import { useState } from 'react'
import { categories } from '../data/categories'

const communityLinks = [
  { icon: MessageSquare, label: '话题', count: null },
  { icon: Mail, label: '我的消息', count: 3 },
  { icon: Shield, label: '审核', count: null },
  { icon: Info, label: '关于', count: null },
  { icon: UserPlus, label: '邀请', count: null },
  { icon: Users, label: '用户', count: null },
  { icon: Award, label: '徽章', count: null },
  { icon: Trophy, label: '排行榜', count: null },
  { icon: Filter, label: '筛选', count: null },
]

const resourceLinks = [
  { icon: Globe, label: 'Connect', href: '#' },
  { icon: Send, label: 'Telegram', href: '#' },
  { icon: Radio, label: 'IDC Flare', href: '#' },
]

export default function Sidebar({ open, activeView, onViewChange }) {
  const [catOpen, setCatOpen] = useState(true)

  return (
    <aside
      className={`shrink-0 border-r border-border bg-bg-primary overflow-y-auto transition-all duration-200 ${
        open ? 'w-56' : 'w-0 overflow-hidden'
      }`}
    >
      <div className="w-56 py-2">
        {/* Community */}
        <div className="px-3 py-1 text-[10px] font-semibold text-muted uppercase tracking-wider">社区</div>
        {communityLinks.map(({ icon: Icon, label, count }) => (
          <button
            key={label}
            className="w-full flex items-center gap-2.5 px-3 py-1.5 text-sm text-primary/80 hover:bg-bg-hover hover:text-primary transition-colors"
          >
            <Icon size={15} className="text-muted shrink-0" />
            <span className="flex-1 text-left truncate">{label}</span>
            {count && (
              <span className="text-[10px] bg-accent text-bg-secondary px-1.5 py-0.5 rounded-full font-bold">
                {count}
              </span>
            )}
          </button>
        ))}

        {/* Special views */}
        <div className="mx-3 my-2 border-t border-border" />
        <div className="px-3 py-1 text-[10px] font-semibold text-muted uppercase tracking-wider">工具</div>
        <button
          onClick={() => onViewChange('rss')}
          className={`w-full flex items-center gap-2.5 px-3 py-1.5 text-sm transition-colors ${
            activeView === 'rss' ? 'bg-bg-selected text-accent' : 'text-primary/80 hover:bg-bg-hover hover:text-primary'
          }`}
        >
          <Rss size={15} className="shrink-0" />
          <span className="flex-1 text-left">RSS 聚合</span>
        </button>
        <button
          onClick={() => onViewChange('ssh')}
          className={`w-full flex items-center gap-2.5 px-3 py-1.5 text-sm transition-colors ${
            activeView === 'ssh' ? 'bg-bg-selected text-accent' : 'text-primary/80 hover:bg-bg-hover hover:text-primary'
          }`}
        >
          <TermIcon size={15} className="shrink-0" />
          <span className="flex-1 text-left">WebSSH</span>
        </button>

        {/* Categories */}
        <div className="mx-3 my-2 border-t border-border" />
        <button
          onClick={() => setCatOpen(!catOpen)}
          className="w-full flex items-center gap-2 px-3 py-1 text-[10px] font-semibold text-muted uppercase tracking-wider hover:text-primary"
        >
          {catOpen ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
          分类
        </button>
        {catOpen &&
          categories.map((cat) => (
            <button
              key={cat.id}
              className="w-full flex items-center gap-2.5 px-3 py-1 text-sm text-primary/80 hover:bg-bg-hover hover:text-primary transition-colors"
              style={{ paddingLeft: cat.parent ? '2rem' : undefined }}
            >
              <span
                className="w-2.5 h-2.5 rounded-sm shrink-0"
                style={{ background: cat.color }}
              />
              <span className="flex-1 text-left truncate text-[13px]">{cat.name}</span>
              <span className="text-[10px] text-muted">{cat.count > 1000 ? `${(cat.count / 1000).toFixed(1)}k` : cat.count}</span>
            </button>
          ))}

        {/* Resources */}
        <div className="mx-3 my-2 border-t border-border" />
        <div className="px-3 py-1 text-[10px] font-semibold text-muted uppercase tracking-wider">资源</div>
        {resourceLinks.map(({ icon: Icon, label }) => (
          <button
            key={label}
            className="w-full flex items-center gap-2.5 px-3 py-1.5 text-sm text-primary/80 hover:bg-bg-hover hover:text-primary transition-colors"
          >
            <Icon size={15} className="text-muted shrink-0" />
            <span className="flex-1 text-left">{label}</span>
          </button>
        ))}
      </div>
    </aside>
  )
}
