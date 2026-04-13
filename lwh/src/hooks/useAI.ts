import { useCallback } from 'react'
import { useStore } from '../store'

export function useAI() {
  const aiProvider = useStore(s => s.aiProvider)
  const chatMessages = useStore(s => s.chatMessages)
  const addChatMessage = useStore(s => s.addChatMessage)
  const updateLastChatMessage = useStore(s => s.updateLastChatMessage)
  const selectedLogText = useStore(s => s.selectedLogText)
  const setAILoading = useStore(s => s.setAILoading)
  const setAIChatOpen = useStore(s => s.setAIChatOpen)

  const analyze = useCallback(async (question: string) => {
    if (!selectedLogText && !question) return

    setAILoading(true)
    setAIChatOpen(true)

    addChatMessage({
      id: crypto.randomUUID(),
      role: 'user',
      content: question || 'Please analyze these logs',
      timestamp: new Date().toISOString(),
    })

    addChatMessage({
      id: crypto.randomUUID(),
      role: 'assistant',
      content: '',
      timestamp: new Date().toISOString(),
    })

    try {
      const history = chatMessages
        .filter(m => m.content)
        .map(m => ({ role: m.role, content: m.content }))

      const response = await fetch('/api/ai/analyze', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          provider: aiProvider,
          logText: selectedLogText,
          question,
          history,
        }),
      })

      if (!response.ok) {
        const err = await response.json()
        throw new Error(err.error || 'AI request failed')
      }

      const reader = response.body?.getReader()
      if (!reader) throw new Error('No response stream')

      const decoder = new TextDecoder()
      let fullText = ''

      while (true) {
        const { done, value } = await reader.read()
        if (done) break

        const chunk = decoder.decode(value, { stream: true })
        const lines = chunk.split('\n')

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const data = line.slice(6)
            if (data === '[DONE]') continue
            try {
              const parsed = JSON.parse(data)
              if (parsed.error) throw new Error(parsed.error)
              fullText += parsed.text
              updateLastChatMessage(fullText)
            } catch (e: any) {
              if (e.message && !e.message.includes('JSON')) throw e
            }
          }
        }
      }
    } catch (err: any) {
      updateLastChatMessage(`Error: ${err.message}`)
    } finally {
      setAILoading(false)
    }
  }, [aiProvider, chatMessages, selectedLogText, addChatMessage, updateLastChatMessage, setAILoading, setAIChatOpen])

  return { analyze }
}
