import { useState } from 'react'
import { scanContent } from '../api'

const badge = {
  info:     'bg-blue-500/20 text-blue-400',
  warning:  'bg-amber-500/20 text-amber-400',
  danger:   'bg-orange-500/20 text-orange-400',
  critical: 'bg-red-500/20 text-red-400',
}

export default function Scanner() {
  const [text, setText] = useState('')
  const [source, setSource] = useState('manual')
  const [result, setResult] = useState(null)
  const [busy, setBusy] = useState(false)

  const scan = async () => {
    if (!text.trim()) return
    setBusy(true)
    try { setResult(await scanContent(text, source)) } finally { setBusy(false) }
  }

  return (
    <div>
      <h2 className="text-xl font-bold text-dark-100 mb-6">内容扫描</h2>

      {/* input */}
      <div className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-6 space-y-4 mb-6">
        <div className="flex gap-4 items-end">
          <div>
            <label className="text-[11px] text-dark-500 block mb-1">来源标签</label>
            <select
              value={source}
              onChange={(e) => setSource(e.target.value)}
              className="bg-dark-800 border border-dark-700 rounded-lg px-3 py-2 text-sm focus:border-cyan-500 focus:outline-none"
            >
              <option value="manual">手动输入</option>
              <option value="comment">评论区</option>
              <option value="danmaku">弹幕</option>
              <option value="chat">聊天消息</option>
              <option value="article">文章/动态</option>
            </select>
          </div>
        </div>

        <div>
          <label className="text-[11px] text-dark-500 block mb-1">待扫描内容</label>
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={7}
            className="w-full bg-dark-800 border border-dark-700 rounded-lg px-4 py-3 text-sm font-mono leading-relaxed focus:border-cyan-500 focus:outline-none resize-y"
            placeholder={'在此粘贴或输入内容…\n\n系统将使用所有活跃插件的规则进行正则匹配扫描。'}
          />
        </div>

        <button
          onClick={scan}
          disabled={busy || !text.trim()}
          className="px-5 py-2.5 rounded-lg text-sm font-medium transition
            bg-cyan-500 hover:bg-cyan-600 disabled:bg-dark-700 disabled:text-dark-500 text-white
            shadow-lg shadow-cyan-500/20"
        >
          {busy ? '扫描中…' : '开始扫描'}
        </button>
      </div>

      {/* result */}
      {result && (
        <div className="bg-dark-900 rounded-xl ring-1 ring-dark-800 p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-sm font-semibold text-dark-200">扫描结果</h3>
            <div className="flex gap-4 text-[11px] text-dark-500">
              <span>耗时 {result.duration}</span>
              <span>插件 {result.plugins_checked}</span>
              <span>预警 {result.alerts?.length || 0}</span>
            </div>
          </div>

          {result.alerts?.length > 0 ? (
            <div className="space-y-3">
              {result.alerts.map((a, i) => (
                <div key={i} className="bg-dark-950 rounded-lg p-4 ring-1 ring-dark-800">
                  <div className="flex items-center gap-2.5 mb-1.5">
                    <span className={`px-2 py-0.5 rounded text-[11px] font-semibold ${badge[a.level] || badge.info}`}>
                      {a.level}
                    </span>
                    <span className="text-sm font-medium text-dark-200">{a.rule_name}</span>
                    <span className="text-[11px] text-dark-500">({a.plugin_name})</span>
                  </div>
                  <p className="text-xs text-amber-300/80 font-mono">match: {a.matched_text}</p>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-10 text-dark-500">
              <span className="text-4xl block mb-3">✅</span>
              <p className="text-sm">扫描完成，未发现违规内容</p>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
