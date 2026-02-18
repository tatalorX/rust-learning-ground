/**
 * Dark Mode Toggle for The Book of Rust
 */

class DarkMode {
  constructor() {
    this.STORAGE_KEY = 'book-dark-mode';
    this.isDark = localStorage.getItem(this.STORAGE_KEY) === 'true';
    this.init();
  }

  init() {
    this.applyMode();
    this.createToggle();
    
    // Listen for system preference changes
    if (window.matchMedia) {
      window.matchMedia('(prefers-color-scheme: dark)')
        .addEventListener('change', e => {
          if (localStorage.getItem(this.STORAGE_KEY) === null) {
            this.isDark = e.matches;
            this.applyMode();
          }
        });
    }
  }

  applyMode() {
    document.documentElement.setAttribute('data-theme', this.isDark ? 'dark' : 'light');
  }

  toggle() {
    this.isDark = !this.isDark;
    localStorage.setItem(this.STORAGE_KEY, this.isDark);
    this.applyMode();
    this.updateToggleIcon();
  }

  createToggle() {
    // Remove existing toggle
    const existing = document.getElementById('dark-mode-toggle');
    if (existing) existing.remove();

    const toggle = document.createElement('button');
    toggle.id = 'dark-mode-toggle';
    toggle.className = 'dark-mode-toggle';
    toggle.setAttribute('aria-label', 'Toggle dark mode');
    toggle.innerHTML = this.isDark ? '☀️' : '🌙';
    toggle.onclick = () => this.toggle();

    // Add styles
    const styles = document.createElement('style');
    styles.textContent = `
      .dark-mode-toggle {
        position: fixed;
        bottom: 20px;
        right: 20px;
        width: 48px;
        height: 48px;
        border-radius: 50%;
        border: none;
        background: var(--bg-dark);
        color: var(--text-ink);
        font-size: 24px;
        cursor: pointer;
        box-shadow: 0 2px 12px rgba(0,0,0,0.2);
        z-index: 1000;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: transform 0.2s, box-shadow 0.2s;
      }
      .dark-mode-toggle:hover {
        transform: scale(1.1);
        box-shadow: 0 4px 20px rgba(0,0,0,0.3);
      }
    `;
    document.head.appendChild(styles);
    document.body.appendChild(toggle);
  }

  updateToggleIcon() {
    const toggle = document.getElementById('dark-mode-toggle');
    if (toggle) {
      toggle.innerHTML = this.isDark ? '☀️' : '🌙';
    }
  }
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
  window.darkMode = new DarkMode();
});
