import { useState, useEffect, useCallback } from 'react'
import {
  Rss, RefreshCw, Star, ExternalLink, Plus, X, ChevronDown,
  ChevronRight, AlertCircle, Check, Loader2,
} from 'lucide-react'
import { defaultFeeds, sampleFeedItems } from '../data/feeds'

function parseFeedItems(xml, sourceName) {
  const doc = new DOMParser().parseFromString(xml, 'text/xml')
  const isAtom = !!doc.querySelector('feed')
  const entries = isAtom
    ? [...doc.querySelectorAll('entry')]
    : [...doc.querySelectorAll('item')]

  return entries.slice(0, 20).map((el) => {
    const title = el.querySelector('title')?.textContent?.trim() || 'Untitled'
    const link = isAtom
      ? el.querySelector('link')?.getAttribute('href')
      : el.querySelector('link')?.textContent?.trim()
    const pubDate = el.querySelector('pubDate, published, updated')?.textContent
    return {
      title,
      url: link || '#',
      source: sourceName,
      time: pubDate ? timeAgo(new Date(pubDate)) : '',
      starred: false,
    }
  })
}

function timeAgo(date) {
  const s = Math.floor((Date.now() - date) / 1000)
  if (s < 60) return `${s}s ago`
  if (s < 3600) return `${Math.floor(s / 60)}m ago`
  if (s < 86400) return `${Math.floor(s / 3600)}h ago`
  return `${Math.floor(s / 86400)}d ago`
}

export default function RSSPanel() {
  const [feeds, setFeeds] = useState(() => {
    const saved = localStorage.getItem('rss-feeds')
    return saved ? JSON.parse(saved) : defaultFeeds
  })
  const [items, setItems] = useState(sampleFeedItems)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const [addOpen, setAddOpen] = useState(false)
  const [newUrl, setNewUrl] = useState('')
  const [newName, setNewName] = useState('')
  const [filter, setFilter] = useState('all')
  const [expandedSource, setExpandedSource] = useState(null)

  useEffect(() => {
    localStorage.setItem('rss-feeds', JSON.stringify(feeds))
  }, [feeds])

  const fetchFeeds = useCallback(async () => {
    setLoading(true)
    setError(null)
    const allItems = []
    const corsProxy = 'https://api.allorigins.win/raw?url='

    for (const feed of feeds) {
      try {
        const res = await fetch(corsProxy + encodeURIComponent(feed.url))
        if (!res.ok) continue
        const xml = await res.text()
        allItems.push(...parseFeedItems(xml, feed.name))
      } catch {
        // Silently skip failed feeds
      }
    }

    if (allItems.length > 0) {
      setItems(allItems.sort((a, b) => (a.time > b.time ? 1 : -1)))
    } else {
      setError('无法获取 RSS 源，使用缓存数据')
    }
    setLoading(false)
  }, [feeds])

  const addFeed = () => {
    if (!newUrl) return
    setFeeds([...feeds, { name: newName || new URL(newUrl).hostname, url: newUrl, icon: '📡' }])
    setNewUrl('')
    setNewName('')
    setAddOpen(false)
  }

  const removeFeed = (i) => setFeeds(feeds.filter((_, idx) => idx !== i))

  const toggleStar = (i) => {
    setItems(items.map((item, idx) => (idx === i ? { ...item, starred: !item.starred } : item)))
  }

  const filtered = filter === 'starred' ? items.filter((i) => i.starred) : items
  const sources = [...new Set(items.map((i) => i.source))]

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border">
        <div className="flex items-center gap-2">
          <Rss size={16} className="text-accent" />
          <h2 className="text-sm font-semibold text-primary">RSS 聚合阅读器</h2>
          <span className="text-[10px] text-muted bg-surface px-1.5 py-0.5 rounded">{items.length} 条</span>
        </div>
        <div className="flex items-center gap-1.5">
          <button
            onClick={() => setFilter(filter === 'all' ? 'starred' : 'all')}
            className={`p-1.5 rounded transition-colors ${filter === 'starred' ? 'text-highlight bg-highlight/10' : 'text-muted hover:text-primary hover:bg-bg-hover'}`}
            title="收藏筛选"
          >
            <Star size={14} />
          </button>
          <button
            onClick={fetchFeeds}
            disabled={loading}
            className="p-1.5 text-muted hover:text-primary hover:bg-bg-hover rounded transition-colors disabled:opacity-50"
            title="刷新"
          >
            <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
          </button>
          <button
            onClick={() => setAddOpen(!addOpen)}
            className="p-1.5 text-muted hover:text-primary hover:bg-bg-hover rounded transition-colors"
            title="添加源"
          >
            <Plus size={14} />
          </button>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div className="mx-4 mt-2 px-3 py-2 bg-danger/10 border border-danger/30 rounded text-xs text-danger flex items-center gap-2">
          <AlertCircle size={12} /> {error}
        </div>
      )}

      {/* Add feed form */}
      {addOpen && (
        <div className="px-4 py-3 border-b border-border bg-surface/50 space-y-2">
          <input
            value={newUrl}
            onChange={(e) => setNewUrl(e.target.value)}
            placeholder="RSS URL (e.g. https://hnrss.org/frontpage)"
            className="w-full bg-bg-primary border border-border rounded px-3 py-1.5 text-sm text-primary outline-none focus:border-accent placeholder:text-muted"
          />
          <div className="flex gap-2">
            <input
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder="名称 (可选)"
              className="flex-1 bg-bg-primary border border-border rounded px-3 py-1.5 text-sm text-primary outline-none focus:border-accent placeholder:text-muted"
            />
            <button onClick={addFeed} className="px-3 py-1.5 bg-accent text-bg-secondary text-xs font-medium rounded hover:opacity-90">
              <Check size={12} />
            </button>
            <button onClick={() => setAddOpen(false)} className="px-3 py-1.5 bg-border text-muted text-xs rounded hover:bg-bg-hover">
              <X size={12} />
            </button>
          </div>
        </div>
      )}

      <div className="flex flex-1 overflow-hidden">
        {/* Source list */}
        <div className="w-48 border-r border-border overflow-y-auto shrink-0 hidden md:block">
          <div className="py-2">
            <button
              onClick={() => setExpandedSource(null)}
              className={`w-full text-left px-3 py-1.5 text-xs transition-colors ${
                !expandedSource ? 'text-accent bg-bg-selected' : 'text-primary/80 hover:bg-bg-hover'
              }`}
            >
              全部来源 ({items.length})
            </button>
            {sources.map((src) => {
              const feed = feeds.find((f) => f.name === src)
              return (
                <button
                  key={src}
                  onClick={() => setExpandedSource(expandedSource === src ? null : src)}
                  className={`w-full flex items-center gap-2 text-left px-3 py-1.5 text-xs transition-colors ${
                    expandedSource === src ? 'text-accent bg-bg-selected' : 'text-primary/70 hover:bg-bg-hover'
                  }`}
                >
                  <span>{feed?.icon || '📰'}</span>
                  <span className="truncate flex-1">{src}</span>
                  <span className="text-muted">{items.filter((i) => i.source === src).length}</span>
                </button>
              )
            })}

            {/* Manage feeds */}
            <div className="mx-3 my-2 border-t border-border" />
            <div className="px-3 py-1 text-[10px] text-muted uppercase">订阅管理</div>
            {feeds.map((feed, i) => (
              <div key={i} className="flex items-center gap-1.5 px-3 py-1 text-xs text-primary/60 group">
                <span>{feed.icon}</span>
                <span className="truncate flex-1">{feed.name}</span>
                <button
                  onClick={() => removeFeed(i)}
                  className="opacity-0 group-hover:opacity-100 text-danger hover:text-danger transition-opacity"
                >
                  <X size={10} />
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* Feed items */}
        <div className="flex-1 overflow-y-auto">
          {loading && (
            <div className="flex items-center justify-center py-12 text-muted">
              <Loader2 size={20} className="animate-spin mr-2" /> 正在获取 RSS 源...
            </div>
          )}
          {!loading &&
            (expandedSource ? filtered.filter((i) => i.source === expandedSource) : filtered).map((item, i) => (
              <div
                key={i}
                className="flex items-start gap-3 px-4 py-2.5 border-b border-border/30 hover:bg-bg-hover transition-colors group"
              >
                <button
                  onClick={() => toggleStar(items.indexOf(item))}
                  className={`mt-0.5 shrink-0 transition-colors ${item.starred ? 'text-highlight' : 'text-border group-hover:text-muted'}`}
                >
                  <Star size={13} fill={item.starred ? 'currentColor' : 'none'} />
                </button>
                <div className="min-w-0 flex-1">
                  <a
                    href={item.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-sm text-primary hover:text-accent transition-colors leading-snug line-clamp-2 no-underline"
                  >
                    {item.title}
                  </a>
                  <div className="flex items-center gap-2 mt-0.5">
                    <span className="text-[10px] text-accent/70">{item.source}</span>
                    <span className="text-[10px] text-muted">{item.time}</span>
                  </div>
                </div>
                <a
                  href={item.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="shrink-0 text-muted opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  <ExternalLink size={12} />
                </a>
              </div>
            ))}
        </div>
      </div>
    </div>
  )
}
