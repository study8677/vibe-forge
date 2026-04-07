import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        notion: {
          bg: '#ffffff',
          'bg-secondary': '#f7f7f5',
          'bg-tertiary': '#f1f1ef',
          'bg-hover': '#efefed',
          'bg-active': '#e8e7e3',
          text: '#37352f',
          'text-secondary': '#787774',
          'text-tertiary': '#9b9a97',
          'text-placeholder': '#c4c4c0',
          border: '#e8e7e3',
          'border-light': '#eeeeec',
          blue: '#2383e2',
          'blue-bg': '#d3e5ef',
          red: '#eb5757',
          'red-bg': '#ffe2dd',
          green: '#4daa57',
          'green-bg': '#dbeddb',
          orange: '#e9820c',
          'orange-bg': '#fdecc8',
          purple: '#9065b0',
          'purple-bg': '#e8deee',
          yellow: '#dfab01',
          'yellow-bg': '#fdecc8',
        },
      },
      fontFamily: {
        sans: [
          '-apple-system',
          'BlinkMacSystemFont',
          '"Segoe UI"',
          'Helvetica',
          '"Apple Color Emoji"',
          'Arial',
          'sans-serif',
          '"Segoe UI Emoji"',
          '"Segoe UI Symbol"',
        ],
      },
      fontSize: {
        'title': ['2.25rem', { lineHeight: '1.2', fontWeight: '700' }],
      },
      boxShadow: {
        'notion': '0 1px 2px rgba(0,0,0,0.04)',
        'notion-md': '0 4px 12px rgba(0,0,0,0.08)',
        'notion-lg': '0 8px 24px rgba(0,0,0,0.12)',
        'notion-hover': '0 2px 8px rgba(0,0,0,0.08)',
      },
      animation: {
        'fade-in': 'fadeIn 0.2s ease-out',
        'slide-in': 'slideIn 0.2s ease-out',
        'scale-in': 'scaleIn 0.15s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { opacity: '0', transform: 'translateY(-4px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        scaleIn: {
          '0%': { opacity: '0', transform: 'scale(0.95)' },
          '100%': { opacity: '1', transform: 'scale(1)' },
        },
      },
    },
  },
  plugins: [],
}

export default config
