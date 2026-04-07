'use client'

import { useState, useEffect, useCallback } from 'react'
import { useRouter, usePathname } from 'next/navigation'
import {
  FileText,
  Search,
  Plus,
  LayoutGrid,
  Pin,
  ChevronLeft,
  Menu,
} from 'lucide-react'
import { onNoteChange, emitNoteChange } from '@/lib/events'

interface Note {
  id: string
  title: string
  emoji: string | null
  pinned: boolean
  status: string
  updatedAt: string
}

export default function Sidebar() {
  const [notes, setNotes] = useState<Note[]>([])
  const [search, setSearch] = useState('')
  const [collapsed, setCollapsed] = useState(false)
  const [mobileOpen, setMobileOpen] = useState(false)
  const router = useRouter()
  const pathname = usePathname()

  const fetchNotes = useCallback(async () => {
    const params = search ? `?search=${encodeURIComponent(search)}` : ''
    const res = await fetch(`/api/notes${params}`)
    if (res.ok) setNotes(await res.json())
  }, [search])

  useEffect(() => {
    fetchNotes()
  }, [fetchNotes])

  useEffect(() => {
    return onNoteChange(() => fetchNotes())
  }, [fetchNotes])

  const handleNewNote = async () => {
    const res = await fetch('/api/notes', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({}),
    })
    const note = await res.json()
    emitNoteChange()
    router.push(`/notes/${note.id}`)
    setMobileOpen(false)
  }

  const activeNoteId = pathname.startsWith('/notes/') ? pathname.split('/')[2] : null

  if (collapsed) {
    return (
      <>
        <div className="hidden md:flex flex-col items-center w-12 bg-notion-bg-secondary border-r border-notion-border py-3 gap-2 shrink-0">
          <button
            onClick={() => setCollapsed(false)}
            className="p-2 rounded hover:bg-notion-bg-hover transition-colors"
          >
            <Menu className="w-4 h-4 text-notion-text-secondary" />
          </button>
          <button
            onClick={handleNewNote}
            className="p-2 rounded hover:bg-notion-bg-hover transition-colors"
          >
            <Plus className="w-4 h-4 text-notion-text-secondary" />
          </button>
        </div>
        {/* Mobile toggle */}
        <button
          onClick={() => {
            setCollapsed(false)
            setMobileOpen(true)
          }}
          className="md:hidden fixed top-3 left-3 z-50 p-2 rounded-lg bg-white shadow-notion-md border border-notion-border"
        >
          <Menu className="w-4 h-4" />
        </button>
      </>
    )
  }

  const sidebarContent = (
    <div className="flex flex-col h-full bg-notion-bg-secondary">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-3 border-b border-notion-border-light">
        <div className="flex items-center gap-2">
          <div className="w-6 h-6 rounded bg-gradient-to-br from-notion-blue to-notion-purple flex items-center justify-center">
            <span className="text-white text-xs font-bold">S</span>
          </div>
          <span className="text-sm font-semibold text-notion-text">Sora 笔记</span>
        </div>
        <button
          onClick={() => {
            setCollapsed(true)
            setMobileOpen(false)
          }}
          className="p-1 rounded hover:bg-notion-bg-hover transition-colors"
        >
          <ChevronLeft className="w-4 h-4 text-notion-text-tertiary" />
        </button>
      </div>

      {/* Search */}
      <div className="px-3 py-2">
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-notion-text-tertiary" />
          <input
            type="text"
            placeholder="搜索笔记..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full pl-8 pr-3 py-1.5 text-sm bg-notion-bg-tertiary rounded-md border-none outline-none placeholder:text-notion-text-placeholder focus:ring-1 focus:ring-notion-blue/30 transition-all"
          />
        </div>
      </div>

      {/* New Note */}
      <div className="px-3 pb-2">
        <button
          onClick={handleNewNote}
          className="w-full flex items-center gap-2 px-2.5 py-1.5 text-sm text-notion-text-secondary hover:bg-notion-bg-hover rounded-md transition-colors"
        >
          <Plus className="w-4 h-4" />
          新建笔记
        </button>
      </div>

      {/* Navigation */}
      <div className="px-3 pb-2 space-y-0.5">
        <button
          onClick={() => {
            router.push('/')
            setMobileOpen(false)
          }}
          className={`w-full flex items-center gap-2 px-2.5 py-1.5 text-sm rounded-md transition-colors ${
            pathname === '/'
              ? 'bg-notion-bg-active text-notion-text font-medium'
              : 'text-notion-text-secondary hover:bg-notion-bg-hover'
          }`}
        >
          <FileText className="w-4 h-4" />
          所有笔记
        </button>
        <button
          onClick={() => {
            router.push('/kanban')
            setMobileOpen(false)
          }}
          className={`w-full flex items-center gap-2 px-2.5 py-1.5 text-sm rounded-md transition-colors ${
            pathname === '/kanban'
              ? 'bg-notion-bg-active text-notion-text font-medium'
              : 'text-notion-text-secondary hover:bg-notion-bg-hover'
          }`}
        >
          <LayoutGrid className="w-4 h-4" />
          任务看板
        </button>
      </div>

      {/* Divider */}
      <div className="px-3">
        <div className="border-t border-notion-border-light" />
      </div>

      {/* Notes List */}
      <div className="flex-1 overflow-y-auto px-3 py-2 space-y-0.5">
        {notes.length === 0 ? (
          <div className="px-2.5 py-8 text-center">
            <p className="text-sm text-notion-text-placeholder">
              {search ? '未找到匹配的笔记' : '还没有笔记'}
            </p>
          </div>
        ) : (
          notes.map((note) => (
            <button
              key={note.id}
              onClick={() => {
                router.push(`/notes/${note.id}`)
                setMobileOpen(false)
              }}
              className={`w-full flex items-center gap-2 px-2.5 py-1.5 text-sm rounded-md transition-colors group ${
                activeNoteId === note.id
                  ? 'bg-notion-bg-active text-notion-text'
                  : 'text-notion-text-secondary hover:bg-notion-bg-hover'
              }`}
            >
              <span className="shrink-0 w-5 text-center">
                {note.emoji || <FileText className="w-4 h-4 inline" />}
              </span>
              <span className="truncate flex-1 text-left">
                {note.title || '无标题'}
              </span>
              {note.pinned && (
                <Pin className="w-3 h-3 shrink-0 text-notion-text-tertiary" />
              )}
            </button>
          ))
        )}
      </div>
    </div>
  )

  return (
    <>
      {/* Desktop sidebar */}
      <div
        className="hidden md:block shrink-0 h-full border-r border-notion-border"
        style={{ width: 'var(--sidebar-width)' }}
      >
        {sidebarContent}
      </div>

      {/* Mobile sidebar */}
      <button
        onClick={() => setMobileOpen(true)}
        className="md:hidden fixed top-3 left-3 z-40 p-2 rounded-lg bg-white shadow-notion-md border border-notion-border"
      >
        <Menu className="w-4 h-4" />
      </button>
      {mobileOpen && (
        <>
          <div
            className="md:hidden fixed inset-0 bg-black/20 z-40"
            onClick={() => setMobileOpen(false)}
          />
          <div className="md:hidden fixed inset-y-0 left-0 z-50 w-[280px] shadow-notion-lg sidebar-enter">
            {sidebarContent}
          </div>
        </>
      )}
    </>
  )
}
