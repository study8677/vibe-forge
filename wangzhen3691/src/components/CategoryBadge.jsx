import { categories } from '../data/categories'

export default function CategoryBadge({ catId, showIcon }) {
  const cat = categories.find((c) => c.id === catId)
  if (!cat) return null
  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-xs font-medium whitespace-nowrap leading-tight"
      style={{ background: cat.color, color: cat.textColor }}
    >
      {showIcon && <span className="text-[10px]">{cat.icon}</span>}
      {cat.name}
    </span>
  )
}
