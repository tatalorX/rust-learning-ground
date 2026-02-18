/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts}",
  ],
  theme: {
    extend: {
      colors: {
        'bg-primary': '#0a0f1a',
        'bg-secondary': '#111827',
        'bg-tertiary': '#1f2937',
        'bg-elevated': '#283548',
        'text-primary': '#f9fafb',
        'text-secondary': '#9ca3af',
        'text-muted': '#6b7280',
        'border-color': '#374151',
        'accent-orange': '#f97316',
        'accent-green': '#10b981',
        'accent-blue': '#3b82f6',
        'accent-purple': '#8b5cf6',
        'accent-yellow': '#eab308',
        'accent-red': '#ef4444',
        'editor-bg': '#0d1117',
        'editor-line': '#161b22',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
}
