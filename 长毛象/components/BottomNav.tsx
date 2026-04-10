"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { IconHome, IconPackage, IconGrid, IconMapPin } from "./icons";
import { cn } from "@/lib/utils";

const tabs = [
  { href: "/", label: "首页", icon: IconHome },
  { href: "/items", label: "物品", icon: IconPackage },
  { href: "/categories", label: "分类", icon: IconGrid },
  { href: "/locations", label: "位置", icon: IconMapPin },
];

export default function BottomNav() {
  const pathname = usePathname();

  return (
    <nav className="fixed bottom-0 left-0 right-0 z-40 bg-white/80 backdrop-blur-xl border-t border-gray-200/60 sm:hidden">
      <div className="flex items-center justify-around h-16 pb-safe">
        {tabs.map((tab) => {
          const active =
            tab.href === "/"
              ? pathname === "/"
              : pathname.startsWith(tab.href);
          const Icon = tab.icon;
          return (
            <Link
              key={tab.href}
              href={tab.href}
              className={cn(
                "flex flex-col items-center gap-0.5 px-3 py-1 rounded-lg transition-colors min-w-[4rem]",
                active
                  ? "text-primary-600"
                  : "text-gray-400 active:text-gray-600"
              )}
            >
              <Icon size={22} />
              <span className="text-[10px] font-medium">{tab.label}</span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
