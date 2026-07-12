export const translations = {
  en: {
    title: "Rust Password",
    tagline:
      "Reveal your last codelock code, command history, and Steam info - instantly.",
    heroDescription:
      "A lightweight, native Windows tool for Rust players. Zero network, zero telemetry. Your data stays on your machine.",
    download: "Download for Windows",
    downloadSubtext: "Free · No install required",
    downloadSubtextLink: "Open source on GitHub",
    heroSourceLink: "Open Source",
    openSource: "Open source",
    backToTop: "Back to top",
    features: "Features",
    feature1Title: "Last Codelock Code",
    feature1Desc:
      "Reveal the last 4-digit code from any codelock you entered, hidden behind a privacy toggle.",
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
      "Reads only your local Windows registry. Never modifies anything, never connects to the internet, no telemetry, no tracking.",
    lightweight: "Lightweight",
    lightweightDesc:
      "Single .exe, tiny footprint, native Windows app. No Electron, no bloat.",
    previewAlt: "Rust Password app preview",
    language: "Русский",
    source: "View source",
    productOverview: "Product overview",
    productHeading: "Everything Rust remembers.",
    productHeadingAccent: "Visible in one scan.",
    localDesign: "Local by design",
    privacyHeading: "Reads your registry.",
    privacyHeadingAccent: "Nothing leaves your PC.",
    getStarted: "Get started",
    downloadHeading: "Find the code.",
    downloadHeadingAccent: "Get back in game."
  },
  ru: {
    title: "Rust Password",
    tagline:
      "Последний код замка, история консоли и данные Steam — в одном окне.",
    heroDescription:
      "Нативное приложение для Windows: читает данные Rust и Steam только на вашем компьютере. Без сети и телеметрии.",
    download: "Скачать для Windows",
    downloadSubtext: "Бесплатно · Без установки",
    downloadSubtextLink: "Открытый исходный код на GitHub",
    heroSourceLink: "Open Source",
    openSource: "Открытый исходный код",
    backToTop: "Наверх",
    features: "Возможности",
    feature1Title: "Последний код замка",
    feature1Desc:
      "Узнайте последние 4 цифры кода с любого замка, который вы вводили - скрыто переключателем приватности.",
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
      "Читает только локальный реестр Windows. Ничего не изменяет, не подключается к интернету, без телеметрии и слежки.",
    lightweight: "Лёгкость",
    lightweightDesc:
      "Один .exe, минимальный размер, нативное Windows-приложение. Без Electron, без лишнего.",
    previewAlt: "Превью приложения Rust Password",
    language: "English",
    source: "Исходный код",
    productOverview: "Обзор приложения",
    productHeading: "Всё, что запоминает Rust.",
    productHeadingAccent: "Доступно за одно сканирование.",
    localDesign: "Локально по умолчанию",
    privacyHeading: "Читает реестр.",
    privacyHeadingAccent: "Данные не покидают ваш ПК.",
    getStarted: "Начать",
    downloadHeading: "Найдите код.",
    downloadHeadingAccent: "Вернитесь в игру."
  }
} as const;

export type Locale = keyof typeof translations;
export type TranslationKey = keyof (typeof translations)["en"];
