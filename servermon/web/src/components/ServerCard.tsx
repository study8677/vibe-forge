import { useNavigate } from 'react-router-dom'
import { Monitor, ArrowDown, ArrowUp } from 'lucide-react'
import type { ServerWithMetrics } from '../api/client'
import { formatBytes, formatSpeed, formatUptime, pct } from '../api/client'

function UsageBar({ label, value, color }: { label: string; value: number; color: string }) {
  const colorClass =
    value > 90 ? 'bg-red-500' : value > 70 ? 'bg-amber-500' : color
  return (
    <div className="space-y-1">
      <div className="flex justify-between text-xs">
        <span className="text-slate-400">{label}</span>
        <span className="text-slate-300">{value.toFixed(1)}%</span>
      </div>
      <div className="h-1.5 bg-slate-700 rounded-full overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-500 ${colorClass}`}
          style={{ width: `${Math.min(value, 100)}%` }}
        />
      </div>
    </div>
  )
}

export default function ServerCard({ data }: { data: ServerWithMetrics }) {
  const navigate = useNavigate()
  const { server: s, metrics: m } = data
  const online = s.status === 1

  return (
    <div
      onClick={() => navigate(`/server/${s.id}`)}
      className="bg-slate-800 rounded-xl border border-slate-700 p-4 cursor-pointer hover:border-slate-600 transition-colors"
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Monitor className="w-4 h-4 text-slate-400" />
          <span className="font-medium text-slate-100 truncate">{s.name}</span>
        </div>
        <span
          className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${
            online
              ? 'bg-emerald-400/10 text-emerald-400'
              : 'bg-red-400/10 text-red-400'
          }`}
        >
          <span className={`w-1.5 h-1.5 rounded-full ${online ? 'bg-emerald-400' : 'bg-red-400'}`} />
          {online ? 'Online' : 'Offline'}
        </span>
      </div>

      {m ? (
        <>
          {/* Usage bars */}
          <div className="space-y-2 mb-3">
            <UsageBar label="CPU" value={m.cpu} color="bg-blue-500" />
            <UsageBar label="Memory" value={pct(m.mem_used, m.mem_total)} color="bg-violet-500" />
            <UsageBar label="Disk" value={pct(m.disk_used, m.disk_total)} color="bg-amber-500" />
          </div>

          {/* Bottom info */}
          <div className="grid grid-cols-2 gap-2 text-xs text-slate-400">
            <div className="flex items-center gap-1">
              <ArrowDown className="w-3 h-3 text-emerald-400" />
              {formatSpeed(m.net_in_speed)}
            </div>
            <div className="flex items-center gap-1">
              <ArrowUp className="w-3 h-3 text-blue-400" />
              {formatSpeed(m.net_out_speed)}
            </div>
            <div>Mem: {formatBytes(m.mem_used)}/{formatBytes(m.mem_total)}</div>
            <div>Up: {formatUptime(m.uptime)}</div>
          </div>
        </>
      ) : (
        <div className="text-sm text-slate-500 py-4 text-center">No data</div>
      )}

      {/* Platform info */}
      {s.platform && (
        <div className="mt-2 pt-2 border-t border-slate-700/50 text-xs text-slate-500 truncate">
          {s.platform} | {s.arch}
        </div>
      )}
    </div>
  )
}
