'use client'

import { useState, useEffect, useCallback, useRef } from 'react'
import { useRouter } from 'next/navigation'
import {
  Eye,
  SplitSquareHorizontal,
  Pencil,
  Trash2,
  Pin,
  PinOff,
  Clock,
  Check,
  Loader2,
} from 'lucide-react'
import { format } from 'date-fns'
import { zhCN } from 'date-fns/locale'
import MarkdownRenderer from './MarkdownRenderer'
import StatusBadge from './StatusBadge'
import EmojiPicker from './EmojiPicker'
import { emitNoteChange } from '@/lib/events'

interface Note {
  id: string
  title: string
  content: string
  status: string
  emoji: string | null
  pinned: boolean
  createdAt: string
  updatedAt: string
}

type ViewMode = 'edit' | 'preview' | 'split'

export default function NoteEditor({ initialNote }: { initialNote: Note }) {
  const [title, setTitle] = useState(initialNote.title)
  const [content, setContent] = useState(initialNote.content)
  const [status, setStatus] = useState(initialNote.status)
  const [emoji, setEmoji] = useState(initialNote.emoji)
  const [pinned, setPinned] = useState(initialNote.pinned)
  const [viewMode, setViewMode] = useState<ViewMode>('split')
  const [saveState, setSaveState] = useState<'idle' | 'saving' | 'saved'>('idle')
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const saveTimerRef = useRef<NodeJS.Timeout>()
  const router = useRouter()

  // Refs for latest values (used in cleanup save)
  const titleRef = useRef(title)
  const contentRef = useRef(content)
  const statusRef = useRef(status)
  const emojiRef = useRef(emoji)
  const pinnedRef = useRef(pinned)

  useEffect(() => { titleRef.current = title }, [title])
  useEffect(() => { contentRef.current = content }, [content])
  useEffect(() => { statusRef.current = status }, [status])
  useEffect(() => { emojiRef.current = emoji }, [emoji])
  useEffect(() => { pinnedRef.current = pinned }, [pinned])

  const save = useCallback(
    async (data: Partial<Note>) => {
      setSaveState('saving')
      try {
        await fetch(`/api/notes/${initialNote.id}`, {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(data),
        })
        setSaveState('saved')
        emitNoteChange()
        setTimeout(() => setSaveState('idle'), 1500)
      } catch {
        setSaveState('idle')
      }
    },
    [initialNote.id]
  )

  // Debounced auto-save
  useEffect(() => {
    if (saveTimerRef.current) clearTimeout(saveTimerRef.current)
    saveTimerRef.current = setTimeout(() => {
      save({ title, content, status, emoji, pinned })
    }, 600)
    return () => {
      if (saveTimerRef.current) clearTimeout(saveTimerRef.current)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [title, content, status, emoji, pinned])

  // Save on unmount
  useEffect(() => {
    return () => {
      fetch(`/api/notes/${initialNote.id}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: titleRef.current,
          content: contentRef.current,
          status: statusRef.current,
          emoji: emojiRef.current,
          pinned: pinnedRef.current,
        }),
      })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [initialNote.id])

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto'
      textareaRef.current.style.height = textareaRef.current.scrollHeight + 'px'
    }
  }, [content, viewMode])

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Tab') {
      e.preventDefault()
      const ta = e.currentTarget
      const start = ta.selectionStart
      const end = ta.selectionEnd
      setContent(content.substring(0, start) + '  ' + content.substring(end))
      requestAnimationFrame(() => {
        if (textareaRef.current) {
          textareaRef.current.selectionStart = start + 2
          textareaRef.current.selectionEnd = start + 2
        }
      })
    }
  }

  const handleDelete = async () => {
    await fetch(`/api/notes/${initialNote.id}`, { method: 'DELETE' })
    emitNoteChange()
    router.push('/')
  }

  const viewModes: { mode: ViewMode; icon: typeof Pencil; label: string }[] = [
    { mode: 'edit', icon: Pencil, label: '编辑' },
    { mode: 'split', icon: SplitSquareHorizontal, label: '分栏' },
    { mode: 'preview', icon: Eye, label: '预览' },
  ]

  return (
    <div className="flex flex-col h-full animate-fade-in">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 md:px-8 py-2 border-b border-notion-border-light shrink-0">
        <div className="flex items-center gap-2">
          <StatusBadge status={status} onChange={setStatus} editable />
          <button
            onClick={() => setPinned(!pinned)}
            className="p-1.5 rounded hover:bg-notion-bg-hover transition-colors"
            title={pinned ? '取消置顶' : '置顶'}
          >
            {pinned ? (
              <PinOff className="w-3.5 h-3.5 text-notion-orange" />
            ) : (
              <Pin className="w-3.5 h-3.5 text-notion-text-tertiary" />
            )}
          </button>
          {/* Save indicator */}
          <div className="flex items-center gap-1 text-xs text-notion-text-tertiary">
            {saveState === 'saving' && (
              <>
                <Loader2 className="w-3 h-3 animate-spin" />
                <span>保存中...</span>
              </>
            )}
            {saveState === 'saved' && (
              <>
                <Check className="w-3 h-3 text-notion-green" />
                <span className="text-notion-green">已保存</span>
              </>
            )}
          </div>
        </div>
        <div className="flex items-center gap-1">
          {/* View mode toggle */}
          <div className="flex items-center bg-notion-bg-secondary rounded-md p-0.5">
            {viewModes.map(({ mode, icon: Icon, label }) => (
              <button
                key={mode}
                onClick={() => setViewMode(mode)}
                className={`p-1.5 rounded transition-colors ${
                  viewMode === mode
                    ? 'bg-white shadow-notion text-notion-text'
                    : 'text-notion-text-tertiary hover:text-notion-text-secondary'
                }`}
                title={label}
              >
                <Icon className="w-3.5 h-3.5" />
              </button>
            ))}
          </div>
          {/* Delete */}
          <div className="relative ml-1">
            <button
              onClick={() => setShowDeleteConfirm(!showDeleteConfirm)}
              className="p-1.5 rounded hover:bg-notion-red-bg transition-colors text-notion-text-tertiary hover:text-notion-red"
              title="删除"
            >
              <Trash2 className="w-3.5 h-3.5" />
            </button>
            {showDeleteConfirm && (
              <div className="absolute right-0 top-full mt-1 bg-white rounded-lg shadow-notion-lg border border-notion-border p-3 z-50 w-[200px] animate-scale-in">
                <p className="text-sm text-notion-text mb-2">确定要删除这篇笔记吗？</p>
                <div className="flex gap-2">
                  <button
                    onClick={() => setShowDeleteConfirm(false)}
                    className="flex-1 px-3 py-1.5 text-xs rounded-md bg-notion-bg-secondary hover:bg-notion-bg-active transition-colors"
                  >
                    取消
                  </button>
                  <button
                    onClick={handleDelete}
                    className="flex-1 px-3 py-1.5 text-xs rounded-md bg-notion-red text-white hover:opacity-90 transition-opacity"
                  >
                    删除
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Editor Area */}
      <div className="flex-1 overflow-hidden flex">
        {/* Edit Pane */}
        {(viewMode === 'edit' || viewMode === 'split') && (
          <div
            className={`flex-1 overflow-y-auto ${
              viewMode === 'split' ? 'border-r border-notion-border-light' : ''
            }`}
          >
            <div className="max-w-3xl mx-auto px-4 md:px-12 py-6 md:py-10">
              {/* Emoji */}
              <EmojiPicker value={emoji} onChange={setEmoji} />
              {/* Title */}
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="无标题"
                className="note-title-input w-full bg-transparent border-none outline-none mb-4"
                autoFocus={!initialNote.title}
              />
              {/* Meta */}
              <div className="flex items-center gap-3 mb-6 text-xs text-notion-text-tertiary">
                <span className="flex items-center gap-1">
                  <Clock className="w-3 h-3" />
                  创建于 {format(new Date(initialNote.createdAt), 'yyyy/MM/dd HH:mm', { locale: zhCN })}
                </span>
              </div>
              {/* Content */}
              <textarea
                ref={textareaRef}
                value={content}
                onChange={(e) => setContent(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="输入 Markdown 内容..."
                className="note-editor-textarea w-full bg-transparent border-none outline-none min-h-[300px]"
              />
            </div>
          </div>
        )}

        {/* Preview Pane */}
        {(viewMode === 'preview' || viewMode === 'split') && (
          <div className="flex-1 overflow-y-auto">
            <div className="max-w-3xl mx-auto px-4 md:px-12 py-6 md:py-10">
              {viewMode === 'preview' && (
                <>
                  {emoji && <div className="text-4xl mb-2">{emoji}</div>}
                  <h1 className="text-title text-notion-text mb-4">
                    {title || <span className="text-notion-text-placeholder">无标题</span>}
                  </h1>
                </>
              )}
              {viewMode === 'split' && (
                <div className="text-xs text-notion-text-tertiary mb-4 uppercase tracking-wider font-medium">
                  预览
                </div>
              )}
              <MarkdownRenderer content={content} />
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
