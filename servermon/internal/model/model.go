package model

import "time"

// Server represents a monitored server.
type Server struct {
	ID         int64     `json:"id"`
	Name       string    `json:"name"`
	SecretKey  string    `json:"secret_key,omitempty"`
	Note       string    `json:"note"`
	SortIndex  int       `json:"sort_index"`
	Platform   string    `json:"platform"`
	CPUInfo    string    `json:"cpu_info"`
	Version    string    `json:"version"`
	Arch       string    `json:"arch"`
	Status     int       `json:"status"` // 0=offline 1=online
	LastActive time.Time `json:"last_active"`
	CreatedAt  time.Time `json:"created_at"`
	UpdatedAt  time.Time `json:"updated_at"`
}

// Metrics is a snapshot of server metrics reported by the agent.
type Metrics struct {
	ServerID     int64   `json:"server_id"`
	CPU          float64 `json:"cpu"`
	MemTotal     uint64  `json:"mem_total"`
	MemUsed      uint64  `json:"mem_used"`
	SwapTotal    uint64  `json:"swap_total"`
	SwapUsed     uint64  `json:"swap_used"`
	DiskTotal    uint64  `json:"disk_total"`
	DiskUsed     uint64  `json:"disk_used"`
	NetInSpeed   uint64  `json:"net_in_speed"`
	NetOutSpeed  uint64  `json:"net_out_speed"`
	NetInTotal   uint64  `json:"net_in_total"`
	NetOutTotal  uint64  `json:"net_out_total"`
	Load1        float64 `json:"load1"`
	Load5        float64 `json:"load5"`
	Load15       float64 `json:"load15"`
	ProcessCount uint64  `json:"process_count"`
	TCPCount     uint32  `json:"tcp_count"`
	UDPCount     uint32  `json:"udp_count"`
	Uptime       uint64  `json:"uptime"`
	Timestamp    int64   `json:"timestamp"`
}

// MetricsHistory stores sampled metrics for chart display.
type MetricsHistory struct {
	ID          int64   `json:"id"`
	ServerID    int64   `json:"server_id"`
	CPU         float64 `json:"cpu"`
	MemUsed     uint64  `json:"mem_used"`
	SwapUsed    uint64  `json:"swap_used"`
	DiskUsed    uint64  `json:"disk_used"`
	NetInSpeed  uint64  `json:"net_in_speed"`
	NetOutSpeed uint64  `json:"net_out_speed"`
	Load1       float64 `json:"load1"`
	CreatedAt   int64   `json:"created_at"`
}

// AlertRule defines a monitoring alert condition.
type AlertRule struct {
	ID         int64   `json:"id"`
	Name       string  `json:"name"`
	ServerIDs  string  `json:"server_ids"` // comma-separated; empty = all
	MetricType string  `json:"metric_type"`
	Operator   string  `json:"operator"`
	Threshold  float64 `json:"threshold"`
	Duration   int     `json:"duration"` // seconds before firing
	Enabled    bool    `json:"enabled"`
	CreatedAt  int64   `json:"created_at"`
	UpdatedAt  int64   `json:"updated_at"`
}

// AlertEvent records a triggered alert.
type AlertEvent struct {
	ID         int64   `json:"id"`
	RuleID     int64   `json:"rule_id"`
	RuleName   string  `json:"rule_name"`
	ServerID   int64   `json:"server_id"`
	ServerName string  `json:"server_name"`
	MetricType string  `json:"metric_type"`
	Value      float64 `json:"value"`
	Threshold  float64 `json:"threshold"`
	Message    string  `json:"message"`
	Status     string  `json:"status"` // firing, resolved
	CreatedAt  int64   `json:"created_at"`
	ResolvedAt int64   `json:"resolved_at,omitempty"`
}

// ServerWithMetrics combines server info with live metrics.
type ServerWithMetrics struct {
	Server  `json:"server"`
	Metrics *Metrics `json:"metrics,omitempty"`
}

// AgentReport is the payload sent by the agent.
type AgentReport struct {
	SecretKey string   `json:"secret_key"`
	Host      HostInfo `json:"host"`
	Metrics   Metrics  `json:"metrics"`
}

// HostInfo describes the host platform.
type HostInfo struct {
	Platform string `json:"platform"`
	CPUInfo  string `json:"cpu_info"`
	Version  string `json:"version"`
	Arch     string `json:"arch"`
}

// DashboardStats holds summary statistics.
type DashboardStats struct {
	TotalServers  int `json:"total_servers"`
	OnlineServers int `json:"online_servers"`
	ActiveAlerts  int `json:"active_alerts"`
	TotalRules    int `json:"total_rules"`
}

// Response is the standard API response wrapper.
type Response struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	Data    any    `json:"data,omitempty"`
}

// WSMessage is a WebSocket push message.
type WSMessage struct {
	Type string `json:"type"`
	Data any    `json:"data"`
}
