"use client";

import { useState, useRef, useEffect, useCallback } from "react";
import type { Message, Conversation } from "@/lib/db";
import { saveConversation } from "@/lib/db";

interface ChatAreaProps {
  conversation: Conversation | null;
  onUpdate: () => void;
}

export default function ChatArea({ conversation, onUpdate }: ChatAreaProps) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [streaming, setStreaming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const abortRef = useRef<AbortController | null>(null);
  const convIdRef = useRef<string>("");

  useEffect(() => {
    if (conversation) {
      setMessages(conversation.messages);
      convIdRef.current = conversation.id;
    } else {
      setMessages([]);
      convIdRef.current = crypto.randomUUID();
    }
    setError(null);
  }, [conversation]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const autoResize = useCallback(() => {
    const el = textareaRef.current;
    if (el) {
      el.style.height = "auto";
      el.style.height = Math.min(el.scrollHeight, 200) + "px";
    }
  }, []);

  const persistConversation = useCallback(
    async (msgs: Message[]) => {
      const title =
        msgs.find((m) => m.role === "user")?.content.slice(0, 50) || "New Chat";
      await saveConversation({
        id: convIdRef.current,
        title,
        messages: msgs,
        createdAt: conversation?.createdAt || Date.now(),
        updatedAt: Date.now(),
      });
      onUpdate();
    },
    [conversation, onUpdate]
  );

  const handleSend = async () => {
    const text = input.trim();
    if (!text || streaming) return;

    const apiUrl = localStorage.getItem("wakusei_api_url") || "";
    const apiKey = localStorage.getItem("wakusei_api_key") || "";
    const model = localStorage.getItem("wakusei_model") || "";

    if (!apiUrl || !apiKey) {
      setError("ERROR: Configure API_ENDPOINT and API_KEY in the sidebar first.");
      return;
    }

    setError(null);
    const userMsg: Message = { role: "user", content: text };
    const newMessages = [...messages, userMsg];
    setMessages(newMessages);
    setInput("");
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }

    setStreaming(true);
    const assistantMsg: Message = { role: "assistant", content: "" };
    setMessages([...newMessages, assistantMsg]);

    try {
      abortRef.current = new AbortController();
      const res = await fetch("/api/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          messages: newMessages.map((m) => ({ role: m.role, content: m.content })),
          apiUrl,
          apiKey,
          model: model || undefined,
        }),
        signal: abortRef.current.signal,
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.error || `HTTP ${res.status}`);
      }

      const reader = res.body?.getReader();
      if (!reader) throw new Error("No response stream");

      const decoder = new TextDecoder();
      let buffer = "";
      let fullContent = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split("\n");
        buffer = lines.pop() || "";

        for (const line of lines) {
          const trimmed = line.trim();
          if (!trimmed || !trimmed.startsWith("data: ")) continue;
          const data = trimmed.slice(6);
          if (data === "[DONE]") continue;

          try {
            const parsed = JSON.parse(data);
            const delta = parsed.choices?.[0]?.delta?.content;
            if (delta) {
              fullContent += delta;
              setMessages((prev) => {
                const updated = [...prev];
                updated[updated.length - 1] = {
                  role: "assistant",
                  content: fullContent,
                };
                return updated;
              });
            }
          } catch {
            // skip malformed chunks
          }
        }
      }

      const finalMessages = [
        ...newMessages,
        { role: "assistant" as const, content: fullContent },
      ];
      setMessages(finalMessages);
      await persistConversation(finalMessages);
    } catch (err) {
      if ((err as Error).name === "AbortError") return;
      setError(`ERROR: ${(err as Error).message}`);
      setMessages(newMessages); // remove empty assistant msg
    } finally {
      setStreaming(false);
      abortRef.current = null;
    }
  };

  const handleStop = () => {
    abortRef.current?.abort();
    setStreaming(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <main className="flex-1 flex flex-col h-full overflow-hidden bg-brutal-black">
      {/* Header bar */}
      <div className="border-b-4 border-brutal-yellow p-4 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-3">
          <span className="text-brutal-yellow text-xs font-bold tracking-wider">
            SESSION
          </span>
          <span className="text-[10px] text-brutal-border font-mono">
            {convIdRef.current.slice(0, 8).toUpperCase()}
          </span>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-[10px] text-brutal-border">
            {messages.filter((m) => m.role === "user").length} PROMPTS
          </span>
          {streaming && (
            <span className="text-[10px] text-brutal-yellow animate-pulse">
              ■ STREAMING
            </span>
          )}
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 && (
          <div className="flex items-center justify-center h-full">
            <div className="text-center space-y-4">
              <div className="text-6xl font-bold text-brutal-yellow tracking-tighter">
                惑星
              </div>
              <div className="text-brutal-border text-xs tracking-widest">
                WAKUSEI // AI CHAT TERMINAL
              </div>
              <div className="border-2 border-brutal-border p-4 max-w-md">
                <p className="text-[11px] text-brutal-border leading-relaxed">
                  Configure your API endpoint and key in the sidebar,
                  then start typing below. All conversations are stored
                  locally in IndexedDB.
                </p>
              </div>
            </div>
          </div>
        )}

        {messages.map((msg, i) => (
          <div key={i} className="group">
            {/* Role label */}
            <div className="flex items-center gap-2 mb-1">
              <span
                className={`text-[10px] font-bold tracking-wider ${
                  msg.role === "user" ? "text-brutal-yellow" : "text-brutal-white"
                }`}
              >
                {msg.role === "user" ? "USER" : "ASSISTANT"}
              </span>
              <div className="flex-1 h-[1px] bg-brutal-gray" />
            </div>

            {/* Message content */}
            <div
              className={`border-l-4 pl-4 py-2 text-sm leading-relaxed whitespace-pre-wrap break-words ${
                msg.role === "user"
                  ? "border-brutal-yellow text-brutal-white"
                  : "border-brutal-border text-brutal-white/90"
              }`}
            >
              {msg.content}
              {streaming && i === messages.length - 1 && msg.role === "assistant" && (
                <span className="cursor-blink" />
              )}
            </div>
          </div>
        ))}

        {error && (
          <div className="border-2 border-red-500 bg-red-500/10 p-3">
            <span className="text-red-500 text-xs font-bold tracking-wider">
              {error}
            </span>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input area */}
      <div className="border-t-4 border-brutal-yellow p-4 shrink-0">
        <div className="flex gap-2">
          <div className="flex-1 flex border-2 border-brutal-border focus-within:border-brutal-yellow transition-colors">
            <span className="text-brutal-yellow text-sm px-3 py-2 select-none border-r-2 border-brutal-border bg-brutal-gray">
              &gt;
            </span>
            <textarea
              ref={textareaRef}
              value={input}
              onChange={(e) => {
                setInput(e.target.value);
                autoResize();
              }}
              onKeyDown={handleKeyDown}
              placeholder="Enter message... (Shift+Enter for newline)"
              rows={1}
              className="flex-1 bg-brutal-black text-brutal-white text-sm p-2 font-mono resize-none placeholder:text-brutal-border"
            />
          </div>
          {streaming ? (
            <button
              onClick={handleStop}
              className="bg-red-500 text-brutal-black font-bold text-xs px-4 border-2 border-red-500 hover:bg-brutal-black hover:text-red-500 transition-colors tracking-wider shrink-0"
            >
              STOP
            </button>
          ) : (
            <button
              onClick={handleSend}
              disabled={!input.trim()}
              className="bg-brutal-yellow text-brutal-black font-bold text-xs px-4 border-2 border-brutal-yellow hover:bg-brutal-black hover:text-brutal-yellow transition-colors tracking-wider disabled:opacity-30 disabled:cursor-not-allowed shrink-0"
            >
              SEND
            </button>
          )}
        </div>
        <div className="text-[9px] text-brutal-border mt-2 tracking-wider">
          ENTER=SEND // SHIFT+ENTER=NEWLINE // ALL_DATA_STORED_LOCALLY
        </div>
      </div>
    </main>
  );
}
