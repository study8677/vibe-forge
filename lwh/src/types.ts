export interface ServerConfig {
  id: string
  name: string
  host: string
  port: number
  username: string
  authType: 'password' | 'key'
  password?: string
  privateKey?: string
  passphrase?: string
  cluster?: string
  services: ServiceConfig[]
  createdAt: string
}

export interface ServiceConfig {
  name: string
  logPath: string
  type: 'file' | 'journalctl' | 'docker' | 'pm2'
  containerId?: string
}

export interface LogEntry {
  id: string
  timestamp: string
  level: 'error' | 'warn' | 'info' | 'debug' | 'trace' | 'unknown'
  message: string
  raw: string
  serverId: string
  service: string
}

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: string
}

export type AIProvider = 'claude' | 'gemini'
export type ViewMode = 'single' | 'cluster'
