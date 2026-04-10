package alert

import (
	"database/sql"
	"fmt"
	"log"
	"strconv"
	"strings"
	"sync"
	"time"

	"servermon/internal/model"
	"servermon/internal/store"
	"servermon/internal/ws"
)

type Engine struct {
	store   *store.Store
	hub     *ws.Hub
	mu      sync.RWMutex
	metrics map[int64]*model.Metrics  // latest per server
	tracker map[string]int64          // "ruleID:serverID" -> first violation unix
}

func NewEngine(s *store.Store, h *ws.Hub) *Engine {
	return &Engine{
		store:   s,
		hub:     h,
		metrics: make(map[int64]*model.Metrics),
		tracker: make(map[string]int64),
	}
}

// UpdateMetrics stores latest metrics and triggers check.
func (e *Engine) UpdateMetrics(serverID int64, m *model.Metrics) {
	e.mu.Lock()
	e.metrics[serverID] = m
	e.mu.Unlock()
}

// Start launches the background alert checker.
func (e *Engine) Start() {
	go e.loop()
	go e.cleanupLoop()
}

func (e *Engine) loop() {
	ticker := time.NewTicker(10 * time.Second)
	defer ticker.Stop()
	for range ticker.C {
		e.check()
	}
}

func (e *Engine) cleanupLoop() {
	ticker := time.NewTicker(1 * time.Hour)
	defer ticker.Stop()
	for range ticker.C {
		cutoff := time.Now().Add(-24 * time.Hour).Unix()
		if n, err := e.store.CleanupOldMetrics(cutoff); err != nil {
			log.Printf("cleanup metrics: %v", err)
		} else if n > 0 {
			log.Printf("cleaned %d old metrics records", n)
		}
	}
}

func (e *Engine) check() {
	rules, err := e.store.ListAlertRules()
	if err != nil {
		log.Printf("alert check: list rules: %v", err)
		return
	}

	servers, err := e.store.ListServers()
	if err != nil {
		log.Printf("alert check: list servers: %v", err)
		return
	}

	serverMap := make(map[int64]model.Server, len(servers))
	for _, s := range servers {
		serverMap[s.ID] = s
	}

	e.mu.RLock()
	defer e.mu.RUnlock()

	for _, rule := range rules {
		if !rule.Enabled {
			continue
		}

		targetIDs := e.resolveTargets(rule, servers)
		for _, sid := range targetIDs {
			srv, ok := serverMap[sid]
			if !ok {
				continue
			}
			value, violated := e.evaluate(rule, sid, srv)
			key := fmt.Sprintf("%d:%d", rule.ID, sid)

			if violated {
				first, exists := e.tracker[key]
				if !exists {
					e.tracker[key] = time.Now().Unix()
					continue
				}
				if time.Now().Unix()-first >= int64(rule.Duration) {
					e.fireAlert(rule, srv, value)
					e.tracker[key] = time.Now().Unix() // reset to avoid spam
				}
			} else {
				delete(e.tracker, key)
				e.tryResolve(rule, srv)
			}
		}
	}
}

func (e *Engine) resolveTargets(rule model.AlertRule, servers []model.Server) []int64 {
	if rule.ServerIDs == "" {
		ids := make([]int64, len(servers))
		for i, s := range servers {
			ids[i] = s.ID
		}
		return ids
	}
	var ids []int64
	for _, s := range strings.Split(rule.ServerIDs, ",") {
		s = strings.TrimSpace(s)
		if id, err := strconv.ParseInt(s, 10, 64); err == nil {
			ids = append(ids, id)
		}
	}
	return ids
}

func (e *Engine) evaluate(rule model.AlertRule, serverID int64, srv model.Server) (float64, bool) {
	if rule.MetricType == "offline" {
		isOffline := srv.Status == 0
		if isOffline {
			return 0, true
		}
		return 1, false
	}

	m, ok := e.metrics[serverID]
	if !ok {
		return 0, false
	}

	var value float64
	switch rule.MetricType {
	case "cpu":
		value = m.CPU
	case "memory":
		if m.MemTotal > 0 {
			value = float64(m.MemUsed) / float64(m.MemTotal) * 100
		}
	case "swap":
		if m.SwapTotal > 0 {
			value = float64(m.SwapUsed) / float64(m.SwapTotal) * 100
		}
	case "disk":
		if m.DiskTotal > 0 {
			value = float64(m.DiskUsed) / float64(m.DiskTotal) * 100
		}
	case "load":
		value = m.Load1
	default:
		return 0, false
	}

	return value, compare(value, rule.Operator, rule.Threshold)
}

func compare(value float64, op string, threshold float64) bool {
	switch op {
	case ">":
		return value > threshold
	case ">=":
		return value >= threshold
	case "<":
		return value < threshold
	case "<=":
		return value <= threshold
	case "==":
		return value == threshold
	}
	return false
}

func (e *Engine) fireAlert(rule model.AlertRule, srv model.Server, value float64) {
	existing, err := e.store.GetFiringAlert(rule.ID, srv.ID)
	if err == nil && existing != nil {
		return // already firing
	}

	msg := fmt.Sprintf("[%s] %s: %s %.1f %s %.1f", srv.Name, rule.Name, rule.MetricType, value, rule.Operator, rule.Threshold)
	event := &model.AlertEvent{
		RuleID:     rule.ID,
		RuleName:   rule.Name,
		ServerID:   srv.ID,
		ServerName: srv.Name,
		MetricType: rule.MetricType,
		Value:      value,
		Threshold:  rule.Threshold,
		Message:    msg,
		Status:     "firing",
	}

	id, err := e.store.CreateAlertEvent(event)
	if err != nil {
		log.Printf("fire alert: %v", err)
		return
	}
	event.ID = id
	log.Printf("ALERT FIRING: %s", msg)

	e.hub.Broadcast(model.WSMessage{Type: "alert", Data: event})
}

func (e *Engine) tryResolve(rule model.AlertRule, srv model.Server) {
	existing, err := e.store.GetFiringAlert(rule.ID, srv.ID)
	if err == sql.ErrNoRows || existing == nil {
		return
	}
	if err != nil {
		return
	}

	if err := e.store.ResolveAlert(existing.ID); err != nil {
		log.Printf("resolve alert: %v", err)
		return
	}
	log.Printf("ALERT RESOLVED: rule=%s server=%s", rule.Name, srv.Name)

	existing.Status = "resolved"
	e.hub.Broadcast(model.WSMessage{Type: "alert_resolved", Data: existing})
}
