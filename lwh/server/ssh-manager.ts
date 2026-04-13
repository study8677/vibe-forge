import { Client, type ConnectConfig } from 'ssh2'
import { readFileSync } from 'fs'
import type { ServerConfig, ServiceConfig } from './types.js'

export class SSHManager {
  private connections = new Map<string, Client>()

  async connect(server: ServerConfig): Promise<Client> {
    if (this.connections.has(server.id)) {
      return this.connections.get(server.id)!
    }

    return new Promise((resolve, reject) => {
      const client = new Client()

      const config: ConnectConfig = {
        host: server.host,
        port: server.port || 22,
        username: server.username,
        readyTimeout: 10000,
      }

      if (server.authType === 'password') {
        config.password = server.password
      } else if (server.authType === 'key') {
        let key = server.privateKey || ''
        // Support file path: read the key file if it looks like a path
        if (key && !key.includes('-----BEGIN') && (key.startsWith('/') || key.startsWith('~'))) {
          const resolved = key.replace(/^~/, process.env.HOME || '/root')
          key = readFileSync(resolved, 'utf-8')
        }
        config.privateKey = key
        if (server.passphrase) config.passphrase = server.passphrase
      }

      client
        .on('ready', () => {
          this.connections.set(server.id, client)
          resolve(client)
        })
        .on('error', (err) => {
          this.connections.delete(server.id)
          reject(err)
        })
        .on('close', () => {
          this.connections.delete(server.id)
        })
        .connect(config)
    })
  }

  async testConnection(server: ServerConfig): Promise<boolean> {
    const client = await this.connect(server)
    return new Promise((resolve, reject) => {
      client.exec('echo ok', (err, stream) => {
        if (err) return reject(err)
        let output = ''
        stream
          .on('data', (data: Buffer) => { output += data.toString() })
          .on('close', () => resolve(output.trim() === 'ok'))
      })
    })
  }

  getConnection(serverId: string): Client | undefined {
    return this.connections.get(serverId)
  }

  isConnected(serverId: string): boolean {
    return this.connections.has(serverId)
  }

  disconnect(serverId: string) {
    const client = this.connections.get(serverId)
    if (client) {
      client.end()
      this.connections.delete(serverId)
    }
  }

  async exec(serverId: string, command: string): Promise<string> {
    const client = this.connections.get(serverId)
    if (!client) throw new Error('Not connected')

    return new Promise((resolve, reject) => {
      client.exec(command, (err, stream) => {
        if (err) return reject(err)
        let output = ''
        let stderr = ''
        stream
          .on('data', (data: Buffer) => { output += data.toString() })
          .stderr.on('data', (data: Buffer) => { stderr += data.toString() })
        stream.on('close', () => resolve(output || stderr))
      })
    })
  }

  async detectServices(serverId: string): Promise<ServiceConfig[]> {
    const services: ServiceConfig[] = []

    // Detect systemd services
    try {
      const out = await this.exec(serverId,
        'systemctl list-units --type=service --state=running --no-pager --no-legend 2>/dev/null | head -20')
      for (const line of out.split('\n').filter(Boolean)) {
        const match = line.match(/^●?\s*(\S+)\.service/)
        if (match) {
          services.push({ name: match[1], logPath: match[1], type: 'journalctl' })
        }
      }
    } catch {}

    // Detect docker containers
    try {
      const out = await this.exec(serverId,
        'docker ps --format "{{.ID}}|{{.Names}}" 2>/dev/null')
      for (const line of out.split('\n').filter(Boolean)) {
        const [id, name] = line.split('|')
        services.push({ name: `docker:${name}`, logPath: name, type: 'docker', containerId: id })
      }
    } catch {}

    // Detect PM2 processes
    try {
      const out = await this.exec(serverId, 'pm2 jlist 2>/dev/null')
      if (out.trim().startsWith('[')) {
        const procs = JSON.parse(out)
        for (const proc of procs) {
          services.push({ name: `pm2:${proc.name}`, logPath: proc.name, type: 'pm2' })
        }
      }
    } catch {}

    // Detect common log files
    try {
      const out = await this.exec(serverId,
        'ls -1 /var/log/*.log /var/log/syslog /var/log/messages 2>/dev/null | head -10')
      for (const path of out.split('\n').filter(Boolean)) {
        const name = path.split('/').pop()!.replace('.log', '')
        services.push({ name: `file:${name}`, logPath: path, type: 'file' })
      }
    } catch {}

    return services
  }

  disconnectAll() {
    for (const [, client] of this.connections) {
      client.end()
    }
    this.connections.clear()
  }
}
