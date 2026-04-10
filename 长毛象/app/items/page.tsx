"use client";

import { Suspense, useState, useMemo } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import { useStore } from "@/lib/store";
import ItemCard from "@/components/ItemCard";
import EmptyState from "@/components/EmptyState";
import { IconPlus, IconSearch, IconFilter, IconX } from "@/components/icons";
import { cn } from "@/lib/utils";

export default function ItemsPage() {
  return (
    <Suspense fallback={
      <div className="flex items-center justify-center h-screen">
        <div className="w-8 h-8 border-2 border-primary-600 border-t-transparent rounded-full animate-spin" />
      </div>
    }>
      <ItemsContent />
    </Suspense>
  );
}

function ItemsContent() {
  const { data, ready } = useStore();
  const searchParams = useSearchParams();
  const initialCategory = searchParams.get("category") || "";

  const [search, setSearch] = useState("");
  const [categoryFilter, setCategoryFilter] = useState(initialCategory);
  const [locationFilter, setLocationFilter] = useState("");
  const [showFilter, setShowFilter] = useState(!!initialCategory);
  const [sortBy, setSortBy] = useState<"newest" | "name" | "price">("newest");

  const filtered = useMemo(() => {
    let result = data.items;

    if (search.trim()) {
      const q = search.toLowerCase();
      result = result.filter(
        (i) =>
          i.name.toLowerCase().includes(q) ||
          i.notes?.toLowerCase().includes(q)
      );
    }

    if (categoryFilter) {
      result = result.filter((i) => i.categoryId === categoryFilter);
    }

    if (locationFilter) {
      result = result.filter((i) => i.locationId === locationFilter);
    }

    switch (sortBy) {
      case "name":
        result = [...result].sort((a, b) => a.name.localeCompare(b.name, "zh"));
        break;
      case "price":
        result = [...result].sort(
          (a, b) => (b.price || 0) - (a.price || 0)
        );
        break;
      default:
        result = [...result].sort(
          (a, b) =>
            new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
        );
    }

    return result;
  }, [data.items, search, categoryFilter, locationFilter, sortBy]);

  if (!ready) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="w-8 h-8 border-2 border-primary-600 border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  const hasActiveFilter = !!categoryFilter || !!locationFilter;

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-5">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">物品</h1>
          <p className="text-sm text-gray-500 mt-0.5">
            共 {data.items.length} 种物品
          </p>
        </div>
        <Link href="/items/new" className="btn-primary">
          <IconPlus size={18} />
          <span className="hidden sm:inline">添加物品</span>
        </Link>
      </div>

      {/* Search */}
      <div className="flex gap-2 mb-4">
        <div className="relative flex-1">
          <IconSearch
            size={18}
            className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            type="text"
            placeholder="搜索物品..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="input pl-10"
          />
          {search && (
            <button
              onClick={() => setSearch("")}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
            >
              <IconX size={16} />
            </button>
          )}
        </div>
        <button
          onClick={() => setShowFilter(!showFilter)}
          className={cn(
            "btn-secondary relative",
            hasActiveFilter && "border-primary-300 text-primary-600"
          )}
        >
          <IconFilter size={18} />
          {hasActiveFilter && (
            <span className="absolute -top-1 -right-1 w-2 h-2 bg-primary-500 rounded-full" />
          )}
        </button>
      </div>

      {/* Filters */}
      {showFilter && (
        <div className="bg-white rounded-xl border border-gray-100 p-4 mb-4 space-y-3">
          <div>
            <label className="label">分类</label>
            <select
              value={categoryFilter}
              onChange={(e) => setCategoryFilter(e.target.value)}
              className="input"
            >
              <option value="">全部分类</option>
              {data.categories.map((c) => (
                <option key={c.id} value={c.id}>
                  {c.icon} {c.name}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className="label">位置</label>
            <select
              value={locationFilter}
              onChange={(e) => setLocationFilter(e.target.value)}
              className="input"
            >
              <option value="">全部位置</option>
              {data.locations.map((l) => (
                <option key={l.id} value={l.id}>
                  {l.name}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className="label">排序</label>
            <select
              value={sortBy}
              onChange={(e) =>
                setSortBy(e.target.value as "newest" | "name" | "price")
              }
              className="input"
            >
              <option value="newest">最新添加</option>
              <option value="name">名称排序</option>
              <option value="price">价格排序</option>
            </select>
          </div>
          {hasActiveFilter && (
            <button
              onClick={() => {
                setCategoryFilter("");
                setLocationFilter("");
              }}
              className="text-sm text-primary-600 font-medium"
            >
              清除筛选
            </button>
          )}
        </div>
      )}

      {/* Items Grid */}
      {filtered.length === 0 ? (
        <EmptyState
          icon="📭"
          title={search || hasActiveFilter ? "没有找到匹配的物品" : "还没有物品"}
          description={
            search || hasActiveFilter
              ? "试试修改搜索条件或筛选"
              : "点击上方按钮添加你的第一件物品"
          }
          action={
            !search &&
            !hasActiveFilter && (
              <Link href="/items/new" className="btn-primary">
                <IconPlus size={18} />
                添加物品
              </Link>
            )
          }
        />
      ) : (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
          {filtered.map((item) => (
            <ItemCard
              key={item.id}
              item={item}
              category={data.categories.find(
                (c) => c.id === item.categoryId
              )}
              location={data.locations.find(
                (l) => l.id === item.locationId
              )}
            />
          ))}
        </div>
      )}

      {/* Mobile FAB */}
      <Link
        href="/items/new"
        className="fixed right-4 bottom-20 sm:hidden w-14 h-14 bg-primary-600 text-white rounded-full shadow-lg shadow-primary-600/30 flex items-center justify-center hover:bg-primary-700 active:bg-primary-800 transition-colors z-30"
      >
        <IconPlus size={24} />
      </Link>
    </div>
  );
}
