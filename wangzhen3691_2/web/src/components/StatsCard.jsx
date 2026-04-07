const themes = {
  cyan:   { ring: 'ring-cyan-500/20',   bg: 'bg-cyan-500/5',   text: 'text-cyan-400'   },
  red:    { ring: 'ring-red-500/20',    bg: 'bg-red-500/5',    text: 'text-red-400'    },
  green:  { ring: 'ring-emerald-500/20', bg: 'bg-emerald-500/5', text: 'text-emerald-400' },
  purple: { ring: 'ring-violet-500/20', bg: 'bg-violet-500/5', text: 'text-violet-400' },
  amber:  { ring: 'ring-amber-500/20',  bg: 'bg-amber-500/5',  text: 'text-amber-400'  },
}

export default function StatsCard({ title, value, sub, color = 'cyan' }) {
  const t = themes[color] || themes.cyan
  return (
    <div className={`rounded-xl ring-1 ${t.ring} ${t.bg} p-5`}>
      <p className="text-dark-400 text-xs font-medium">{title}</p>
      <p className={`text-3xl font-bold mt-1.5 tracking-tight ${t.text}`}>
        {value ?? '—'}
      </p>
      {sub && <p className="text-dark-500 text-[11px] mt-1">{sub}</p>}
    </div>
  )
}
