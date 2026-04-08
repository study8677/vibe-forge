"use client";

import { useState, useCallback } from "react";
import Sidebar from "@/components/Sidebar";
import ChatArea from "@/components/ChatArea";
import { getConversation, type Conversation } from "@/lib/db";

export default function Home() {
  const [currentConv, setCurrentConv] = useState<Conversation | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);
  const [sidebarOpen, setSidebarOpen] = useState(true);

  const handleSelect = useCallback(async (id: string) => {
    const conv = await getConversation(id);
    if (conv) setCurrentConv(conv);
  }, []);

  const handleNew = useCallback(() => {
    setCurrentConv(null);
  }, []);

  const handleUpdate = useCallback(() => {
    setRefreshKey((k) => k + 1);
  }, []);

  return (
    <div className="flex h-screen overflow-hidden bg-brutal-black">
      {/* Mobile toggle */}
      <button
        onClick={() => setSidebarOpen(!sidebarOpen)}
        className="fixed top-3 left-3 z-50 md:hidden bg-brutal-yellow text-brutal-black font-bold text-xs p-2 border-2 border-brutal-yellow"
      >
        {sidebarOpen ? "[X]" : "[=]"}
      </button>

      {/* Sidebar */}
      <div
        className={`${
          sidebarOpen ? "translate-x-0" : "-translate-x-full"
        } fixed md:relative md:translate-x-0 z-40 h-full transition-transform duration-200`}
      >
        <Sidebar
          currentId={currentConv?.id ?? null}
          onSelect={handleSelect}
          onNew={handleNew}
          refreshKey={refreshKey}
        />
      </div>

      {/* Overlay for mobile */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-30 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Chat area */}
      <ChatArea conversation={currentConv} onUpdate={handleUpdate} />
    </div>
  );
}
