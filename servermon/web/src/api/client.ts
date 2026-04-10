const BASE = ''

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(BASE + path, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  })
  const json = await res.json()
  if (json.code !== 0) {
    throw new Error(json.message || 'Request failed')
  }
  return json.data
}

// ── Types ──

export interface Server {
  id: number
  name: string
  secret_key?: string
  note: string
  sort_index: number
  platform: string
  cpu_info: string
  version: string
  arch: string
  status: number
  last_active: string
  created_at: string
  updated_at: string
}

export interface Metrics {
  server_id: number
  cpu: number
  mem_total: number
  mem_used: number
  swap_total: number
  swap_used: number
  disk_total: number
  disk_used: number
  net_in_speed: number
  net_out_speed: number
  net_in_total: number
  net_out_total: number
  load1: number
  load5: number
  load15: number
  process_count: number
  tcp_count: number
  udp_count: number
  uptime: number
  timestamp: number
}

export interface ServerWithMetrics {
  server: Server
  metrics?: Metrics
}

export interface MetricsHistory {
  id: number
  server_id: number
  cpu: number
  mem_used: number
  swap_used: number
  disk_used: number
  net_in_speed: number
  net_out_speed: number
  load1: number
  created_at: number
}

export interface AlertRule {
  id: number
  name: string
  server_ids: string
  metric_type: string
  operator: string
  threshold: number
  duration: number
  enabled: boolean
  created_at: number
  updated_at: number
}

export interface AlertEvent {
  id: number
  rule_id: number
  rule_name: string
  server_id: number
  server_name: string
  metric_type: string
  value: number
  threshold: number
  message: string
  status: string
  created_at: number
  resolved_at: number
}

export interface DashboardStats {
  total_servers: number
  online_servers: number
  active_alerts: number
  total_rules: number
}

// ── API Calls ──

export const api = {
  // Dashboard
  getStats: () => request<DashboardStats>('/api/dashboard/stats'),

  // Servers
  listServers: () => request<ServerWithMetrics[]>('/api/servers'),
  getServer: (id: number) => request<ServerWithMetrics>(`/api/servers/${id}`),
  createServer: (name: string) =>
    request<Server>('/api/servers', { method: 'POST', body: JSON.stringify({ name }) }),
  updateServer: (id: number, data: { name: string; note: string; sort_index: number }) =>
    request<null>(`/api/servers/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteServer: (id: number) =>
    request<null>(`/api/servers/${id}`, { method: 'DELETE' }),
  getServerMetrics: (id: number, duration = '1h') =>
    request<MetricsHistory[]>(`/api/servers/${id}/metrics?duration=${duration}`),

  // Alert Rules
  listAlertRules: () => request<AlertRule[]>('/api/alerts/rules'),
  createAlertRule: (rule: Partial<AlertRule>) =>
    request<AlertRule>('/api/alerts/rules', { method: 'POST', body: JSON.stringify(rule) }),
  updateAlertRule: (id: number, rule: Partial<AlertRule>) =>
    request<null>(`/api/alerts/rules/${id}`, { method: 'PUT', body: JSON.stringify(rule) }),
  deleteAlertRule: (id: number) =>
    request<null>(`/api/alerts/rules/${id}`, { method: 'DELETE' }),

  // Alert Events
  listAlertEvents: (limit = 100) => request<AlertEvent[]>(`/api/alerts/events?limit=${limit}`),
  resolveAlert: (id: number) =>
    request<null>(`/api/alerts/events/${id}/resolve`, { method: 'POST' }),
}

// ── Helpers ──

export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i]
}

export function formatSpeed(bytesPerSec: number): string {
  return formatBytes(bytesPerSec) + '/s'
}

export function formatUptime(seconds: number): string {
  const d = Math.floor(seconds / 86400)
  const h = Math.floor((seconds % 86400) / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  if (d > 0) return `${d}d ${h}h`
  if (h > 0) return `${h}h ${m}m`
  return `${m}m`
}

export function formatTime(ts: number): string {
  if (!ts) return '-'
  return new Date(ts * 1000).toLocaleString('zh-CN')
}

export function pct(used: number, total: number): number {
  if (total === 0) return 0
  return Math.round((used / total) * 1000) / 10
}
