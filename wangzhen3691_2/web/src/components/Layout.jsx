const nav = [
  { id: 'dashboard', label: '监控面板', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-4 0h4' },
  { id: 'alerts',    label: '预警中心', icon: 'M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9' },
  { id: 'rules',     label: '规则管理', icon: 'M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01' },
  { id: 'plugins',   label: '插件系统', icon: 'M17 14v6m-3-3h6M6 10h2a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2zm10 0h2a2 2 0 002-2V6a2 2 0 00-2-2h-2a2 2 0 00-2 2v2a2 2 0 002 2zM6 20h2a2 2 0 002-2v-2a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2z' },
  { id: 'scanner',   label: '内容扫描', icon: 'M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z' },
]

function Icon({ d }) {
  return (
    <svg className="w-5 h-5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d={d} />
    </svg>
  )
}

export default function Layout({ page, setPage, connected, children }) {
  return (
    <div className="flex h-screen overflow-hidden">
      {/* ---- sidebar ---- */}
      <aside className="w-60 bg-dark-900 border-r border-dark-800 flex flex-col shrink-0">
        {/* logo */}
        <div className="px-5 py-6 border-b border-dark-800">
          <h1 className="text-lg font-bold tracking-wide bg-gradient-to-r from-cyan-400 to-blue-500 bg-clip-text text-transparent">
            始皇防蛐蛐
          </h1>
          <p className="text-[11px] text-dark-500 mt-0.5">ShiHuang Guard · v1.0</p>
        </div>

        {/* nav */}
        <nav className="flex-1 px-3 py-4 space-y-0.5 overflow-y-auto">
          {nav.map((n) => (
            <button
              key={n.id}
              onClick={() => setPage(n.id)}
              className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-[13px] transition-all duration-150
                ${page === n.id
                  ? 'bg-cyan-500/10 text-cyan-400 shadow-[inset_0_0_0_1px_rgba(34,211,238,.15)]'
                  : 'text-dark-400 hover:text-dark-200 hover:bg-dark-800'}`}
            >
              <Icon d={n.icon} />
              {n.label}
            </button>
          ))}
        </nav>

        {/* status */}
        <div className="px-5 py-4 border-t border-dark-800 text-[11px] flex items-center gap-2">
          <span className={`w-1.5 h-1.5 rounded-full ${connected ? 'bg-emerald-400' : 'bg-red-400 animate-pulse'}`} />
          <span className="text-dark-500">{connected ? 'WebSocket 已连接' : '连接中…'}</span>
        </div>
      </aside>

      {/* ---- main ---- */}
      <main className="flex-1 overflow-y-auto bg-dark-950">
        <div className="max-w-[1400px] mx-auto px-8 py-8">
          {children}
        </div>
      </main>
    </div>
  )
}
