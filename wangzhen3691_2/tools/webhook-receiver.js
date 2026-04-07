#!/usr/bin/env node
// Node.js webhook receiver — forwards incoming payloads to ShiHuang Guard.
// Usage:  node tools/webhook-receiver.js
// Env:    GUARD_API (default http://localhost:8080)
//         WEBHOOK_PORT (default 3000)

const http = require('http')

const API  = process.env.GUARD_API    || 'http://localhost:8080'
const PORT = process.env.WEBHOOK_PORT || 3000

const server = http.createServer((req, res) => {
  // health
  if (req.method === 'GET' && req.url === '/health') {
    res.writeHead(200, { 'Content-Type': 'application/json' })
    return res.end(JSON.stringify({ status: 'ok', forwarding_to: API }))
  }

  // webhook receiver
  if (req.method === 'POST' && req.url === '/webhook') {
    let body = ''
    req.on('data', (c) => (body += c))
    req.on('end', async () => {
      try {
        const payload = JSON.parse(body)
        const text = payload.content || payload.text || payload.message || ''
        if (!text) {
          res.writeHead(400, { 'Content-Type': 'application/json' })
          return res.end(JSON.stringify({ error: 'no text found in payload' }))
        }

        const scanRes = await fetch(`${API}/api/scan`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ text, source: payload.source || 'webhook' }),
        })
        const result = await scanRes.json()
        res.writeHead(200, { 'Content-Type': 'application/json' })
        res.end(JSON.stringify(result))
      } catch (err) {
        res.writeHead(502, { 'Content-Type': 'application/json' })
        res.end(JSON.stringify({ error: err.message }))
      }
    })
    return
  }

  res.writeHead(404)
  res.end('Not Found\n')
})

server.listen(PORT, () => {
  console.log(`[webhook-receiver] listening on :${PORT}`)
  console.log(`[webhook-receiver] forwarding to ${API}`)
  console.log(`[webhook-receiver] POST /webhook  { "text": "…", "source": "…" }`)
})
