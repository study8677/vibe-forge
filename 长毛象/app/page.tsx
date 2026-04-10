"use client";

import Link from "next/link";
import { useStore } from "@/lib/store";
import { formatPrice } from "@/lib/utils";
import StatsCard from "@/components/StatsCard";
import ItemCard from "@/components/ItemCard";
import { IconPackage, IconGrid, IconMapPin, IconDollar, IconPlus } from "@/components/icons";

export default function HomePage() {
  const { data, ready } = useStore();

  if (!ready) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="w-8 h-8 border-2 border-primary-600 border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  const { items, categories, locations } = data;
  const totalValue = items.reduce((sum, item) => sum + (item.price || 0) * item.quantity, 0);
  const totalCount = items.reduce((sum, item) => sum + item.quantity, 0);
  const recentItems = items.slice(0, 4);

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">首页</h1>
        <p className="text-sm text-gray-500 mt-1">管理你的所有物品</p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-2 gap-3 mb-8">
        <StatsCard
          icon={<IconPackage size={20} />}
          label="物品总数"
          value={totalCount}
          sub={`${items.length} 种物品`}
          color="#6366F1"
        />
        <StatsCard
          icon={<IconDollar size={20} />}
          label="总价值"
          value={formatPrice(totalValue)}
          color="#10B981"
        />
        <StatsCard
          icon={<IconGrid size={20} />}
          label="分类"
          value={categories.length}
          color="#F59E0B"
        />
        <StatsCard
          icon={<IconMapPin size={20} />}
          label="存放位置"
          value={locations.length}
          color="#EC4899"
        />
      </div>

      {/* Quick Add */}
      <Link
        href="/items/new"
        className="btn-primary w-full mb-8"
      >
        <IconPlus size={18} />
        添加新物品
      </Link>

      {/* Recent Items */}
      <div className="mb-4">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-gray-900">最近添加</h2>
          {items.length > 4 && (
            <Link
              href="/items"
              className="text-sm text-primary-600 font-medium hover:text-primary-700"
            >
              查看全部
            </Link>
          )}
        </div>

        {recentItems.length === 0 ? (
          <div className="text-center py-12 bg-white rounded-2xl border border-gray-100">
            <span className="text-4xl block mb-3">📦</span>
            <p className="text-sm text-gray-500">还没有添加物品</p>
            <Link href="/items/new" className="text-sm text-primary-600 font-medium mt-2 inline-block">
              立即添加
            </Link>
          </div>
        ) : (
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
            {recentItems.map((item) => (
              <ItemCard
                key={item.id}
                item={item}
                category={categories.find((c) => c.id === item.categoryId)}
                location={locations.find((l) => l.id === item.locationId)}
              />
            ))}
          </div>
        )}
      </div>

      {/* Category Quick View */}
      {items.length > 0 && (
        <div className="mt-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">分类概览</h2>
          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
            {categories
              .map((cat) => ({
                ...cat,
                count: items.filter((i) => i.categoryId === cat.id).length,
              }))
              .filter((cat) => cat.count > 0)
              .sort((a, b) => b.count - a.count)
              .map((cat) => (
                <Link
                  key={cat.id}
                  href={`/items?category=${cat.id}`}
                  className="flex items-center gap-3 bg-white rounded-xl border border-gray-100 p-3 hover:border-primary-200 transition-colors"
                >
                  <span className="text-2xl">{cat.icon}</span>
                  <div>
                    <p className="text-sm font-medium text-gray-900">{cat.name}</p>
                    <p className="text-xs text-gray-400">{cat.count} 件</p>
                  </div>
                </Link>
              ))}
          </div>
        </div>
      )}
    </div>
  );
}
