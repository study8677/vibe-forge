import { useStore } from '../../store'
import { ServerCard } from './ServerCard'
import { ClusterGroup } from './ClusterGroup'
import { Plus, Monitor, Network } from 'lucide-react'
import type { ServerConfig } from '../../types'

export function Sidebar() {
  const servers = useStore(s => s.servers)
  const viewMode = useStore(s => s.viewMode)
  const setViewMode = useStore(s => s.setViewMode)
  const setShowServerForm = useStore(s => s.setShowServerForm)
  const setEditingServer = useStore(s => s.setEditingServer)

  const handleAdd = () => {
    setEditingServer(null)
    setShowServerForm(true)
  }

  // Group by cluster
  const clusters = new Map<string, ServerConfig[]>()
  const standalone: ServerConfig[] = []

  for (const server of servers) {
    if (server.cluster) {
      const list = clusters.get(server.cluster) || []
      list.push(server)
      clusters.set(server.cluster, list)
    } else {
      standalone.push(server)
    }
  }

  return (
    <div className="h-full flex flex-col">
      <div className="p-3 border-b border-slate-700">
        <div className="flex items-center justify-between mb-3">
          <span className="text-xs font-medium text-slate-400 uppercase tracking-wider">Servers</span>
          <button
            onClick={handleAdd}
            className="p-1.5 hover:bg-slate-600 rounded transition-colors text-slate-400 hover:text-slate-200"
            title="Add Server"
          >
            <Plus size={16} />
          </button>
        </div>

        {/* View Mode */}
        <div className="flex bg-slate-900 rounded-lg p-0.5">
          <button
            onClick={() => setViewMode('single')}
            className={`flex-1 flex items-center justify-center gap-1.5 py-1.5 px-2 rounded text-xs transition-colors ${
              viewMode === 'single' ? 'bg-slate-700 text-white' : 'text-slate-400 hover:text-slate-300'
            }`}
          >
            <Monitor size={13} />
            Single
          </button>
          <button
            onClick={() => setViewMode('cluster')}
            className={`flex-1 flex items-center justify-center gap-1.5 py-1.5 px-2 rounded text-xs transition-colors ${
              viewMode === 'cluster' ? 'bg-slate-700 text-white' : 'text-slate-400 hover:text-slate-300'
            }`}
          >
            <Network size={13} />
            Cluster
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-2 space-y-1">
        {viewMode === 'cluster' ? (
          <>
            {Array.from(clusters.entries()).map(([name, srvs]) => (
              <ClusterGroup key={name} name={name} servers={srvs} />
            ))}
            {standalone.length > 0 && (
              <>
                {clusters.size > 0 && (
                  <div className="text-xs text-slate-500 px-2 pt-3 pb-1">Standalone</div>
                )}
                {standalone.map(s => <ServerCard key={s.id} server={s} />)}
              </>
            )}
          </>
        ) : (
          servers.map(s => <ServerCard key={s.id} server={s} />)
        )}

        {servers.length === 0 && (
          <div className="text-center py-8 text-slate-500">
            <Monitor size={32} className="mx-auto mb-2 opacity-40" />
            <p className="text-sm">No servers configured</p>
            <button onClick={handleAdd} className="text-xs text-blue-400 hover:text-blue-300 mt-1">
              Add your first server
            </button>
          </div>
        )}
      </div>
    </div>
  )
}
