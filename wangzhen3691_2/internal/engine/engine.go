package engine

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"

	"shihuang-guard/internal/model"
	"shihuang-guard/internal/plugin"
)

// Engine dispatches content to all enabled plugins and collects alerts.
type Engine struct {
	registry   *plugin.Registry
	alertCh    chan *model.Alert
	scanCount  atomic.Int64
	alertCount atomic.Int64
	startTime  time.Time
	mu         sync.RWMutex
}

// New creates a scanning engine backed by the given plugin registry.
func New(registry *plugin.Registry) *Engine {
	return &Engine{
		registry:  registry,
		alertCh:   make(chan *model.Alert, 4096),
		startTime: time.Now(),
	}
}

// Scan runs content through every enabled plugin concurrently and returns
// the aggregated result.
func (e *Engine) Scan(content *model.Content) (*model.ScanResult, error) {
	start := time.Now()
	e.scanCount.Add(1)

	if content.ID == "" {
		content.ID = fmt.Sprintf("scan_%d", time.Now().UnixNano())
	}
	if content.Timestamp.IsZero() {
		content.Timestamp = time.Now()
	}

	plugins := e.registry.Enabled()

	// Fan‑out: scan concurrently across plugins.
	type result struct {
		alerts []*model.Alert
		err    error
	}
	ch := make(chan result, len(plugins))
	for _, p := range plugins {
		go func(p plugin.Plugin) {
			a, err := p.Scan(content)
			ch <- result{a, err}
		}(p)
	}

	var allAlerts []*model.Alert
	for range plugins {
		r := <-ch
		if r.err == nil {
			allAlerts = append(allAlerts, r.alerts...)
		}
	}

	// Push alerts to the broadcast channel.
	for _, a := range allAlerts {
		e.alertCount.Add(1)
		select {
		case e.alertCh <- a:
		default: // back‑pressure: drop oldest if channel is full
		}
	}

	return &model.ScanResult{
		ContentID: content.ID,
		Alerts:    allAlerts,
		Plugins:   len(plugins),
		ScannedAt: time.Now(),
		Duration:  time.Since(start).String(),
	}, nil
}

// AlertChan returns a read‑only channel of emitted alerts.
func (e *Engine) AlertChan() <-chan *model.Alert {
	return e.alertCh
}

// Stats computes live dashboard statistics.
func (e *Engine) Stats() *model.DashboardStats {
	plugins := e.registry.All()
	activePlugins, activeRules := 0, 0
	for _, p := range plugins {
		if p.Enabled() {
			activePlugins++
			for _, r := range p.Rules() {
				if r.Enabled {
					activeRules++
				}
			}
		}
	}

	elapsed := time.Since(e.startTime).Minutes()
	rate := 0.0
	if elapsed > 0 {
		rate = float64(e.scanCount.Load()) / elapsed
	}

	return &model.DashboardStats{
		TotalScans:    e.scanCount.Load(),
		TotalAlerts:   e.alertCount.Load(),
		ActiveRules:   activeRules,
		ActivePlugins: activePlugins,
		AlertsByLevel: make(map[string]int64),
		ScanRate:      rate,
	}
}

// Registry returns the underlying plugin registry.
func (e *Engine) Registry() *plugin.Registry {
	return e.registry
}
