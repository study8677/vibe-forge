import { useState, useEffect, useCallback } from 'react'
import { Plus, Trash2, Pencil } from 'lucide-react'
import { api, type AlertRule } from '../api/client'

const metricTypes = [
  { value: 'cpu', label: 'CPU %' },
  { value: 'memory', label: 'Memory %' },
  { value: 'swap', label: 'Swap %' },
  { value: 'disk', label: 'Disk %' },
  { value: 'load', label: 'Load Average' },
  { value: 'offline', label: 'Offline' },
]

const operators = ['>', '>=', '<', '<=', '==']

const defaultRule: Partial<AlertRule> = {
  name: '', metric_type: 'cpu', operator: '>', threshold: 90, duration: 60, enabled: true, server_ids: '',
}

export default function AlertRules() {
  const [rules, setRules] = useState<AlertRule[]>([])
  const [showForm, setShowForm] = useState(false)
  const [editing, setEditing] = useState<Partial<AlertRule>>(defaultRule)
  const [isEdit, setIsEdit] = useState(false)

  const load = useCallback(async () => {
    const list = await api.listAlertRules()
    setRules(list || [])
  }, [])

  useEffect(() => { load() }, [load])

  const openCreate = () => {
    setEditing({ ...defaultRule })
    setIsEdit(false)
    setShowForm(true)
  }

  const openEdit = (rule: AlertRule) => {
    setEditing({ ...rule })
    setIsEdit(true)
    setShowForm(true)
  }

  const handleSave = async () => {
    if (!editing.name?.trim() || !editing.metric_type) return
    if (isEdit && editing.id) {
      await api.updateAlertRule(editing.id, editing)
    } else {
      await api.createAlertRule(editing)
    }
    setShowForm(false)
    load()
  }

  const handleDelete = async (id: number, name: string) => {
    if (!confirm(`Delete rule "${name}"?`)) return
    await api.deleteAlertRule(id)
    load()
  }

  const toggleEnabled = async (rule: AlertRule) => {
    await api.updateAlertRule(rule.id, { ...rule, enabled: !rule.enabled })
    load()
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-slate-100">Alert Rules</h1>
        <button onClick={openCreate}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors">
          <Plus className="w-4 h-4" /> Add Rule
        </button>
      </div>

      {/* Form modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={() => setShowForm(false)}>
          <div className="bg-slate-800 rounded-xl border border-slate-700 p-6 w-full max-w-md mx-4" onClick={(e) => e.stopPropagation()}>
            <h2 className="text-lg font-bold text-slate-100 mb-4">{isEdit ? 'Edit Rule' : 'Create Rule'}</h2>
            <div className="space-y-3">
              <div>
                <label className="block text-sm text-slate-400 mb-1">Name</label>
                <input value={editing.name || ''} onChange={(e) => setEditing({ ...editing, name: e.target.value })}
                  className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500" autoFocus />
              </div>
              <div>
                <label className="block text-sm text-slate-400 mb-1">Metric Type</label>
                <select value={editing.metric_type || 'cpu'} onChange={(e) => setEditing({ ...editing, metric_type: e.target.value })}
                  className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500">
                  {metricTypes.map((mt) => <option key={mt.value} value={mt.value}>{mt.label}</option>)}
                </select>
              </div>
              {editing.metric_type !== 'offline' && (
                <div className="grid grid-cols-2 gap-3">
                  <div>
                    <label className="block text-sm text-slate-400 mb-1">Operator</label>
                    <select value={editing.operator || '>'} onChange={(e) => setEditing({ ...editing, operator: e.target.value })}
                      className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500">
                      {operators.map((op) => <option key={op} value={op}>{op}</option>)}
                    </select>
                  </div>
                  <div>
                    <label className="block text-sm text-slate-400 mb-1">Threshold</label>
                    <input type="number" value={editing.threshold ?? 90}
                      onChange={(e) => setEditing({ ...editing, threshold: Number(e.target.value) })}
                      className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500" />
                  </div>
                </div>
              )}
              <div>
                <label className="block text-sm text-slate-400 mb-1">Duration (seconds)</label>
                <input type="number" value={editing.duration ?? 60}
                  onChange={(e) => setEditing({ ...editing, duration: Number(e.target.value) })}
                  className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500" />
              </div>
              <div>
                <label className="block text-sm text-slate-400 mb-1">Server IDs (comma-separated, empty = all)</label>
                <input value={editing.server_ids || ''} onChange={(e) => setEditing({ ...editing, server_ids: e.target.value })}
                  placeholder="e.g. 1,2,3"
                  className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 text-sm focus:outline-none focus:border-blue-500" />
              </div>
              <div className="flex gap-2 pt-2">
                <button onClick={() => setShowForm(false)}
                  className="flex-1 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg text-sm">Cancel</button>
                <button onClick={handleSave} disabled={!editing.name?.trim()}
                  className="flex-1 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white rounded-lg text-sm font-medium">Save</button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Rules table */}
      <div className="bg-slate-800 rounded-xl border border-slate-700 overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-slate-700 text-left">
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Name</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Condition</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden sm:table-cell">Duration</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase hidden md:table-cell">Scope</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Status</th>
              <th className="px-4 py-3 text-xs font-medium text-slate-400 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody>
            {rules.length === 0 ? (
              <tr><td colSpan={6} className="px-4 py-10 text-center text-slate-500">No alert rules</td></tr>
            ) : (
              rules.map((r) => (
                <tr key={r.id} className="border-b border-slate-700/50 hover:bg-slate-700/30">
                  <td className="px-4 py-3 font-medium text-slate-100">{r.name}</td>
                  <td className="px-4 py-3 text-sm text-slate-300">
                    {r.metric_type === 'offline' ? 'Offline Detection' : `${r.metric_type} ${r.operator} ${r.threshold}`}
                  </td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden sm:table-cell">{r.duration}s</td>
                  <td className="px-4 py-3 text-sm text-slate-400 hidden md:table-cell">
                    {r.server_ids ? `Servers: ${r.server_ids}` : 'All servers'}
                  </td>
                  <td className="px-4 py-3">
                    <button onClick={() => toggleEnabled(r)}
                      className={`text-xs font-medium px-2 py-0.5 rounded-full ${
                        r.enabled ? 'bg-emerald-400/10 text-emerald-400' : 'bg-slate-600/30 text-slate-500'
                      }`}>
                      {r.enabled ? 'Enabled' : 'Disabled'}
                    </button>
                  </td>
                  <td className="px-4 py-3">
                    <div className="flex items-center gap-1">
                      <button onClick={() => openEdit(r)} className="p-1.5 rounded-lg hover:bg-slate-700 text-slate-400 hover:text-slate-200">
                        <Pencil className="w-4 h-4" />
                      </button>
                      <button onClick={() => handleDelete(r.id, r.name)} className="p-1.5 rounded-lg hover:bg-red-500/10 text-slate-400 hover:text-red-400">
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
