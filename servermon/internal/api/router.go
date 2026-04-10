package api

import (
	"io/fs"
	"net/http"
	"os"
	"path/filepath"

	"servermon/internal/alert"
	"servermon/internal/store"
	"servermon/internal/ws"
)

func NewRouter(st *store.Store, hub *ws.Hub, eng *alert.Engine, distDir string) http.Handler {
	h := NewHandler(st, hub, eng)

	mux := http.NewServeMux()

	// Server management
	mux.HandleFunc("GET /api/servers", h.ListServers)
	mux.HandleFunc("GET /api/servers/{id}", h.GetServer)
	mux.HandleFunc("POST /api/servers", h.CreateServer)
	mux.HandleFunc("PUT /api/servers/{id}", h.UpdateServer)
	mux.HandleFunc("DELETE /api/servers/{id}", h.DeleteServer)
	mux.HandleFunc("GET /api/servers/{id}/metrics", h.GetServerMetrics)

	// Alert rules
	mux.HandleFunc("GET /api/alerts/rules", h.ListAlertRules)
	mux.HandleFunc("POST /api/alerts/rules", h.CreateAlertRule)
	mux.HandleFunc("PUT /api/alerts/rules/{id}", h.UpdateAlertRule)
	mux.HandleFunc("DELETE /api/alerts/rules/{id}", h.DeleteAlertRule)

	// Alert events
	mux.HandleFunc("GET /api/alerts/events", h.ListAlertEvents)
	mux.HandleFunc("POST /api/alerts/events/{id}/resolve", h.ResolveAlert)

	// Agent
	mux.HandleFunc("POST /api/agent/report", h.AgentReport)

	// Dashboard stats
	mux.HandleFunc("GET /api/dashboard/stats", h.DashboardStats)

	// WebSocket
	mux.HandleFunc("GET /ws", func(w http.ResponseWriter, r *http.Request) {
		hub.HandleWS(w, r)
	})

	// Serve frontend (SPA fallback)
	mux.HandleFunc("GET /", frontendHandler(distDir))

	return corsMiddleware(mux)
}

func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")
		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}
		next.ServeHTTP(w, r)
	})
}

func frontendHandler(distDir string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// check if dist dir exists
		if _, err := os.Stat(distDir); os.IsNotExist(err) {
			if r.URL.Path == "/" {
				w.Header().Set("Content-Type", "text/html")
				w.Write([]byte(`<!DOCTYPE html><html><body style="font-family:sans-serif;display:flex;justify-content:center;align-items:center;height:100vh;background:#0f172a;color:#94a3b8">
<div style="text-align:center"><h1 style="color:#e2e8f0">ServerMon</h1><p>Frontend not built. Run: <code style="background:#1e293b;padding:4px 8px;border-radius:4px">cd web && npm install && npm run build</code></p></div>
</body></html>`))
				return
			}
			http.NotFound(w, r)
			return
		}

		// try to serve the actual file
		path := filepath.Join(distDir, r.URL.Path)
		if info, err := os.Stat(path); err == nil && !info.IsDir() {
			http.ServeFile(w, r, path)
			return
		}

		// try as directory with index.html
		if info, err := os.Stat(path); err == nil && info.IsDir() {
			index := filepath.Join(path, "index.html")
			if _, err := os.Stat(index); err == nil {
				http.ServeFile(w, r, index)
				return
			}
		}

		// SPA fallback
		index := filepath.Join(distDir, "index.html")
		if _, err := os.Stat(index); err == nil {
			http.ServeFile(w, r, index)
			return
		}

		http.NotFound(w, r)
	}
}

// EmbedFrontend can be used if you want to embed the dist folder.
// For now we serve from disk.
var _ fs.FS
