import { useState, useEffect, useCallback } from 'react'
import { Server, Wifi, WifiOff, AlertTriangle } from 'lucide-react'
import { api, type ServerWithMetrics, type DashboardStats } from '../api/client'
import { useWebSocket } from '../hooks/useWebSocket'
import ServerCard from '../components/ServerCard'

function StatCard({ icon: Icon, label, value, color }: {
  icon: React.ElementType; label: string; value: number; color: string
}) {
  return (
    <div className="bg-slate-800 rounded-xl border border-slate-700 p-4 flex items-center gap-4">
      <div className={`p-2.5 rounded-lg ${color}`}>
        <Icon className="w-5 h-5" />
      </div>
      <div>
        <div className="text-2xl font-bold text-slate-100">{value}</div>
        <div className="text-sm text-slate-400">{label}</div>
      </div>
    </div>
  )
}

export default function Dashboard() {
  const [servers, setServers] = useState<ServerWithMetrics[]>([])
  const [stats, setStats] = useState<DashboardStats>({
    total_servers: 0, online_servers: 0, active_alerts: 0, total_rules: 0,
  })

  const load = useCallback(async () => {
    const [srvs, st] = await Promise.all([api.listServers(), api.getStats()])
    setServers(srvs || [])
    setStats(st)
  }, [])

  useEffect(() => { load() }, [load])

  useWebSocket((msg) => {
    if (msg.type === 'metrics') {
      const { server_id, metrics } = msg.data
      setServers((prev) =>
        prev.map((s) =>
          s.server.id === server_id ? { ...s, metrics } : s
        )
      )
    }
    if (msg.type === 'server_status') {
      const { server_id, status } = msg.data
      setServers((prev) =>
        prev.map((s) =>
          s.server.id === server_id
            ? { ...s, server: { ...s.server, status } }
            : s
        )
      )
      // refresh stats
      api.getStats().then(setStats)
    }
    if (msg.type === 'alert' || msg.type === 'alert_resolved') {
      api.getStats().then(setStats)
    }
  })

  const offline = stats.total_servers - stats.online_servers

  return (
    <div>
      <h1 className="text-2xl font-bold text-slate-100 mb-6">Dashboard</h1>

      {/* Stats */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <StatCard icon={Server} label="Total Servers" value={stats.total_servers} color="bg-blue-500/10 text-blue-400" />
        <StatCard icon={Wifi} label="Online" value={stats.online_servers} color="bg-emerald-500/10 text-emerald-400" />
        <StatCard icon={WifiOff} label="Offline" value={offline} color="bg-slate-500/10 text-slate-400" />
        <StatCard icon={AlertTriangle} label="Active Alerts" value={stats.active_alerts} color="bg-red-500/10 text-red-400" />
      </div>

      {/* Server grid */}
      {servers.length === 0 ? (
        <div className="text-center py-20 text-slate-500">
          <Server className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p className="text-lg">No servers yet</p>
          <p className="text-sm mt-1">Go to Servers page to add one</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {servers.map((s) => (
            <ServerCard key={s.server.id} data={s} />
          ))}
        </div>
      )}
    </div>
  )
}
