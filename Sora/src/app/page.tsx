'use client'

import { useState, useEffect, useCallback } from 'react'
import { useRouter } from 'next/navigation'
import { Plus, FileText } from 'lucide-react'
import { emitNoteChange, onNoteChange } from '@/lib/events'
import NoteCard from '@/components/NoteCard'

interface Note {
  id: string
  title: string
  content: string
  status: string
  emoji: string | null
  pinned: boolean
  order: number
  updatedAt: string
}

export default function HomePage() {
  const [notes, setNotes] = useState<Note[]>([])
  const [loading, setLoading] = useState(true)
  const router = useRouter()

  const fetchNotes = useCallback(async () => {
    const res = await fetch('/api/notes')
    if (res.ok) {
      setNotes(await res.json())
      setLoading(false)
    }
  }, [])

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
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-notion-text-tertiary text-sm">加载中...</div>
      </div>
    )
  }

  return (
    <div className="h-full overflow-y-auto">
      <div className="max-w-5xl mx-auto px-4 md:px-8 py-6 md:py-10">
        {/* Header */}
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-2xl font-bold text-notion-text">所有笔记</h1>
            <p className="text-sm text-notion-text-tertiary mt-1">
              {notes.length} 篇笔记
            </p>
          </div>
          <button
            onClick={handleNewNote}
            className="flex items-center gap-2 px-4 py-2 bg-notion-text text-white rounded-lg text-sm font-medium hover:opacity-90 transition-opacity"
          >
            <Plus className="w-4 h-4" />
            新建笔记
          </button>
        </div>

        {/* Notes Grid */}
        {notes.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20">
            <div className="w-16 h-16 rounded-2xl bg-notion-bg-secondary flex items-center justify-center mb-4">
              <FileText className="w-8 h-8 text-notion-text-tertiary" />
            </div>
            <h2 className="text-lg font-medium text-notion-text mb-1">
              还没有笔记
            </h2>
            <p className="text-sm text-notion-text-tertiary mb-4">
              创建你的第一篇笔记，支持 Markdown 语法
            </p>
            <button
              onClick={handleNewNote}
              className="flex items-center gap-2 px-4 py-2 bg-notion-blue text-white rounded-lg text-sm font-medium hover:opacity-90 transition-opacity"
            >
              <Plus className="w-4 h-4" />
              新建笔记
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            {notes.map((note) => (
              <NoteCard
                key={note.id}
                note={note}
                onClick={() => router.push(`/notes/${note.id}`)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
