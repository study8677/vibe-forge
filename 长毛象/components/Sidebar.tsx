"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { IconHome, IconPackage, IconGrid, IconMapPin } from "./icons";
import { cn } from "@/lib/utils";

const links = [
  { href: "/", label: "首页", icon: IconHome },
  { href: "/items", label: "物品", icon: IconPackage },
  { href: "/categories", label: "分类", icon: IconGrid },
  { href: "/locations", label: "位置", icon: IconMapPin },
];

export default function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="hidden sm:flex flex-col w-60 min-h-screen bg-white border-r border-gray-200">
      <div className="px-6 py-6">
        <h1 className="text-xl font-bold text-gray-900">
          <span className="text-primary-600">My</span>Items
        </h1>
        <p className="text-xs text-gray-400 mt-0.5">个人物品管理</p>
      </div>
      <nav className="flex-1 px-3 space-y-1">
        {links.map((link) => {
          const active =
            link.href === "/"
              ? pathname === "/"
              : pathname.startsWith(link.href);
          const Icon = link.icon;
          return (
            <Link
              key={link.href}
              href={link.href}
              className={cn(
                "flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-colors",
                active
                  ? "bg-primary-50 text-primary-700"
                  : "text-gray-600 hover:bg-gray-50 hover:text-gray-900"
              )}
            >
              <Icon size={20} />
              {link.label}
            </Link>
          );
        })}
      </nav>
      <div className="px-6 py-4 border-t border-gray-100">
        <p className="text-xs text-gray-400">数据存储在本地浏览器中</p>
      </div>
    </aside>
  );
}
