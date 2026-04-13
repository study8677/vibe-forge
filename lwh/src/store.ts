import { create } from 'zustand'
import type { ServerConfig, ServiceConfig, LogEntry, ChatMessage, AIProvider, ViewMode } from './types'

interface AppState {
  // Servers
  servers: ServerConfig[]
  setServers: (servers: ServerConfig[]) => void
  addServer: (server: ServerConfig) => void
  updateServer: (id: string, updates: Partial<ServerConfig>) => void
  removeServer: (id: string) => void

  // Connection status
  connectedServers: Set<string>
  setConnected: (id: string, connected: boolean) => void

  // Active selection
  activeServerId: string | null
  activeService: ServiceConfig | null
  setActiveServer: (id: string | null) => void
  setActiveService: (service: ServiceConfig | null) => void

  // View mode
  viewMode: ViewMode
  setViewMode: (mode: ViewMode) => void

  // Logs
  logs: LogEntry[]
  addLog: (entry: LogEntry) => void
  clearLogs: () => void
  maxLogs: number

  // Search/filter
  logFilter: string
  setLogFilter: (filter: string) => void
  logLevelFilter: Set<string>
  toggleLevelFilter: (level: string) => void

  // Auto-scroll
  autoScroll: boolean
  toggleAutoScroll: () => void

  // AI Chat
  aiProvider: AIProvider
  setAIProvider: (provider: AIProvider) => void
  chatMessages: ChatMessage[]
  addChatMessage: (message: ChatMessage) => void
  updateLastChatMessage: (content: string) => void
  clearChat: () => void
  selectedLogText: string
  setSelectedLogText: (text: string) => void
  aiChatOpen: boolean
  setAIChatOpen: (open: boolean) => void
  isAILoading: boolean
  setAILoading: (loading: boolean) => void

  // Server form
  showServerForm: boolean
  setShowServerForm: (show: boolean) => void
  editingServer: ServerConfig | null
  setEditingServer: (server: ServerConfig | null) => void
}

export const useStore = create<AppState>((set) => ({
  servers: [],
  setServers: (servers) => set({ servers }),
  addServer: (server) => set((s) => ({ servers: [...s.servers, server] })),
  updateServer: (id, updates) => set((s) => ({
    servers: s.servers.map(srv => srv.id === id ? { ...srv, ...updates } : srv),
  })),
  removeServer: (id) => set((s) => ({
    servers: s.servers.filter(srv => srv.id !== id),
  })),

  connectedServers: new Set(),
  setConnected: (id, connected) => set((s) => {
    const next = new Set(s.connectedServers)
    connected ? next.add(id) : next.delete(id)
    return { connectedServers: next }
  }),

  activeServerId: null,
  activeService: null,
  setActiveServer: (id) => set({ activeServerId: id, activeService: null, logs: [] }),
  setActiveService: (service) => set({ activeService: service, logs: [] }),

  viewMode: 'single',
  setViewMode: (mode) => set({ viewMode: mode }),

  logs: [],
  addLog: (entry) => set((s) => {
    const next = [...s.logs, entry]
    return { logs: next.length > s.maxLogs ? next.slice(-s.maxLogs) : next }
  }),
  clearLogs: () => set({ logs: [] }),
  maxLogs: 5000,

  logFilter: '',
  setLogFilter: (filter) => set({ logFilter: filter }),
  logLevelFilter: new Set(['error', 'warn', 'info', 'debug', 'unknown']),
  toggleLevelFilter: (level) => set((s) => {
    const next = new Set(s.logLevelFilter)
    next.has(level) ? next.delete(level) : next.add(level)
    return { logLevelFilter: next }
  }),

  autoScroll: true,
  toggleAutoScroll: () => set((s) => ({ autoScroll: !s.autoScroll })),

  aiProvider: 'claude',
  setAIProvider: (provider) => set({ aiProvider: provider }),
  chatMessages: [],
  addChatMessage: (message) => set((s) => ({ chatMessages: [...s.chatMessages, message] })),
  updateLastChatMessage: (content) => set((s) => {
    const msgs = [...s.chatMessages]
    if (msgs.length > 0) {
      msgs[msgs.length - 1] = { ...msgs[msgs.length - 1], content }
    }
    return { chatMessages: msgs }
  }),
  clearChat: () => set({ chatMessages: [] }),
  selectedLogText: '',
  setSelectedLogText: (text) => set({ selectedLogText: text }),
  aiChatOpen: false,
  setAIChatOpen: (open) => set({ aiChatOpen: open }),
  isAILoading: false,
  setAILoading: (loading) => set({ isAILoading: loading }),

  showServerForm: false,
  setShowServerForm: (show) => set({ showServerForm: show }),
  editingServer: null,
  setEditingServer: (server) => set({ editingServer: server }),
}))
