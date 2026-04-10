# Technical Research — Next.js 14 + Tailwind v4 + shadcn/ui

Specifics for building a single-page static landing site with the metaphaze stack. All guidance is Next.js 14 App Router + Tailwind v4 + shadcn/ui specific, not generic React.

## App Router vs Pages Router

**Decision: App Router.**

For a single-page static marketing site, App Router is the right choice in 2026 for these reasons:

1. App Router is the default in Next.js 14+ and receives all new features (Metadata API, React Server Components, parallel routes).
2. Pages Router is in maintenance mode; new projects shouldn't start there.
3. App Router's default is Server Components — meaning the entire landing page ships with zero client JavaScript by default, which is exactly what metaphaze wants.
4. The Metadata API (in `app/layout.tsx`) handles OG images, favicons, and `<meta>` tags without a `next/head` wrapper.

The file structure for a single-page site:

```
app/
  layout.tsx          # root layout, fonts, metadata
  page.tsx            # the single page — all sections inline
  globals.css         # tailwind v4 @import + @theme config
  favicon.ico
  opengraph-image.png # auto-picked up by metadata API
public/
  demo.webm           # VHS recording
  fonts/              # self-hosted Berkeley Mono if licensed
```

No `pages/` directory, no `api/` routes (nothing to serve), no middleware.

## Tailwind v4 `@theme` Pattern

Tailwind v4 (released late 2024, stable through 2026) replaced `tailwind.config.js` with CSS-first config via the `@theme` block. The pattern:

```css
/* app/globals.css */
@import "tailwindcss";

@theme {
  --color-bg: #0a0a0a;
  --color-fg: #ededed;
  --color-signal: #5fb878;
  --color-border: #2a2a2a;
  --color-muted: #8a8a8a;
  --color-amber: #d4a017;

  --font-mono: "JetBrains Mono", "Berkeley Mono", ui-monospace, monospace;
  --font-sans: var(--font-mono);  /* force monospace everywhere */

  --radius-none: 0;
  --shadow-none: none;
}
```

Key differences from v3:
- No `tailwind.config.js` file at all (unless you need plugins)
- `@import "tailwindcss"` replaces the three `@tailwind` directives
- CSS variables defined in `@theme` are automatically available as utility classes (`bg-bg`, `text-fg`, `border-border`)
- Dark mode via CSS variables + `@media (prefers-color-scheme)` — no `dark:` prefix hacks needed if you use variables
- `@utility` block for custom utilities
- `@variant` block for custom variants

## Overriding shadcn/ui CSS Variables in Tailwind v4

shadcn/ui uses CSS variables for all its component tokens. In Tailwind v4, you override them by setting the variables at the `:root` level — not by extending a theme config.

shadcn's default variables (from the `new-york` preset):
```css
:root {
  --background: 0 0% 100%;
  --foreground: 240 10% 3.9%;
  --primary: 240 5.9% 10%;
  --primary-foreground: 0 0% 98%;
  --border: 240 5.9% 90%;
  --radius: 0.5rem;
  /* ... */
}
```

metaphaze overrides:
```css
:root {
  --background: 10 10 10;         /* #0a0a0a as RGB channels */
  --foreground: 237 237 237;      /* #ededed */
  --primary: 95 184 120;          /* #5fb878 signal */
  --primary-foreground: 10 10 10; /* #0a0a0a on signal */
  --border: 42 42 42;             /* #2a2a2a */
  --muted: 138 138 138;           /* #8a8a8a */
  --destructive: 212 160 23;      /* #d4a017 amber */
  --radius: 0;                    /* no border radius anywhere */
}
```

Important: shadcn historically used HSL values; newer installs (2024+) support RGB or OKLCH. Check `components.json` for the color format when you init shadcn. For metaphaze, set `style: "default"` and `baseColor: "zinc"`, then aggressively override.

The `--radius: 0` line is the single most important override — it removes rounded corners from every shadcn component in one line.

## Heavy CSS Overrides for Button and Badge

Only two shadcn components are needed: `Button` and `Badge`. Both will be heavily restyled. The override pattern:

```css
/* app/globals.css, after @theme and :root */

/* Button: bracketed style */
.btn-bracketed {
  @apply bg-transparent border border-border text-fg;
  @apply px-3 py-1 font-mono text-sm;
  @apply hover:border-fg hover:text-fg;
  @apply focus-visible:outline-signal focus-visible:outline-2 focus-visible:outline-offset-2;
}
.btn-bracketed::before { content: "[ "; }
.btn-bracketed::after { content: " ]"; }

/* Badge: bracketed code-style */
.badge-bracketed {
  @apply inline-flex items-center;
  @apply bg-transparent text-muted font-mono text-xs uppercase;
  @apply border border-border px-2 py-0.5;
}
.badge-bracketed::before { content: "["; }
.badge-bracketed::after { content: "]"; }
```

Apply via `<Button className="btn-bracketed">copy</Button>` — the shadcn component handles focus/disabled states, the custom class handles appearance.

## Font Loading

**Option A: JetBrains Mono via `next/font/google`** (recommended for MVP):

```tsx
// app/layout.tsx
import { JetBrains_Mono } from 'next/font/google';

const jetbrains = JetBrains_Mono({
  subsets: ['latin'],
  weight: ['400', '500', '700'],
  display: 'swap',
  variable: '--font-mono',
});

export default function RootLayout({ children }) {
  return (
    <html lang="en" className={jetbrains.variable}>
      <body className="font-mono">{children}</body>
    </html>
  );
}
```

`next/font/google` self-hosts the font file, strips unused glyphs, and emits a `font-display: swap` at build time. Zero network requests to Google Fonts at runtime.

**Option B: Berkeley Mono self-hosted** (if licensed):

```tsx
// app/layout.tsx
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

Berkeley Mono is a commercial font from US Graphics (see reference-specs.md). If budget allows, it's a better fit than JetBrains Mono for metaphaze's Sage archetype because it's sharper and has tighter metrics.

## Server Components Only

The entire landing page is static content. There's no interactivity beyond:
- A "copy" button on the install command (which needs `'use client'`)
- Optional asciinema player (which needs `'use client'` for the player library)

Everything else should be a Server Component. The rule:

```
app/page.tsx                       -> Server Component (default)
components/hero.tsx                -> Server Component
components/install-block.tsx       -> Server Component wrapper
  components/copy-button.tsx       -> 'use client' (only the button)
components/vhs-embed.tsx           -> Server Component (just a <video>)
```

The `'use client'` directive should appear in ONE file: `copy-button.tsx`. This minimizes the client JS bundle to under 10KB (the copy button + React runtime).

Estimated bundle size for this architecture:
- Next.js framework: ~80KB (gzipped, unavoidable)
- Copy button component: ~2KB
- Total JS: ~82KB gzipped

That's above the stretch goal of <50KB, but realistic for Next.js 14. If metaphaze wants <50KB, consider a plain HTML/CSS build with a 50-line `copy-to-clipboard` inline `<script>` — but that loses Next.js benefits (Metadata API, RSC, image optimization).

## Static Export vs Vercel

**Decision: depends on hosting.**

Option A — `output: 'export'` for GitHub Pages or any static host:

```js
// next.config.js
module.exports = {
  output: 'export',
  trailingSlash: true,  // required for GH Pages
  images: { unoptimized: true },  // required with static export
};
```

`next build` produces an `out/` directory with pure HTML/CSS/JS. Deployable to GitHub Pages, Cloudflare Pages, Netlify, S3, or any static host. No Node.js runtime needed.

Limitations of static export:
- No `next/image` optimization (must set `unoptimized: true`)
- No ISR or on-demand revalidation
- No API routes
- No middleware

All of these are fine for a landing page.

Option B — Vercel deploy (default):

Deploy via `vercel --prod` or GitHub integration. Vercel handles the build, CDN, analytics, and ISR. For a landing page there's no benefit over static export unless you're already on Vercel for other projects.

**Recommendation: static export to GitHub Pages.** It matches the "single binary, MIT license, no dependencies" ethos. The site itself should be hostable from the same GitHub repo as the Rust CLI.

## Image Handling

For a landing page with ONE image asset (the VHS recording), the `next/image` component is overkill. The VHS recording is a `.webm` file served as a `<video>` element:

```tsx
<video
  src="/demo.webm"
  autoPlay
  loop
  muted
  playsInline
  preload="metadata"
  aria-label="metaphaze CLI demonstration"
  className="w-full max-w-[800px] border border-border"
/>
```

Key attributes:
- `autoPlay muted playsInline` — required for autoplay on iOS
- `loop` — the demo should repeat
- `preload="metadata"` — don't download the whole file until needed
- `aria-label` — describes the video for screen readers

The favicon can be handled by placing `app/icon.png` or `app/favicon.ico` in the app directory — Next.js Metadata API picks them up automatically.

## Metadata API for OG Images

Next.js 14 Metadata API handles all OG images and `<meta>` tags declaratively:

```tsx
// app/layout.tsx
export const metadata = {
  title: "metaphaze — orchestrate claude code",
  description: "a rust cli that orchestrates claude code. mit licensed. single binary.",
  openGraph: {
    title: "metaphaze",
    description: "a rust cli that orchestrates claude code",
    images: ['/og.png'],
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'metaphaze',
    images: ['/og.png'],
  },
  icons: {
    icon: '/favicon.ico',
  },
};
```

For dynamic OG images, use `app/opengraph-image.tsx` with `next/og` `ImageResponse` — but for metaphaze, a static `og.png` is fine and faster.

## Streaming / SSR

Streaming and SSR matter for pages with dynamic data. The metaphaze landing page has zero dynamic data. With `output: 'export'`, there's no runtime at all — the HTML is pre-rendered at build time.

Decision: skip streaming entirely. Build output is static HTML.

## Build Output Targets

Realistic targets for the metaphaze landing page:

- Total HTML size: <20KB
- Total CSS size: <15KB (Tailwind v4 with PurgeCSS is aggressive)
- Total JS size: <100KB (80KB framework + 20KB app)
- VHS recording: 100-500KB (the biggest asset by far)
- Total first-paint weight: <150KB (excluding VHS, which loads after metadata)
- Lighthouse performance score: 100
- Lighthouse accessibility score: 100

These are achievable with the stack above. The VHS file is the dominant constraint — keep it under 500KB.
