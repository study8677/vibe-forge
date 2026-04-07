const tabs = [
  { id: 'latest', label: '最新' },
  { id: 'new', label: '新话题' },
  { id: 'unread', label: '未读' },
  { id: 'top', label: '热门' },
  { id: 'hot', label: '🔥 Hot' },
  { id: 'categories', label: '分类' },
  { id: 'votes', label: '投票' },
]

export default function TabNav({ active, onChange }) {
  return (
    <nav className="flex items-center gap-0.5 px-4 border-b border-border overflow-x-auto">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onChange(tab.id)}
          className={`px-3 py-2.5 text-sm whitespace-nowrap transition-colors border-b-2 ${
            active === tab.id
              ? 'border-accent text-accent font-medium'
              : 'border-transparent text-muted hover:text-primary'
          }`}
        >
          {tab.label}
        </button>
      ))}
    </nav>
  )
}
