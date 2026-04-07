package model

import (
	"encoding/json"
	"fmt"
	"time"
)

// ---------------------------------------------------------------------------
// AlertLevel
// ---------------------------------------------------------------------------

type AlertLevel int

const (
	LevelInfo AlertLevel = iota
	LevelWarning
	LevelDanger
	LevelCritical
)

var (
	levelNames = map[AlertLevel]string{
		LevelInfo:     "info",
		LevelWarning:  "warning",
		LevelDanger:   "danger",
		LevelCritical: "critical",
	}
	levelValues = map[string]AlertLevel{
		"info":     LevelInfo,
		"warning":  LevelWarning,
		"danger":   LevelDanger,
		"critical": LevelCritical,
	}
)

func (l AlertLevel) String() string {
	if s, ok := levelNames[l]; ok {
		return s
	}
	return "unknown"
}

func ParseAlertLevel(s string) AlertLevel {
	if l, ok := levelValues[s]; ok {
		return l
	}
	return LevelInfo
}

func (l AlertLevel) MarshalJSON() ([]byte, error) {
	return json.Marshal(l.String())
}

func (l *AlertLevel) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err == nil {
		*l = ParseAlertLevel(s)
		return nil
	}
	var i int
	if err := json.Unmarshal(data, &i); err == nil {
		*l = AlertLevel(i)
		return nil
	}
	return fmt.Errorf("invalid alert level: %s", string(data))
}

// ---------------------------------------------------------------------------
// Domain Models
// ---------------------------------------------------------------------------

// Rule defines a keyword‑matching rule inside a plugin.
type Rule struct {
	ID        string     `json:"id"`
	Name      string     `json:"name"`
	Pattern   string     `json:"pattern"`
	Level     AlertLevel `json:"level"`
	Category  string     `json:"category"`
	Enabled   bool       `json:"enabled"`
	PluginID  string     `json:"plugin_id"`
	CreatedAt time.Time  `json:"created_at"`
	UpdatedAt time.Time  `json:"updated_at"`
}

// Alert is produced when content matches a rule.
type Alert struct {
	ID             string     `json:"id"`
	PluginName     string     `json:"plugin_name"`
	Level          AlertLevel `json:"level"`
	ContentSnippet string     `json:"content_snippet"`
	MatchedText    string     `json:"matched_text"`
	RuleID         string     `json:"rule_id"`
	RuleName       string     `json:"rule_name"`
	Source         string     `json:"source"`
	Timestamp      time.Time  `json:"timestamp"`
}

// Content is the payload submitted for scanning.
type Content struct {
	ID        string            `json:"id"`
	Source    string            `json:"source"`
	Text      string            `json:"text"`
	Metadata  map[string]string `json:"metadata,omitempty"`
	Timestamp time.Time         `json:"timestamp"`
}

// ScanResult is returned after scanning a piece of content.
type ScanResult struct {
	ContentID string   `json:"content_id"`
	Alerts    []*Alert `json:"alerts"`
	Plugins   int      `json:"plugins_checked"`
	ScannedAt time.Time `json:"scanned_at"`
	Duration  string   `json:"duration"`
}

// PluginInfo is a read‑only summary of a registered plugin.
type PluginInfo struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Version     string `json:"version"`
	Enabled     bool   `json:"enabled"`
	RuleCount   int    `json:"rule_count"`
}

// DashboardStats powers the monitoring dashboard.
type DashboardStats struct {
	TotalScans    int64            `json:"total_scans"`
	TotalAlerts   int64            `json:"total_alerts"`
	ActiveRules   int              `json:"active_rules"`
	ActivePlugins int              `json:"active_plugins"`
	AlertsByLevel map[string]int64 `json:"alerts_by_level"`
	RecentAlerts  []*Alert         `json:"recent_alerts"`
	ScanRate      float64          `json:"scan_rate"`
}

// WSMessage is the WebSocket envelope.
type WSMessage struct {
	Type    string      `json:"type"`
	Payload interface{} `json:"payload"`
}
