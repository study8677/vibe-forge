import { useState, useEffect, useCallback } from 'react'
import { CheckCircle2, AlertTriangle } from 'lucide-react'
import { api, type AlertEvent, formatTime } from '../api/client'
import { useWebSocket } from '../hooks/useWebSocket'

export default function AlertEvents() {
  const [events, setEvents] = useState<AlertEvent[]>([])
  const [filter, setFilter] = useState<'all' | 'firing' | 'resolved'>('all')

  const load = useCallback(async () => {
    const list = await api.listAlertEvents(200)
    setEvents(list || [])
  }, [])

  useEffect(() => { load() }, [load])

  useWebSocket((msg) => {
    if (msg.type === 'alert' || msg.type === 'alert_resolved') {
      load()
    }
  })

  const handleResolve = async (id: number) => {
    await api.resolveAlert(id)
    load()
  }

  const filtered = events.filter((e) => filter === 'all' || e.status === filter)

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-slate-100">Alert Events</h1>
        <div className="flex items-center gap-1 bg-slate-800 rounded-lg p-1">
          {(['all', 'firing', 'resolved'] as const).map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                filter === f ? 'bg-slate-700 text-slate-100' : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              {f === 'all' ? 'All' : f === 'firing' ? 'Firing' : 'Resolved'}
            </button>
          ))}
        </div>
      </div>

      <div className="bg-slate-800 rounded-xl border border-slate-700 overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-slate-700 text-left">
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Status</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Server</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Rule</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden sm:table-cell">Metric</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden md:table-cell">Value</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden lg:table-cell">Time</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Action</th>
            </tr>
          </thead>
          <tbody>
            {filtered.length === 0 ? (
              <tr>
                <td colSpan={7} className="px-4 py-10 text-center text-slate-500">
                  <CheckCircle2 className="w-8 h-8 mx-auto mb-2 opacity-50" />
                  No alerts
                </td>
              </tr>
            ) : (
              filtered.map((e) => (
                <tr key={e.id} className="border-b border-slate-700/50 hover:bg-slate-700/30">
                  <td className="px-4 py-3">
                    {e.status === 'firing' ? (
                      <span className="inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full bg-red-400/10 text-red-400">
                        <AlertTriangle className="w-3 h-3" /> Firing
                      </span>
                    ) : (
                      <span className="inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full bg-emerald-400/10 text-emerald-400">
                        <CheckCircle2 className="w-3 h-3" /> Resolved
                      </span>
                    )}
                  </td>
                  <td className="px-4 py-3 font-medium text-slate-100">{e.server_name}</td>
                  <td className="px-4 py-3 text-sm text-slate-300">{e.rule_name}</td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden sm:table-cell">{e.metric_type}</td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden md:table-cell">
                    {e.value.toFixed(1)} (threshold: {e.threshold})
                  </td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden lg:table-cell">
                    {formatTime(e.created_at)}
                    {e.resolved_at > 0 && (
                      <div className="text-xs text-slate-500">Resolved: {formatTime(e.resolved_at)}</div>
                    )}
                  </td>
                  <td className="px-4 py-3">
                    {e.status === 'firing' && (
                      <button
                        onClick={() => handleResolve(e.id)}
                        className="text-xs px-2 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded-md"
                      >
                        Resolve
                      </button>
                    )}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}
