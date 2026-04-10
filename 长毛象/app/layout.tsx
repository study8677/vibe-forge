import type { Metadata, Viewport } from "next";
import { StoreProvider } from "@/lib/store";
import Sidebar from "@/components/Sidebar";
import BottomNav from "@/components/BottomNav";
import "./globals.css";

export const metadata: Metadata = {
  title: "MyItems - 个人物品管理",
  description: "轻松管理你的个人物品",
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 1,
  viewportFit: "cover",
  themeColor: "#ffffff",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN">
      <body>
        <StoreProvider>
          <div className="flex min-h-screen">
            <Sidebar />
            <main className="flex-1 min-w-0 pb-20 sm:pb-0">{children}</main>
          </div>
          <BottomNav />
        </StoreProvider>
      </body>
    </html>
  );
}
