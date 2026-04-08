"use client";

import { useEffect, useState } from "react";
import { getAllConversations, deleteConversation, type Conversation } from "@/lib/db";

interface SidebarProps {
  currentId: string | null;
  onSelect: (id: string) => void;
  onNew: () => void;
  refreshKey: number;
}

export default function Sidebar({ currentId, onSelect, onNew, refreshKey }: SidebarProps) {
  const [apiUrl, setApiUrl] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [model, setModel] = useState("");
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [showKey, setShowKey] = useState(false);

  // Load settings from localStorage
  useEffect(() => {
    setApiUrl(localStorage.getItem("wakusei_api_url") || "");
    setApiKey(localStorage.getItem("wakusei_api_key") || "");
    setModel(localStorage.getItem("wakusei_model") || "");
  }, []);

  // Save settings to localStorage
  useEffect(() => {
    localStorage.setItem("wakusei_api_url", apiUrl);
  }, [apiUrl]);
  useEffect(() => {
    localStorage.setItem("wakusei_api_key", apiKey);
  }, [apiKey]);
  useEffect(() => {
    localStorage.setItem("wakusei_model", model);
  }, [model]);

  // Load conversations
  useEffect(() => {
    getAllConversations().then(setConversations);
  }, [refreshKey]);

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    await deleteConversation(id);
    setConversations((prev) => prev.filter((c) => c.id !== id));
    if (currentId === id) onNew();
  };

  const formatTime = (ts: number) => {
    const d = new Date(ts);
    const pad = (n: number) => String(n).padStart(2, "0");
    return `${pad(d.getMonth() + 1)}/${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
  };

  return (
    <aside className="w-80 shrink-0 border-r-4 border-brutal-yellow bg-brutal-black flex flex-col h-full overflow-hidden">
      {/* Header */}
      <div className="border-b-4 border-brutal-yellow p-4">
        <h1 className="text-brutal-yellow text-lg font-bold tracking-widest">
          WAKUSEI<span className="text-brutal-white">//</span>AI
        </h1>
        <p className="text-[10px] text-brutal-border mt-1 tracking-wider">BRUTALIST CHAT TERMINAL v1.0</p>
      </div>

      {/* Settings */}
      <div className="border-b-4 border-brutal-yellow p-4 space-y-3">
        <div className="text-[11px] text-brutal-yellow font-bold tracking-wider mb-2">
          [ CONFIG ]
        </div>

        <div>
          <label className="text-[10px] text-brutal-border tracking-wider block mb-1">API_ENDPOINT</label>
          <input
            type="text"
            value={apiUrl}
            onChange={(e) => setApiUrl(e.target.value)}
            placeholder="https://api.openai.com"
            className="w-full bg-brutal-gray border-2 border-brutal-border text-brutal-white text-xs p-2 font-mono placeholder:text-brutal-border"
          />
        </div>

        <div>
          <label className="text-[10px] text-brutal-border tracking-wider block mb-1">API_KEY</label>
          <div className="flex">
            <input
              type={showKey ? "text" : "password"}
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder="sk-..."
              className="flex-1 bg-brutal-gray border-2 border-brutal-border text-brutal-white text-xs p-2 font-mono placeholder:text-brutal-border"
            />
            <button
              onClick={() => setShowKey(!showKey)}
              className="bg-brutal-gray border-2 border-l-0 border-brutal-border text-brutal-border hover:text-brutal-yellow text-[10px] px-2 transition-colors"
            >
              {showKey ? "HIDE" : "SHOW"}
            </button>
          </div>
        </div>

        <div>
          <label className="text-[10px] text-brutal-border tracking-wider block mb-1">MODEL</label>
          <input
            type="text"
            value={model}
            onChange={(e) => setModel(e.target.value)}
            placeholder="gpt-4o-mini"
            className="w-full bg-brutal-gray border-2 border-brutal-border text-brutal-white text-xs p-2 font-mono placeholder:text-brutal-border"
          />
        </div>
      </div>

      {/* New Chat */}
      <div className="p-4 border-b-4 border-brutal-yellow">
        <button
          onClick={onNew}
          className="w-full bg-brutal-yellow text-brutal-black font-bold text-xs py-2 border-2 border-brutal-yellow hover:bg-brutal-black hover:text-brutal-yellow transition-colors tracking-wider"
        >
          + NEW_SESSION
        </button>
      </div>

      {/* History */}
      <div className="flex-1 overflow-y-auto">
        <div className="p-4">
          <div className="text-[11px] text-brutal-yellow font-bold tracking-wider mb-3">
            [ HISTORY ] <span className="text-brutal-border font-normal">({conversations.length})</span>
          </div>
          {conversations.length === 0 ? (
            <p className="text-[10px] text-brutal-border">NO_RECORDS_FOUND</p>
          ) : (
            <div className="space-y-1">
              {conversations.map((conv) => (
                <div
                  key={conv.id}
                  onClick={() => onSelect(conv.id)}
                  className={`group cursor-pointer p-2 border-2 transition-colors ${
                    currentId === conv.id
                      ? "border-brutal-yellow bg-brutal-gray"
                      : "border-transparent hover:border-brutal-border"
                  }`}
                >
                  <div className="flex items-start justify-between gap-2">
                    <span className="text-xs text-brutal-white truncate flex-1">
                      {currentId === conv.id && (
                        <span className="text-brutal-yellow mr-1">&gt;</span>
                      )}
                      {conv.title}
                    </span>
                    <button
                      onClick={(e) => handleDelete(e, conv.id)}
                      className="text-[10px] text-brutal-border hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
                    >
                      [X]
                    </button>
                  </div>
                  <div className="text-[10px] text-brutal-border mt-1">
                    {formatTime(conv.updatedAt)} // {conv.messages.length} msgs
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Footer */}
      <div className="border-t-4 border-brutal-yellow p-3">
        <div className="text-[9px] text-brutal-border tracking-wider text-center">
          LOCAL_STORAGE: {apiUrl ? "CONFIGURED" : "EMPTY"} // {apiKey ? "KEY_SET" : "NO_KEY"}
        </div>
      </div>
    </aside>
  );
}
