import { Search, Menu, X, Terminal, Rss, Palette } from 'lucide-react'
import { useState } from 'react'

export default function Header({ sidebarOpen, setSidebarOpen, onThemeClick }) {
  const [searchOpen, setSearchOpen] = useState(false)
  const [query, setQuery] = useState('')

  return (
    <header className="sticky top-0 z-50 h-12 bg-bg-secondary border-b border-border flex items-center px-4 gap-3">
      {/* Hamburger */}
      <button
        onClick={() => setSidebarOpen(!sidebarOpen)}
        className="text-primary/70 hover:text-primary transition-colors"
      >
        {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
      </button>

      {/* Logo */}
      <a href="/" className="flex items-center gap-2 shrink-0 no-underline">
        <div className="w-7 h-7 bg-accent rounded flex items-center justify-center">
          <Terminal size={16} className="text-bg-secondary" />
        </div>
        <span className="text-primary font-bold text-sm tracking-wide hidden sm:block">
          LINUX<span className="text-accent">.DO</span>
        </span>
      </a>

      {/* Search */}
      <div className="flex-1 flex justify-center">
        {searchOpen ? (
          <div className="w-full max-w-md flex items-center bg-surface border border-border rounded-md overflow-hidden">
            <Search size={14} className="text-muted ml-3 shrink-0" />
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="搜索话题、用户、标签..."
              className="flex-1 bg-transparent text-primary text-sm px-2 py-1.5 outline-none placeholder:text-muted"
              autoFocus
              onKeyDown={(e) => e.key === 'Escape' && setSearchOpen(false)}
            />
            <button onClick={() => setSearchOpen(false)} className="text-muted hover:text-primary px-2">
              <X size={14} />
            </button>
          </div>
        ) : (
          <div />
        )}
      </div>

      {/* Right controls */}
      <div className="flex items-center gap-1">
        <button
          onClick={() => setSearchOpen(!searchOpen)}
          className="p-1.5 text-primary/70 hover:text-primary transition-colors rounded hover:bg-bg-hover"
          title="搜索"
        >
          <Search size={18} />
        </button>
        <button
          onClick={onThemeClick}
          className="p-1.5 text-primary/70 hover:text-primary transition-colors rounded hover:bg-bg-hover"
          title="主题"
        >
          <Palette size={18} />
        </button>

        {/* User avatar */}
        <div className="w-7 h-7 rounded-full bg-accent-orange flex items-center justify-center text-white text-xs font-bold ml-1 cursor-pointer">
          G
        </div>
      </div>
    </header>
  )
}
