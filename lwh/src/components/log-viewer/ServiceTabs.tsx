import { useStore } from '../../store'
import { useSocket } from '../../hooks/useSocket'
import type { ServiceConfig } from '../../types'

export function ServiceTabs() {
  const servers = useStore(s => s.servers)
  const activeServerId = useStore(s => s.activeServerId)
  const activeService = useStore(s => s.activeService)
  const setActiveService = useStore(s => s.setActiveService)
  const clearLogs = useStore(s => s.clearLogs)

  const { subscribe, unsubscribe } = useSocket()

  const server = servers.find(s => s.id === activeServerId)
  if (!server) return null

  const services = server.services || []
  if (services.length === 0) return null

  const handleSwitch = (svc: ServiceConfig) => {
    if (activeServerId) unsubscribe(activeServerId)
    clearLogs()
    setActiveService(svc)
    subscribe(activeServerId!, svc)
  }

  return (
    <div className="flex items-center gap-1 px-3 py-1.5 bg-slate-900 border-b border-slate-800 overflow-x-auto">
      <span className="text-[10px] text-slate-500 mr-2 shrink-0">{server.name}</span>
      {services.map(svc => (
        <button
          key={svc.name}
          onClick={() => handleSwitch(svc)}
          className={`shrink-0 px-2.5 py-1 rounded text-xs transition-colors ${
            activeService?.name === svc.name
              ? 'bg-blue-600/30 text-blue-300 ring-1 ring-blue-500/30'
              : 'text-slate-400 hover:text-slate-300 hover:bg-slate-800'
          }`}
        >
          {svc.name}
        </button>
      ))}
    </div>
  )
}
