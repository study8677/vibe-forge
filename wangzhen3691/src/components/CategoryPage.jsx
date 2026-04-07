import { categories } from '../data/categories'
import { topics } from '../data/topics'

export default function CategoryPage() {
  return (
    <div className="flex-1 overflow-auto p-4">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
        {categories.map((cat) => {
          const catTopics = topics.filter((t) => t.catId === cat.id)
          return (
            <div
              key={cat.id}
              className="border border-border rounded-md overflow-hidden hover:border-accent/30 transition-colors"
              style={{ paddingLeft: cat.parent ? '1rem' : 0 }}
            >
              <div className="flex items-center gap-3 px-4 py-3" style={{ borderLeft: `3px solid ${cat.color}` }}>
                <span className="text-lg">{cat.icon}</span>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <h3 className="text-sm font-semibold text-primary">{cat.name}</h3>
                    <span
                      className="text-[10px] px-1.5 py-0.5 rounded font-medium"
                      style={{ background: cat.color, color: cat.textColor }}
                    >
                      {cat.count.toLocaleString()}
                    </span>
                  </div>
                  <div className="mt-1.5 space-y-0.5">
                    {catTopics.slice(0, 3).map((t) => (
                      <div key={t.id} className="text-xs text-muted truncate hover:text-accent cursor-pointer transition-colors">
                        {t.title}
                      </div>
                    ))}
                    {catTopics.length === 0 && (
                      <div className="text-xs text-muted/50 italic">暂无话题</div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          )
        })}
      </div>
    </div>
  )
}
