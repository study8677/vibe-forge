package main

import (
	"flag"
	"log"
	"net/http"
	"os"
	"path/filepath"

	"servermon/internal/alert"
	"servermon/internal/api"
	"servermon/internal/store"
	"servermon/internal/ws"
)

func main() {
	addr := flag.String("addr", ":8080", "listen address")
	dbPath := flag.String("db", "servermon.db", "SQLite database path")
	distDir := flag.String("dist", "", "frontend dist directory (auto-detected if empty)")
	flag.Parse()

	// auto-detect frontend dist directory
	if *distDir == "" {
		candidates := []string{"web/dist", "../web/dist", "dist"}
		for _, c := range candidates {
			if info, err := os.Stat(c); err == nil && info.IsDir() {
				abs, _ := filepath.Abs(c)
				*distDir = abs
				break
			}
		}
		if *distDir == "" {
			*distDir = "web/dist"
		}
	}

	// init store
	db, err := store.New(*dbPath)
	if err != nil {
		log.Fatalf("init store: %v", err)
	}
	defer db.Close()

	// init websocket hub
	hub := ws.NewHub()

	// init alert engine
	eng := alert.NewEngine(db, hub)
	eng.Start()

	// init router
	router := api.NewRouter(db, hub, eng, *distDir)

	log.Printf("ServerMon starting on %s", *addr)
	log.Printf("Frontend: %s", *distDir)
	log.Printf("Database: %s", *dbPath)

	if err := http.ListenAndServe(*addr, router); err != nil {
		log.Fatalf("server: %v", err)
	}
}
