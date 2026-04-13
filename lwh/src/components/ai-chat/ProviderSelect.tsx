import { useStore } from '../../store'
import type { AIProvider } from '../../types'

const providers: { id: AIProvider; label: string }[] = [
  { id: 'claude', label: 'Claude' },
  { id: 'gemini', label: 'Gemini' },
]

export function ProviderSelect() {
  const aiProvider = useStore(s => s.aiProvider)
  const setAIProvider = useStore(s => s.setAIProvider)

  return (
    <div className="flex bg-slate-900 rounded p-0.5 gap-0.5">
      {providers.map(p => (
        <button
          key={p.id}
          onClick={() => setAIProvider(p.id)}
          className={`px-2 py-0.5 rounded text-[10px] font-medium transition-colors ${
            aiProvider === p.id
              ? 'bg-slate-700 text-white'
              : 'text-slate-500 hover:text-slate-400'
          }`}
        >
          {p.label}
        </button>
      ))}
    </div>
  )
}
