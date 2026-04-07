import { useState } from 'react'
import { MessageCircle, Eye, Clock, Users } from 'lucide-react'
import TopicRow from './TopicRow'
import TopicDetail from './TopicDetail'

export default function TopicList({ topics }) {
  const [selectedId, setSelectedId] = useState(null)
  const selected = topics.find((t) => t.id === selectedId)

  if (selected) {
    return <TopicDetail topic={selected} onBack={() => setSelectedId(null)} />
  }

  return (
    <div className="flex-1 overflow-auto">
      <table className="w-full">
        <thead>
          <tr className="text-xs text-muted border-b border-border">
            <th className="text-left py-2 px-4 font-medium">话题</th>
            <th className="py-2 px-2 font-medium hidden lg:table-cell">
              <Users size={12} className="inline" />
            </th>
            <th className="py-2 px-3 font-medium hidden md:table-cell">
              <MessageCircle size={12} className="inline" />
            </th>
            <th className="py-2 px-3 font-medium hidden md:table-cell">
              <Eye size={12} className="inline" />
            </th>
            <th className="text-right py-2 px-3 font-medium">
              <Clock size={12} className="inline" />
            </th>
          </tr>
        </thead>
        <tbody>
          {topics.map((t) => (
            <TopicRow
              key={t.id}
              topic={t}
              selected={t.id === selectedId}
              onClick={setSelectedId}
            />
          ))}
        </tbody>
      </table>

      {/* Footer */}
      <div className="text-center py-6 text-xs text-muted border-t border-border">
        <span>共 {topics.length} 个话题</span>
        <span className="mx-2">·</span>
        <a href="#" className="text-accent hover:underline">服务条款</a>
        <span className="mx-1">·</span>
        <a href="#" className="text-accent hover:underline">隐私政策</a>
        <span className="mx-1">·</span>
        <a href="#" className="text-accent hover:underline">社区准则</a>
      </div>
    </div>
  )
}
