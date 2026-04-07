'use client'

import { getStatus, STATUSES } from '@/lib/constants'
import { ChevronDown } from 'lucide-react'
import { useState, useRef, useEffect } from 'react'

interface StatusBadgeProps {
  status: string
  onChange?: (status: string) => void
  editable?: boolean
}

export default function StatusBadge({ status, onChange, editable = false }: StatusBadgeProps) {
  const [open, setOpen] = useState(false)
  const ref = useRef<HTMLDivElement>(null)
  const s = getStatus(status)

  useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false)
      }
    }
    document.addEventListener('mousedown', handleClick)
    return () => document.removeEventListener('mousedown', handleClick)
  }, [])

  if (!editable) {
    return (
      <span className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${s.color}`}>
        {s.label}
      </span>
    )
  }

  return (
    <div ref={ref} className="relative">
      <button
        onClick={() => setOpen(!open)}
        className={`inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs font-medium transition-colors hover:opacity-80 ${s.color}`}
      >
        {s.label}
        <ChevronDown className="w-3 h-3" />
      </button>
      {open && (
        <div className="absolute top-full left-0 mt-1 bg-white rounded-lg shadow-notion-lg border border-notion-border py-1 z-50 min-w-[120px] animate-scale-in">
          {STATUSES.map((st) => (
            <button
              key={st.id}
              onClick={() => {
                onChange?.(st.id)
                setOpen(false)
              }}
              className={`w-full text-left px-3 py-1.5 text-sm hover:bg-notion-bg-hover transition-colors flex items-center gap-2 ${
                st.id === status ? 'bg-notion-bg-secondary' : ''
              }`}
            >
              <span className={`inline-block w-2 h-2 rounded-full ${st.color.split(' ')[0]}`} />
              {st.label}
            </button>
          ))}
        </div>
      )}
    </div>
  )
}
