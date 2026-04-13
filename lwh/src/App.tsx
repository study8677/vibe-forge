import { useEffect } from 'react'
import { useStore } from './store'
import { Layout } from './components/Layout'

export default function App() {
  const setServers = useStore(s => s.setServers)

  useEffect(() => {
    fetch('/api/servers')
      .then(r => r.json())
      .then(setServers)
      .catch(console.error)
  }, [setServers])

  return <Layout />
}
