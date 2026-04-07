# wangzhen3691 — LINUX.DO Clone

> **Vibe Coding Prompt:**
> 角色为全栈专家，用 React 复刻 LINUX.DO。加入 CLI 交互模式，支持命令行导航论坛。代码追求极致轻量与高内聚，视觉风格硬核。

## Preview

```
╔══════════════════════════════════════════════╗
║  LINUX.DO WebSSH Terminal                    ║
║  Powered by xterm.js + WebSocket             ║
╚══════════════════════════════════════════════╝

guest@linux.do:~$ neofetch
       _,met$$$$$gg.          guest@linux.do
    ,g$$$$$$$$$$$$$$$P.       ───────────────
  ,g$$P"     """Y$$.".        OS: Debian GNU/Linux 12
 ,$$P'              `$$$.     Kernel: 6.8.0-generic
',$$P       ,ggs.     `$$b:   Shell: bash 5.2.15
`d$$'     ,$P"'   .    $$$    Terminal: xterm.js
 $$:      $$.   -    ,d$$'    CPU: EPYC 7763 (4) @ 2.45GHz
 $$;      Y$b._   _,d$P'     Memory: 2048MiB / 8192MiB
```

## Tech Stack

| Layer | Choice |
|-------|--------|
| Framework | React 19 + Vite 8 |
| Styling | Tailwind CSS v4 |
| Terminal | @xterm/xterm + WebSocket |
| Icons | lucide-react |
| RSS | Native fetch + DOMParser |

## Features

- **Pixel-perfect LINUX.DO UI** — Dark theme (`#222/#111/#099dd7`), 17 categories with exact color badges, Discourse-style topic table
- **CLI 交互模式 (WebSSH)** — xterm.js terminal with multi-tab sessions, built-in demo shell (`neofetch`/`ls`/`cat`/`fortune`), WebSocket real server support
- **RSS 聚合阅读器** — 8 tech sources (HN/Lobsters/LWN/Phoronix...), add/remove feeds, star articles, CORS proxy fetch
- **6 Geek Themes** — Dark / Dracula / Solarized / Nord / Monokai / Gruvbox
- **Topic Detail View** — Post content, code blocks, replies, Markdown editor
- **Responsive** — Collapsible sidebar, mobile-adaptive columns

## Project Structure

```
src/
├── App.jsx                  # Root layout & view routing
├── main.jsx                 # Entry point
├── index.css                # Tailwind v4 + CSS custom properties
├── components/
│   ├── Header.jsx           # Top bar: logo / search / theme / avatar
│   ├── Sidebar.jsx          # Collapsible: community / tools / categories / resources
│   ├── TabNav.jsx           # Latest / New / Unread / Top / Hot / Categories / Votes
│   ├── TopicList.jsx        # Discourse-style topic table
│   ├── TopicRow.jsx         # Row: title + badge + tags + avatars + stats
│   ├── TopicDetail.jsx      # Full post view with replies
│   ├── CategoryBadge.jsx    # Colored category pill
│   ├── CategoryPage.jsx     # Category grid overview
│   ├── Avatar.jsx           # HSL-hashed letter avatar
│   ├── RSSPanel.jsx         # RSS reader with source management
│   ├── WebSSH.jsx           # xterm.js multi-session terminal
│   └── ThemeModal.jsx       # Theme picker
├── data/
│   ├── categories.js        # 17 categories (exact LINUX.DO colors)
│   ├── topics.js            # 40 mock topics
│   └── feeds.js             # 8 RSS sources + sample items
└── hooks/
    └── useTheme.js          # 6 themes with CSS variable injection
```

## Quick Start

```bash
cd wangzhen3691
npm install
npm run dev
```

## CLI Demo Commands

Terminal built-in shell supports:

| Command | Description |
|---------|-------------|
| `help` | Show available commands |
| `ls` | List directory contents |
| `cd <dir>` | Change directory |
| `cat <file>` | Read file contents |
| `neofetch` | System info (styled) |
| `fortune` | Random programming quote |
| `pwd` | Print working directory |
| `uname -a` | System information |
| `clear` | Clear terminal |

## License

MIT
