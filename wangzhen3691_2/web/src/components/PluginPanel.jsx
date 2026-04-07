import { useState, useEffect } from 'react'
import { fetchPlugins, togglePlugin } from '../api'

export default function PluginPanel() {
  const [plugins, setPlugins] = useState([])
  const load = () => fetchPlugins().then((d) => setPlugins(d || []))
  useEffect(() => { load() }, [])

  const toggle = async (name) => { await togglePlugin(name); load() }

  return (
    <div>
      <h2 className="text-xl font-bold text-dark-100 mb-6">插件系统</h2>

      <div className="grid sm:grid-cols-2 gap-4">
        {plugins.map((p) => (
          <div key={p.name} className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-5 flex flex-col gap-4">
            <div className="flex justify-between items-start gap-3">
              <div className="min-w-0">
                <h3 className="font-semibold text-dark-100 truncate">{p.name}</h3>
                <p className="text-xs text-dark-400 mt-1 line-clamp-2">{p.description}</p>
              </div>
              {/* toggle */}
              <button
                onClick={() => toggle(p.name)}
                className={`relative w-11 h-6 rounded-full shrink-0 transition-colors ${p.enabled ? 'bg-cyan-500' : 'bg-dark-700'}`}
              >
                <span className={`absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform ${p.enabled ? 'translate-x-[22px]' : 'translate-x-1'}`} />
              </button>
            </div>
            <div className="flex gap-5 text-[11px] text-dark-500">
              <span>v{p.version}</span>
              <span>{p.rule_count} 条规则</span>
              <span className={p.enabled ? 'text-emerald-400' : 'text-red-400'}>
                {p.enabled ? '运行中' : '已停用'}
              </span>
            </div>
          </div>
        ))}

        {/* add placeholder */}
        <div className="rounded-xl border-2 border-dashed border-dark-800 p-5 flex flex-col items-center justify-center text-dark-600 hover:border-dark-600 hover:text-dark-400 transition cursor-pointer min-h-[130px]">
          <svg className="w-8 h-8 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
          </svg>
          <span className="text-sm font-medium">加载新插件</span>
          <span className="text-[11px] mt-0.5">支持热加载插件模块</span>
        </div>
      </div>
    </div>
  )
}
