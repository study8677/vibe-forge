import { useState, useEffect } from 'react'
import { fetchAlerts } from '../api'

const badge = {
  info:     'bg-blue-500/20 text-blue-300',
  warning:  'bg-amber-500/20 text-amber-300',
  danger:   'bg-orange-500/20 text-orange-300',
  critical: 'bg-red-500/20 text-red-300 animate-pulse',
}

const labels = { all: '全部', info: 'Info', warning: 'Warning', danger: 'Danger', critical: 'Critical' }

export default function AlertFeed({ alerts: realtime }) {
  const [stored, setStored] = useState([])
  const [filter, setFilter] = useState('all')

  useEffect(() => { fetchAlerts().then((d) => setStored(d || [])) }, [])

  // merge realtime (newest first) + stored, deduplicate by id
  const seen = new Set()
  const merged = [...realtime, ...stored].filter((a) => {
    if (seen.has(a.id)) return false
    seen.add(a.id)
    return true
  })

  const list = filter === 'all' ? merged : merged.filter((a) => a.level === filter)

  return (
    <div>
      {/* header */}
      <div className="flex flex-wrap items-center justify-between gap-4 mb-6">
        <h2 className="text-xl font-bold text-dark-100">预警中心</h2>
        <div className="flex gap-1.5">
          {Object.entries(labels).map(([k, v]) => (
            <button
              key={k}
              onClick={() => setFilter(k)}
              className={`px-3 py-1 rounded-full text-xs font-medium transition
                ${filter === k
                  ? 'bg-cyan-500 text-white shadow-lg shadow-cyan-500/20'
                  : 'bg-dark-800 text-dark-400 hover:text-dark-200'}`}
            >
              {v}
            </button>
          ))}
        </div>
      </div>

      {/* table */}
      <div className="bg-dark-900 rounded-xl ring-1 ring-dark-800 overflow-hidden">
        {/* thead */}
        <div className="grid grid-cols-[80px_100px_1fr_180px_140px] gap-3 px-4 py-3 border-b border-dark-800 text-[11px] text-dark-500 uppercase tracking-wider font-semibold">
          <span>级别</span><span>插件</span><span>规则</span><span>匹配</span><span>时间</span>
        </div>

        {/* tbody */}
        <div className="max-h-[calc(100vh-230px)] overflow-y-auto divide-y divide-dark-800/60">
          {list.length === 0 ? (
            <div className="py-16 text-center text-dark-600 text-sm">暂无预警数据</div>
          ) : list.map((a, i) => (
            <div key={a.id || i} className="grid grid-cols-[80px_100px_1fr_180px_140px] gap-3 px-4 py-3 text-sm hover:bg-dark-800/40 transition-colors items-center">
              <span className={`inline-flex justify-center px-2 py-0.5 rounded text-[11px] font-semibold ${badge[a.level] || badge.info}`}>
                {a.level}
              </span>
              <span className="text-dark-400 truncate text-xs">{a.plugin_name}</span>
              <span className="text-dark-200 truncate">{a.rule_name}</span>
              <span className="text-amber-300/80 truncate font-mono text-xs">{a.matched_text}</span>
              <span className="text-dark-500 text-xs tabular-nums">
                {new Date(a.timestamp).toLocaleString('zh-CN', { hour12: false })}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
