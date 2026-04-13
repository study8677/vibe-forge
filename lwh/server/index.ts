import express from 'express'
import { createServer } from 'http'
import { Server } from 'socket.io'
import cors from 'cors'
import dotenv from 'dotenv'
import { mkdirSync, readFileSync, writeFileSync, existsSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'
import { SSHManager } from './ssh-manager.js'
import { LogStreamer } from './log-streamer.js'
import { AIService } from './ai-service.js'
import type { ServerConfig } from './types.js'

dotenv.config()

const __dirname = dirname(fileURLToPath(import.meta.url))
const DATA_DIR = join(__dirname, '..', 'data')
const SERVERS_FILE = join(DATA_DIR, 'servers.json')

if (!existsSync(DATA_DIR)) {
  mkdirSync(DATA_DIR, { recursive: true })
}

function loadServers(): ServerConfig[] {
  if (!existsSync(SERVERS_FILE)) return []
  try {
    return JSON.parse(readFileSync(SERVERS_FILE, 'utf-8'))
  } catch {
    return []
  }
}

function saveServers(servers: ServerConfig[]) {
  writeFileSync(SERVERS_FILE, JSON.stringify(servers, null, 2))
}

const app = express()
const httpServer = createServer(app)
const io = new Server(httpServer, { cors: { origin: '*' } })

app.use(cors())
app.use(express.json())

// Serve static files in production
if (process.env.NODE_ENV === 'production') {
  app.use(express.static(join(__dirname, '..', 'dist')))
}

const sshManager = new SSHManager()
const logStreamer = new LogStreamer(sshManager, io)
const aiService = new AIService()

// ── REST API: Server Management ──────────────────────────

app.get('/api/servers', (_req, res) => {
  res.json(loadServers())
})

app.post('/api/servers', (req, res) => {
  const servers = loadServers()
  const server: ServerConfig = {
    id: crypto.randomUUID(),
    ...req.body,
    createdAt: new Date().toISOString(),
  }
  servers.push(server)
  saveServers(servers)
  res.json(server)
})

app.put('/api/servers/:id', (req, res) => {
  const servers = loadServers()
  const idx = servers.findIndex(s => s.id === req.params.id)
  if (idx === -1) { res.status(404).json({ error: 'Not found' }); return }
  servers[idx] = { ...servers[idx], ...req.body }
  saveServers(servers)
  res.json(servers[idx])
})

app.delete('/api/servers/:id', (req, res) => {
  let servers = loadServers()
  servers = servers.filter(s => s.id !== req.params.id)
  saveServers(servers)
  sshManager.disconnect(req.params.id)
  res.json({ ok: true })
})

// ── REST API: Connection ─────────────────────────────────

app.post('/api/servers/:id/test', async (req, res) => {
  try {
    const servers = loadServers()
    const server = servers.find(s => s.id === req.params.id)
    if (!server) { res.status(404).json({ error: 'Not found' }); return }
    await sshManager.testConnection(server)
    res.json({ ok: true })
  } catch (err: any) {
    res.status(400).json({ error: err.message })
  }
})

app.post('/api/servers/:id/connect', async (req, res) => {
  try {
    const servers = loadServers()
    const server = servers.find(s => s.id === req.params.id)
    if (!server) { res.status(404).json({ error: 'Not found' }); return }
    await sshManager.connect(server)
    res.json({ ok: true, connected: true })
  } catch (err: any) {
    res.status(400).json({ error: err.message })
  }
})

app.post('/api/servers/:id/disconnect', (req, res) => {
  sshManager.disconnect(req.params.id)
  res.json({ ok: true })
})

// ── REST API: Service Detection ──────────────────────────

app.get('/api/servers/:id/services', async (req, res) => {
  try {
    const services = await sshManager.detectServices(req.params.id)
    res.json(services)
  } catch (err: any) {
    res.status(400).json({ error: err.message })
  }
})

// ── REST API: AI Analysis (SSE streaming) ────────────────

app.post('/api/ai/analyze', async (req, res) => {
  try {
    const { provider, logText, question, history } = req.body

    res.setHeader('Content-Type', 'text/event-stream')
    res.setHeader('Cache-Control', 'no-cache')
    res.setHeader('Connection', 'keep-alive')

    const stream = aiService.analyze(provider, logText, question, history)

    for await (const chunk of stream) {
      res.write(`data: ${JSON.stringify({ text: chunk })}\n\n`)
    }
    res.write('data: [DONE]\n\n')
    res.end()
  } catch (err: any) {
    if (!res.headersSent) {
      res.status(400).json({ error: err.message })
    } else {
      res.write(`data: ${JSON.stringify({ error: err.message })}\n\n`)
      res.end()
    }
  }
})

// ── Socket.IO: Real-time Log Streaming ───────────────────

io.on('connection', (socket) => {
  console.log(`[WS] Client connected: ${socket.id}`)

  socket.on('subscribe-logs', async (data: {
    serverId: string
    service: string
    logPath: string
    type?: string
  }) => {
    try {
      await logStreamer.subscribe(socket, data.serverId, data.service, data.logPath, data.type)
      console.log(`[WS] Subscribed ${socket.id} -> ${data.serverId}/${data.service}`)
    } catch (err: any) {
      socket.emit('log-error', { error: err.message })
    }
  })

  socket.on('unsubscribe-logs', (data: { serverId: string }) => {
    logStreamer.unsubscribe(socket, data.serverId)
  })

  socket.on('disconnect', () => {
    logStreamer.cleanupSocket(socket)
    console.log(`[WS] Client disconnected: ${socket.id}`)
  })
})

// ── Start ────────────────────────────────────────────────

const PORT = parseInt(process.env.PORT || '3210')
httpServer.listen(PORT, () => {
  console.log(`\n  LWH - Log Watcher Hub`)
  console.log(`  Server running on http://localhost:${PORT}\n`)
})
