/**
 * Website Internationalization System
 * Supports: EN, RO, IT, DE, RU, ZH, FA, PS, ARC + Auto-translate
 */

class WebsiteI18n {
  constructor() {
    this.currentLang = localStorage.getItem('website-lang') || 'en';
    this.translations = {};
    this.supportedLangs = ['en', 'ro', 'it', 'de', 'ru', 'zh', 'fa', 'ps', 'arc'];
    this.rtlLangs = ['fa', 'ps', 'arc'];
    this.autoTranslateLangs = ['fr', 'es', 'pt', 'ja', 'ko', 'ar', 'hi', 'tr', 'pl', 'nl'];
    this.init();
  }

  async init() {
    await this.loadTranslation(this.currentLang);
    this.applyTranslations();
    this.createLanguageSwitcher();
    this.updateDirection();
  }

  async loadTranslation(lang) {
    if (this.translations[lang]) return;
    
    try {
      const response = await fetch(`translations/website-${lang}.json`);
      this.translations[lang] = await response.json();
    } catch (e) {
      console.error(`Failed to load translation for ${lang}:`, e);
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
      const translation = this.t(key);
      if (translation) {
        el.textContent = translation;
      }
    });

    // Apply data-i18n-placeholder
    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
      const key = el.getAttribute('data-i18n-placeholder');
      const translation = this.t(key);
      if (translation) {
        el.placeholder = translation;
      }
    });

    // Apply data-i18n-title
    document.querySelectorAll('[data-i18n-title]').forEach(el => {
      const key = el.getAttribute('data-i18n-title');
      const translation = this.t(key);
      if (translation) {
        el.title = translation;
      }
    });

    // Update meta
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
    localStorage.setItem('website-lang', lang);
    
    if (this.supportedLangs.includes(lang)) {
      await this.loadTranslation(lang);
      this.applyTranslations();
      this.updateDirection();
    } else {
      this.triggerAutoTranslate(lang);
    }
    
    this.updateLanguageSwitcher();
  }

  updateDirection() {
    const isRTL = this.rtlLangs.includes(this.currentLang);
    document.documentElement.dir = isRTL ? 'rtl' : 'ltr';
    
    // Adjust styles for RTL
    if (isRTL) {
      document.body.classList.add('rtl');
    } else {
      document.body.classList.remove('rtl');
    }
  }

  triggerAutoTranslate(targetLang) {
    if (window.google && window.google.translate) {
      const select = document.querySelector('.goog-te-combo');
      if (select) {
        select.value = targetLang;
        select.dispatchEvent(new Event('change'));
      }
    } else {
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
    const existing = document.getElementById('website-lang-switcher');
    if (existing) existing.remove();

    const switcher = document.createElement('div');
    switcher.id = 'website-lang-switcher';
    switcher.className = 'website-lang-switcher';
    
    const currentMeta = this.translations[this.currentLang]?.meta;
    
    switcher.innerHTML = `
      <button class="lang-btn" onclick="websiteI18n.toggleLangMenu()">
        <span class="lang-flag">${currentMeta?.flag || '🌐'}</span>
        <span class="lang-code">${currentMeta?.code?.toUpperCase() || 'EN'}</span>
      </button>
      <div class="lang-menu" id="website-lang-menu">
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
      .website-lang-switcher {
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 10000;
        font-family: system-ui, sans-serif;
      }
      .lang-btn {
        background: rgba(30, 41, 59, 0.9);
        border: 1px solid rgba(255,255,255,0.1);
        border-radius: 8px;
        padding: 8px 12px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 14px;
        color: white;
        backdrop-filter: blur(10px);
      }
      .lang-btn:hover {
        background: rgba(51, 65, 85, 0.9);
      }
      .lang-menu {
        display: none;
        position: absolute;
        top: 100%;
        right: 0;
        margin-top: 8px;
        background: rgba(15, 23, 42, 0.95);
        border-radius: 12px;
        box-shadow: 0 8px 32px rgba(0,0,0,0.4);
        min-width: 220px;
        padding: 8px 0;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255,255,255,0.1);
        max-height: 70vh;
        overflow-y: auto;
      }
      .lang-menu.active {
        display: block;
      }
      .lang-section {
        padding: 8px 0;
        border-bottom: 1px solid rgba(255,255,255,0.1);
      }
      .lang-section:last-child {
        border-bottom: none;
      }
      .lang-section-title {
        display: block;
        padding: 4px 16px;
        font-size: 11px;
        text-transform: uppercase;
        color: rgba(255,255,255,0.5);
        font-weight: 600;
      }
      .lang-option {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 10px 16px;
        cursor: pointer;
        font-size: 14px;
        color: rgba(255,255,255,0.9);
        transition: background 0.2s;
      }
      .lang-option:hover {
        background: rgba(255,255,255,0.1);
      }
      .lang-option.active {
        background: rgba(59, 130, 246, 0.3);
        color: #60a5fa;
      }
      .lang-flag {
        font-size: 16px;
      }
      .lang-name {
        flex: 1;
      }
      .lang-auto-badge {
        font-size: 10px;
        background: rgba(255,255,255,0.1);
        padding: 2px 6px;
        border-radius: 4px;
        color: rgba(255,255,255,0.6);
      }
      #google_translate_element {
        display: none;
      }
      
      /* RTL Support */
      body.rtl .website-lang-switcher {
        right: auto;
        left: 20px;
      }
      body.rtl .lang-menu {
        right: auto;
        left: 0;
      }
    `;
    document.head.appendChild(styles);
    document.body.appendChild(switcher);

    document.addEventListener('click', (e) => {
      if (!switcher.contains(e.target)) {
        document.getElementById('website-lang-menu')?.classList.remove('active');
      }
    });
  }

  createLangOption(lang, isAuto = false) {
    const isActive = this.currentLang === lang;
    
    const langNames = {
      en: 'English',
      ro: 'Română',
      it: 'Italiano',
      de: 'Deutsch',
      ru: 'Русский',
      zh: '简体中文',
      fa: 'فارسی',
      ps: 'پښتو',
      arc: 'ܐܪܡܝܐ',
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
      fa: '🇮🇷', ps: '🇵🇸', arc: '🏛️',
      fr: '🇫🇷', es: '🇪🇸', pt: '🇵🇹', ja: '🇯🇵', ko: '🇰🇷', ar: '🇸🇦',
      hi: '🇮🇳', tr: '🇹🇷', pl: '🇵🇱', nl: '🇳🇱'
    };
    
    return `
      <div class="lang-option ${isActive ? 'active' : ''}" onclick="websiteI18n.setLanguage('${lang}')">
        <span class="lang-flag">${flags[lang] || '🌐'}</span>
        <span class="lang-name">${langNames[lang] || lang}</span>
        ${isAuto ? '<span class="lang-auto-badge">Auto</span>' : ''}
      </div>
    `;
  }

  toggleLangMenu() {
    document.getElementById('website-lang-menu')?.classList.toggle('active');
  }

  updateLanguageSwitcher() {
    this.createLanguageSwitcher();
  }
}

// Initialize
const websiteI18n = new WebsiteI18n();
window.websiteI18n = websiteI18n;
