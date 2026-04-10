# Reference Specs for Build Phase

Collected technical references that `project-build` will need. Each entry: source URL, key takeaways, how it applies to metaphaze.

## Next.js 14 App Router

**URL:** https://nextjs.org/docs/app

**Key takeaways:**
- `app/layout.tsx` is the root layout; wraps all pages with `<html>` and `<body>`
- `app/page.tsx` is the single page for `/`
- Server Components are the default — no `'use client'` needed unless you use hooks, event handlers, or browser APIs
- `app/globals.css` is imported once in `layout.tsx` and applies globally
- Metadata API via exported `metadata` object or `generateMetadata()` function

**Specific APIs metaphaze needs:**
- `export const metadata: Metadata = {...}` in `layout.tsx` for OG tags
- `export default function RootLayout({ children })` — standard pattern
- `import { JetBrains_Mono } from 'next/font/google'` — font loading
- `next.config.js` with `output: 'export'` for static build

**Apply to metaphaze:**
- Single-file page structure: everything inline in `app/page.tsx` or broken into sibling components under `components/`
- Zero client components except the copy button
- Static export to `out/` directory for GitHub Pages or direct S3/CF Pages deploy

---

## Tailwind CSS v4

**URL:** https://tailwindcss.com/docs (v4 docs)

**Key takeaways (v4 vs v3 changes):**
- No `tailwind.config.js` by default — config lives in CSS via `@theme { ... }`
- Single import: `@import "tailwindcss";` replaces `@tailwind base/components/utilities`
- CSS variables declared in `@theme` become utility classes automatically
- First-class CSS variable API: `var(--color-bg)` is accessible everywhere
- `@utility` directive for custom utilities
- `@variant` directive for custom variants (e.g. `@variant dark (&:where(.dark *));`)
- No JS config needed for colors, fonts, spacing

**Specific APIs metaphaze needs:**
- `@import "tailwindcss";`
- `@theme { --color-*: ...; --font-*: ...; }`
- `@utility` for `.btn-bracketed` and `.badge-bracketed`
- Arbitrary values: `max-w-[66ch]`, `border-[#2a2a2a]`

**Apply to metaphaze:**
- Single `globals.css` with `@import`, `@theme`, `:root`, then component classes
- No separate config file
- shadcn CSS variables override at `:root` level

---

## shadcn/ui

**URL:** https://ui.shadcn.com

**Key takeaways:**
- Components are copy-paste, not installed from npm
- `npx shadcn@latest init` sets up `components.json`, `lib/utils.ts`, `components/ui/` directory
- `npx shadcn@latest add button badge` adds only the needed components
- Each component is a `.tsx` file you own and can edit freely
- CSS variables are the theming API; shadcn components reference them via Tailwind classes
- v4-compatible shadcn uses OKLCH or RGB color values in CSS variables

**Specific components metaphaze needs:**
- `Button` — from `components/ui/button.tsx`. Variants: default, outline, ghost. metaphaze will heavily restyle.
- `Badge` — from `components/ui/badge.tsx`. Variants: default, outline. metaphaze will restyle to bracketed style.
- Nothing else. No Card, no Dialog, no Tooltip.

**CSS variable keys to override:**
```
--background, --foreground, --primary, --primary-foreground,
--secondary, --secondary-foreground, --muted, --muted-foreground,
--accent, --accent-foreground, --destructive, --destructive-foreground,
--border, --input, --ring, --radius
```

**Apply to metaphaze:**
- Run `npx shadcn@latest init`, select New York style, zinc base color, CSS variables: yes, Tailwind v4: yes
- Override all color variables in `app/globals.css` at `:root`
- Set `--radius: 0` to kill all border-radius globally
- Add `.btn-bracketed` and `.badge-bracketed` as custom classes, apply via `className` prop

---

## next/font/google

**URL:** https://nextjs.org/docs/app/api-reference/components/font

**Key takeaways:**
- `next/font/google` auto-downloads fonts at build time and self-hosts them
- Zero runtime requests to Google Fonts (privacy + performance win)
- Returns a CSS variable you expose via className on `<html>` or `<body>`
- Subsetting happens automatically based on usage
- `display: 'swap'` recommended to prevent FOIT

**JetBrains Mono specifically:**
```tsx
import { JetBrains_Mono } from 'next/font/google';

const jetbrains = JetBrains_Mono({
  subsets: ['latin'],
  weight: ['400', '500', '700'],
  style: ['normal'],
  display: 'swap',
  variable: '--font-mono',
});
```

Weights needed: 400 (body), 500 (medium emphasis), 700 (bold headlines). Italic not used in metaphaze brand.

**Apply to metaphaze:**
- Use `next/font/google` for JetBrains Mono as the default
- If Berkeley Mono license is purchased, switch to `next/font/local` with the `.woff2` files in `public/fonts/`
- Expose as `--font-mono` variable; reference via `font-mono` utility

---

## Charmbracelet VHS

**URL:** https://github.com/charmbracelet/vhs

**Key takeaways:**
- VHS records terminal sessions and outputs `.gif`, `.webm`, or `.mp4`
- Input is a `.tape` file with simple DSL: `Type`, `Enter`, `Sleep`, `Show`, `Output`
- Theme configuration via `Set Theme "GruvboxDark"` or custom JSON
- Output resolution, font, padding, margin all configurable
- Runs in Docker for CI, or natively via `go install github.com/charmbracelet/vhs@latest`

**Example `.tape` file:**
```
Output demo.webm

Set FontSize 16
Set FontFamily "JetBrains Mono"
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set Padding 20

Type "metaphaze run ./example.tape"
Sleep 500ms
Enter
Sleep 2s
Type "ls -la logs/"
Enter
Sleep 1s
```

**Output format choice:**
- `.gif`: large file size, no quality, universally supported in `<img>`
- `.webm`: small file, high quality, supported in `<video>` in all modern browsers
- `.mp4`: universally supported but larger than webm

**Recommendation:** output `.webm` with `Set Framerate 30`, `Set Width 1200`, `Set Height 600`. Target file size: <500KB.

**Custom theme for metaphaze:**
```
Set Theme { "background": "#0a0a0a", "foreground": "#ededed", "cursor": "#5fb878", ... }
```

**Apply to metaphaze:**
- Create `.tape` file in repo (committed, reproducible)
- Run `vhs demo.tape` in CI or locally to regenerate
- Commit the `.webm` to `public/demo.webm`
- Embed via `<video src="/demo.webm" autoplay muted loop playsinline>`

---

## asciinema Player

**URL:** https://docs.asciinema.org/manual/player/embedding/

**Key takeaways:**
- asciinema is the OG terminal recorder; VHS is the polished alternative
- Embedding options: iframe (from asciinema.org), standalone player (self-hosted), or static `.cast` file + player JS
- The standalone player is `asciinema-player` (npm package, ~50KB)
- Plays `.cast` files (JSON-L format) — smaller than video, scalable to any resolution
- Supports keyboard shortcuts (pause, seek, speed), playback controls
- Lazy load by importing the player only on intersection

**Decision for metaphaze:**
- Use VHS, not asciinema. VHS is simpler to embed (`<video>` tag, no JS), smaller footprint, and matches the static-export deploy target.
- Only use asciinema if the demo needs to be paused/seeked by visitors — which it doesn't on a landing page.

**If asciinema is needed later:**
- `npm install asciinema-player`
- Dynamic import in a Client Component
- Lazy load on scroll via Intersection Observer

---

## WCAG 2.1 Level AA

**URL:** https://www.w3.org/WAI/WCAG21/quickref/?levels=aa

**Key criteria for metaphaze:**

- **1.1.1 Non-text Content** — alt text, `aria-label`, `aria-hidden` for decorative
- **1.3.1 Info and Relationships** — semantic HTML (`<header>`, `<main>`, `<footer>`, `<nav>`, heading hierarchy)
- **1.4.3 Contrast (Minimum)** — 4.5:1 normal text, 3:1 large text. metaphaze hits 16.97:1 for body text.
- **1.4.4 Resize Text** — all text in `rem`, no fixed `px`
- **1.4.10 Reflow** — single-column on mobile, no horizontal scroll at 320px
- **1.4.12 Text Spacing** — line-height ≥ 1.5, paragraph spacing ≥ 2× line-height
- **2.1.1 Keyboard** — all interactive elements reachable by Tab, activatable by Enter/Space
- **2.4.1 Bypass Blocks** — skip link to main content
- **2.4.6 Headings and Labels** — descriptive headings
- **2.4.7 Focus Visible** — custom focus ring required
- **3.1.1 Language of Page** — `<html lang="en">`
- **4.1.2 Name, Role, Value** — all buttons and inputs have accessible names

**Apply to metaphaze:**
- Run Lighthouse accessibility audit pre-ship (target: 100)
- Run axe DevTools pre-ship (target: 0 violations)
- Manual keyboard nav test
- VoiceOver announcement test for ASCII diagrams

---

## Google Fonts API

**URL:** https://fonts.google.com/specimen/JetBrains+Mono

**Key takeaways for JetBrains Mono:**
- Weights available: 100-800 (variable font)
- Styles: normal, italic
- Supports Latin, Latin Extended, Cyrillic, Greek, Vietnamese
- Full family size: ~300KB unsubsetted; ~40KB for Latin-only weight 400

**Direct API URL (if not using next/font):**
```
https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;700&display=swap
```

**Apply to metaphaze:**
- Use `next/font/google` — it handles subsetting and self-hosting automatically
- If fallback needed: `font-family: "JetBrains Mono", ui-monospace, "SF Mono", Menlo, Consolas, monospace;`

---

## Berkeley Mono

**URL:** https://usgraphics.com/products/berkeley-mono

**Key takeaways:**
- Commercial font by US Graphics Company (Berkeley Graphics)
- Pricing: ~$75 for personal license, ~$200+ for commercial, site license for companies
- Purchased via usgraphics.com; delivered as `.woff2`, `.woff`, `.otf`, `.ttf` files
- Self-hosting is allowed per the standard EULA
- Designed as a reference-grade monospace — tighter metrics than JetBrains Mono
- The visual reference for the metaphaze brand

**Apply to metaphaze:**
- If budget permits and this is a flagship site, purchase Berkeley Mono
- Self-host via `next/font/local`:
```tsx
import localFont from 'next/font/local';
const berkeley = localFont({
  src: [
    { path: '../public/fonts/BerkeleyMono-Regular.woff2', weight: '400' },
    { path: '../public/fonts/BerkeleyMono-Bold.woff2', weight: '700' },
  ],
  variable: '--font-mono',
  display: 'swap',
});
```
- Fallback: `"Berkeley Mono", "JetBrains Mono", ui-monospace, monospace`

---

## MDN prefers-reduced-motion

**URL:** https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion

**Key takeaways:**
- CSS media query: `@media (prefers-reduced-motion: reduce) { ... }`
- JS equivalent: `window.matchMedia('(prefers-reduced-motion: reduce)').matches`
- Values: `no-preference` (default), `reduce` (user opted in)
- Browser support: universal in 2026

**Apply to metaphaze:**
- Wrap cursor blink animation in `@media (prefers-reduced-motion: reduce)` to disable
- Optionally wrap VHS autoplay in the same query

---

## GitHub Pages with Next.js Static Export

**URL:** https://nextjs.org/docs/app/guides/static-exports

**Key takeaways:**
- Add `output: 'export'` to `next.config.js`
- Add `trailingSlash: true` for GitHub Pages (needed because GH Pages serves `/about/index.html` not `/about.html`)
- Add `images: { unoptimized: true }` (static export doesn't run `next/image` optimizer)
- Build output in `out/` directory
- Push `out/` contents to `gh-pages` branch, or use a GitHub Action:

```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages
on:
  push:
    branches: [main]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm ci
      - run: npm run build
      - uses: actions/upload-pages-artifact@v3
        with:
          path: ./out
  deploy:
    needs: build
    runs-on: ubuntu-latest
    permissions:
      pages: write
      id-token: write
    steps:
      - uses: actions/deploy-pages@v4
```

**Apply to metaphaze:**
- If the repo is `metaphaze/metaphaze`, deploy to `metaphaze.github.io/metaphaze`
- Custom domain via `public/CNAME` file (e.g. `metaphaze.sh`)
- Add `basePath: '/metaphaze'` if deploying to a project page, omit if custom domain

---

## Vercel Deploy (Alternative)

**URL:** https://vercel.com/docs/frameworks/nextjs

**Key takeaways:**
- Zero-config Next.js deploy
- Automatic preview deployments per branch
- Built-in analytics (Vercel Analytics, if desired — but metaphaze's brand rejects analytics on marketing pages)
- `vercel --prod` or GitHub integration

**Decision:**
- Static export to GitHub Pages is more honest to the brand (no third-party infrastructure on the critical path)
- Vercel is fine if the dev team already uses it

---

## Quick-Reference Summary Table

| Dependency | Version target | Source |
|---|---|---|
| Next.js | 14.2+ | nextjs.org |
| React | 19 (bundled with Next.js 14.2+) | react.dev |
| Tailwind CSS | 4.0+ | tailwindcss.com |
| shadcn/ui | latest | ui.shadcn.com |
| TypeScript | 5.4+ | typescriptlang.org |
| JetBrains Mono | via next/font/google | fonts.google.com |
| Berkeley Mono | optional, $75-$200 | usgraphics.com |
| VHS | latest | github.com/charmbracelet/vhs |
| Node.js (build only) | 20 LTS | nodejs.org |
