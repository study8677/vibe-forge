// Package cricket implements the "始皇防蛐蛐" plugin — a regex‑based
// keyword monitor that watches all incoming content for configurable
// patterns and emits alerts in real time.
package cricket

import (
	"fmt"
	"regexp"
	"strings"
	"sync"
	"time"

	"shihuang-guard/internal/model"
)

const (
	Name    = "始皇防蛐蛐"
	Desc    = "全站关键词正则监控与自动预警 · 秦始皇の蛐蛐防御系统"
	Version = "1.0.0"
)

// Guard is the core cricket‑guard plugin.
type Guard struct {
	rules    []*model.Rule
	compiled map[string]*regexp.Regexp
	enabled  bool
	mu       sync.RWMutex
}

// New creates a Guard pre‑loaded with a sensible set of default rules.
func New() *Guard {
	g := &Guard{
		compiled: make(map[string]*regexp.Regexp),
		enabled:  true,
	}
	g.seedDefaults()
	return g
}

// ---------- Plugin interface ------------------------------------------------

func (g *Guard) Name() string        { return Name }
func (g *Guard) Description() string  { return Desc }
func (g *Guard) Version() string      { return Version }
func (g *Guard) Enabled() bool        { g.mu.RLock(); defer g.mu.RUnlock(); return g.enabled }
func (g *Guard) SetEnabled(v bool)    { g.mu.Lock(); g.enabled = v; g.mu.Unlock() }

func (g *Guard) Init(_ map[string]interface{}) error { return nil }
func (g *Guard) Start() error                        { return nil }
func (g *Guard) Stop() error                         { return nil }

// Scan checks content against every enabled rule and returns any alerts.
func (g *Guard) Scan(content *model.Content) ([]*model.Alert, error) {
	g.mu.RLock()
	defer g.mu.RUnlock()

	var alerts []*model.Alert
	for _, rule := range g.rules {
		if !rule.Enabled {
			continue
		}
		re, ok := g.compiled[rule.ID]
		if !ok {
			continue
		}
		matches := re.FindAllString(content.Text, -1)
		if len(matches) == 0 {
			continue
		}

		snippet := content.Text
		if len(snippet) > 120 {
			snippet = snippet[:120] + "…"
		}

		alerts = append(alerts, &model.Alert{
			ID:             fmt.Sprintf("alert_%d", time.Now().UnixNano()),
			PluginName:     Name,
			Level:          rule.Level,
			ContentSnippet: snippet,
			MatchedText:    strings.Join(dedup(matches), " | "),
			RuleID:         rule.ID,
			RuleName:       rule.Name,
			Source:         content.Source,
			Timestamp:      time.Now(),
		})
	}
	return alerts, nil
}

// ---------- Rule management -------------------------------------------------

func (g *Guard) Rules() []*model.Rule {
	g.mu.RLock()
	defer g.mu.RUnlock()
	cp := make([]*model.Rule, len(g.rules))
	copy(cp, g.rules)
	return cp
}

func (g *Guard) AddRule(rule *model.Rule) error {
	re, err := regexp.Compile(rule.Pattern)
	if err != nil {
		return fmt.Errorf("invalid regex pattern: %w", err)
	}
	g.mu.Lock()
	defer g.mu.Unlock()

	if rule.ID == "" {
		rule.ID = fmt.Sprintf("cricket_%d", time.Now().UnixNano())
	}
	rule.PluginID = Name
	now := time.Now()
	rule.CreatedAt = now
	rule.UpdatedAt = now

	g.rules = append(g.rules, rule)
	g.compiled[rule.ID] = re
	return nil
}

func (g *Guard) UpdateRule(rule *model.Rule) error {
	re, err := regexp.Compile(rule.Pattern)
	if err != nil {
		return fmt.Errorf("invalid regex pattern: %w", err)
	}
	g.mu.Lock()
	defer g.mu.Unlock()

	for i, r := range g.rules {
		if r.ID == rule.ID {
			rule.CreatedAt = r.CreatedAt
			rule.UpdatedAt = time.Now()
			rule.PluginID = Name
			g.rules[i] = rule
			g.compiled[rule.ID] = re
			return nil
		}
	}
	return fmt.Errorf("rule %q not found", rule.ID)
}

func (g *Guard) RemoveRule(id string) error {
	g.mu.Lock()
	defer g.mu.Unlock()
	for i, r := range g.rules {
		if r.ID == id {
			g.rules = append(g.rules[:i], g.rules[i+1:]...)
			delete(g.compiled, id)
			return nil
		}
	}
	return fmt.Errorf("rule %q not found", id)
}

// ---------- defaults --------------------------------------------------------

func (g *Guard) seedDefaults() {
	defs := []struct {
		name, pattern, category string
		level                   model.AlertLevel
	}{
		// 垃圾广告 / Spam
		{"垃圾广告检测", `(?i)(加微信|加qq|免费领取|点击链接|↓↓↓|扫码|加群)`, "spam", model.LevelWarning},
		// 恶意短链 / Malicious short URLs
		{"恶意链接检测", `(?i)(bit\.ly|t\.cn|dwz\.cn|url\.cn)/[a-zA-Z0-9]+`, "malicious", model.LevelDanger},
		// 刷屏 / Flood
		{"刷屏行为检测", `(.)\1{9,}`, "flood", model.LevelWarning},
		// 引战 / Flame‑bait
		{"引战话题检测", `(?i)(地域黑|互撕|引战|对立|挑拨)`, "flame", model.LevelWarning},
		// 诈骗 / Fraud
		{"诈骗关键词检测", `(?i)(中奖|恭喜你获得|银行卡号?|验证码泄露|请?转账|汇款到)`, "fraud", model.LevelDanger},
		// 违规引流 / Illicit promotion
		{"违规引流检测", `(?i)(私聊|私我|看主页|看简介|戳我头像|商务合作加)`, "promotion", model.LevelInfo},
		// 机器人 / Bot signatures
		{"机器人特征检测", `(?i)(自动回复|bot_test|机器人测试|auto[-_ ]?reply)`, "bot", model.LevelInfo},
		// 敏感数据泄露 / PII leak
		{"敏感数据泄露检测", `(\b1[3-9]\d{9}\b)`, "data_leak", model.LevelCritical},
	}

	now := time.Now()
	for i, d := range defs {
		rule := &model.Rule{
			ID:        fmt.Sprintf("cricket_default_%d", i+1),
			Name:      d.name,
			Pattern:   d.pattern,
			Level:     d.level,
			Category:  d.category,
			Enabled:   true,
			PluginID:  Name,
			CreatedAt: now,
			UpdatedAt: now,
		}
		g.rules = append(g.rules, rule)
		if re, err := regexp.Compile(d.pattern); err == nil {
			g.compiled[rule.ID] = re
		}
	}
}

// ---------- helpers ---------------------------------------------------------

func dedup(ss []string) []string {
	seen := make(map[string]struct{}, len(ss))
	out := make([]string, 0, len(ss))
	for _, s := range ss {
		if _, ok := seen[s]; !ok {
			seen[s] = struct{}{}
			out = append(out, s)
		}
	}
	return out
}
