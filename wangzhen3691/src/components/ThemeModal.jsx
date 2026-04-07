import { X, Check } from 'lucide-react'

export default function ThemeModal({ current, themes, onChange, onClose }) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={onClose}>
      <div
        className="bg-bg-primary border border-border rounded-lg shadow-xl w-80"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-4 py-3 border-b border-border">
          <h3 className="text-sm font-semibold text-primary">选择主题</h3>
          <button onClick={onClose} className="text-muted hover:text-primary">
            <X size={16} />
          </button>
        </div>
        <div className="p-2">
          {themes.map((theme) => (
            <button
              key={theme.id}
              onClick={() => { onChange(theme.id); onClose() }}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded transition-colors ${
                current === theme.id ? 'bg-bg-selected text-accent' : 'text-primary/80 hover:bg-bg-hover'
              }`}
            >
              <span className="text-sm flex-1 text-left">{theme.name}</span>
              {current === theme.id && <Check size={14} className="text-accent" />}
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}
