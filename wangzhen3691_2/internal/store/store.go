package store

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sync"

	"shihuang-guard/internal/model"
)

const maxAlerts = 10000

// Store provides simple JSON‑file‑backed persistence for alerts.
type Store struct {
	dir    string
	alerts []*model.Alert
	mu     sync.RWMutex
}

// New initialises the store, creating the data directory if needed.
func New(dir string) (*Store, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}
	s := &Store{dir: dir, alerts: make([]*model.Alert, 0, 256)}
	s.load()
	return s, nil
}

// SaveAlert persists a new alert and triggers async file write.
func (s *Store) SaveAlert(alert *model.Alert) {
	s.mu.Lock()
	s.alerts = append(s.alerts, alert)
	if len(s.alerts) > maxAlerts {
		s.alerts = s.alerts[len(s.alerts)-maxAlerts:]
	}
	s.mu.Unlock()
	go s.persist()
}

// RecentAlerts returns the N most recent alerts (newest first).
func (s *Store) RecentAlerts(n int) []*model.Alert {
	s.mu.RLock()
	defer s.mu.RUnlock()
	if n > len(s.alerts) {
		n = len(s.alerts)
	}
	out := make([]*model.Alert, n)
	for i := 0; i < n; i++ {
		out[i] = s.alerts[len(s.alerts)-1-i]
	}
	return out
}

// AlertCount returns the total stored alert count.
func (s *Store) AlertCount() int64 {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return int64(len(s.alerts))
}

func (s *Store) persist() {
	s.mu.RLock()
	defer s.mu.RUnlock()
	data, err := json.MarshalIndent(s.alerts, "", "  ")
	if err != nil {
		return
	}
	_ = os.WriteFile(filepath.Join(s.dir, "alerts.json"), data, 0o644)
}

func (s *Store) load() {
	data, err := os.ReadFile(filepath.Join(s.dir, "alerts.json"))
	if err != nil {
		return
	}
	_ = json.Unmarshal(data, &s.alerts)
}
