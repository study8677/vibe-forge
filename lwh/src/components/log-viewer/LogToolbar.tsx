import { useStore } from '../../store'
import {
  Search, Trash2, ArrowDownToLine, Pause,
  AlertCircle, AlertTriangle, Info, Bug, Sparkles
} from 'lucide-react'

export function LogToolbar() {
  const logFilter = useStore(s => s.logFilter)
  const setLogFilter = useStore(s => s.setLogFilter)
  const logLevelFilter = useStore(s => s.logLevelFilter)
  const toggleLevelFilter = useStore(s => s.toggleLevelFilter)
  const autoScroll = useStore(s => s.autoScroll)
  const toggleAutoScroll = useStore(s => s.toggleAutoScroll)
  const clearLogs = useStore(s => s.clearLogs)
  const logs = useStore(s => s.logs)
  const setAIChatOpen = useStore(s => s.setAIChatOpen)
  const aiChatOpen = useStore(s => s.aiChatOpen)

  const levels = [
    { key: 'error', label: 'Error', icon: AlertCircle, color: 'text-red-400', bg: 'bg-red-500/20' },
    { key: 'warn', label: 'Warn', icon: AlertTriangle, color: 'text-amber-400', bg: 'bg-amber-500/20' },
    { key: 'info', label: 'Info', icon: Info, color: 'text-blue-400', bg: 'bg-blue-500/20' },
    { key: 'debug', label: 'Debug', icon: Bug, color: 'text-slate-400', bg: 'bg-slate-500/20' },
  ]

  return (
    <div className="flex items-center gap-2 px-3 py-1.5 bg-slate-900 border-b border-slate-800">
      <div className="relative flex-1 max-w-xs">
        <Search size={13} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-slate-500" />
        <input
          value={logFilter}
          onChange={e => setLogFilter(e.target.value)}
          placeholder="Filter logs..."
          className="w-full bg-slate-800 border border-slate-700 rounded pl-8 pr-3 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-blue-500"
        />
      </div>

      <div className="flex items-center gap-1">
        {levels.map(l => {
          const active = logLevelFilter.has(l.key)
          return (
            <button
              key={l.key}
              onClick={() => toggleLevelFilter(l.key)}
              className={`flex items-center gap-1 px-2 py-1 rounded text-[10px] font-medium transition-colors ${
                active ? `${l.bg} ${l.color}` : 'text-slate-600 hover:text-slate-400'
              }`}
              title={l.label}
            >
              <l.icon size={11} />
              {l.label}
            </button>
          )
        })}
      </div>

      <div className="flex-1" />

      <span className="text-[10px] text-slate-500">{logs.length} lines</span>

      <button
        onClick={() => setAIChatOpen(!aiChatOpen)}
        className={`p-1.5 rounded transition-colors ${
          aiChatOpen ? 'bg-purple-500/20 text-purple-400' : 'text-slate-400 hover:text-purple-400 hover:bg-slate-800'
        }`}
        title="AI Analysis"
      >
        <Sparkles size={14} />
      </button>

      <button
        onClick={toggleAutoScroll}
        className={`p-1.5 rounded transition-colors ${
          autoScroll ? 'bg-blue-500/20 text-blue-400' : 'text-slate-400 hover:text-slate-300 hover:bg-slate-800'
        }`}
        title={autoScroll ? 'Auto-scroll ON' : 'Auto-scroll OFF'}
      >
        {autoScroll ? <ArrowDownToLine size={14} /> : <Pause size={14} />}
      </button>

      <button
        onClick={clearLogs}
        className="p-1.5 rounded text-slate-400 hover:text-red-400 hover:bg-slate-800 transition-colors"
        title="Clear logs"
      >
        <Trash2 size={14} />
      </button>
    </div>
  )
}
