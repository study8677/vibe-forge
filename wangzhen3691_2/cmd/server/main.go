package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"shihuang-guard/internal/engine"
	"shihuang-guard/internal/handler"
	"shihuang-guard/internal/hub"
	"shihuang-guard/internal/plugin"
	"shihuang-guard/internal/plugins/cricket"
	"shihuang-guard/internal/store"
)

func main() {
	fmt.Println(`
  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
  ┃      始 皇 防 蛐 蛐  ·  ShiHuang Guard          ┃
  ┃   全站关键词正则监控与自动预警系统  v1.0.0        ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛`)

	port := envOr("PORT", "8080")
	dataDir := envOr("DATA_DIR", "./data")

	// ---- storage -----------------------------------------------------------
	s, err := store.New(dataDir)
	if err != nil {
		log.Fatalf("[FATAL] store init: %v", err)
	}

	// ---- plugin registry ---------------------------------------------------
	reg := plugin.NewRegistry()

	cricketGuard := cricket.New()
	_ = reg.Register(cricketGuard)
	log.Printf("[plugin] registered: %s v%s  (%d default rules)",
		cricketGuard.Name(), cricketGuard.Version(), len(cricketGuard.Rules()))

	// ---- engine ------------------------------------------------------------
	eng := engine.New(reg)

	// ---- websocket hub -----------------------------------------------------
	wsHub := hub.New()
	go wsHub.Run()

	// Forward engine alerts → store + websocket
	go func() {
		for alert := range eng.AlertChan() {
			s.SaveAlert(alert)
			wsHub.BroadcastAlert(alert)
		}
	}()

	// ---- HTTP --------------------------------------------------------------
	mux := http.NewServeMux()
	h := handler.New(eng, wsHub, s)
	h.Register(mux)

	// Serve built frontend (npm run build → web/dist)
	if info, err := os.Stat("./web/dist"); err == nil && info.IsDir() {
		fs := http.FileServer(http.Dir("./web/dist"))
		mux.Handle("GET /", fs)
		log.Println("[static] serving ./web/dist")
	} else {
		mux.HandleFunc("GET /", func(w http.ResponseWriter, r *http.Request) {
			if r.URL.Path != "/" {
				http.NotFound(w, r)
				return
			}
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			fmt.Fprint(w, fallbackPage)
		})
	}

	srv := &http.Server{
		Addr:    ":" + port,
		Handler: handler.CORS(mux),
	}

	// graceful shutdown
	go func() {
		ch := make(chan os.Signal, 1)
		signal.Notify(ch, syscall.SIGINT, syscall.SIGTERM)
		<-ch
		log.Println("[shutdown] stopping server…")
		srv.Close()
	}()

	log.Printf("[server]    http://localhost:%s", port)
	log.Printf("[websocket] ws://localhost:%s/ws", port)
	log.Printf("[dashboard] http://localhost:%s", port)

	if err := srv.ListenAndServe(); err != http.ErrServerClosed {
		log.Fatalf("[FATAL] server: %v", err)
	}
	log.Println("[shutdown] done")
}

func envOr(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

const fallbackPage = `<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8"/>
  <title>始皇防蛐蛐 · ShiHuang Guard</title>
  <style>
    *{margin:0;padding:0;box-sizing:border-box}
    body{background:#0f172a;color:#e2e8f0;font-family:system-ui,-apple-system,sans-serif;
         display:flex;align-items:center;justify-content:center;height:100vh}
    .c{text-align:center;max-width:480px}
    h1{font-size:2rem;margin-bottom:.5rem}
    p{color:#94a3b8;margin:.75rem 0;line-height:1.6}
    code{background:#1e293b;padding:.15rem .4rem;border-radius:.25rem;font-size:.85rem}
    a{color:#38bdf8;text-decoration:none}
    a:hover{text-decoration:underline}
  </style>
</head>
<body>
  <div class="c">
    <h1>始皇防蛐蛐</h1>
    <p>Frontend not built yet.</p>
    <p>Run <code>cd web && npm install && npm run build</code></p>
    <p style="margin-top:1.5rem">
      API: <a href="/api/stats">/api/stats</a> ·
      <a href="/api/plugins">/api/plugins</a> ·
      <a href="/api/rules">/api/rules</a>
    </p>
  </div>
</body>
</html>`
