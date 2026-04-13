import Anthropic from '@anthropic-ai/sdk'
import { GoogleGenerativeAI } from '@google/generative-ai'

const SYSTEM_PROMPT = `You are an expert DevOps/SRE engineer analyzing server logs. Your task is to:
1. Identify errors, warnings, and anomalies in log output
2. Explain the root cause of issues
3. Suggest concrete fixes or next steps
4. Highlight any patterns that might indicate systemic problems

Be concise but thorough. Use code blocks for commands or config changes.
Reply in the same language as the user's question.`

export class AIService {
  private anthropic: Anthropic | null = null
  private gemini: GoogleGenerativeAI | null = null

  constructor() {
    if (process.env.ANTHROPIC_API_KEY) {
      this.anthropic = new Anthropic({ apiKey: process.env.ANTHROPIC_API_KEY })
    }
    if (process.env.GEMINI_API_KEY) {
      this.gemini = new GoogleGenerativeAI(process.env.GEMINI_API_KEY)
    }
  }

  async *analyze(
    provider: 'claude' | 'gemini',
    logText: string,
    question: string,
    history: Array<{ role: 'user' | 'assistant'; content: string }> = []
  ): AsyncGenerator<string> {
    const userMessage = question
      ? `Here are the log entries:\n\`\`\`\n${logText}\n\`\`\`\n\nQuestion: ${question}`
      : `Please analyze these log entries and identify any issues:\n\`\`\`\n${logText}\n\`\`\``

    if (provider === 'claude') {
      yield* this.analyzeClaude(userMessage, history)
    } else if (provider === 'gemini') {
      yield* this.analyzeGemini(userMessage, history)
    } else {
      throw new Error(`Unknown provider: ${provider}`)
    }
  }

  private async *analyzeClaude(
    userMessage: string,
    history: Array<{ role: 'user' | 'assistant'; content: string }>
  ): AsyncGenerator<string> {
    if (!this.anthropic) throw new Error('Claude API key not configured. Set ANTHROPIC_API_KEY in .env')

    const messages = [
      ...history.map(h => ({ role: h.role as 'user' | 'assistant', content: h.content })),
      { role: 'user' as const, content: userMessage },
    ]

    const stream = this.anthropic.messages.stream({
      model: 'claude-sonnet-4-20250514',
      max_tokens: 4096,
      system: SYSTEM_PROMPT,
      messages,
    })

    for await (const event of stream) {
      if (event.type === 'content_block_delta' && event.delta.type === 'text_delta') {
        yield event.delta.text
      }
    }
  }

  private async *analyzeGemini(
    userMessage: string,
    history: Array<{ role: 'user' | 'assistant'; content: string }>
  ): AsyncGenerator<string> {
    if (!this.gemini) throw new Error('Gemini API key not configured. Set GEMINI_API_KEY in .env')

    const model = this.gemini.getGenerativeModel({ model: 'gemini-2.0-flash' })

    const chat = model.startChat({
      history: [
        { role: 'user', parts: [{ text: SYSTEM_PROMPT }] },
        { role: 'model', parts: [{ text: 'Understood. I will analyze logs as an expert DevOps/SRE engineer.' }] },
        ...history.map(h => ({
          role: h.role === 'user' ? 'user' as const : 'model' as const,
          parts: [{ text: h.content }],
        })),
      ],
    })

    const result = await chat.sendMessageStream(userMessage)

    for await (const chunk of result.stream) {
      const text = chunk.text()
      if (text) yield text
    }
  }
}
