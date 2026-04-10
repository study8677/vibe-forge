import { Routes, Route } from 'react-router-dom'
import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import Servers from './pages/Servers'
import ServerDetail from './pages/ServerDetail'
import AlertRules from './pages/AlertRules'
import AlertEvents from './pages/AlertEvents'

export default function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/servers" element={<Servers />} />
        <Route path="/server/:id" element={<ServerDetail />} />
        <Route path="/alerts/rules" element={<AlertRules />} />
        <Route path="/alerts/events" element={<AlertEvents />} />
      </Routes>
    </Layout>
  )
}
