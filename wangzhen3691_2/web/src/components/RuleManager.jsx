import { useState, useEffect } from 'react'
import { fetchRules, createRule, deleteRule, testRule } from '../api'

const levelColor = {
  info: 'text-blue-400', warning: 'text-amber-400', danger: 'text-orange-400', critical: 'text-red-400',
}

const empty = { name: '', pattern: '', level: 'warning', category: '', enabled: true }

export default function RuleManager() {
  const [rules, setRules] = useState([])
  const [open, setOpen] = useState(false)
  const [form, setForm] = useState({ ...empty })
  const [testText, setTestText] = useState('')
  const [testRes, setTestRes] = useState(null)

  const load = () => fetchRules().then((d) => setRules(d || []))
  useEffect(() => { load() }, [])

  const handleCreate = async () => {
    if (!form.name || !form.pattern) return
    await createRule(form)
    setForm({ ...empty })
    setOpen(false)
    load()
  }

  const handleDelete = async (id) => {
    await deleteRule(id)
    load()
  }

  const handleTest = async () => {
    if (!form.pattern) return
    setTestRes(await testRule(form.pattern, testText))
  }

  const set = (k, v) => setForm((f) => ({ ...f, [k]: v }))

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold text-dark-100">规则管理</h2>
        <button
          onClick={() => setOpen(!open)}
          className="px-4 py-2 rounded-lg text-sm font-medium transition
            bg-cyan-500 hover:bg-cyan-600 text-white shadow-lg shadow-cyan-500/20"
        >
          {open ? '取消' : '+ 新建规则'}
        </button>
      </div>

      {/* ---- create form ---- */}
      {open && (
        <div className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-6 mb-6 space-y-5">
          <h3 className="text-sm font-semibold text-dark-200">新建规则</h3>

          <div className="grid sm:grid-cols-2 gap-4">
            <Field label="规则名称" value={form.name} onChange={(v) => set('name', v)} placeholder="例: 垃圾广告检测" />
            <Field label="分类 (category)" value={form.category} onChange={(v) => set('category', v)} placeholder="spam / fraud / …" />
            <Field label="正则表达式" value={form.pattern} onChange={(v) => set('pattern', v)} placeholder="(?i)(加微信|免费领取)" mono />
            <div>
              <label className="text-[11px] text-dark-500 block mb-1">预警级别</label>
              <select
                value={form.level}
                onChange={(e) => set('level', e.target.value)}
                className="w-full bg-dark-800 border border-dark-700 rounded-lg px-3 py-2 text-sm focus:border-cyan-500 focus:outline-none"
              >
                <option value="info">Info</option>
                <option value="warning">Warning</option>
                <option value="danger">Danger</option>
                <option value="critical">Critical</option>
              </select>
            </div>
          </div>

          {/* regex tester */}
          <div className="bg-dark-950 rounded-lg p-4 space-y-3">
            <label className="text-[11px] text-dark-500">正则在线测试</label>
            <div className="flex gap-2">
              <input
                value={testText}
                onChange={(e) => setTestText(e.target.value)}
                placeholder="输入测试文本…"
                className="flex-1 bg-dark-800 border border-dark-700 rounded-lg px-3 py-2 text-sm focus:border-cyan-500 focus:outline-none"
              />
              <button onClick={handleTest} className="px-4 py-2 bg-dark-700 hover:bg-dark-600 rounded-lg text-sm transition">
                测试
              </button>
            </div>
            {testRes && (
              <p className="text-sm">
                {testRes.valid
                  ? testRes.count > 0
                    ? <span className="text-emerald-400">匹配 {testRes.count} 处: <span className="font-mono">{testRes.matches.join(', ')}</span></span>
                    : <span className="text-dark-500">未匹配到内容</span>
                  : <span className="text-red-400">语法错误: {testRes.error}</span>}
              </p>
            )}
          </div>

          <button
            onClick={handleCreate}
            disabled={!form.name || !form.pattern}
            className="px-5 py-2 rounded-lg text-sm font-medium transition
              bg-cyan-500 hover:bg-cyan-600 disabled:bg-dark-700 disabled:text-dark-500 text-white"
          >
            创建
          </button>
        </div>
      )}

      {/* ---- rule table ---- */}
      <div className="bg-dark-900 rounded-xl ring-1 ring-dark-800 overflow-hidden">
        <div className="grid grid-cols-[1fr_170px_80px_90px_60px_60px] gap-3 px-4 py-3 border-b border-dark-800 text-[11px] text-dark-500 uppercase tracking-wider font-semibold">
          <span>名称</span><span>正则</span><span>级别</span><span>分类</span><span>状态</span><span></span>
        </div>
        <div className="max-h-[calc(100vh-360px)] overflow-y-auto divide-y divide-dark-800/60">
          {rules.length === 0
            ? <div className="py-12 text-center text-dark-600 text-sm">暂无规则</div>
            : rules.map((r) => (
              <div key={r.id} className="grid grid-cols-[1fr_170px_80px_90px_60px_60px] gap-3 px-4 py-3 text-sm items-center hover:bg-dark-800/40 transition-colors">
                <span className="text-dark-200 truncate font-medium">{r.name}</span>
                <span className="text-dark-400 font-mono text-xs truncate">{r.pattern}</span>
                <span className={`text-xs ${levelColor[r.level] || 'text-dark-400'}`}>{r.level}</span>
                <span className="text-dark-500 text-xs">{r.category || '—'}</span>
                <span className={`text-xs ${r.enabled ? 'text-emerald-400' : 'text-dark-600'}`}>
                  {r.enabled ? '启用' : '禁用'}
                </span>
                <button onClick={() => handleDelete(r.id)} className="text-xs text-red-400/70 hover:text-red-400 transition">
                  删除
                </button>
              </div>
            ))}
        </div>
      </div>
    </div>
  )
}

function Field({ label, value, onChange, placeholder, mono }) {
  return (
    <div>
      <label className="text-[11px] text-dark-500 block mb-1">{label}</label>
      <input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className={`w-full bg-dark-800 border border-dark-700 rounded-lg px-3 py-2 text-sm focus:border-cyan-500 focus:outline-none ${mono ? 'font-mono' : ''}`}
      />
    </div>
  )
}
