import { useState, useEffect } from 'react'
import { fetchStats } from '../api'
import StatsCard from './StatsCard'

const levelStyle = {
  info:     'bg-blue-500/10 text-blue-400 border-blue-500/15',
  warning:  'bg-amber-500/10 text-amber-400 border-amber-500/15',
  danger:   'bg-orange-500/10 text-orange-400 border-orange-500/15',
  critical: 'bg-red-500/10 text-red-400 border-red-500/15 alert-critical',
}

export default function Dashboard({ alerts, stats: wsStats }) {
  const [stats, setStats] = useState(null)

  useEffect(() => {
    fetchStats().then(setStats)
    const t = setInterval(() => fetchStats().then(setStats), 5000)
    return () => clearInterval(t)
  }, [])

  const s = wsStats || stats
  if (!s) return <p className="text-dark-500 animate-pulse">loading…</p>

  return (
    <div>
      <h2 className="text-xl font-bold mb-6 text-dark-100">监控面板</h2>

      {/* stat cards */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <StatsCard title="总扫描次数" value={s.total_scans} color="cyan" />
        <StatsCard title="触发预警" value={s.total_alerts} color="red" />
        <StatsCard title="活跃规则" value={s.active_rules} color="green" />
        <StatsCard title="活跃插件" value={s.active_plugins} color="purple" />
      </div>

      <div className="grid lg:grid-cols-2 gap-6">
        {/* live alert stream */}
        <section className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-5">
          <h3 className="text-sm font-semibold text-dark-200 mb-4">实时预警流</h3>
          <div className="space-y-2.5 max-h-[420px] overflow-y-auto pr-1">
            {alerts.length === 0 ? (
              <p className="text-dark-600 text-sm py-8 text-center">暂无预警 — 系统正常运行中</p>
            ) : alerts.slice(0, 30).map((a, i) => (
              <div key={i} className={`px-3.5 py-2.5 rounded-lg border text-sm ${levelStyle[a.level] || levelStyle.info}`}>
                <div className="flex justify-between items-start gap-2">
                  <span className="font-medium leading-snug">{a.rule_name}</span>
                  <span className="text-[10px] opacity-60 shrink-0 tabular-nums">
                    {new Date(a.timestamp).toLocaleTimeString('zh-CN')}
                  </span>
                </div>
                <p className="text-xs mt-1 opacity-75 font-mono truncate">
                  match: {a.matched_text}
                </p>
              </div>
            ))}
          </div>
        </section>

        {/* system info */}
        <section className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-5">
          <h3 className="text-sm font-semibold text-dark-200 mb-4">系统状态</h3>
          <dl className="divide-y divide-dark-800">
            {[
              ['扫描速率',   <><span className="text-cyan-400 tabular-nums">{s.scan_rate?.toFixed(1)}</span> <span className="text-dark-500">/ min</span></>],
              ['总预警数',   <span className="text-red-400 tabular-nums">{s.total_alerts}</span>],
              ['引擎版本',   'v1.0.0'],
              ['核心插件',   <span className="text-amber-400">始皇防蛐蛐</span>],
              ['架构',      <span className="text-dark-300">Go · Plugin · WebSocket</span>],
            ].map(([k, v], i) => (
              <div key={i} className="flex justify-between py-3 text-sm">
                <dt className="text-dark-400">{k}</dt>
                <dd>{v}</dd>
              </div>
            ))}
          </dl>
        </section>
      </div>
    </div>
  )
}
