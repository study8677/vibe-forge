"use client";

import Link from "next/link";
import type { Item, Category, Location } from "@/lib/types";
import { formatPrice } from "@/lib/utils";

interface ItemCardProps {
  item: Item;
  category?: Category;
  location?: Location;
}

export default function ItemCard({ item, category, location }: ItemCardProps) {
  return (
    <Link
      href={`/items/${item.id}`}
      className="group block bg-white rounded-2xl border border-gray-100 hover:border-primary-200 hover:shadow-md transition-all overflow-hidden"
    >
      {item.imageData ? (
        <div className="aspect-[4/3] bg-gray-50 overflow-hidden">
          <img
            src={item.imageData}
            alt={item.name}
            className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
          />
        </div>
      ) : (
        <div className="aspect-[4/3] bg-gradient-to-br from-gray-50 to-gray-100 flex items-center justify-center">
          <span className="text-4xl">{category?.icon || "📦"}</span>
        </div>
      )}
      <div className="p-3.5">
        <h3 className="font-semibold text-gray-900 text-sm truncate">
          {item.name}
        </h3>
        <div className="flex items-center gap-2 mt-1.5">
          {category && (
            <span
              className="inline-flex items-center gap-1 text-[11px] px-2 py-0.5 rounded-full font-medium"
              style={{
                backgroundColor: category.color + "18",
                color: category.color,
              }}
            >
              {category.icon} {category.name}
            </span>
          )}
        </div>
        <div className="flex items-center justify-between mt-2">
          <span className="text-xs text-gray-400 truncate">
            {location?.name || "未设置位置"}
          </span>
          {item.price != null && item.price > 0 && (
            <span className="text-xs font-semibold text-gray-700">
              {formatPrice(item.price)}
            </span>
          )}
        </div>
        {item.quantity > 1 && (
          <span className="text-[11px] text-gray-400 mt-1 block">
            x{item.quantity}
          </span>
        )}
      </div>
    </Link>
  );
}
