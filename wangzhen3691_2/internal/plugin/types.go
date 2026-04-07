package plugin

import "shihuang-guard/internal/model"

// Plugin is the interface every monitoring plugin must implement.
type Plugin interface {
	// Identity
	Name() string
	Description() string
	Version() string

	// Lifecycle
	Init(config map[string]interface{}) error
	Start() error
	Stop() error

	// Scanning — called concurrently by the engine.
	Scan(content *model.Content) ([]*model.Alert, error)

	// Rule management
	Rules() []*model.Rule
	AddRule(rule *model.Rule) error
	UpdateRule(rule *model.Rule) error
	RemoveRule(id string) error

	// Toggle
	Enabled() bool
	SetEnabled(enabled bool)
}
