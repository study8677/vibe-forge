import type { ChatMessage as ChatMessageType } from '../../types'
import { User, Bot } from 'lucide-react'

export function ChatMessage({ message }: { message: ChatMessageType }) {
  const isUser = message.role === 'user'

  return (
    <div className="flex gap-2">
      <div className={`shrink-0 w-6 h-6 rounded-full flex items-center justify-center ${
        isUser ? 'bg-blue-600/30' : 'bg-purple-600/30'
      }`}>
        {isUser
          ? <User size={12} className="text-blue-400" />
          : <Bot size={12} className="text-purple-400" />
        }
      </div>
      <div className="flex-1 min-w-0">
        <div className={`text-xs leading-relaxed whitespace-pre-wrap break-words ${
          isUser ? 'text-slate-300' : 'text-slate-200'
        }`}>
          {message.content || (
            <span className="inline-flex items-center gap-1 text-slate-500">
              <span className="animate-pulse">Thinking...</span>
            </span>
          )}
        </div>
      </div>
    </div>
  )
}
