import { useState } from 'react'
import { useStore } from '../../store'
import { X } from 'lucide-react'

export function ServerForm() {
  const editingServer = useStore(s => s.editingServer)
  const setShowServerForm = useStore(s => s.setShowServerForm)
  const addServer = useStore(s => s.addServer)
  const updateServer = useStore(s => s.updateServer)

  const [form, setForm] = useState({
    name: editingServer?.name || '',
    host: editingServer?.host || '',
    port: editingServer?.port || 22,
    username: editingServer?.username || 'root',
    authType: (editingServer?.authType || 'password') as 'password' | 'key',
    password: editingServer?.password || '',
    privateKey: editingServer?.privateKey || '',
    passphrase: editingServer?.passphrase || '',
    cluster: editingServer?.cluster || '',
  })

  const [saving, setSaving] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)

    try {
      if (editingServer) {
        const res = await fetch(`/api/servers/${editingServer.id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(form),
        })
        const data = await res.json()
        updateServer(editingServer.id, data)
      } else {
        const res = await fetch('/api/servers', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ ...form, services: [] }),
        })
        const data = await res.json()
        addServer(data)
      }
      setShowServerForm(false)
    } catch (err: any) {
      alert(err.message)
    } finally {
      setSaving(false)
    }
  }

  const close = () => setShowServerForm(false)

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50" onClick={close}>
      <div
        className="bg-slate-800 rounded-xl w-full max-w-md shadow-2xl border border-slate-700"
        onClick={e => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-slate-700">
          <h2 className="text-sm font-semibold">
            {editingServer ? 'Edit Server' : 'Add Server'}
          </h2>
          <button onClick={close} className="p-1 hover:bg-slate-700 rounded">
            <X size={16} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-4 space-y-3">
          <div className="grid grid-cols-2 gap-3">
            <label className="col-span-2">
              <span className="text-xs text-slate-400 mb-1 block">Server Name</span>
              <input
                value={form.name}
                onChange={e => setForm(f => ({ ...f, name: e.target.value }))}
                placeholder="My Server"
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                required
              />
            </label>

            <label>
              <span className="text-xs text-slate-400 mb-1 block">Host</span>
              <input
                value={form.host}
                onChange={e => setForm(f => ({ ...f, host: e.target.value }))}
                placeholder="192.168.1.100"
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                required
              />
            </label>

            <label>
              <span className="text-xs text-slate-400 mb-1 block">Port</span>
              <input
                type="number"
                value={form.port}
                onChange={e => setForm(f => ({ ...f, port: parseInt(e.target.value) || 22 }))}
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </label>

            <label>
              <span className="text-xs text-slate-400 mb-1 block">Username</span>
              <input
                value={form.username}
                onChange={e => setForm(f => ({ ...f, username: e.target.value }))}
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                required
              />
            </label>

            <label>
              <span className="text-xs text-slate-400 mb-1 block">Cluster (optional)</span>
              <input
                value={form.cluster}
                onChange={e => setForm(f => ({ ...f, cluster: e.target.value }))}
                placeholder="production"
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </label>
          </div>

          <div>
            <span className="text-xs text-slate-400 mb-2 block">Authentication</span>
            <div className="flex gap-2 mb-2">
              <button
                type="button"
                onClick={() => setForm(f => ({ ...f, authType: 'password' }))}
                className={`px-3 py-1.5 text-xs rounded-lg transition-colors ${
                  form.authType === 'password' ? 'bg-blue-600 text-white' : 'bg-slate-700 text-slate-400'
                }`}
              >
                Password
              </button>
              <button
                type="button"
                onClick={() => setForm(f => ({ ...f, authType: 'key' }))}
                className={`px-3 py-1.5 text-xs rounded-lg transition-colors ${
                  form.authType === 'key' ? 'bg-blue-600 text-white' : 'bg-slate-700 text-slate-400'
                }`}
              >
                SSH Key
              </button>
            </div>

            {form.authType === 'password' ? (
              <input
                type="password"
                value={form.password}
                onChange={e => setForm(f => ({ ...f, password: e.target.value }))}
                placeholder="Password"
                className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            ) : (
              <div className="space-y-2">
                <textarea
                  value={form.privateKey}
                  onChange={e => setForm(f => ({ ...f, privateKey: e.target.value }))}
                  placeholder="Paste private key or enter path (e.g. ~/.ssh/id_rsa)"
                  className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500 h-24 resize-none font-mono text-xs"
                />
                <input
                  type="password"
                  value={form.passphrase}
                  onChange={e => setForm(f => ({ ...f, passphrase: e.target.value }))}
                  placeholder="Passphrase (optional)"
                  className="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
            )}
          </div>

          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={close}
              className="px-4 py-2 text-sm text-slate-400 hover:text-slate-300 transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={saving}
              className="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 rounded-lg transition-colors disabled:opacity-50"
            >
              {saving ? 'Saving...' : editingServer ? 'Update' : 'Add Server'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
