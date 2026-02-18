// Book translations for The Book of Rust
// Supports: EN, RO, IT, DE, RU, ZH, FA, PS, ARC

const RTL_LANGUAGES = ['fa', 'ps', 'arc'];

let currentLang = 'en';
let translations = {};

// Load translation file
async function loadTranslation(lang) {
    if (lang === 'en' || !lang) return null;
    
    try {
        const response = await fetch(`translations/${lang}.json`);
        if (!response.ok) throw new Error(`Failed to load ${lang}`);
        return await response.json();
    } catch (e) {
        console.warn(`Translation ${lang} not available, using English`);
        return null;
    }
}

// Initialize i18n
async function initI18n(lang = 'en') {
    currentLang = lang;
    
    // Set RTL if needed
    if (RTL_LANGUAGES.includes(lang)) {
        document.documentElement.setAttribute('dir', 'rtl');
        document.body.classList.add('rtl');
    } else {
        document.documentElement.removeAttribute('dir');
        document.body.classList.remove('rtl');
    }
    
    translations = await loadTranslation(lang);
    
    if (translations) {
        applyTranslations();
    }
    
    // Save preference
    localStorage.setItem('book-language', lang);
    
    // Update language buttons
    updateLanguageButtons(lang);
}

// Apply translations to the page
function applyTranslations() {
    if (!translations) return;
    
    // Translate elements with data-i18n
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        const translation = getNestedValue(translations, key);
        if (translation) {
            if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
                el.placeholder = translation;
            } else {
                el.textContent = translation;
            }
        }
    });
    
    // Translate title
    if (translations.book && translations.book.title) {
        const mainTitle = document.querySelector('.book-title');
        if (mainTitle) mainTitle.textContent = translations.book.title;
        
        const subtitle = document.querySelector('.book-subtitle');
        if (subtitle && translations.book.subtitle) {
            subtitle.textContent = translations.book.subtitle;
        }
    }
}

// Get nested object value
function getNestedValue(obj, path) {
    return path.split('.').reduce((current, key) => current?.[key], obj);
}

// Update language button states
function updateLanguageButtons(activeLang) {
    document.querySelectorAll('.lang-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.lang === activeLang);
    });
}

// Switch language
async function switchLanguage(lang) {
    await initI18n(lang);
}

// Get translation
function t(key, fallback = '') {
    return getNestedValue(translations, key) || fallback;
}

// Initialize on load
document.addEventListener('DOMContentLoaded', () => {
    const savedLang = localStorage.getItem('book-language') || 'en';
    initI18n(savedLang);
});
