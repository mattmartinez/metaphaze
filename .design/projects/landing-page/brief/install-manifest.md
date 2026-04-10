# Install Manifest

> Phase: brief | Project: landing-page | Generated: 2026-04-10

---

## Overview

Copy-paste-ready install commands for scaffolding the Next.js 14 landing page with Tailwind v4 and shadcn/ui primitives that wrap brand components.

## Prerequisites

- Node.js 20+
- npm or pnpm (examples below use `npm`, but `pnpm` is recommended for faster installs)

## Step 1: Scaffold Next.js 14

```bash
npx create-next-app@14 metaphaze-www \
  --typescript \
  --app \
  --src-dir=false \
  --import-alias="@/*" \
  --tailwind \
  --no-eslint
```

Answers to prompts (if any are interactive):

| Prompt | Answer |
|---|---|
| Would you like to use ESLint? | No (metaphaze brand is minimal) |
| Would you like to use Tailwind CSS? | Yes |
| Would you like to use `src/` directory? | No (app/ at root) |
| Would you like to use App Router? | Yes |
| Would you like to customize the default import alias? | `@/*` |

## Step 2: Upgrade to Tailwind v4

`create-next-app` installs Tailwind v3 by default. Upgrade:

```bash
cd metaphaze-www
npm uninstall tailwindcss postcss autoprefixer
npm install tailwindcss@next @tailwindcss/postcss@next
```

Replace `postcss.config.mjs`:

```js
// postcss.config.mjs
export default {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};
```

Delete the v3 `tailwind.config.ts` — v4 uses CSS `@theme` instead.

Replace `app/globals.css` contents with the block from `target-adaptations.md` (the `@import "tailwindcss"; @theme { ... }` block).

## Step 3: Initialize shadcn/ui

```bash
npx shadcn@latest init
```

Answers:

| Prompt | Answer |
|---|---|
| Which style would you like to use? | `default` (metaphaze overrides them all anyway) |
| Which color would you like to use as base color? | `neutral` (will be overridden by metaphaze tokens) |
| Where is your `globals.css`? | `app/globals.css` |
| Would you like to use CSS variables for theming? | Yes |
| Where is your `tailwind.config`? | (skip — v4 uses `@theme` in CSS) |
| Configure import alias for components? | `@/components` |
| Configure import alias for utils? | `@/lib/utils` |
| Are you using React Server Components? | Yes |
| Write config to `components.json`? | Yes |

After init, open `components.json` and verify the paths match. The `tailwind.config` entry will be ignored in v4.

## Step 4: Install shadcn primitives

Only two are needed:

```bash
npx shadcn@latest add button badge
```

This writes:
- `components/ui/button.tsx`
- `components/ui/badge.tsx`

These are the base primitives that `<BracketedButton />` and `<StatusBadge />` wrap.

## Step 5: Paste the shadcn variable mapping

After the `@theme` block in `app/globals.css`, append the shadcn CSS variable mapping from `{BRAND_PATH}/patterns/components/token-mapping.md`:

```css
:root {
  --background: var(--color-mz-bg);
  --foreground: var(--color-mz-fg);
  --primary: var(--color-mz-signal);
  --primary-foreground: var(--color-mz-bg);
  --secondary: var(--color-mz-fg-muted);
  --muted: var(--color-mz-fg-muted);
  --muted-foreground: var(--color-mz-fg-muted);
  --border: var(--color-mz-border);
  --input: var(--color-mz-border);
  --ring: transparent;
  --destructive: var(--color-mz-error);
  --destructive-foreground: var(--color-mz-bg);
  --card: var(--color-mz-bg);
  --card-foreground: var(--color-mz-fg);
  --popover: var(--color-mz-bg);
  --popover-foreground: var(--color-mz-fg);
  --radius: 0;
}

/* Global rules: kill radius, shadows, and rings everywhere */
* {
  border-radius: 0 !important;
  box-shadow: none !important;
}

*:focus {
  outline: none;
  box-shadow: none;
}
```

## Step 6: Install JetBrains Mono from Google Fonts

Next.js 14 uses `next/font/google` for automatic font optimization:

```tsx
// app/layout.tsx
import { JetBrains_Mono } from "next/font/google";

const jetbrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  weight: ["400", "600", "700"],
  variable: "--font-mono-runtime",
  display: "swap",
});

export default function RootLayout({ children }) {
  return (
    <html lang="en" className={jetbrainsMono.variable}>
      <body className="font-mono bg-[var(--color-mz-bg)] text-[var(--color-mz-fg)]">
        {children}
      </body>
    </html>
  );
}
```

The `--font-mono-runtime` variable is injected by Next.js; the `--font-mono` CSS variable in the `@theme` block should include it in its fallback stack:

```css
--font-mono: "Berkeley Mono", var(--font-mono-runtime), "JetBrains Mono", ui-monospace, monospace;
```

Berkeley Mono is first in the stack — if the maintainer self-hosts it later, it activates automatically.

## Step 7: Add Berkeley Mono (optional — only if licensed)

If the maintainer has a Berkeley Mono license, drop the `.woff2` files in `public/fonts/` and add to `app/globals.css`:

```css
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Regular.woff2") format("woff2");
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-SemiBold.woff2") format("woff2");
  font-weight: 600;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Bold.woff2") format("woff2");
  font-weight: 700;
  font-style: normal;
  font-display: swap;
}
```

Berkeley Mono is already first in the `--font-mono` stack from Step 6, so adding the files activates it automatically.

## Step 8: Scaffold the brand components

Create `components/brand/`:

```
components/brand/
├── cursor.tsx
├── pane.tsx
├── bracketed-button.tsx
├── status-badge.tsx
├── code-block.tsx
├── phase-transition.tsx
└── footer.tsx
```

Each file implements the corresponding adaptation from `target-adaptations.md`. The design and build phases produce the actual code — this file just lists what gets created.

## Step 9: Verify

```bash
npm run dev
```

Open `http://localhost:3000` — the page should load in dark mode with JetBrains Mono, the cursor blinking in signal green.

Smoke checks:
- `View Source` shows the manifesto and install command in plain text
- Disable JavaScript in DevTools — page still renders and reads correctly
- Toggle OS theme to light — page inverts cleanly
- Run the page through WAVE or axe-core — zero contrast violations

## Step 10: Ship

Deploy to Vercel:

```bash
npm install -g vercel
vercel --prod
```

Or ship to GitHub Pages:

```bash
# Add to next.config.js
module.exports = {
  output: "export",
  basePath: "/metaphaze",
};

# Build and deploy
npm run build
# then push the `out/` directory to a `gh-pages` branch
```

## Total install time

Scaffold + shadcn init + component creation: ~10 minutes if working sequentially.

The tight scope is intentional — every command above is copy-paste. No custom configuration, no decision points, no "figure out which package to use." The brand system already made every decision; this file just records the commands to apply them.
