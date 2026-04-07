'use client'

import { useState, useEffect, useCallback } from 'react'
import { useRouter } from 'next/navigation'
import {
  DragDropContext,
  Droppable,
  Draggable,
  type DropResult,
} from '@hello-pangea/dnd'
import { Plus } from 'lucide-react'
import { STATUSES } from '@/lib/constants'
import { emitNoteChange, onNoteChange } from '@/lib/events'
import NoteCard from './NoteCard'

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

const COLUMN_COLORS: Record<string, string> = {
  backlog: 'bg-notion-bg-active',
  todo: 'bg-notion-blue-bg',
  in_progress: 'bg-notion-orange-bg',
  done: 'bg-notion-green-bg',
}

export default function KanbanBoard() {
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

  const handleDragEnd = async (result: DropResult) => {
    if (!result.destination) return

    const { source, destination, draggableId } = result

    // Optimistic update
    const newNotes = [...notes]
    const noteIdx = newNotes.findIndex((n) => n.id === draggableId)
    if (noteIdx === -1) return

    const movedNote = { ...newNotes[noteIdx] }
    movedNote.status = destination.droppableId

    // Remove from old position
    newNotes.splice(noteIdx, 1)

    // Get destination column notes (excluding the moved one)
    const destNotes = newNotes
      .filter((n) => n.status === destination.droppableId)
      .sort((a, b) => a.order - b.order)

    // Insert at new position
    destNotes.splice(destination.index, 0, movedNote)

    // Update orders
    destNotes.forEach((n, i) => {
      n.order = i
    })

    // Rebuild full list
    const otherNotes = newNotes.filter((n) => n.status !== destination.droppableId)
    setNotes([...otherNotes, ...destNotes])

    // Persist
    await fetch('/api/notes/reorder', {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        noteId: draggableId,
        newStatus: destination.droppableId,
        newIndex: destination.index,
      }),
    })

    if (source.droppableId !== destination.droppableId) {
      emitNoteChange()
    }
  }

  const handleAddNote = async (status: string) => {
    const res = await fetch('/api/notes', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ status }),
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
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="px-4 md:px-8 py-4 border-b border-notion-border-light shrink-0">
        <h1 className="text-xl font-bold text-notion-text">任务看板</h1>
        <p className="text-sm text-notion-text-tertiary mt-0.5">
          拖拽卡片来管理笔记状态
        </p>
      </div>

      {/* Board */}
      <div className="flex-1 overflow-x-auto p-4 md:p-6">
        <DragDropContext onDragEnd={handleDragEnd}>
          <div className="flex gap-4 h-full min-w-max">
            {STATUSES.map((col) => {
              const columnNotes = notes
                .filter((n) => n.status === col.id)
                .sort((a, b) => a.order - b.order)

              return (
                <div
                  key={col.id}
                  className="w-[280px] flex flex-col shrink-0 rounded-xl bg-notion-bg-secondary"
                >
                  {/* Column header */}
                  <div className="px-3 py-3 flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <div
                        className={`w-2.5 h-2.5 rounded-full ${COLUMN_COLORS[col.id]}`}
                      />
                      <span className="text-sm font-medium text-notion-text">
                        {col.label}
                      </span>
                      <span className="text-xs text-notion-text-tertiary bg-notion-bg-tertiary px-1.5 py-0.5 rounded">
                        {columnNotes.length}
                      </span>
                    </div>
                    <button
                      onClick={() => handleAddNote(col.id)}
                      className="p-1 rounded hover:bg-notion-bg-hover transition-colors text-notion-text-tertiary hover:text-notion-text-secondary"
                    >
                      <Plus className="w-4 h-4" />
                    </button>
                  </div>

                  {/* Cards */}
                  <Droppable droppableId={col.id}>
                    {(provided, snapshot) => (
                      <div
                        ref={provided.innerRef}
                        {...provided.droppableProps}
                        className={`flex-1 px-2 pb-2 space-y-2 kanban-column overflow-y-auto transition-colors rounded-b-xl ${
                          snapshot.isDraggingOver ? 'bg-notion-bg-tertiary/50' : ''
                        }`}
                      >
                        {columnNotes.map((note, index) => (
                          <Draggable
                            key={note.id}
                            draggableId={note.id}
                            index={index}
                          >
                            {(provided, snapshot) => (
                              <div
                                ref={provided.innerRef}
                                {...provided.draggableProps}
                                {...provided.dragHandleProps}
                                className={snapshot.isDragging ? 'kanban-card-dragging' : ''}
                              >
                                <NoteCard
                                  note={note}
                                  onClick={() => router.push(`/notes/${note.id}`)}
                                  variant="kanban"
                                />
                              </div>
                            )}
                          </Draggable>
                        ))}
                        {provided.placeholder}
                        {columnNotes.length === 0 && !snapshot.isDraggingOver && (
                          <div className="py-8 text-center">
                            <p className="text-xs text-notion-text-placeholder">
                              暂无笔记
                            </p>
                          </div>
                        )}
                      </div>
                    )}
                  </Droppable>
                </div>
              )
            })}
          </div>
        </DragDropContext>
      </div>
    </div>
  )
}
