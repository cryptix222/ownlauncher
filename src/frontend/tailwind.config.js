/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        canvas:   'var(--bg-canvas)',
        primary:  'var(--bg-primary)',
        surface:  'var(--bg-surface)',
        elevated: 'var(--bg-elevated)',
        hover:    'var(--bg-hover)',
        'border-subtle': 'var(--border-subtle)',
        'border-strong': 'var(--border-strong)',
        'text-main':      'var(--text-primary)',
        'text-secondary': 'var(--text-secondary)',
        'text-muted':     'var(--text-muted)',
        accent: {
          DEFAULT: 'var(--accent)',
          hover:   'var(--accent-hover)',
          fg:      'var(--accent-fg)',
          muted:   'var(--accent-muted)',
        },
        launch: {
          DEFAULT: 'var(--launch-btn)',
          hover:   'var(--launch-btn-hover)',
          fg:      'var(--launch-btn-fg)',
        },
        status: {
          success: 'var(--status-success)',
          warning: 'var(--status-warning)',
          error:   'var(--status-error)',
        },
      },
      fontFamily: {
        /* GitHub / Primer system-font stack */
        sans: ['-apple-system', 'BlinkMacSystemFont', '"Segoe UI"', '"Noto Sans"', 'Helvetica', 'Arial', 'sans-serif'],
        mono: ['ui-monospace', 'SFMono-Regular', '"SF Mono"', 'Menlo', 'Consolas', 'monospace'],
      },
      transitionTimingFunction: {
        'gh': 'cubic-bezier(0.2, 0, 0, 1)',
      },
      transitionDuration: {
        '120': '120ms',
        '140': '140ms',
      },
    },
  },
  plugins: [],
}
