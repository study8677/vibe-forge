'use client'

import { FileText, Pin } from 'lucide-react'
import { format } from 'date-fns'
import { zhCN } from 'date-fns/locale'
import StatusBadge from './StatusBadge'

interface Note {
  id: string
  title: string
  content: string
  status: string
  emoji: string | null
  pinned: boolean
  updatedAt: string
}

interface NoteCardProps {
  note: Note
  onClick: () => void
  variant?: 'grid' | 'kanban'
}

export default function NoteCard({ note, onClick, variant = 'grid' }: NoteCardProps) {
  const preview = note.content
    .replace(/[#*`~>\-\[\]()!|]/g, '')
    .slice(0, 120)
    .trim()

  if (variant === 'kanban') {
    return (
      <div
        onClick={onClick}
        className="bg-white rounded-lg border border-notion-border-light p-3 cursor-pointer hover:shadow-notion-hover transition-all group"
      >
        <div className="flex items-start gap-2 mb-1">
          <span className="shrink-0 text-sm">
            {note.emoji || <FileText className="w-4 h-4 text-notion-text-tertiary mt-0.5" />}
          </span>
          <h4 className="text-sm font-medium text-notion-text leading-snug line-clamp-2 flex-1">
            {note.title || '无标题'}
          </h4>
          {note.pinned && <Pin className="w-3 h-3 text-notion-text-tertiary shrink-0 mt-0.5" />}
        </div>
        {preview && (
          <p className="text-xs text-notion-text-tertiary line-clamp-2 ml-6 mb-2">
            {preview}
          </p>
        )}
        <div className="ml-6 text-[10px] text-notion-text-placeholder">
          {format(new Date(note.updatedAt), 'MM/dd HH:mm', { locale: zhCN })}
        </div>
      </div>
    )
  }

  return (
    <div
      onClick={onClick}
      className="bg-white rounded-lg border border-notion-border-light p-4 cursor-pointer hover:shadow-notion-hover hover:border-notion-border transition-all group"
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex items-center gap-2">
          <span className="text-xl">
            {note.emoji || <FileText className="w-5 h-5 text-notion-text-tertiary" />}
          </span>
          <h3 className="font-semibold text-notion-text line-clamp-1">
            {note.title || '无标题'}
          </h3>
        </div>
        <div className="flex items-center gap-1 shrink-0">
          {note.pinned && <Pin className="w-3.5 h-3.5 text-notion-text-tertiary" />}
          <StatusBadge status={note.status} />
        </div>
      </div>
      {preview && (
        <p className="text-sm text-notion-text-secondary line-clamp-2 mb-3">
          {preview}
        </p>
      )}
      <div className="text-xs text-notion-text-tertiary">
        {format(new Date(note.updatedAt), 'yyyy/MM/dd HH:mm', { locale: zhCN })}
      </div>
    </div>
  )
}
