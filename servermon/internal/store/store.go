package store

import (
	"database/sql"
	"fmt"
	"time"

	_ "modernc.org/sqlite"

	"servermon/internal/model"
)

type Store struct {
	db *sql.DB
}

func New(dbPath string) (*Store, error) {
	db, err := sql.Open("sqlite", dbPath+"?_pragma=journal_mode(WAL)&_pragma=busy_timeout(5000)&_pragma=foreign_keys(1)")
	if err != nil {
		return nil, fmt.Errorf("open db: %w", err)
	}
	s := &Store{db: db}
	if err := s.migrate(); err != nil {
		return nil, fmt.Errorf("migrate: %w", err)
	}
	return s, nil
}

func (s *Store) Close() error { return s.db.Close() }

func (s *Store) migrate() error {
	schema := `
	CREATE TABLE IF NOT EXISTS servers (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT NOT NULL,
		secret_key TEXT NOT NULL UNIQUE,
		note TEXT DEFAULT '',
		sort_index INTEGER DEFAULT 0,
		platform TEXT DEFAULT '',
		cpu_info TEXT DEFAULT '',
		version TEXT DEFAULT '',
		arch TEXT DEFAULT '',
		status INTEGER DEFAULT 0,
		last_active INTEGER DEFAULT 0,
		created_at INTEGER NOT NULL,
		updated_at INTEGER NOT NULL
	);

	CREATE TABLE IF NOT EXISTS metrics_history (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		server_id INTEGER NOT NULL,
		cpu REAL DEFAULT 0,
		mem_used INTEGER DEFAULT 0,
		swap_used INTEGER DEFAULT 0,
		disk_used INTEGER DEFAULT 0,
		net_in_speed INTEGER DEFAULT 0,
		net_out_speed INTEGER DEFAULT 0,
		load1 REAL DEFAULT 0,
		created_at INTEGER NOT NULL,
		FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
	);

	CREATE INDEX IF NOT EXISTS idx_metrics_server_time ON metrics_history(server_id, created_at);

	CREATE TABLE IF NOT EXISTS alert_rules (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT NOT NULL,
		server_ids TEXT DEFAULT '',
		metric_type TEXT NOT NULL,
		operator TEXT NOT NULL DEFAULT '>',
		threshold REAL NOT NULL,
		duration INTEGER DEFAULT 60,
		enabled INTEGER DEFAULT 1,
		created_at INTEGER NOT NULL,
		updated_at INTEGER NOT NULL
	);

	CREATE TABLE IF NOT EXISTS alert_events (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		rule_id INTEGER NOT NULL,
		rule_name TEXT NOT NULL,
		server_id INTEGER NOT NULL,
		server_name TEXT NOT NULL,
		metric_type TEXT NOT NULL,
		value REAL NOT NULL,
		threshold REAL NOT NULL,
		message TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'firing',
		created_at INTEGER NOT NULL,
		resolved_at INTEGER DEFAULT 0,
		FOREIGN KEY (rule_id) REFERENCES alert_rules(id) ON DELETE CASCADE,
		FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
	);
	`
	_, err := s.db.Exec(schema)
	return err
}

// ── Server CRUD ──

func (s *Store) CreateServer(name, secretKey string) (*model.Server, error) {
	now := time.Now().Unix()
	res, err := s.db.Exec(
		`INSERT INTO servers (name, secret_key, created_at, updated_at) VALUES (?, ?, ?, ?)`,
		name, secretKey, now, now,
	)
	if err != nil {
		return nil, err
	}
	id, _ := res.LastInsertId()
	return &model.Server{
		ID: id, Name: name, SecretKey: secretKey,
		CreatedAt: time.Unix(now, 0), UpdatedAt: time.Unix(now, 0),
	}, nil
}

func (s *Store) GetServer(id int64) (*model.Server, error) {
	row := s.db.QueryRow(`SELECT id, name, secret_key, note, sort_index, platform, cpu_info, version, arch, status, last_active, created_at, updated_at FROM servers WHERE id = ?`, id)
	return scanServer(row)
}

func (s *Store) GetServerByKey(key string) (*model.Server, error) {
	row := s.db.QueryRow(`SELECT id, name, secret_key, note, sort_index, platform, cpu_info, version, arch, status, last_active, created_at, updated_at FROM servers WHERE secret_key = ?`, key)
	return scanServer(row)
}

func (s *Store) ListServers() ([]model.Server, error) {
	rows, err := s.db.Query(`SELECT id, name, secret_key, note, sort_index, platform, cpu_info, version, arch, status, last_active, created_at, updated_at FROM servers ORDER BY sort_index ASC, id ASC`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var list []model.Server
	for rows.Next() {
		srv, err := scanServerRows(rows)
		if err != nil {
			return nil, err
		}
		list = append(list, *srv)
	}
	return list, rows.Err()
}

func (s *Store) UpdateServer(srv *model.Server) error {
	_, err := s.db.Exec(
		`UPDATE servers SET name=?, note=?, sort_index=?, updated_at=? WHERE id=?`,
		srv.Name, srv.Note, srv.SortIndex, time.Now().Unix(), srv.ID,
	)
	return err
}

func (s *Store) DeleteServer(id int64) error {
	_, err := s.db.Exec(`DELETE FROM servers WHERE id = ?`, id)
	return err
}

func (s *Store) UpdateServerStatus(id int64, status int) error {
	_, err := s.db.Exec(`UPDATE servers SET status=?, last_active=?, updated_at=? WHERE id=?`,
		status, time.Now().Unix(), time.Now().Unix(), id)
	return err
}

func (s *Store) UpdateServerHost(id int64, info model.HostInfo) error {
	_, err := s.db.Exec(`UPDATE servers SET platform=?, cpu_info=?, version=?, arch=?, updated_at=? WHERE id=?`,
		info.Platform, info.CPUInfo, info.Version, info.Arch, time.Now().Unix(), id)
	return err
}

// ── Metrics ──

func (s *Store) InsertMetrics(m *model.MetricsHistory) error {
	_, err := s.db.Exec(
		`INSERT INTO metrics_history (server_id, cpu, mem_used, swap_used, disk_used, net_in_speed, net_out_speed, load1, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		m.ServerID, m.CPU, m.MemUsed, m.SwapUsed, m.DiskUsed, m.NetInSpeed, m.NetOutSpeed, m.Load1, m.CreatedAt,
	)
	return err
}

func (s *Store) GetMetricsHistory(serverID int64, since int64) ([]model.MetricsHistory, error) {
	rows, err := s.db.Query(
		`SELECT id, server_id, cpu, mem_used, swap_used, disk_used, net_in_speed, net_out_speed, load1, created_at FROM metrics_history WHERE server_id = ? AND created_at >= ? ORDER BY created_at ASC`,
		serverID, since,
	)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var list []model.MetricsHistory
	for rows.Next() {
		var m model.MetricsHistory
		if err := rows.Scan(&m.ID, &m.ServerID, &m.CPU, &m.MemUsed, &m.SwapUsed, &m.DiskUsed, &m.NetInSpeed, &m.NetOutSpeed, &m.Load1, &m.CreatedAt); err != nil {
			return nil, err
		}
		list = append(list, m)
	}
	return list, rows.Err()
}

func (s *Store) CleanupOldMetrics(before int64) (int64, error) {
	res, err := s.db.Exec(`DELETE FROM metrics_history WHERE created_at < ?`, before)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

// ── Alert Rules ──

func (s *Store) CreateAlertRule(r *model.AlertRule) (int64, error) {
	now := time.Now().Unix()
	res, err := s.db.Exec(
		`INSERT INTO alert_rules (name, server_ids, metric_type, operator, threshold, duration, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		r.Name, r.ServerIDs, r.MetricType, r.Operator, r.Threshold, r.Duration, boolToInt(r.Enabled), now, now,
	)
	if err != nil {
		return 0, err
	}
	return res.LastInsertId()
}

func (s *Store) ListAlertRules() ([]model.AlertRule, error) {
	rows, err := s.db.Query(`SELECT id, name, server_ids, metric_type, operator, threshold, duration, enabled, created_at, updated_at FROM alert_rules ORDER BY id DESC`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var list []model.AlertRule
	for rows.Next() {
		var r model.AlertRule
		var enabled int
		if err := rows.Scan(&r.ID, &r.Name, &r.ServerIDs, &r.MetricType, &r.Operator, &r.Threshold, &r.Duration, &enabled, &r.CreatedAt, &r.UpdatedAt); err != nil {
			return nil, err
		}
		r.Enabled = enabled != 0
		list = append(list, r)
	}
	return list, rows.Err()
}

func (s *Store) GetAlertRule(id int64) (*model.AlertRule, error) {
	var r model.AlertRule
	var enabled int
	err := s.db.QueryRow(
		`SELECT id, name, server_ids, metric_type, operator, threshold, duration, enabled, created_at, updated_at FROM alert_rules WHERE id = ?`, id,
	).Scan(&r.ID, &r.Name, &r.ServerIDs, &r.MetricType, &r.Operator, &r.Threshold, &r.Duration, &enabled, &r.CreatedAt, &r.UpdatedAt)
	if err != nil {
		return nil, err
	}
	r.Enabled = enabled != 0
	return &r, nil
}

func (s *Store) UpdateAlertRule(r *model.AlertRule) error {
	_, err := s.db.Exec(
		`UPDATE alert_rules SET name=?, server_ids=?, metric_type=?, operator=?, threshold=?, duration=?, enabled=?, updated_at=? WHERE id=?`,
		r.Name, r.ServerIDs, r.MetricType, r.Operator, r.Threshold, r.Duration, boolToInt(r.Enabled), time.Now().Unix(), r.ID,
	)
	return err
}

func (s *Store) DeleteAlertRule(id int64) error {
	_, err := s.db.Exec(`DELETE FROM alert_rules WHERE id = ?`, id)
	return err
}

// ── Alert Events ──

func (s *Store) CreateAlertEvent(e *model.AlertEvent) (int64, error) {
	res, err := s.db.Exec(
		`INSERT INTO alert_events (rule_id, rule_name, server_id, server_name, metric_type, value, threshold, message, status, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		e.RuleID, e.RuleName, e.ServerID, e.ServerName, e.MetricType, e.Value, e.Threshold, e.Message, e.Status, time.Now().Unix(),
	)
	if err != nil {
		return 0, err
	}
	return res.LastInsertId()
}

func (s *Store) ListAlertEvents(limit int) ([]model.AlertEvent, error) {
	rows, err := s.db.Query(`SELECT id, rule_id, rule_name, server_id, server_name, metric_type, value, threshold, message, status, created_at, resolved_at FROM alert_events ORDER BY id DESC LIMIT ?`, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var list []model.AlertEvent
	for rows.Next() {
		var e model.AlertEvent
		if err := rows.Scan(&e.ID, &e.RuleID, &e.RuleName, &e.ServerID, &e.ServerName, &e.MetricType, &e.Value, &e.Threshold, &e.Message, &e.Status, &e.CreatedAt, &e.ResolvedAt); err != nil {
			return nil, err
		}
		list = append(list, e)
	}
	return list, rows.Err()
}

func (s *Store) GetFiringAlert(ruleID, serverID int64) (*model.AlertEvent, error) {
	var e model.AlertEvent
	err := s.db.QueryRow(
		`SELECT id, rule_id, rule_name, server_id, server_name, metric_type, value, threshold, message, status, created_at, resolved_at FROM alert_events WHERE rule_id = ? AND server_id = ? AND status = 'firing' ORDER BY id DESC LIMIT 1`,
		ruleID, serverID,
	).Scan(&e.ID, &e.RuleID, &e.RuleName, &e.ServerID, &e.ServerName, &e.MetricType, &e.Value, &e.Threshold, &e.Message, &e.Status, &e.CreatedAt, &e.ResolvedAt)
	if err != nil {
		return nil, err
	}
	return &e, nil
}

func (s *Store) ResolveAlert(id int64) error {
	_, err := s.db.Exec(`UPDATE alert_events SET status='resolved', resolved_at=? WHERE id=?`, time.Now().Unix(), id)
	return err
}

func (s *Store) CountFiringAlerts() (int, error) {
	var count int
	err := s.db.QueryRow(`SELECT COUNT(*) FROM alert_events WHERE status = 'firing'`).Scan(&count)
	return count, err
}

func (s *Store) CountAlertRules() (int, error) {
	var count int
	err := s.db.QueryRow(`SELECT COUNT(*) FROM alert_rules`).Scan(&count)
	return count, err
}

// ── helpers ──

type scanner interface {
	Scan(dest ...any) error
}

func scanServer(row scanner) (*model.Server, error) {
	var srv model.Server
	var lastActive, createdAt, updatedAt int64
	err := row.Scan(&srv.ID, &srv.Name, &srv.SecretKey, &srv.Note, &srv.SortIndex,
		&srv.Platform, &srv.CPUInfo, &srv.Version, &srv.Arch, &srv.Status,
		&lastActive, &createdAt, &updatedAt)
	if err != nil {
		return nil, err
	}
	srv.LastActive = time.Unix(lastActive, 0)
	srv.CreatedAt = time.Unix(createdAt, 0)
	srv.UpdatedAt = time.Unix(updatedAt, 0)
	return &srv, nil
}

func scanServerRows(rows *sql.Rows) (*model.Server, error) {
	return scanServer(rows)
}

func boolToInt(b bool) int {
	if b {
		return 1
	}
	return 0
}
