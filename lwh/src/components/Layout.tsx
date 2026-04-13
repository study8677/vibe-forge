import { useState } from 'react'
import { useStore } from '../store'
import { Sidebar } from './sidebar/Sidebar'
import { LogViewer } from './log-viewer/LogViewer'
import { AIChat } from './ai-chat/AIChat'
import { ServerForm } from './sidebar/ServerForm'
import { Terminal, PanelLeftClose, PanelLeft } from 'lucide-react'

export function Layout() {
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false)
  const aiChatOpen = useStore(s => s.aiChatOpen)
  const showServerForm = useStore(s => s.showServerForm)

  return (
    <div className="h-screen flex flex-col bg-slate-900 text-slate-100 overflow-hidden">
      {/* Header */}
      <header className="h-12 flex items-center justify-between px-4 bg-slate-800 border-b border-slate-700 shrink-0">
        <div className="flex items-center gap-3">
          <button
            onClick={() => setSidebarCollapsed(!sidebarCollapsed)}
            className="p-1.5 hover:bg-slate-700 rounded transition-colors"
          >
            {sidebarCollapsed ? <PanelLeft size={18} /> : <PanelLeftClose size={18} />}
          </button>
          <Terminal size={20} className="text-blue-400" />
          <h1 className="text-sm font-semibold tracking-wide">LWH</h1>
          <span className="text-xs text-slate-500">Log Watcher Hub</span>
        </div>
      </header>

      {/* Main */}
      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar */}
        <div
          className={`${
            sidebarCollapsed ? 'w-0' : 'w-72'
          } transition-all duration-200 overflow-hidden border-r border-slate-700 bg-slate-800`}
        >
          <Sidebar />
        </div>

        {/* Right Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <div className={`${aiChatOpen ? 'flex-1 min-h-0' : 'flex-1'} flex flex-col overflow-hidden`}>
            <LogViewer />
          </div>
          {aiChatOpen && (
            <div className="h-80 border-t border-slate-700 flex flex-col shrink-0">
              <AIChat />
            </div>
          )}
        </div>
      </div>

      {showServerForm && <ServerForm />}
    </div>
  )
}
