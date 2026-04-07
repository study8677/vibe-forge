import { useState, useRef, useEffect, useCallback } from 'react'
import { Terminal as TermIcon, Wifi, WifiOff, Plus, X, Settings, Maximize2, Minimize2 } from 'lucide-react'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebLinksAddon } from '@xterm/addon-web-links'
import '@xterm/xterm/css/xterm.css'

const defaultConfig = {
  host: 'localhost',
  port: 22,
  username: 'root',
  wsUrl: 'ws://localhost:8080/ssh',
}

function TermSession({ config, active, onClose, id }) {
  const termRef = useRef(null)
  const containerRef = useRef(null)
  const wsRef = useRef(null)
  const fitRef = useRef(null)
  const [status, setStatus] = useState('disconnected')

  useEffect(() => {
    if (!containerRef.current) return

    const term = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      fontSize: 13,
      fontFamily: '"JetBrains Mono", "Fira Code", "SF Mono", "Cascadia Code", ui-monospace, monospace',
      lineHeight: 1.2,
      theme: {
        background: '#1a1a1a',
        foreground: '#dddddd',
        cursor: '#099dd7',
        cursorAccent: '#1a1a1a',
        selectionBackground: '#052e3d',
        selectionForeground: '#dddddd',
        black: '#1a1a1a',
        red: '#e45735',
        green: '#1ca551',
        yellow: '#a87137',
        blue: '#099dd7',
        magenta: '#c14924',
        cyan: '#32c3c3',
        white: '#dddddd',
        brightBlack: '#666666',
        brightRed: '#fa6c8d',
        brightGreen: '#3AB54A',
        brightYellow: '#e0d900',
        brightBlue: '#88c0d0',
        brightMagenta: '#BB8FCE',
        brightCyan: '#47dcd0',
        brightWhite: '#ffffff',
      },
      scrollback: 5000,
      allowProposedApi: true,
    })

    const fit = new FitAddon()
    fitRef.current = fit
    term.loadAddon(fit)
    term.loadAddon(new WebLinksAddon())
    term.open(containerRef.current)
    fit.fit()
    termRef.current = term

    // Write welcome banner
    term.writeln('\x1b[1;36m╔══════════════════════════════════════════════╗\x1b[0m')
    term.writeln('\x1b[1;36m║\x1b[0m  \x1b[1;37mLINUX.DO WebSSH Terminal\x1b[0m                     \x1b[1;36m║\x1b[0m')
    term.writeln('\x1b[1;36m║\x1b[0m  \x1b[2mPowered by xterm.js + WebSocket\x1b[0m              \x1b[1;36m║\x1b[0m')
    term.writeln('\x1b[1;36m╚══════════════════════════════════════════════╝\x1b[0m')
    term.writeln('')

    if (config.wsUrl) {
      connect(term, config)
    } else {
      startLocalShell(term)
    }

    const ro = new ResizeObserver(() => fit.fit())
    ro.observe(containerRef.current)

    return () => {
      ro.disconnect()
      wsRef.current?.close()
      term.dispose()
    }
  }, [])

  useEffect(() => {
    if (active && fitRef.current) {
      setTimeout(() => fitRef.current.fit(), 50)
      termRef.current?.focus()
    }
  }, [active])

  function connect(term, cfg) {
    setStatus('connecting')
    term.writeln(`\x1b[33m→ Connecting to ${cfg.host}:${cfg.port}...\x1b[0m`)

    try {
      const ws = new WebSocket(cfg.wsUrl)
      wsRef.current = ws

      ws.onopen = () => {
        setStatus('connected')
        term.writeln('\x1b[32m✓ Connected\x1b[0m\r\n')
        ws.send(JSON.stringify({ type: 'auth', user: cfg.username }))
      }

      ws.onmessage = (e) => term.write(e.data)

      ws.onerror = () => {
        setStatus('error')
        term.writeln('\x1b[31m✗ WebSocket error\x1b[0m')
        term.writeln('\x1b[2mFalling back to local demo shell...\x1b[0m\r\n')
        startLocalShell(term)
      }

      ws.onclose = () => {
        if (status !== 'error') {
          setStatus('disconnected')
          term.writeln('\r\n\x1b[33m⚡ Connection closed\x1b[0m')
          startLocalShell(term)
        }
      }

      term.onData((data) => {
        if (ws.readyState === WebSocket.OPEN) ws.send(data)
      })
    } catch {
      setStatus('error')
      term.writeln('\x1b[31m✗ Failed to connect\x1b[0m')
      term.writeln('\x1b[2mStarting local demo shell...\x1b[0m\r\n')
      startLocalShell(term)
    }
  }

  function startLocalShell(term) {
    setStatus('local')
    let cwd = '/home/guest'
    let cmdBuf = ''
    const user = 'guest'
    const host = 'linux.do'

    const fs = {
      '/home/guest': ['Documents', 'Downloads', '.bashrc', '.ssh', 'projects'],
      '/home/guest/projects': ['linux-do-clone', 'dotfiles', 'scripts'],
      '/home/guest/Documents': ['notes.md', 'todo.txt'],
      '/': ['bin', 'etc', 'home', 'tmp', 'usr', 'var'],
    }

    const commands = {
      help: () => [
        '\x1b[1mAvailable commands:\x1b[0m',
        '  help         Show this help',
        '  ls           List directory contents',
        '  cd <dir>     Change directory',
        '  pwd          Print working directory',
        '  whoami       Print username',
        '  uname -a     System information',
        '  date         Current date',
        '  cat <file>   Read file',
        '  neofetch     System info (styled)',
        '  clear        Clear screen',
        '  echo <text>  Print text',
        '  fortune      Random fortune',
      ],
      ls: () => {
        const entries = fs[cwd] || ['(empty)']
        return entries.map((e) =>
          e.startsWith('.') ? `\x1b[2m${e}\x1b[0m` : fs[cwd + '/' + e] ? `\x1b[1;34m${e}/\x1b[0m` : e
        )
      },
      pwd: () => [cwd],
      whoami: () => [user],
      hostname: () => [host],
      'uname -a': () => ['Linux linux.do 6.8.0-generic #1 SMP x86_64 GNU/Linux'],
      date: () => [new Date().toString()],
      clear: () => { term.clear(); return [] },
      uptime: () => [' 23:42:17 up 142 days,  7:31,  1 user,  load average: 0.42, 0.38, 0.35'],
      fortune: () => {
        const fortunes = [
          '"There are only two hard things in Computer Science: cache invalidation and naming things." - Phil Karlton',
          '"Talk is cheap. Show me the code." - Linus Torvalds',
          '"Any sufficiently advanced technology is indistinguishable from magic." - Arthur C. Clarke',
          '"The best way to predict the future is to implement it." - David Heinemeier Hansson',
          '"First, solve the problem. Then, write the code." - John Johnson',
          '"UNIX is simple. It just takes a genius to understand its simplicity." - Dennis Ritchie',
        ]
        return [fortunes[Math.floor(Math.random() * fortunes.length)]]
      },
      neofetch: () => [
        '\x1b[1;36m       _,met$$$$$gg.          \x1b[1;37mguest\x1b[0m@\x1b[1;36mlinux.do\x1b[0m',
        '\x1b[1;36m    ,g$$$$$$$$$$$$$$$P.       \x1b[0m───────────────',
        '\x1b[1;36m  ,g$$P"     """Y$$.".        \x1b[1;37mOS:\x1b[0m Debian GNU/Linux 12',
        '\x1b[1;36m ,$$P\'              `$$$.      \x1b[1;37mKernel:\x1b[0m 6.8.0-generic',
        '\x1b[1;36m\',$$P       ,ggs.     `$$b:   \x1b[1;37mUptime:\x1b[0m 142 days',
        '\x1b[1;36m`d$$\'     ,$P"\'   .    $$$    \x1b[1;37mShell:\x1b[0m bash 5.2.15',
        '\x1b[1;36m $$P      d$\'     ,    $$P    \x1b[1;37mTerminal:\x1b[0m xterm.js',
        '\x1b[1;36m $$:      $$.   -    ,d$$\'    \x1b[1;37mCPU:\x1b[0m EPYC 7763 (4) @ 2.45GHz',
        '\x1b[1;36m $$;      Y$b._   _,d$P\'     \x1b[1;37mMemory:\x1b[0m 2048MiB / 8192MiB',
        '\x1b[1;36m Y$$.    `.`"Y$$$$P"\'         \x1b[1;37mDisk:\x1b[0m 42G / 80G (52%)',
        '\x1b[1;36m `$$b      "-.__              \x1b[1;37mLocale:\x1b[0m zh_CN.UTF-8',
        '\x1b[1;36m  `Y$$                        ',
        '\x1b[1;36m   `Y$$.                      \x1b[40;31m███\x1b[42;32m███\x1b[43;33m███\x1b[44;34m███\x1b[45;35m███\x1b[46;36m███\x1b[47;37m███\x1b[0m',
        '\x1b[1;36m     `$$b.                    ',
        '\x1b[1;36m       `Y$$b.                 ',
        '\x1b[1;36m          `"Y$b._             ',
        '\x1b[1;36m              `"""            \x1b[0m',
      ],
    }

    const prompt = () => `\x1b[1;32m${user}@${host}\x1b[0m:\x1b[1;34m${cwd.replace('/home/guest', '~')}\x1b[0m$ `

    term.write(prompt())

    term.onData((data) => {
      if (data === '\r') {
        term.write('\r\n')
        const cmd = cmdBuf.trim()
        cmdBuf = ''

        if (cmd) {
          if (cmd.startsWith('cd ')) {
            const target = cmd.slice(3).trim()
            if (target === '~' || target === '') {
              cwd = '/home/guest'
            } else if (target === '..') {
              cwd = cwd.split('/').slice(0, -1).join('/') || '/'
            } else if (target.startsWith('/')) {
              cwd = target
            } else {
              cwd = cwd === '/' ? `/${target}` : `${cwd}/${target}`
            }
          } else if (cmd.startsWith('echo ')) {
            term.writeln(cmd.slice(5))
          } else if (cmd.startsWith('cat ')) {
            const file = cmd.slice(4).trim()
            if (file === '.bashrc') {
              term.writeln('# ~/.bashrc\nexport PS1="\\u@\\h:\\w$ "\nalias ll="ls -la"\nalias vim="nvim"\nexport EDITOR=nvim')
            } else if (file === 'notes.md') {
              term.writeln('# Notes\n\n- Finish LINUX.DO clone\n- Set up CI/CD pipeline\n- Review PR #42')
            } else {
              term.writeln(`cat: ${file}: No such file or directory`)
            }
          } else if (commands[cmd]) {
            const output = commands[cmd]()
            output.forEach((line) => term.writeln(line))
          } else if (cmd === 'exit') {
            term.writeln('\x1b[33mGoodbye! 👋\x1b[0m')
            onClose?.()
            return
          } else {
            term.writeln(`\x1b[31mbash: ${cmd}: command not found\x1b[0m`)
            term.writeln('\x1b[2mType "help" for available commands\x1b[0m')
          }
        }
        term.write(prompt())
      } else if (data === '\x7f') {
        if (cmdBuf.length > 0) {
          cmdBuf = cmdBuf.slice(0, -1)
          term.write('\b \b')
        }
      } else if (data === '\x03') {
        cmdBuf = ''
        term.write('^C\r\n')
        term.write(prompt())
      } else if (data >= ' ') {
        cmdBuf += data
        term.write(data)
      }
    })
  }

  const statusColor = {
    connected: 'text-success',
    connecting: 'text-highlight',
    disconnected: 'text-muted',
    error: 'text-danger',
    local: 'text-accent',
  }

  return (
    <div className={`flex-1 flex flex-col ${active ? '' : 'hidden'}`}>
      <div className="flex items-center gap-2 px-3 py-1 border-b border-border/50 text-xs">
        <span className={statusColor[status]}>
          {status === 'connected' ? <Wifi size={10} className="inline" /> : <WifiOff size={10} className="inline" />}
        </span>
        <span className="text-muted">{config.username}@{config.host}</span>
        <span className={`${statusColor[status]} text-[10px]`}>
          {status === 'local' ? '(demo)' : `(${status})`}
        </span>
      </div>
      <div ref={containerRef} className="flex-1" />
    </div>
  )
}

export default function WebSSH() {
  const [sessions, setSessions] = useState([{ id: 1, config: defaultConfig }])
  const [activeTab, setActiveTab] = useState(1)
  const [showConfig, setShowConfig] = useState(false)
  const [config, setConfig] = useState(defaultConfig)
  const [fullscreen, setFullscreen] = useState(false)
  const nextId = useRef(2)

  const addSession = () => {
    const id = nextId.current++
    setSessions([...sessions, { id, config: { ...config } }])
    setActiveTab(id)
  }

  const removeSession = (id) => {
    const next = sessions.filter((s) => s.id !== id)
    setSessions(next)
    if (activeTab === id && next.length) setActiveTab(next[next.length - 1].id)
  }

  return (
    <div className={`flex-1 flex flex-col overflow-hidden ${fullscreen ? 'fixed inset-0 z-50 bg-bg-primary' : ''}`}>
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-border">
        <div className="flex items-center gap-2">
          <TermIcon size={16} className="text-accent" />
          <h2 className="text-sm font-semibold text-primary">WebSSH 终端</h2>
        </div>
        <div className="flex items-center gap-1.5">
          <button
            onClick={() => setShowConfig(!showConfig)}
            className="p-1.5 text-muted hover:text-primary hover:bg-bg-hover rounded transition-colors"
          >
            <Settings size={14} />
          </button>
          <button
            onClick={() => setFullscreen(!fullscreen)}
            className="p-1.5 text-muted hover:text-primary hover:bg-bg-hover rounded transition-colors"
          >
            {fullscreen ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
          </button>
        </div>
      </div>

      {/* Config panel */}
      {showConfig && (
        <div className="px-4 py-3 border-b border-border bg-surface/50 grid grid-cols-2 gap-2 text-xs">
          <input
            value={config.host}
            onChange={(e) => setConfig({ ...config, host: e.target.value })}
            placeholder="Host"
            className="bg-bg-primary border border-border rounded px-2 py-1.5 text-primary outline-none focus:border-accent"
          />
          <input
            value={config.port}
            onChange={(e) => setConfig({ ...config, port: +e.target.value })}
            placeholder="Port"
            className="bg-bg-primary border border-border rounded px-2 py-1.5 text-primary outline-none focus:border-accent"
          />
          <input
            value={config.username}
            onChange={(e) => setConfig({ ...config, username: e.target.value })}
            placeholder="Username"
            className="bg-bg-primary border border-border rounded px-2 py-1.5 text-primary outline-none focus:border-accent"
          />
          <input
            value={config.wsUrl}
            onChange={(e) => setConfig({ ...config, wsUrl: e.target.value })}
            placeholder="WebSocket URL"
            className="bg-bg-primary border border-border rounded px-2 py-1.5 text-primary outline-none focus:border-accent"
          />
        </div>
      )}

      {/* Session tabs */}
      <div className="flex items-center border-b border-border bg-bg-secondary/50 px-2">
        {sessions.map((s) => (
          <div
            key={s.id}
            className={`flex items-center gap-1.5 px-3 py-1.5 text-xs cursor-pointer border-b-2 transition-colors ${
              activeTab === s.id
                ? 'border-accent text-accent'
                : 'border-transparent text-muted hover:text-primary'
            }`}
            onClick={() => setActiveTab(s.id)}
          >
            <TermIcon size={11} />
            <span>{s.config.username}@{s.config.host}</span>
            {sessions.length > 1 && (
              <button
                onClick={(e) => { e.stopPropagation(); removeSession(s.id) }}
                className="ml-1 text-muted hover:text-danger"
              >
                <X size={10} />
              </button>
            )}
          </div>
        ))}
        <button
          onClick={addSession}
          className="p-1.5 ml-1 text-muted hover:text-accent transition-colors"
          title="新建会话"
        >
          <Plus size={12} />
        </button>
      </div>

      {/* Terminal sessions */}
      <div className="flex-1 bg-[#1a1a1a] relative">
        {sessions.map((s) => (
          <TermSession
            key={s.id}
            id={s.id}
            config={s.config}
            active={activeTab === s.id}
            onClose={() => removeSession(s.id)}
          />
        ))}
      </div>
    </div>
  )
}
