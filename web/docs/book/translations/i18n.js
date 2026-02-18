/**
 * The Book of Rust - Internationalization System
 * Supports: EN, RO, IT, DE, RU, ZH (primary) + Auto-translate (secondary)
 */

class I18n {
  constructor() {
    this.currentLang = localStorage.getItem('book-lang') || 'en';
    this.translations = {};
    this.supportedLangs = ['en', 'ro', 'it', 'de', 'ru', 'zh'];
    this.autoTranslateLangs = ['fr', 'es', 'pt', 'ja', 'ko', 'ar', 'hi', 'tr', 'pl', 'nl'];
    this.init();
  }

  async init() {
    await this.loadTranslation(this.currentLang);
    this.applyTranslations();
    this.createLanguageSwitcher();
    this.updatePageDirection();
  }

  async loadTranslation(lang) {
    if (this.translations[lang]) return;
    
    try {
      const response = await fetch(`translations/${lang}.json`);
      this.translations[lang] = await response.json();
    } catch (e) {
      console.error(`Failed to load translation for ${lang}:`, e);
      // Fallback to English
      if (lang !== 'en') {
        this.currentLang = 'en';
        await this.loadTranslation('en');
      }
    }
  }

  t(key) {
    const keys = key.split('.');
    let value = this.translations[this.currentLang];
    
    for (const k of keys) {
      if (value && value[k] !== undefined) {
        value = value[k];
      } else {
        // Fallback to English
        value = this.translations['en'];
        for (const k2 of keys) {
          value = value?.[k2];
        }
        break;
      }
    }
    
    return value || key;
  }

  applyTranslations() {
    // Apply data-i18n attributes
    document.querySelectorAll('[data-i18n]').forEach(el => {
      const key = el.getAttribute('data-i18n');
      el.textContent = this.t(key);
    });

    // Apply data-i18n-placeholder for inputs
    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
      const key = el.getAttribute('data-i18n-placeholder');
      el.placeholder = this.t(key);
    });

    // Apply data-i18n-title for tooltips
    document.querySelectorAll('[data-i18n-title]').forEach(el => {
      const key = el.getAttribute('data-i18n-title');
      el.title = this.t(key);
    });

    // Update meta tags
    const meta = this.translations[this.currentLang]?.meta;
    if (meta) {
      document.documentElement.lang = meta.code;
      document.documentElement.dir = meta.direction;
    }
  }

  async setLanguage(lang) {
    if (!this.supportedLangs.includes(lang) && !this.autoTranslateLangs.includes(lang)) {
      console.warn(`Language ${lang} not supported`);
      return;
    }

    this.currentLang = lang;
    localStorage.setItem('book-lang', lang);
    
    if (this.supportedLangs.includes(lang)) {
      await this.loadTranslation(lang);
      this.applyTranslations();
      this.updatePageDirection();
    } else {
      // Trigger auto-translation
      this.triggerAutoTranslate(lang);
    }
    
    this.updateLanguageSwitcher();
  }

  triggerAutoTranslate(targetLang) {
    // Check if Google Translate is available
    if (window.google && window.google.translate) {
      const select = document.querySelector('.goog-te-combo');
      if (select) {
        select.value = targetLang;
        select.dispatchEvent(new Event('change'));
      }
    } else {
      // Inject Google Translate script
      this.injectGoogleTranslate(targetLang);
    }
  }

  injectGoogleTranslate(targetLang) {
    const script = document.createElement('script');
    script.src = 'https://translate.google.com/translate_a/element.js?cb=googleTranslateElementInit';
    script.async = true;
    
    window.googleTranslateElementInit = () => {
      new google.translate.TranslateElement(
        { 
          pageLanguage: 'en',
          includedLanguages: this.autoTranslateLangs.join(','),
          layout: google.translate.TranslateElement.InlineLayout.SIMPLE
        },
        'google_translate_element'
      );
      
      // Auto-select the target language after a short delay
      setTimeout(() => {
        const select = document.querySelector('.goog-te-combo');
        if (select) {
          select.value = targetLang;
          select.dispatchEvent(new Event('change'));
        }
      }, 1000);
    };
    
    document.body.appendChild(script);
  }

  createLanguageSwitcher() {
    // Remove existing switcher
    const existing = document.getElementById('lang-switcher');
    if (existing) existing.remove();

    const switcher = document.createElement('div');
    switcher.id = 'lang-switcher';
    switcher.className = 'lang-switcher';
    
    const currentMeta = this.translations[this.currentLang]?.meta;
    
    switcher.innerHTML = `
      <button class="lang-btn" onclick="i18n.toggleLangMenu()">
        <span class="lang-flag">${currentMeta?.flag || '🌐'}</span>
        <span class="lang-code">${currentMeta?.code?.toUpperCase() || 'EN'}</span>
      </button>
      <div class="lang-menu" id="lang-menu">
        <div class="lang-section">
          <span class="lang-section-title">${this.t('language.primary')}</span>
          ${this.supportedLangs.map(lang => this.createLangOption(lang)).join('')}
        </div>
        <div class="lang-section">
          <span class="lang-section-title">${this.t('language.secondary')}</span>
          ${this.autoTranslateLangs.map(lang => this.createLangOption(lang, true)).join('')}
        </div>
      </div>
    `;
    
    // Add styles
    const styles = document.createElement('style');
    styles.textContent = `
      .lang-switcher {
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
        font-family: system-ui, sans-serif;
      }
      .lang-btn {
        background: rgba(255,255,255,0.9);
        border: 1px solid #ddd;
        border-radius: 8px;
        padding: 8px 12px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 14px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
      }
      .lang-btn:hover {
        background: white;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
      }
      .lang-menu {
        display: none;
        position: absolute;
        top: 100%;
        right: 0;
        margin-top: 8px;
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 20px rgba(0,0,0,0.15);
        min-width: 200px;
        padding: 8px 0;
      }
      .lang-menu.active {
        display: block;
      }
      .lang-section {
        padding: 8px 0;
        border-bottom: 1px solid #eee;
      }
      .lang-section:last-child {
        border-bottom: none;
      }
      .lang-section-title {
        display: block;
        padding: 4px 16px;
        font-size: 11px;
        text-transform: uppercase;
        color: #666;
        font-weight: 600;
      }
      .lang-option {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 16px;
        cursor: pointer;
        font-size: 14px;
      }
      .lang-option:hover {
        background: #f5f5f5;
      }
      .lang-option.active {
        background: #e3f2fd;
        color: #1976d2;
      }
      .lang-flag {
        font-size: 16px;
      }
      .lang-name {
        flex: 1;
      }
      .lang-auto-badge {
        font-size: 10px;
        background: #f0f0f0;
        padding: 2px 6px;
        border-radius: 4px;
        color: #666;
      }
      #google_translate_element {
        display: none;
      }
    `;
    document.head.appendChild(styles);
    
    document.body.appendChild(switcher);
    
    // Close menu on outside click
    document.addEventListener('click', (e) => {
      if (!switcher.contains(e.target)) {
        document.getElementById('lang-menu')?.classList.remove('active');
      }
    });
  }

  createLangOption(lang, isAuto = false) {
    const meta = this.translations[lang]?.meta;
    const isActive = this.currentLang === lang;
    
    const langNames = {
      en: 'English',
      ro: 'Română',
      it: 'Italiano',
      de: 'Deutsch',
      ru: 'Русский',
      zh: '简体中文',
      fr: 'Français',
      es: 'Español',
      pt: 'Português',
      ja: '日本語',
      ko: '한국어',
      ar: 'العربية',
      hi: 'हिन्दी',
      tr: 'Türkçe',
      pl: 'Polski',
      nl: 'Nederlands'
    };
    
    const flags = {
      en: '🇬🇧', ro: '🇷🇴', it: '🇮🇹', de: '🇩🇪', ru: '🇷🇺', zh: '🇨🇳',
      fr: '🇫🇷', es: '🇪🇸', pt: '🇵🇹', ja: '🇯🇵', ko: '🇰🇷', ar: '🇸🇦',
      hi: '🇮🇳', tr: '🇹🇷', pl: '🇵🇱', nl: '🇳🇱'
    };
    
    return `
      <div class="lang-option ${isActive ? 'active' : ''}" onclick="i18n.setLanguage('${lang}')">
        <span class="lang-flag">${flags[lang] || '🌐'}</span>
        <span class="lang-name">${langNames[lang] || lang}</span>
        ${isAuto ? '<span class="lang-auto-badge">Auto</span>' : ''}
      </div>
    `;
  }

  toggleLangMenu() {
    document.getElementById('lang-menu')?.classList.toggle('active');
  }

  updateLanguageSwitcher() {
    this.createLanguageSwitcher();
  }

  updatePageDirection() {
    const meta = this.translations[this.currentLang]?.meta;
    if (meta?.direction) {
      document.documentElement.dir = meta.direction;
    }
  }
}

// Initialize
const i18n = new I18n();

// Expose for inline handlers
window.i18n = i18n;
