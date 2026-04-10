import { useState, useEffect, useCallback } from 'react'
import { Plus, Copy, Trash2, Check, Server as ServerIcon } from 'lucide-react'
import { api, type ServerWithMetrics, formatTime } from '../api/client'

export default function Servers() {
  const [servers, setServers] = useState<ServerWithMetrics[]>([])
  const [showAdd, setShowAdd] = useState(false)
  const [newName, setNewName] = useState('')
  const [copiedId, setCopiedId] = useState<number | null>(null)
  const [newServer, setNewServer] = useState<{ id: number; name: string; secret_key: string } | null>(null)

  const load = useCallback(async () => {
    const list = await api.listServers()
    setServers(list || [])
  }, [])

  useEffect(() => { load() }, [load])

  const handleCreate = async () => {
    if (!newName.trim()) return
    const srv = await api.createServer(newName.trim())
    setNewServer({ id: srv.id, name: srv.name, secret_key: srv.secret_key || '' })
    setNewName('')
    load()
  }

  const handleDelete = async (id: number, name: string) => {
    if (!confirm(`Delete server "${name}"?`)) return
    await api.deleteServer(id)
    load()
  }

  const copyCommand = (secretKey: string, id: number) => {
    const cmd = `./agent -s http://YOUR_SERVER:8080 -k ${secretKey}`
    navigator.clipboard.writeText(cmd)
    setCopiedId(id)
    setTimeout(() => setCopiedId(null), 2000)
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-slate-100">Servers</h1>
        <button
          onClick={() => { setShowAdd(true); setNewServer(null) }}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors"
        >
          <Plus className="w-4 h-4" /> Add Server
        </button>
      </div>

      {/* Add server modal */}
      {showAdd && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={() => setShowAdd(false)}>
          <div className="bg-slate-800 rounded-xl border border-slate-700 p-6 w-full max-w-md mx-4" onClick={(e) => e.stopPropagation()}>
            <h2 className="text-lg font-bold text-slate-100 mb-4">
              {newServer ? 'Server Created' : 'Add Server'}
            </h2>

            {newServer ? (
              <div className="space-y-4">
                <div>
                  <label className="text-sm text-slate-400">Name</label>
                  <div className="text-slate-100 font-medium">{newServer.name}</div>
                </div>
                <div>
                  <label className="text-sm text-slate-400">Secret Key</label>
                  <div className="bg-slate-900 rounded-lg p-3 font-mono text-sm text-emerald-400 break-all">
                    {newServer.secret_key}
                  </div>
                </div>
                <div>
                  <label className="text-sm text-slate-400">Agent Command</label>
                  <div className="bg-slate-900 rounded-lg p-3 font-mono text-xs text-slate-300 break-all">
                    ./agent -s http://YOUR_SERVER:8080 -k {newServer.secret_key}
                  </div>
                </div>
                <p className="text-xs text-amber-400">
                  Save the secret key now. It won't be shown again for security.
                </p>
                <button
                  onClick={() => setShowAdd(false)}
                  className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium"
                >
                  Done
                </button>
              </div>
            ) : (
              <div className="space-y-4">
                <div>
                  <label className="block text-sm text-slate-400 mb-1">Server Name</label>
                  <input
                    value={newName}
                    onChange={(e) => setNewName(e.target.value)}
                    onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
                    placeholder="e.g. Production Web Server"
                    className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500"
                    autoFocus
                  />
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => setShowAdd(false)}
                    className="flex-1 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg text-sm"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleCreate}
                    disabled={!newName.trim()}
                    className="flex-1 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white rounded-lg text-sm font-medium"
                  >
                    Create
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Server table */}
      <div className="bg-slate-800 rounded-xl border border-slate-700 overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-slate-700 text-left">
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Name</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Status</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden md:table-cell">Platform</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden lg:table-cell">CPU Info</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden sm:table-cell">Last Active</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody>
            {servers.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-10 text-center text-slate-500">
                  <ServerIcon className="w-8 h-8 mx-auto mb-2 opacity-50" />
                  No servers
                </td>
              </tr>
            ) : (
              servers.map(({ server: s }) => (
                <tr key={s.id} className="border-b border-slate-700/50 hover:bg-slate-750 hover:bg-slate-700/30">
                  <td className="px-4 py-3">
                    <span className="font-medium text-slate-100">{s.name}</span>
                    {s.note && <p className="text-xs text-slate-500 mt-0.5">{s.note}</p>}
                  </td>
                  <td className="px-4 py-3">
                    <span className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${
                      s.status === 1 ? 'bg-emerald-400/10 text-emerald-400' : 'bg-red-400/10 text-red-400'
                    }`}>
                      <span className={`w-1.5 h-1.5 rounded-full ${s.status === 1 ? 'bg-emerald-400' : 'bg-red-400'}`} />
                      {s.status === 1 ? 'Online' : 'Offline'}
                    </span>
                  </td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden md:table-cell">{s.platform || '-'}</td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden lg:table-cell truncate max-w-[200px]">{s.cpu_info || '-'}</td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden sm:table-cell">{formatTime(new Date(s.last_active).getTime() / 1000)}</td>
                  <td className="px-4 py-3">
                    <div className="flex items-center gap-1">
                      <button
                        onClick={() => copyCommand(s.secret_key || '', s.id)}
                        title="Copy agent command"
                        className="p-1.5 rounded-lg hover:bg-slate-700 text-slate-400 hover:text-slate-200"
                      >
                        {copiedId === s.id ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                      </button>
                      <button
                        onClick={() => handleDelete(s.id, s.name)}
                        title="Delete"
                        className="p-1.5 rounded-lg hover:bg-red-500/10 text-slate-400 hover:text-red-400"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
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
