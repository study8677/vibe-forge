package handler

import (
	"encoding/json"
	"log"
	"net/http"
	"regexp"

	"github.com/gorilla/websocket"

	"shihuang-guard/internal/engine"
	"shihuang-guard/internal/hub"
	"shihuang-guard/internal/model"
	"shihuang-guard/internal/store"
)

// Handler wires the HTTP/WebSocket layer to the engine.
type Handler struct {
	engine   *engine.Engine
	hub      *hub.Hub
	store    *store.Store
	upgrader websocket.Upgrader
}

// New creates a Handler.
func New(e *engine.Engine, h *hub.Hub, s *store.Store) *Handler {
	return &Handler{
		engine: e,
		hub:    h,
		store:  s,
		upgrader: websocket.Upgrader{
			CheckOrigin: func(*http.Request) bool { return true },
		},
	}
}

// Register mounts all routes on the given ServeMux.
func (h *Handler) Register(mux *http.ServeMux) {
	mux.HandleFunc("GET /api/stats", h.stats)
	mux.HandleFunc("GET /api/alerts", h.listAlerts)
	mux.HandleFunc("POST /api/scan", h.scan)

	mux.HandleFunc("GET /api/rules", h.listRules)
	mux.HandleFunc("POST /api/rules", h.createRule)
	mux.HandleFunc("PUT /api/rules/{id}", h.updateRule)
	mux.HandleFunc("DELETE /api/rules/{id}", h.deleteRule)
	mux.HandleFunc("POST /api/rules/test", h.testRule)

	mux.HandleFunc("GET /api/plugins", h.listPlugins)
	mux.HandleFunc("POST /api/plugins/{name}/toggle", h.togglePlugin)

	mux.HandleFunc("GET /ws", h.ws)
}

// CORS wraps a handler with permissive CORS headers for dev.
func CORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type,Authorization")
		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}
		next.ServeHTTP(w, r)
	})
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

func writeJSON(w http.ResponseWriter, code int, v interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	_ = json.NewEncoder(w).Encode(v)
}

func writeErr(w http.ResponseWriter, code int, msg string) {
	writeJSON(w, code, map[string]string{"error": msg})
}

// ---------------------------------------------------------------------------
// route handlers
// ---------------------------------------------------------------------------

func (h *Handler) stats(w http.ResponseWriter, _ *http.Request) {
	s := h.engine.Stats()
	s.RecentAlerts = h.store.RecentAlerts(20)
	s.TotalAlerts = h.store.AlertCount()
	writeJSON(w, http.StatusOK, s)
}

func (h *Handler) listAlerts(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, http.StatusOK, h.store.RecentAlerts(200))
}

func (h *Handler) scan(w http.ResponseWriter, r *http.Request) {
	var c model.Content
	if err := json.NewDecoder(r.Body).Decode(&c); err != nil {
		writeErr(w, http.StatusBadRequest, "invalid JSON body")
		return
	}
	if c.Text == "" {
		writeErr(w, http.StatusBadRequest, "text is required")
		return
	}
	if c.Source == "" {
		c.Source = "api"
	}

	res, err := h.engine.Scan(&c)
	if err != nil {
		writeErr(w, http.StatusInternalServerError, err.Error())
		return
	}

	// persist + broadcast
	for _, a := range res.Alerts {
		h.store.SaveAlert(a)
		h.hub.BroadcastAlert(a)
	}
	stats := h.engine.Stats()
	stats.RecentAlerts = h.store.RecentAlerts(20)
	stats.TotalAlerts = h.store.AlertCount()
	h.hub.BroadcastStats(stats)

	writeJSON(w, http.StatusOK, res)
}

// ---- rules -----------------------------------------------------------------

func (h *Handler) listRules(w http.ResponseWriter, _ *http.Request) {
	var all []*model.Rule
	for _, p := range h.engine.Registry().All() {
		all = append(all, p.Rules()...)
	}
	if all == nil {
		all = []*model.Rule{}
	}
	writeJSON(w, http.StatusOK, all)
}

func (h *Handler) createRule(w http.ResponseWriter, r *http.Request) {
	var rule model.Rule
	if err := json.NewDecoder(r.Body).Decode(&rule); err != nil {
		writeErr(w, http.StatusBadRequest, "invalid JSON body")
		return
	}
	if rule.Pattern == "" {
		writeErr(w, http.StatusBadRequest, "pattern is required")
		return
	}
	pluginName := rule.PluginID
	if pluginName == "" {
		pluginName = "始皇防蛐蛐"
	}
	p, ok := h.engine.Registry().Get(pluginName)
	if !ok {
		writeErr(w, http.StatusNotFound, "plugin not found: "+pluginName)
		return
	}
	if err := p.AddRule(&rule); err != nil {
		writeErr(w, http.StatusBadRequest, err.Error())
		return
	}
	writeJSON(w, http.StatusCreated, rule)
}

func (h *Handler) updateRule(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	var rule model.Rule
	if err := json.NewDecoder(r.Body).Decode(&rule); err != nil {
		writeErr(w, http.StatusBadRequest, "invalid JSON body")
		return
	}
	rule.ID = id
	for _, p := range h.engine.Registry().All() {
		for _, existing := range p.Rules() {
			if existing.ID == id {
				if err := p.UpdateRule(&rule); err != nil {
					writeErr(w, http.StatusBadRequest, err.Error())
					return
				}
				writeJSON(w, http.StatusOK, rule)
				return
			}
		}
	}
	writeErr(w, http.StatusNotFound, "rule not found")
}

func (h *Handler) deleteRule(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	for _, p := range h.engine.Registry().All() {
		for _, rule := range p.Rules() {
			if rule.ID == id {
				if err := p.RemoveRule(id); err != nil {
					writeErr(w, http.StatusInternalServerError, err.Error())
					return
				}
				writeJSON(w, http.StatusOK, map[string]string{"deleted": id})
				return
			}
		}
	}
	writeErr(w, http.StatusNotFound, "rule not found")
}

func (h *Handler) testRule(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Pattern string `json:"pattern"`
		Text    string `json:"text"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeErr(w, http.StatusBadRequest, "invalid JSON body")
		return
	}

	re, err := regexp.Compile(req.Pattern)
	if err != nil {
		writeJSON(w, http.StatusOK, map[string]interface{}{
			"valid": false, "error": err.Error(), "matches": []string{},
		})
		return
	}
	matches := re.FindAllString(req.Text, -1)
	if matches == nil {
		matches = []string{}
	}
	writeJSON(w, http.StatusOK, map[string]interface{}{
		"valid": true, "matches": matches, "count": len(matches),
	})
}

// ---- plugins ---------------------------------------------------------------

func (h *Handler) listPlugins(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, http.StatusOK, h.engine.Registry().Info())
}

func (h *Handler) togglePlugin(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("name")
	if err := h.engine.Registry().Toggle(name); err != nil {
		writeErr(w, http.StatusNotFound, err.Error())
		return
	}
	writeJSON(w, http.StatusOK, map[string]string{"toggled": name})
}

// ---- websocket -------------------------------------------------------------

func (h *Handler) ws(w http.ResponseWriter, r *http.Request) {
	conn, err := h.upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("[WS] upgrade error: %v", err)
		return
	}
	h.hub.AddClient(conn)
}
