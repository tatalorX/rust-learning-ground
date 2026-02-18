import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import enTranslations from "../locales/en.json";
import esTranslations from "../locales/es.json";
import frTranslations from "../locales/fr.json";
import deTranslations from "../locales/de.json";
import zhTranslations from "../locales/zh.json";
import jaTranslations from "../locales/ja.json";
import ptTranslations from "../locales/pt.json";
import ruTranslations from "../locales/ru.json";
import arTranslations from "../locales/ar.json";
import hiTranslations from "../locales/hi.json";

const resources = {
  en: { translation: enTranslations },
  es: { translation: esTranslations },
  fr: { translation: frTranslations },
  de: { translation: deTranslations },
  zh: { translation: zhTranslations },
  ja: { translation: jaTranslations },
  pt: { translation: ptTranslations },
  ru: { translation: ruTranslations },
  ar: { translation: arTranslations },
  hi: { translation: hiTranslations },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: "en",
    debug: false,
    interpolation: {
      escapeValue: false,
    },
    detection: {
      order: ["localStorage", "navigator", "htmlTag"],
      caches: ["localStorage"],
    },
  });

export default i18n;
