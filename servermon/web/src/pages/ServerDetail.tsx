import { useState, useEffect, useCallback } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowLeft, Cpu, HardDrive, MemoryStick, Activity, ArrowDown, ArrowUp, Clock } from 'lucide-react'
import { api, type ServerWithMetrics, type MetricsHistory, formatBytes, formatSpeed, formatUptime, pct } from '../api/client'
import { useWebSocket } from '../hooks/useWebSocket'
import MetricsChart from '../components/MetricsChart'

export default function ServerDetail() {
  const { id } = useParams()
  const navigate = useNavigate()
  const serverId = Number(id)
  const [server, setServer] = useState<ServerWithMetrics | null>(null)
  const [history, setHistory] = useState<MetricsHistory[]>([])
  const [duration, setDuration] = useState('1h')

  const load = useCallback(async () => {
    const [srv, hist] = await Promise.all([
      api.getServer(serverId),
      api.getServerMetrics(serverId, duration),
    ])
    setServer(srv)
    setHistory(hist || [])
  }, [serverId, duration])

  useEffect(() => { load() }, [load])

  useWebSocket((msg) => {
    if (msg.type === 'metrics' && msg.data.server_id === serverId) {
      setServer((prev) => prev ? { ...prev, metrics: msg.data.metrics } : prev)
    }
  })

  if (!server) {
    return <div className="text-slate-400 py-10 text-center">Loading...</div>
  }

  const { server: s, metrics: m } = server
  const online = s.status === 1

  const chartData = history.map((h) => ({
    time: new Date(h.created_at * 1000).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
    cpu: Number(h.cpu.toFixed(1)),
    mem: m ? Number((h.mem_used / (m.mem_total || 1) * 100).toFixed(1)) : 0,
    disk: m ? Number((h.disk_used / (m.disk_total || 1) * 100).toFixed(1)) : 0,
    netIn: h.net_in_speed,
    netOut: h.net_out_speed,
    load: Number(h.load1.toFixed(2)),
  }))

  return (
    <div>
      {/* Header */}
      <div className="flex items-center gap-3 mb-6">
        <button onClick={() => navigate(-1)} className="p-1.5 rounded-lg hover:bg-slate-800 text-slate-400">
          <ArrowLeft className="w-5 h-5" />
        </button>
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-2xl font-bold text-slate-100">{s.name}</h1>
            <span className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${
              online ? 'bg-emerald-400/10 text-emerald-400' : 'bg-red-400/10 text-red-400'
            }`}>
              <span className={`w-1.5 h-1.5 rounded-full ${online ? 'bg-emerald-400' : 'bg-red-400'}`} />
              {online ? 'Online' : 'Offline'}
            </span>
          </div>
          <p className="text-sm text-slate-400 mt-0.5">
            {s.platform} {s.arch && `| ${s.arch}`} {s.cpu_info && `| ${s.cpu_info}`}
          </p>
        </div>
      </div>

      {/* Current metrics */}
      {m && (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3 mb-6">
          <MetricBox icon={Cpu} label="CPU" value={`${m.cpu.toFixed(1)}%`}
            color={m.cpu > 90 ? 'text-red-400' : m.cpu > 70 ? 'text-amber-400' : 'text-blue-400'} />
          <MetricBox icon={MemoryStick} label="Memory"
            value={`${pct(m.mem_used, m.mem_total)}%`}
            sub={`${formatBytes(m.mem_used)} / ${formatBytes(m.mem_total)}`}
            color="text-violet-400" />
          <MetricBox icon={HardDrive} label="Disk"
            value={`${pct(m.disk_used, m.disk_total)}%`}
            sub={`${formatBytes(m.disk_used)} / ${formatBytes(m.disk_total)}`}
            color="text-amber-400" />
          <MetricBox icon={ArrowDown} label="Net In" value={formatSpeed(m.net_in_speed)} color="text-emerald-400" />
          <MetricBox icon={ArrowUp} label="Net Out" value={formatSpeed(m.net_out_speed)} color="text-blue-400" />
          <MetricBox icon={Clock} label="Uptime" value={formatUptime(m.uptime)} color="text-slate-300" />
        </div>
      )}

      {/* Extra info row */}
      {m && (
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-6">
          <InfoBox label="Load Avg" value={`${m.load1.toFixed(2)} / ${m.load5.toFixed(2)} / ${m.load15.toFixed(2)}`} />
          <InfoBox label="Processes" value={String(m.process_count)} />
          <InfoBox label="TCP Conns" value={String(m.tcp_count)} />
          <InfoBox label="Total Traffic"
            value={`${formatBytes(m.net_in_total)} / ${formatBytes(m.net_out_total)}`} />
        </div>
      )}

      {/* Duration selector */}
      <div className="flex items-center gap-2 mb-4">
        <span className="text-sm text-slate-400">Period:</span>
        {['1h', '6h', '24h', '7d'].map((d) => (
          <button
            key={d}
            onClick={() => setDuration(d)}
            className={`px-3 py-1 text-sm rounded-lg transition-colors ${
              duration === d
                ? 'bg-blue-600 text-white'
                : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
            }`}
          >
            {d}
          </button>
        ))}
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <MetricsChart
          title="CPU Usage"
          data={chartData}
          lines={[{ key: 'cpu', color: '#3b82f6', name: 'CPU %' }]}
          unit="%"
        />
        <MetricsChart
          title="Memory Usage"
          data={chartData}
          lines={[{ key: 'mem', color: '#8b5cf6', name: 'Memory %' }]}
          unit="%"
        />
        <MetricsChart
          title="Network Speed"
          data={chartData}
          lines={[
            { key: 'netIn', color: '#10b981', name: 'In' },
            { key: 'netOut', color: '#3b82f6', name: 'Out' },
          ]}
          yFormatter={(v) => formatBytes(v)}
        />
        <MetricsChart
          title="System Load"
          data={chartData}
          lines={[{ key: 'load', color: '#f59e0b', name: 'Load 1m' }]}
        />
      </div>
    </div>
  )
}

function MetricBox({ icon: Icon, label, value, sub, color }: {
  icon: React.ElementType; label: string; value: string; sub?: string; color: string
}) {
  return (
    <div className="bg-slate-800 rounded-xl border border-slate-700 p-3">
      <div className="flex items-center gap-1.5 mb-1">
        <Icon className={`w-4 h-4 ${color}`} />
        <span className="text-xs text-slate-400">{label}</span>
      </div>
      <div className={`text-lg font-bold ${color}`}>{value}</div>
      {sub && <div className="text-xs text-slate-500 mt-0.5">{sub}</div>}
    </div>
  )
}

function InfoBox({ label, value }: { label: string; value: string }) {
  return (
    <div className="bg-slate-800 rounded-xl border border-slate-700 p-3">
      <div className="text-xs text-slate-400 mb-1">{label}</div>
      <div className="text-sm font-medium text-slate-200">{value}</div>
    </div>
  )
}
