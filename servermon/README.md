# ServerMon

Lightweight server monitoring platform with real-time dashboard, alerting, and agent-based metric collection. Built with Go + React.

## Architecture

```
┌─────────────┐     HTTP POST      ┌──────────────────┐     WebSocket     ┌──────────────┐
│   Agent      │ ──────────────────>│   Go Server      │ ────────────────> │   React UI   │
│  (per host)  │  /api/agent/report │  (API + WS Hub)  │   real-time push  │  (Dashboard)  │
│  gopsutil    │                    │  SQLite + Alert   │                   │  Recharts     │
└─────────────┘                    └──────────────────┘                   └──────────────┘
```

## Features

- **Dashboard**: Server grid with CPU/Memory/Disk bars, network speed, uptime
- **Real-time**: WebSocket push from server to browser on every agent report
- **Server Detail**: Historical charts (CPU, Memory, Network, Load) with 1h/6h/24h/7d range
- **Alert Rules**: Configurable thresholds for CPU, Memory, Disk, Load, Offline detection
- **Alert Events**: Firing/resolved status with auto-resolve when conditions clear
- **Agent**: Lightweight Go binary collecting CPU, memory, swap, disk, network, load, processes

## Quick Start

### 1. Build

```bash
make build
```

Or step by step:

```bash
# Backend
go build -o bin/server ./cmd/server
go build -o bin/agent ./cmd/agent

# Frontend
cd web && npm install && npm run build
```

### 2. Run Server

```bash
./bin/server -addr :8080
```

Open http://localhost:8080 in browser.

### 3. Add a Server

Go to **Servers** page, click **Add Server**. You'll get a secret key and an agent command.

### 4. Run Agent

On the monitored host:

```bash
./agent -s http://YOUR_SERVER:8080 -k SECRET_KEY -i 5
```

Flags:
- `-s` Server URL (default: http://localhost:8080)
- `-k` Secret key (required)
- `-i` Report interval in seconds (default: 5)

### 5. Cross-compile Agent for Linux

```bash
make agent-linux
# Produces: bin/agent-linux-amd64, bin/agent-linux-arm64
```

## Development

Run backend and frontend dev server simultaneously:

```bash
# Terminal 1: Go server
./bin/server -addr :8080

# Terminal 2: React dev server (with HMR + proxy to :8080)
cd web && npm run dev
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/dashboard/stats | Summary statistics |
| GET | /api/servers | List servers with metrics |
| POST | /api/servers | Create server |
| GET | /api/servers/:id | Get server detail |
| PUT | /api/servers/:id | Update server |
| DELETE | /api/servers/:id | Delete server |
| GET | /api/servers/:id/metrics | Get metrics history |
| POST | /api/agent/report | Agent metric report |
| GET | /api/alerts/rules | List alert rules |
| POST | /api/alerts/rules | Create alert rule |
| PUT | /api/alerts/rules/:id | Update alert rule |
| DELETE | /api/alerts/rules/:id | Delete alert rule |
| GET | /api/alerts/events | List alert events |
| POST | /api/alerts/events/:id/resolve | Resolve alert |
| GET | /ws | WebSocket connection |

## Tech Stack

- **Backend**: Go 1.22+, net/http, gorilla/websocket, modernc.org/sqlite
- **Frontend**: React 18, TypeScript, Vite, TailwindCSS, Recharts, Lucide Icons
- **Agent**: Go, gopsutil/v3
- **Database**: SQLite (WAL mode, zero config)
