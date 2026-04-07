import { useState, useEffect } from 'react'

const themes = {
  dark: {
    name: 'Dark',
    vars: {
      '--color-bg-primary': '#222222',
      '--color-bg-secondary': '#111111',
      '--color-bg-hover': '#313131',
      '--color-bg-selected': '#052e3d',
      '--color-primary': '#dddddd',
      '--color-accent': '#099dd7',
      '--color-border': '#3a3a3a',
      '--color-surface': '#2a2a2a',
      '--color-muted': '#999999',
    },
  },
  dracula: {
    name: 'Dracula',
    vars: {
      '--color-bg-primary': '#282a36',
      '--color-bg-secondary': '#1e1f29',
      '--color-bg-hover': '#44475a',
      '--color-bg-selected': '#3d2b55',
      '--color-primary': '#f8f8f2',
      '--color-accent': '#bd93f9',
      '--color-border': '#44475a',
      '--color-surface': '#343746',
      '--color-muted': '#6272a4',
    },
  },
  solarized: {
    name: 'Solarized',
    vars: {
      '--color-bg-primary': '#002b36',
      '--color-bg-secondary': '#001f27',
      '--color-bg-hover': '#073642',
      '--color-bg-selected': '#0a4052',
      '--color-primary': '#839496',
      '--color-accent': '#268bd2',
      '--color-border': '#073642',
      '--color-surface': '#073642',
      '--color-muted': '#586e75',
    },
  },
  nord: {
    name: 'Nord',
    vars: {
      '--color-bg-primary': '#2e3440',
      '--color-bg-secondary': '#242933',
      '--color-bg-hover': '#3b4252',
      '--color-bg-selected': '#2e4a5e',
      '--color-primary': '#d8dee9',
      '--color-accent': '#88c0d0',
      '--color-border': '#3b4252',
      '--color-surface': '#3b4252',
      '--color-muted': '#616e88',
    },
  },
  monokai: {
    name: 'Monokai',
    vars: {
      '--color-bg-primary': '#272822',
      '--color-bg-secondary': '#1e1f1c',
      '--color-bg-hover': '#3e3d32',
      '--color-bg-selected': '#3e3d32',
      '--color-primary': '#f8f8f2',
      '--color-accent': '#66d9ef',
      '--color-border': '#3e3d32',
      '--color-surface': '#3e3d32',
      '--color-muted': '#75715e',
    },
  },
  gruvbox: {
    name: 'Gruvbox',
    vars: {
      '--color-bg-primary': '#282828',
      '--color-bg-secondary': '#1d2021',
      '--color-bg-hover': '#3c3836',
      '--color-bg-selected': '#3c3836',
      '--color-primary': '#ebdbb2',
      '--color-accent': '#83a598',
      '--color-border': '#3c3836',
      '--color-surface': '#3c3836',
      '--color-muted': '#928374',
    },
  },
}

export function useTheme() {
  const [current, setCurrent] = useState(() => localStorage.getItem('theme') || 'dark')

  useEffect(() => {
    const theme = themes[current]
    if (!theme) return
    Object.entries(theme.vars).forEach(([k, v]) => document.documentElement.style.setProperty(k, v))
    localStorage.setItem('theme', current)
  }, [current])

  return { current, setCurrent, themes: Object.entries(themes).map(([k, v]) => ({ id: k, name: v.name })) }
}
