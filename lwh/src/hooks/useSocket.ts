import { useEffect, useRef, useCallback } from 'react'
import { io, Socket } from 'socket.io-client'
import { useStore } from '../store'
import type { LogEntry, ServiceConfig } from '../types'

let socket: Socket | null = null

function getSocket(): Socket {
  if (!socket) {
    socket = io(window.location.origin, {
      transports: ['websocket', 'polling'],
    })
  }
  return socket
}

export function useSocket() {
  const addLog = useStore(s => s.addLog)
  const socketRef = useRef<Socket | null>(null)

  useEffect(() => {
    const s = getSocket()
    socketRef.current = s

    s.on('log-line', (entry: LogEntry) => {
      addLog(entry)
    })

    s.on('log-error', ({ error }: { error: string }) => {
      console.error('Log stream error:', error)
    })

    s.on('log-stream-ended', ({ serverId, service }: { serverId: string; service: string }) => {
      console.log(`Log stream ended: ${serverId}/${service}`)
    })

    return () => {
      s.off('log-line')
      s.off('log-error')
      s.off('log-stream-ended')
    }
  }, [addLog])

  const subscribe = useCallback((serverId: string, service: ServiceConfig) => {
    const s = getSocket()
    s.emit('subscribe-logs', {
      serverId,
      service: service.name,
      logPath: service.logPath,
      type: service.type,
    })
  }, [])

  const unsubscribe = useCallback((serverId: string) => {
    const s = getSocket()
    s.emit('unsubscribe-logs', { serverId })
  }, [])

  return { subscribe, unsubscribe }
}
