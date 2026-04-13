import { useRef, useEffect, useMemo, useCallback } from 'react'
import { useStore } from '../../store'
import { LogLine } from './LogLine'
import { LogToolbar } from './LogToolbar'
import { ServiceTabs } from './ServiceTabs'
import { FileText } from 'lucide-react'

export function LogViewer() {
  const logs = useStore(s => s.logs)
  const logFilter = useStore(s => s.logFilter)
  const logLevelFilter = useStore(s => s.logLevelFilter)
  const autoScroll = useStore(s => s.autoScroll)
  const activeServerId = useStore(s => s.activeServerId)
  const activeService = useStore(s => s.activeService)
  const setSelectedLogText = useStore(s => s.setSelectedLogText)
  const setAIChatOpen = useStore(s => s.setAIChatOpen)

  const containerRef = useRef<HTMLDivElement>(null)

  const filteredLogs = useMemo(() => {
    return logs.filter(log => {
      if (!logLevelFilter.has(log.level)) return false
      if (logFilter && !log.raw.toLowerCase().includes(logFilter.toLowerCase())) return false
      return true
    })
  }, [logs, logFilter, logLevelFilter])

  useEffect(() => {
    if (autoScroll && containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight
    }
  }, [filteredLogs.length, autoScroll])

  const handleMouseUp = useCallback(() => {
    const selection = window.getSelection()
    const text = selection?.toString().trim()
    if (text && text.length > 10) {
      setSelectedLogText(text)
    }
  }, [setSelectedLogText])

  const handleAnalyze = useCallback(() => {
    const selection = window.getSelection()
    const text = selection?.toString().trim()
    if (text) {
      setSelectedLogText(text)
      setAIChatOpen(true)
    }
  }, [setSelectedLogText, setAIChatOpen])

  if (!activeServerId || !activeService) {
    return (
      <div className="flex-1 flex items-center justify-center text-slate-500">
        <div className="text-center">
          <FileText size={48} className="mx-auto mb-3 opacity-30" />
          <p className="text-sm">Select a server and service to view logs</p>
          <p className="text-xs text-slate-600 mt-1">Connect to a server, then click a service name</p>
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <ServiceTabs />
      <LogToolbar />
      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto bg-slate-950 font-mono text-xs"
        onMouseUp={handleMouseUp}
        onDoubleClick={handleAnalyze}
      >
        {filteredLogs.length === 0 ? (
          <div className="flex items-center justify-center h-full text-slate-600">
            <p>Waiting for log data...</p>
          </div>
        ) : (
          filteredLogs.map(log => <LogLine key={log.id} entry={log} />)
        )}
      </div>
    </div>
  )
}
