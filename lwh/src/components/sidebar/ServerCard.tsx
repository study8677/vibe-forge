import { useState } from 'react'
import { useStore } from '../../store'
import { useSocket } from '../../hooks/useSocket'
import type { ServerConfig, ServiceConfig } from '../../types'
import {
  Server, ChevronDown, ChevronRight, Plug, Unplug,
  Settings, Trash2, RefreshCw, Circle
} from 'lucide-react'

const typeIcons: Record<string, string> = {
  docker: '\u{1F433}',
  journalctl: '\u{1F4CB}',
  pm2: '\u26A1',
  file: '\u{1F4C4}',
}

export function ServerCard({ server }: { server: ServerConfig }) {
  const [expanded, setExpanded] = useState(false)
  const [loading, setLoading] = useState(false)
  const [services, setServices] = useState<ServiceConfig[]>(server.services || [])

  const activeServerId = useStore(s => s.activeServerId)
  const activeService = useStore(s => s.activeService)
  const connectedServers = useStore(s => s.connectedServers)
  const setActiveServer = useStore(s => s.setActiveServer)
  const setActiveService = useStore(s => s.setActiveService)
  const setConnected = useStore(s => s.setConnected)
  const removeServer = useStore(s => s.removeServer)
  const setShowServerForm = useStore(s => s.setShowServerForm)
  const setEditingServer = useStore(s => s.setEditingServer)
  const updateServer = useStore(s => s.updateServer)

  const { subscribe, unsubscribe } = useSocket()

  const isConnected = connectedServers.has(server.id)
  const isActive = activeServerId === server.id

  const handleConnect = async () => {
    setLoading(true)
    try {
      const res = await fetch(`/api/servers/${server.id}/connect`, { method: 'POST' })
      if (!res.ok) throw new Error((await res.json()).error)
      setConnected(server.id, true)

      const svcRes = await fetch(`/api/servers/${server.id}/services`)
      if (svcRes.ok) {
        const detected = await svcRes.json()
        setServices(detected)
        updateServer(server.id, { services: detected })
      }
      setExpanded(true)
    } catch (err: any) {
      alert(`Connection failed: ${err.message}`)
    } finally {
      setLoading(false)
    }
  }

  const handleDisconnect = async () => {
    await fetch(`/api/servers/${server.id}/disconnect`, { method: 'POST' })
    setConnected(server.id, false)
    if (isActive) {
      unsubscribe(server.id)
      setActiveServer(null)
    }
  }

  const handleDelete = async () => {
    if (!confirm(`Delete server "${server.name}"?`)) return
    await fetch(`/api/servers/${server.id}`, { method: 'DELETE' })
    removeServer(server.id)
    if (isActive) setActiveServer(null)
  }

  const handleEdit = () => {
    setEditingServer(server)
    setShowServerForm(true)
  }

  const handleServiceClick = (svc: ServiceConfig) => {
    if (!isConnected) return
    if (isActive) unsubscribe(server.id)
    setActiveServer(server.id)
    setActiveService(svc)
    subscribe(server.id, svc)
  }

  return (
    <div className={`rounded-lg overflow-hidden transition-colors ${
      isActive ? 'bg-slate-700/50 ring-1 ring-blue-500/30' : 'hover:bg-slate-700/30'
    }`}>
      {/* Header */}
      <div
        className="flex items-center gap-2 px-2.5 py-2 cursor-pointer"
        onClick={() => setExpanded(!expanded)}
      >
        <span className="text-slate-400">
          {expanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
        </span>
        <Circle
          size={8}
          className={isConnected ? 'fill-green-400 text-green-400' : 'fill-slate-600 text-slate-600'}
        />
        <Server size={14} className="text-slate-400" />
        <span className="flex-1 text-sm truncate">{server.name}</span>

        <div className="flex items-center gap-0.5" onClick={e => e.stopPropagation()}>
          {!isConnected ? (
            <button
              onClick={handleConnect}
              disabled={loading}
              className="p-1 hover:bg-slate-600 rounded text-slate-400 hover:text-green-400 transition-colors"
              title="Connect"
            >
              {loading ? <RefreshCw size={13} className="animate-spin" /> : <Plug size={13} />}
            </button>
          ) : (
            <button
              onClick={handleDisconnect}
              className="p-1 hover:bg-slate-600 rounded text-slate-400 hover:text-red-400 transition-colors"
              title="Disconnect"
            >
              <Unplug size={13} />
            </button>
          )}
          <button
            onClick={handleEdit}
            className="p-1 hover:bg-slate-600 rounded text-slate-400 hover:text-slate-200 transition-colors"
            title="Edit"
          >
            <Settings size={13} />
          </button>
          <button
            onClick={handleDelete}
            className="p-1 hover:bg-slate-600 rounded text-slate-400 hover:text-red-400 transition-colors"
            title="Delete"
          >
            <Trash2 size={13} />
          </button>
        </div>
      </div>

      {/* Services */}
      {expanded && (
        <div className="pb-1.5 px-2.5">
          <div className="text-xs text-slate-500 mb-1 px-2">
            {server.host}:{server.port || 22}
          </div>
          {isConnected ? (
            services.length > 0 ? (
              <div className="space-y-0.5">
                {services.map(svc => (
                  <button
                    key={svc.name}
                    onClick={() => handleServiceClick(svc)}
                    className={`w-full text-left text-xs px-2 py-1.5 rounded transition-colors truncate ${
                      isActive && activeService?.name === svc.name
                        ? 'bg-blue-600/30 text-blue-300'
                        : 'text-slate-400 hover:bg-slate-600/50 hover:text-slate-300'
                    }`}
                  >
                    <span className="mr-1">{typeIcons[svc.type] || '\u{1F4C4}'}</span>
                    {svc.name}
                  </button>
                ))}
              </div>
            ) : (
              <div className="text-xs text-slate-500 px-2 py-1">No services detected</div>
            )
          ) : (
            <div className="text-xs text-slate-500 px-2 py-1">Connect to view services</div>
          )}
        </div>
      )}
    </div>
  )
}
