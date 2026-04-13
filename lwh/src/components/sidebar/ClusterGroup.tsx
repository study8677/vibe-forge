import { useState } from 'react'
import { ServerCard } from './ServerCard'
import { ChevronDown, ChevronRight, Layers } from 'lucide-react'
import type { ServerConfig } from '../../types'

export function ClusterGroup({ name, servers }: { name: string; servers: ServerConfig[] }) {
  const [expanded, setExpanded] = useState(true)

  return (
    <div>
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center gap-2 px-2.5 py-2 text-xs font-medium text-slate-400 hover:text-slate-300 transition-colors"
      >
        {expanded ? <ChevronDown size={13} /> : <ChevronRight size={13} />}
        <Layers size={13} />
        <span className="uppercase tracking-wider">{name}</span>
        <span className="ml-auto text-slate-500">{servers.length}</span>
      </button>
      {expanded && (
        <div className="space-y-1 ml-2">
          {servers.map(s => <ServerCard key={s.id} server={s} />)}
        </div>
      )}
    </div>
  )
}
