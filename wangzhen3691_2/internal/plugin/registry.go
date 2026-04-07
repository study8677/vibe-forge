package plugin

import (
	"fmt"
	"sync"

	"shihuang-guard/internal/model"
)

// Registry is a thread‑safe plugin container.
type Registry struct {
	plugins map[string]Plugin
	mu      sync.RWMutex
}

func NewRegistry() *Registry {
	return &Registry{plugins: make(map[string]Plugin)}
}

// Register adds a plugin. Duplicate names are rejected.
func (r *Registry) Register(p Plugin) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	if _, ok := r.plugins[p.Name()]; ok {
		return fmt.Errorf("plugin %q already registered", p.Name())
	}
	r.plugins[p.Name()] = p
	return nil
}

// Get returns a plugin by name.
func (r *Registry) Get(name string) (Plugin, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()
	p, ok := r.plugins[name]
	return p, ok
}

// All returns every registered plugin.
func (r *Registry) All() []Plugin {
	r.mu.RLock()
	defer r.mu.RUnlock()
	out := make([]Plugin, 0, len(r.plugins))
	for _, p := range r.plugins {
		out = append(out, p)
	}
	return out
}

// Enabled returns only enabled plugins.
func (r *Registry) Enabled() []Plugin {
	r.mu.RLock()
	defer r.mu.RUnlock()
	var out []Plugin
	for _, p := range r.plugins {
		if p.Enabled() {
			out = append(out, p)
		}
	}
	return out
}

// Info returns a read‑only summary for the dashboard.
func (r *Registry) Info() []*model.PluginInfo {
	r.mu.RLock()
	defer r.mu.RUnlock()
	infos := make([]*model.PluginInfo, 0, len(r.plugins))
	for _, p := range r.plugins {
		infos = append(infos, &model.PluginInfo{
			Name:        p.Name(),
			Description: p.Description(),
			Version:     p.Version(),
			Enabled:     p.Enabled(),
			RuleCount:   len(p.Rules()),
		})
	}
	return infos
}

// Toggle flips the enabled state of a plugin.
func (r *Registry) Toggle(name string) error {
	r.mu.RLock()
	defer r.mu.RUnlock()
	p, ok := r.plugins[name]
	if !ok {
		return fmt.Errorf("plugin %q not found", name)
	}
	p.SetEnabled(!p.Enabled())
	return nil
}
