import { useState } from 'react'
import Header from './components/Header'
import Sidebar from './components/Sidebar'
import TabNav from './components/TabNav'
import TopicList from './components/TopicList'
import CategoryPage from './components/CategoryPage'
import RSSPanel from './components/RSSPanel'
import WebSSH from './components/WebSSH'
import ThemeModal from './components/ThemeModal'
import { useTheme } from './hooks/useTheme'
import { topics } from './data/topics'

export default function App() {
  const [sidebarOpen, setSidebarOpen] = useState(true)
  const [activeTab, setActiveTab] = useState('latest')
  const [activeView, setActiveView] = useState('forum') // forum | rss | ssh
  const [themeOpen, setThemeOpen] = useState(false)
  const { current: theme, setCurrent: setTheme, themes } = useTheme()

  const handleTabChange = (tab) => {
    setActiveTab(tab)
    setActiveView('forum')
  }

  const handleViewChange = (view) => {
    setActiveView(activeView === view ? 'forum' : view)
  }

  const sortedTopics = (() => {
    const pinned = topics.filter((t) => t.pinned)
    const rest = topics.filter((t) => !t.pinned)
    switch (activeTab) {
      case 'new': return [...pinned, ...rest.filter((t) => t.isNew)]
      case 'hot': return [...pinned, ...rest.filter((t) => t.isHot || t.replies > 50)]
      case 'top': return [...pinned, ...[...rest].sort((a, b) => b.views - a.views)]
      default: return [...pinned, ...rest]
    }
  })()

  return (
    <div className="h-screen flex flex-col overflow-hidden">
      <Header
        sidebarOpen={sidebarOpen}
        setSidebarOpen={setSidebarOpen}
        onThemeClick={() => setThemeOpen(true)}
      />

      <div className="flex flex-1 overflow-hidden">
        <Sidebar open={sidebarOpen} activeView={activeView} onViewChange={handleViewChange} />

        <main className="flex-1 flex flex-col overflow-hidden">
          {activeView === 'forum' && (
            <>
              <TabNav active={activeTab} onChange={handleTabChange} />
              {activeTab === 'categories' ? (
                <CategoryPage />
              ) : (
                <TopicList topics={sortedTopics} />
              )}
            </>
          )}
          {activeView === 'rss' && <RSSPanel />}
          {activeView === 'ssh' && <WebSSH />}
        </main>
      </div>

      {themeOpen && (
        <ThemeModal
          current={theme}
          themes={themes}
          onChange={setTheme}
          onClose={() => setThemeOpen(false)}
        />
      )}
    </div>
  )
}
