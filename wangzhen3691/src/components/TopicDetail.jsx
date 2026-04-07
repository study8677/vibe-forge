import { ArrowLeft, Heart, Bookmark, Share2, Flag, MessageCircle, Eye, Clock } from 'lucide-react'
import Avatar from './Avatar'
import CategoryBadge from './CategoryBadge'

export default function TopicDetail({ topic, onBack }) {
  const fmt = (n) => (n >= 1000 ? `${(n / 1000).toFixed(1)}k` : n)

  return (
    <div className="flex-1 overflow-auto">
      {/* Breadcrumb */}
      <div className="flex items-center gap-2 px-4 py-2 border-b border-border text-sm">
        <button onClick={onBack} className="text-accent hover:underline flex items-center gap-1">
          <ArrowLeft size={14} /> 返回
        </button>
        <span className="text-muted">/</span>
        <CategoryBadge catId={topic.catId} showIcon />
      </div>

      {/* Topic header */}
      <div className="px-6 py-5 border-b border-border">
        <h1 className="text-xl font-semibold text-primary leading-snug">{topic.title}</h1>
        <div className="flex items-center gap-3 mt-3 flex-wrap">
          {topic.tags.map((t) => (
            <span key={t} className="text-xs text-muted bg-border/50 px-2 py-0.5 rounded">{t}</span>
          ))}
          <span className="text-xs text-muted flex items-center gap-1"><Eye size={11} /> {fmt(topic.views)}</span>
          <span className="text-xs text-muted flex items-center gap-1"><MessageCircle size={11} /> {topic.replies}</span>
          <span className="text-xs text-muted flex items-center gap-1"><Clock size={11} /> {topic.activity}</span>
        </div>
      </div>

      {/* OP Post */}
      <div className="px-6 py-5 border-b border-border">
        <div className="flex gap-3">
          <Avatar user={topic.op} size={42} />
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <span className="font-medium text-sm text-primary">{topic.op.name}</span>
              <span className="text-xs text-muted">{topic.activity}</span>
            </div>
            <div className="text-sm text-primary/80 leading-relaxed space-y-3">
              <p>这是一篇关于 <strong>{topic.title}</strong> 的详细讨论帖。</p>
              <p>在当前的技术生态下，越来越多的开发者开始关注这个领域。本帖将从实际使用场景出发，分享一些经验和心得，希望对大家有所帮助。</p>
              <div className="bg-surface border border-border rounded-md p-4 font-mono text-xs">
                <div className="text-muted mb-1"># 示例代码</div>
                <div><span className="text-accent">$</span> curl -sSL https://example.com/install.sh | bash</div>
                <div><span className="text-accent">$</span> sudo systemctl enable --now myservice</div>
                <div><span className="text-success">✓</span> Service started successfully</div>
              </div>
              <p>欢迎大家在评论区分享你的看法和经验。</p>
            </div>

            {/* Actions */}
            <div className="flex items-center gap-4 mt-4 pt-3 border-t border-border/50">
              <button className="flex items-center gap-1.5 text-xs text-muted hover:text-love transition-colors">
                <Heart size={14} /> 赞
              </button>
              <button className="flex items-center gap-1.5 text-xs text-muted hover:text-accent transition-colors">
                <Bookmark size={14} /> 收藏
              </button>
              <button className="flex items-center gap-1.5 text-xs text-muted hover:text-accent transition-colors">
                <Share2 size={14} /> 分享
              </button>
              <button className="flex items-center gap-1.5 text-xs text-muted hover:text-danger transition-colors">
                <Flag size={14} /> 举报
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Replies preview */}
      {topic.posters.slice(0, 3).map((poster, i) => (
        <div key={i} className="px-6 py-4 border-b border-border/50">
          <div className="flex gap-3">
            <Avatar user={poster} size={36} />
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                <span className="font-medium text-sm text-primary">{poster.name}</span>
                <span className="text-xs text-muted">{Math.floor(Math.random() * 12) + 1}小时前</span>
              </div>
              <p className="text-sm text-primary/70">
                {['感谢分享，非常有用的信息！已收藏。', '这个方案我之前也试过，补充一点：建议配合 systemd timer 使用效果更好。', '楼主的配置很优雅，不过在生产环境中建议再加一层安全措施。'][i]}
              </p>
              <button className="flex items-center gap-1 mt-2 text-xs text-muted hover:text-love transition-colors">
                <Heart size={12} /> {Math.floor(Math.random() * 20) + 1}
              </button>
            </div>
          </div>
        </div>
      ))}

      {/* Reply box */}
      <div className="px-6 py-4">
        <div className="border border-border rounded-md overflow-hidden">
          <textarea
            placeholder="在此输入回复... (支持 Markdown)"
            className="w-full bg-surface text-primary text-sm p-3 outline-none resize-none placeholder:text-muted"
            rows={4}
          />
          <div className="flex items-center justify-between px-3 py-2 bg-bg-hover/50 border-t border-border">
            <span className="text-[10px] text-muted">Markdown / BBCode</span>
            <button className="px-3 py-1 bg-accent text-bg-secondary text-xs font-medium rounded hover:opacity-90 transition-opacity">
              回复
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
