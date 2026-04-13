import { memo } from 'react'
import type { LogEntry } from '../../types'

const levelColors: Record<string, string> = {
  error: 'text-red-400',
  warn: 'text-amber-400',
  info: 'text-blue-400',
  debug: 'text-slate-500',
  unknown: 'text-slate-400',
}

const levelBadges: Record<string, string> = {
  error: 'bg-red-500/20 text-red-400',
  warn: 'bg-amber-500/20 text-amber-400',
  info: 'bg-blue-500/20 text-blue-400',
  debug: 'bg-slate-500/20 text-slate-500',
  unknown: '',
}

export const LogLine = memo(function LogLine({ entry }: { entry: LogEntry }) {
  const time = new Date(entry.timestamp).toLocaleTimeString('en-US', { hour12: false })

  return (
    <div className={`log-line-${entry.level} flex items-start px-3 py-0.5 hover:bg-white/5 group`}>
      <span className="text-slate-600 shrink-0 w-20 select-none">{time}</span>
      {entry.level !== 'unknown' && (
        <span className={`shrink-0 w-14 text-center text-[10px] font-semibold uppercase rounded px-1 py-0.5 mr-2 ${levelBadges[entry.level]}`}>
          {entry.level}
        </span>
      )}
      <span className={`flex-1 break-all whitespace-pre-wrap ${levelColors[entry.level]}`}>
        {entry.raw}
      </span>
    </div>
  )
})
