export const translations = {
  en: {
    title: "Rust Pass Reveal",
    tagline:
      "Your last codelock code, console history, and Steam data in one place.",
    heroDescription:
      "A native Windows app that reads Rust and Steam data locally. No network. No telemetry.",
    download: "Download for Windows",
    downloadFree: "Free",
    downloadNoInstall: "No install required",
    heroSourceLink: "GitHub",
    openSource: "Open source",
    backToTop: "Back to top",
    features: "Features",
    feature1Title: "Last Codelock Code",
    feature1Desc:
      "See the last four-digit code you entered. Reveal it only when you choose.",
    feature2Title: "Command History",
    feature2Desc:
      "Browse every console command you typed in-game, newest first.",
    feature3Title: "Steam Info",
    feature3Desc:
      "See your active Steam display name and SteamID64 at a glance.",
    feature4Title: "Game Status",
    feature4Desc:
      "Check if Rust is installed and whether it's currently running.",
    privacy: "Privacy First",
    privacyDesc:
      "Reads local Windows registry data. Never changes it or sends it anywhere.",
    lightweight: "Lightweight",
    lightweightDesc:
      "Single .exe, tiny footprint, native Windows app. No Electron, no bloat.",
    previewAlt: "Rust Pass Reveal app preview",
    language: "Русский",
    source: "View source",
    productOverview: "Product overview",
    productHeading: "Everything Rust remembers.",
    productHeadingAccent: "Visible in one scan.",
    localDesign: "Local by design",
    privacyHeading: "Reads the registry.",
    privacyHeadingAccent: "Nothing leaves your PC.",
    getStarted: "Download",
    downloadHeading: "Rust data.",
    downloadHeadingAccent: "Kept local."
  },
  ru: {
    title: "Rust Pass Reveal",
    tagline:
      "Последний код замка, история консоли и данные Steam - в одном окне.",
    heroDescription:
      "Нативное Windows-приложение для локальных данных Rust и Steam. Без сети и телеметрии.",
    download: "Скачать для Windows",
    downloadFree: "Бесплатно",
    downloadNoInstall: "Без установки",
    heroSourceLink: "GitHub",
    openSource: "Открытый исходный код",
    backToTop: "Наверх",
    features: "Возможности",
    feature1Title: "Последний код замка",
    feature1Desc: "Показывает последние четыре цифры введённого кода.",
    feature2Title: "История команд",
    feature2Desc:
      "Просматривайте все консольные команды, введённые в игре, от новых к старым.",
    feature3Title: "Информация Steam",
    feature3Desc: "Узнайте имя отображения Steam и SteamID64 одним взглядом.",
    feature4Title: "Статус игры",
    feature4Desc:
      "Проверьте, установлен ли Rust и запущен ли он в данный момент.",
    privacy: "Приватность прежде всего",
    privacyDesc:
      "Читает локальные данные из реестра Windows. Ничего не меняет и не отправляет в сеть.",
    lightweight: "Лёгкость",
    lightweightDesc:
      "Один .exe. Нативное Windows-приложение без Electron и лишнего.",
    previewAlt: "Превью приложения Rust Pass Reveal",
    language: "English",
    source: "Исходный код",
    productOverview: "Обзор приложения",
    productHeading: "Всё, что запоминает Rust.",
    productHeadingAccent: "За одно сканирование.",
    localDesign: "Локально по умолчанию",
    privacyHeading: "Читает реестр.",
    privacyHeadingAccent: "Данные остаются на ПК.",
    getStarted: "Скачать",
    downloadHeading: "Данные Rust.",
    downloadHeadingAccent: "Локально."
  }
} as const;

export type Locale = keyof typeof translations;
export type TranslationKey = keyof (typeof translations)["en"];
