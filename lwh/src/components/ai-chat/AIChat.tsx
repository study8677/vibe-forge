import { useState, useRef, useEffect } from 'react'
import { useStore } from '../../store'
import { useAI } from '../../hooks/useAI'
import { ChatMessage } from './ChatMessage'
import { ProviderSelect } from './ProviderSelect'
import { Send, X, Trash2 } from 'lucide-react'

export function AIChat() {
  const [input, setInput] = useState('')
  const chatMessages = useStore(s => s.chatMessages)
  const selectedLogText = useStore(s => s.selectedLogText)
  const isAILoading = useStore(s => s.isAILoading)
  const setAIChatOpen = useStore(s => s.setAIChatOpen)
  const clearChat = useStore(s => s.clearChat)
  const setSelectedLogText = useStore(s => s.setSelectedLogText)

  const messagesRef = useRef<HTMLDivElement>(null)
  const { analyze } = useAI()

  useEffect(() => {
    if (messagesRef.current) {
      messagesRef.current.scrollTop = messagesRef.current.scrollHeight
    }
  }, [chatMessages])

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!input.trim() && !selectedLogText) return
    analyze(input.trim())
    setInput('')
  }

  return (
    <div className="flex flex-col h-full bg-slate-800">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b border-slate-700 shrink-0">
        <div className="flex items-center gap-3">
          <span className="text-xs font-medium text-slate-300">AI Analysis</span>
          <ProviderSelect />
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={clearChat}
            className="p-1 hover:bg-slate-700 rounded text-slate-400 hover:text-slate-300 transition-colors"
            title="Clear chat"
          >
            <Trash2 size={13} />
          </button>
          <button
            onClick={() => setAIChatOpen(false)}
            className="p-1 hover:bg-slate-700 rounded text-slate-400 hover:text-slate-300 transition-colors"
          >
            <X size={13} />
          </button>
        </div>
      </div>

      {/* Selected text preview */}
      {selectedLogText && (
        <div className="px-3 py-1.5 bg-slate-900/50 border-b border-slate-700 flex items-start gap-2 shrink-0">
          <span className="text-[10px] text-slate-500 shrink-0 mt-0.5">Selected:</span>
          <pre className="text-[10px] text-slate-400 flex-1 max-h-12 overflow-hidden font-mono truncate">
            {selectedLogText.slice(0, 200)}
          </pre>
          <button
            onClick={() => setSelectedLogText('')}
            className="shrink-0 p-0.5 hover:bg-slate-700 rounded text-slate-500"
          >
            <X size={10} />
          </button>
        </div>
      )}

      {/* Messages */}
      <div ref={messagesRef} className="flex-1 overflow-y-auto p-3 space-y-3">
        {chatMessages.length === 0 && (
          <div className="text-center text-slate-500 text-xs py-4">
            Select log text and ask AI to analyze errors.
            <br />
            Double-click a log line to quick-select.
          </div>
        )}
        {chatMessages.map(msg => (
          <ChatMessage key={msg.id} message={msg} />
        ))}
      </div>

      {/* Input */}
      <form onSubmit={handleSubmit} className="p-2 border-t border-slate-700 flex gap-2 shrink-0">
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder={selectedLogText ? 'Ask about the selected logs...' : 'Select log text first...'}
          className="flex-1 bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-xs focus:outline-none focus:ring-1 focus:ring-blue-500"
          disabled={isAILoading}
        />
        <button
          type="submit"
          disabled={isAILoading || (!input.trim() && !selectedLogText)}
          className="px-3 py-2 bg-blue-600 hover:bg-blue-500 rounded-lg text-xs transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Send size={14} />
        </button>
      </form>
    </div>
  )
}
