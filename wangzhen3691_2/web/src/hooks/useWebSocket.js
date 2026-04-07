import { useEffect, useRef, useCallback, useState } from 'react'

export function useWebSocket(onMessage) {
  const ws = useRef(null)
  const timer = useRef(null)
  const [connected, setConnected] = useState(false)

  const connect = useCallback(() => {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:'
    const sock = new WebSocket(`${proto}//${location.host}/ws`)
    ws.current = sock

    sock.onopen = () => {
      setConnected(true)
      console.log('[WS] connected')
    }

    sock.onmessage = (e) => {
      try { onMessage(JSON.parse(e.data)) } catch { /* ignore */ }
    }

    sock.onclose = () => {
      setConnected(false)
      timer.current = setTimeout(connect, 3000)
    }

    sock.onerror = () => sock.close()
  }, [onMessage])

  useEffect(() => {
    connect()
    return () => {
      clearTimeout(timer.current)
      ws.current?.close()
    }
  }, [connect])

  return { connected }
}
