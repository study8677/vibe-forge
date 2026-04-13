import type { Server, Socket } from 'socket.io'
import type { ClientChannel } from 'ssh2'
import { SSHManager } from './ssh-manager.js'

interface StreamInfo {
  serverId: string
  service: string
  stream: ClientChannel
}

export class LogStreamer {
  private activeStreams = new Map<string, StreamInfo[]>()

  constructor(
    private sshManager: SSHManager,
    private io: Server
  ) {}

  private getStreamCommand(type: string, logPath: string): string {
    switch (type) {
      case 'journalctl':
        return `journalctl -u ${logPath} -f --no-pager -n 100 2>&1`
      case 'docker':
        return `docker logs -f --tail 100 ${logPath} 2>&1`
      case 'pm2':
        return `pm2 logs ${logPath} --lines 100 --raw 2>&1`
      case 'file':
      default:
        return `tail -f -n 100 ${logPath} 2>&1`
    }
  }

  private parseLogLevel(line: string): string {
    const lower = line.toLowerCase()
    if (/\b(error|fatal|critical|panic|exception)\b/.test(lower)) return 'error'
    if (/\b(warn|warning)\b/.test(lower)) return 'warn'
    if (/\b(info|notice)\b/.test(lower)) return 'info'
    if (/\b(debug|trace|verbose)\b/.test(lower)) return 'debug'
    return 'unknown'
  }

  async subscribe(
    socket: Socket,
    serverId: string,
    service: string,
    logPath: string,
    type: string = 'file'
  ) {
    const client = this.sshManager.getConnection(serverId)
    if (!client) throw new Error('Server not connected')

    // Clean up existing streams for this server from this socket
    this.unsubscribeService(socket, serverId, service)

    const command = this.getStreamCommand(type, logPath)

    return new Promise<void>((resolve, reject) => {
      client.exec(command, (err, stream) => {
        if (err) return reject(err)

        const streams = this.activeStreams.get(socket.id) || []
        streams.push({ serverId, service, stream })
        this.activeStreams.set(socket.id, streams)

        let buffer = ''

        stream.on('data', (data: Buffer) => {
          buffer += data.toString()
          const lines = buffer.split('\n')
          buffer = lines.pop() || ''

          for (const line of lines) {
            if (!line.trim()) continue
            socket.emit('log-line', {
              id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
              serverId,
              service,
              raw: line,
              level: this.parseLogLevel(line),
              timestamp: new Date().toISOString(),
              message: line,
            })
          }
        })

        stream.stderr.on('data', (data: Buffer) => {
          const text = data.toString().trim()
          if (!text) return
          socket.emit('log-line', {
            id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
            serverId,
            service,
            raw: text,
            level: 'error',
            timestamp: new Date().toISOString(),
            message: text,
          })
        })

        stream.on('close', () => {
          socket.emit('log-stream-ended', { serverId, service })
        })

        resolve()
      })
    })
  }

  private unsubscribeService(socket: Socket, serverId: string, service: string) {
    const streams = this.activeStreams.get(socket.id)
    if (!streams) return

    const remaining = streams.filter(s => {
      if (s.serverId === serverId && s.service === service) {
        try { s.stream.close() } catch {}
        return false
      }
      return true
    })

    if (remaining.length === 0) {
      this.activeStreams.delete(socket.id)
    } else {
      this.activeStreams.set(socket.id, remaining)
    }
  }

  unsubscribe(socket: Socket, serverId: string) {
    const streams = this.activeStreams.get(socket.id)
    if (!streams) return

    const remaining = streams.filter(s => {
      if (s.serverId === serverId) {
        try { s.stream.close() } catch {}
        return false
      }
      return true
    })

    if (remaining.length === 0) {
      this.activeStreams.delete(socket.id)
    } else {
      this.activeStreams.set(socket.id, remaining)
    }
  }

  cleanupSocket(socket: Socket) {
    const streams = this.activeStreams.get(socket.id)
    if (!streams) return

    for (const s of streams) {
      try { s.stream.close() } catch {}
    }
    this.activeStreams.delete(socket.id)
  }
}
