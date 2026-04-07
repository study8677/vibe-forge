'use client'

import { EMOJI_LIST } from '@/lib/constants'
import { Smile, X } from 'lucide-react'
import { useState, useRef, useEffect } from 'react'

interface EmojiPickerProps {
  value: string | null
  onChange: (emoji: string | null) => void
}

export default function EmojiPicker({ value, onChange }: EmojiPickerProps) {
  const [open, setOpen] = useState(false)
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false)
      }
    }
    document.addEventListener('mousedown', handleClick)
    return () => document.removeEventListener('mousedown', handleClick)
  }, [])

  return (
    <div ref={ref} className="relative">
      <button
        onClick={() => setOpen(!open)}
        className="w-10 h-10 flex items-center justify-center rounded hover:bg-notion-bg-hover transition-colors text-xl"
        title="选择图标"
      >
        {value || <Smile className="w-5 h-5 text-notion-text-tertiary" />}
      </button>
      {open && (
        <div className="absolute top-full left-0 mt-1 bg-white rounded-lg shadow-notion-lg border border-notion-border p-2 z-50 w-[280px] animate-scale-in">
          <div className="flex items-center justify-between mb-2 px-1">
            <span className="text-xs font-medium text-notion-text-secondary">选择图标</span>
            {value && (
              <button
                onClick={() => {
                  onChange(null)
                  setOpen(false)
                }}
                className="text-xs text-notion-text-tertiary hover:text-notion-red flex items-center gap-0.5 transition-colors"
              >
                <X className="w-3 h-3" />
                移除
              </button>
            )}
          </div>
          <div className="grid grid-cols-9 gap-0.5">
            {EMOJI_LIST.map((emoji) => (
              <button
                key={emoji}
                onClick={() => {
                  onChange(emoji)
                  setOpen(false)
                }}
                className="w-7 h-7 flex items-center justify-center rounded hover:bg-notion-bg-hover transition-colors text-base"
              >
                {emoji}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
