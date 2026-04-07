import { useState, useCallback } from 'react'
import { useWebSocket } from './hooks/useWebSocket'
import Layout from './components/Layout'
import Dashboard from './components/Dashboard'
import AlertFeed from './components/AlertFeed'
import RuleManager from './components/RuleManager'
import PluginPanel from './components/PluginPanel'
import Scanner from './components/Scanner'

const pages = { dashboard: Dashboard, alerts: AlertFeed, rules: RuleManager, plugins: PluginPanel, scanner: Scanner }

export default function App() {
  const [page, setPage] = useState('dashboard')
  const [alerts, setAlerts] = useState([])
  const [stats, setStats] = useState(null)

  const onMsg = useCallback((msg) => {
    if (msg.type === 'alert')
      setAlerts((prev) => [msg.payload, ...prev].slice(0, 500))
    if (msg.type === 'stats')
      setStats(msg.payload)
  }, [])

  const { connected } = useWebSocket(onMsg)
  const Page = pages[page]

  return (
    <Layout page={page} setPage={setPage} connected={connected}>
      <Page alerts={alerts} stats={stats} />
    </Layout>
  )
}
