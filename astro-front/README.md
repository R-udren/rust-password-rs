# Rust Password Landing Page

Astro-based landing page for [Rust Password](https://pass.rchecker.com/), a small Windows app that shows information saved locally by Rust and Steam.

The site is available in English and Russian.

## Quick start

```sh
bun install
bun run dev
```

Opens at `http://localhost:4321`.

## Build

```sh
bun run build
```

Output goes to `dist/`.

## Preview production build

```sh
bun run preview
```

## Project structure

```text
astro-front/
├── src/
│   ├── assets/          # images (icon, preview)
│   ├── components/      # Astro components (Header, Hero, Features, Footer)
│   ├── i18n/            # translations (en, ru)
│   ├── layouts/         # HTML layout
│   ├── pages/           # page routes
│   ├── scripts/         # client-side scripts (lang redirect)
│   └── styles/          # global CSS
└── public/              # static files
```

## Deployment

Pushing to `main` triggers the `deploy-astro-front.yml` workflow, which builds with Bun and deploys to GitHub Pages.
