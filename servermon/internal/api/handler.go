package api

import (
	"encoding/json"
	"log"
	"net/http"
	"strconv"
	"sync"
	"time"

	"github.com/google/uuid"

	"servermon/internal/alert"
	"servermon/internal/model"
	"servermon/internal/store"
	"servermon/internal/ws"
)

type Handler struct {
	store   *store.Store
	hub     *ws.Hub
	alert   *alert.Engine
	metrics sync.Map // int64 -> *model.Metrics
	lastSave sync.Map // int64 -> int64 (unix)
}

func NewHandler(s *store.Store, h *ws.Hub, a *alert.Engine) *Handler {
	handler := &Handler{store: s, hub: h, alert: a}
	go handler.offlineChecker()
	return handler
}

// ── JSON helpers ──

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func ok(w http.ResponseWriter, data any) {
	writeJSON(w, 200, model.Response{Code: 0, Message: "ok", Data: data})
}

func fail(w http.ResponseWriter, status int, msg string) {
	writeJSON(w, status, model.Response{Code: status, Message: msg})
}

func bind(r *http.Request, v any) error {
	defer r.Body.Close()
	return json.NewDecoder(r.Body).Decode(v)
}

func pathID(r *http.Request) (int64, error) {
	return strconv.ParseInt(r.PathValue("id"), 10, 64)
}

// ── Server Handlers ──

func (h *Handler) ListServers(w http.ResponseWriter, r *http.Request) {
	servers, err := h.store.ListServers()
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	result := make([]model.ServerWithMetrics, len(servers))
	for i, s := range servers {
		result[i].Server = s
		if m, loaded := h.metrics.Load(s.ID); loaded {
			metrics := m.(*model.Metrics)
			result[i].Metrics = metrics
		}
	}
	ok(w, result)
}

func (h *Handler) GetServer(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	srv, err := h.store.GetServer(id)
	if err != nil {
		fail(w, 404, "server not found")
		return
	}
	result := model.ServerWithMetrics{Server: *srv}
	if m, loaded := h.metrics.Load(id); loaded {
		result.Metrics = m.(*model.Metrics)
	}
	ok(w, result)
}

func (h *Handler) CreateServer(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Name string `json:"name"`
	}
	if err := bind(r, &req); err != nil || req.Name == "" {
		fail(w, 400, "name is required")
		return
	}
	key := uuid.New().String()
	srv, err := h.store.CreateServer(req.Name, key)
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	ok(w, srv)
}

func (h *Handler) UpdateServer(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	var req struct {
		Name      string `json:"name"`
		Note      string `json:"note"`
		SortIndex int    `json:"sort_index"`
	}
	if err := bind(r, &req); err != nil {
		fail(w, 400, "invalid request")
		return
	}
	srv := &model.Server{ID: id, Name: req.Name, Note: req.Note, SortIndex: req.SortIndex}
	if err := h.store.UpdateServer(srv); err != nil {
		fail(w, 500, err.Error())
		return
	}
	ok(w, nil)
}

func (h *Handler) DeleteServer(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	if err := h.store.DeleteServer(id); err != nil {
		fail(w, 500, err.Error())
		return
	}
	h.metrics.Delete(id)
	h.lastSave.Delete(id)
	ok(w, nil)
}

func (h *Handler) GetServerMetrics(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	duration := r.URL.Query().Get("duration")
	since := parseDuration(duration)
	list, err := h.store.GetMetricsHistory(id, since)
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	if list == nil {
		list = []model.MetricsHistory{}
	}
	ok(w, list)
}

// ── Alert Rule Handlers ──

func (h *Handler) ListAlertRules(w http.ResponseWriter, r *http.Request) {
	rules, err := h.store.ListAlertRules()
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	if rules == nil {
		rules = []model.AlertRule{}
	}
	ok(w, rules)
}

func (h *Handler) CreateAlertRule(w http.ResponseWriter, r *http.Request) {
	var rule model.AlertRule
	if err := bind(r, &rule); err != nil || rule.Name == "" || rule.MetricType == "" {
		fail(w, 400, "name and metric_type are required")
		return
	}
	if rule.Operator == "" {
		rule.Operator = ">"
	}
	if rule.Duration == 0 {
		rule.Duration = 60
	}
	id, err := h.store.CreateAlertRule(&rule)
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	rule.ID = id
	ok(w, rule)
}

func (h *Handler) UpdateAlertRule(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	var rule model.AlertRule
	if err := bind(r, &rule); err != nil {
		fail(w, 400, "invalid request")
		return
	}
	rule.ID = id
	if err := h.store.UpdateAlertRule(&rule); err != nil {
		fail(w, 500, err.Error())
		return
	}
	ok(w, nil)
}

func (h *Handler) DeleteAlertRule(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	if err := h.store.DeleteAlertRule(id); err != nil {
		fail(w, 500, err.Error())
		return
	}
	ok(w, nil)
}

// ── Alert Event Handlers ──

func (h *Handler) ListAlertEvents(w http.ResponseWriter, r *http.Request) {
	limit := 100
	if v := r.URL.Query().Get("limit"); v != "" {
		if n, err := strconv.Atoi(v); err == nil && n > 0 {
			limit = n
		}
	}
	events, err := h.store.ListAlertEvents(limit)
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	if events == nil {
		events = []model.AlertEvent{}
	}
	ok(w, events)
}

func (h *Handler) ResolveAlert(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r)
	if err != nil {
		fail(w, 400, "invalid id")
		return
	}
	if err := h.store.ResolveAlert(id); err != nil {
		fail(w, 500, err.Error())
		return
	}
	ok(w, nil)
}

// ── Agent Report ──

func (h *Handler) AgentReport(w http.ResponseWriter, r *http.Request) {
	var report model.AgentReport
	if err := bind(r, &report); err != nil {
		fail(w, 400, "invalid payload")
		return
	}

	srv, err := h.store.GetServerByKey(report.SecretKey)
	if err != nil {
		fail(w, 401, "invalid secret key")
		return
	}

	// update status
	if err := h.store.UpdateServerStatus(srv.ID, 1); err != nil {
		log.Printf("update status: %v", err)
	}

	// update host info if changed
	if report.Host.Platform != "" && (srv.Platform != report.Host.Platform || srv.Arch != report.Host.Arch) {
		h.store.UpdateServerHost(srv.ID, report.Host)
	}

	// update in-memory metrics
	report.Metrics.ServerID = srv.ID
	report.Metrics.Timestamp = time.Now().Unix()
	h.metrics.Store(srv.ID, &report.Metrics)

	// update alert engine
	h.alert.UpdateMetrics(srv.ID, &report.Metrics)

	// sample to history (every 30 seconds)
	now := time.Now().Unix()
	shouldSave := true
	if v, loaded := h.lastSave.Load(srv.ID); loaded {
		if now-v.(int64) < 30 {
			shouldSave = false
		}
	}
	if shouldSave {
		h.lastSave.Store(srv.ID, now)
		h.store.InsertMetrics(&model.MetricsHistory{
			ServerID:    srv.ID,
			CPU:         report.Metrics.CPU,
			MemUsed:     report.Metrics.MemUsed,
			SwapUsed:    report.Metrics.SwapUsed,
			DiskUsed:    report.Metrics.DiskUsed,
			NetInSpeed:  report.Metrics.NetInSpeed,
			NetOutSpeed: report.Metrics.NetOutSpeed,
			Load1:       report.Metrics.Load1,
			CreatedAt:   now,
		})
	}

	// broadcast to dashboard clients
	h.hub.Broadcast(model.WSMessage{
		Type: "metrics",
		Data: map[string]any{
			"server_id": srv.ID,
			"metrics":   report.Metrics,
		},
	})

	ok(w, nil)
}

// ── Dashboard ──

func (h *Handler) DashboardStats(w http.ResponseWriter, r *http.Request) {
	servers, err := h.store.ListServers()
	if err != nil {
		fail(w, 500, err.Error())
		return
	}
	online := 0
	for _, s := range servers {
		if s.Status == 1 {
			online++
		}
	}
	firingCount, _ := h.store.CountFiringAlerts()
	ruleCount, _ := h.store.CountAlertRules()

	ok(w, model.DashboardStats{
		TotalServers:  len(servers),
		OnlineServers: online,
		ActiveAlerts:  firingCount,
		TotalRules:    ruleCount,
	})
}

// ── Offline Checker ──

func (h *Handler) offlineChecker() {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()
	for range ticker.C {
		servers, err := h.store.ListServers()
		if err != nil {
			continue
		}
		for _, s := range servers {
			if s.Status == 1 && time.Since(s.LastActive) > 60*time.Second {
				h.store.UpdateServerStatus(s.ID, 0)
				h.metrics.Delete(s.ID)
				h.hub.Broadcast(model.WSMessage{
					Type: "server_status",
					Data: map[string]any{"server_id": s.ID, "status": 0},
				})
				log.Printf("server %s (#%d) marked offline", s.Name, s.ID)
			}
		}
	}
}

// ── Helpers ──

func parseDuration(d string) int64 {
	now := time.Now().Unix()
	switch d {
	case "6h":
		return now - 6*3600
	case "24h":
		return now - 24*3600
	case "7d":
		return now - 7*86400
	default: // "1h" or empty
		return now - 3600
	}
}
