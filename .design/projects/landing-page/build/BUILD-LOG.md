# Build Log

> Phase: build | Project: landing-page | Generated: 2026-04-10

## Implementation Summary

Built the complete metaphaze landing page as a statically generated Next.js 16 site. Single page (`/`) with five sections: hero, what it does, why it's different, install, and docs link.

All brand tokens from `metaphaze.yml` are integrated into `app/globals.css`. Seven custom brand components implement the metaphaze design system. The page renders as fully static HTML with zero JavaScript on initial load (the copy button is the only interactive element and uses React Server Component hydration).

The VHS recording (`/public/vhs/demo-desktop.webm`) is a known pending asset — the `<video>` element renders a fallback until it lands.

## Files Created

| File | Purpose |
|------|---------|
| `app/globals.css` | Brand tokens, global reset, effects vocabulary (cursor-blink, video-invert, underline-reveal) |
| `app/layout.tsx` | Root layout with JetBrains Mono font, nav shell, footer shell, skip link |
| `app/page.tsx` | Complete landing page — all 5 sections |
| `components/brand/cursor.tsx` | Blinking `▌` cursor with prefers-reduced-motion |
| `components/brand/pane.tsx` | Bordered container with optional title bar |
| `components/brand/status-badge.tsx` | `[OK]` `[ERR]` `[WARN]` etc. bracketed badges |
| `components/brand/code-block.tsx` | Monospace code display with prompt, highlight, copy button |
| `components/brand/phase-transition.tsx` | Static ASCII phase transition screen |
| `components/brand/comparison-table.tsx` | Desktop table + mobile stacked view |
| `components/brand/bracketed-button.tsx` | `[ action ]` button with video-invert hover |
| `public/vhs/.gitkeep` | Placeholder for VHS recording directory |
| `build/SCAFFOLD-LOG.md` | Scaffold phase log |
| `build/logs/foundations.md` | Foundations phase log |
| `build/logs/component-all.md` | Components phase log |
| `build/logs/screen-01-landing.md` | Screen phase log |

## Files Modified

| File | Changes |
|------|---------|
| `app/globals.css` | Replaced shadcn oklch defaults with metaphaze brand tokens |
| `app/layout.tsx` | Replaced Next.js default with brand nav + footer shells |
| `app/page.tsx` | Replaced placeholder with complete landing page |

## Component Map

| Design Component | Codebase File | Status |
|---|---|---|
| `<Cursor />` | `components/brand/cursor.tsx` | complete |
| `<Pane />` | `components/brand/pane.tsx` | complete |
| `<StatusBadge />` | `components/brand/status-badge.tsx` | complete |
| `<CodeBlock />` | `components/brand/code-block.tsx` | complete |
| `<PhaseTransitionScreen />` | `components/brand/phase-transition.tsx` | complete |
| `<ComparisonTable />` | `components/brand/comparison-table.tsx` | complete |
| `<BracketedButton />` | `components/brand/bracketed-button.tsx` | complete |

## Patterns Applied

- **Monospace everywhere**: all elements use the `var(--font-mono)` stack via `globals.css` global reset
- **No border-radius**: `border-radius: 0 !important` applied globally
- **No shadows**: `box-shadow: none !important` applied globally
- **Token discipline**: zero hardcoded hex values in component or page files
- **Effects vocabulary**: cursor-blink (step-end, 1.06s), video-invert (hover), underline-reveal + > prefix (nav links)
- **Accessibility**: skip link, semantic HTML (nav/main/footer), aria-label on sections, figure/pre pattern for ASCII diagrams, WCAG AA contrast validated
- **Static export**: all routes render as fully static HTML

## Dependencies Added

From scaffold phase (see SCAFFOLD-LOG.md):
- `next@16.2.3`, `react@19.2.4`, `react-dom@19.2.4`
- `tailwindcss@4.x`, `@tailwindcss/postcss`
- `@radix-ui/react-slot`, `class-variance-authority`, `clsx`, `tailwind-merge`
- `tw-animate-css`
- JetBrains Mono from Google Fonts (via `next/font/google`, no npm package)

## Known Gaps

| Gap | Reason | Resolution |
|---|---|---|
| `/public/vhs/demo-desktop.webm` | VHS recording requires the actual running `mz` CLI | Run `charmbracelet/vhs` with `demo.tape` from the metaphaze repo; drop `.webm` in `public/vhs/` |
| `/public/vhs/demo-mobile.webm` | Same as above — mobile crop | Produce 1200×600 variant from same recording session |
| Berkeley Mono typeface | Paid license not purchased — JetBrains Mono is the active font | If/when licensed, drop `.woff2` files in `public/fonts/` and add `@font-face` declarations to `globals.css` |
| Light-mode VHS variant | Only dark recording exists | Acceptable for v1.0; flagged with HTML comment in `<picture>` element |
| Favicon assets | Default Next.js favicon | Replace `app/favicon.ico` with `mz` logo mark |

## Post-build scan

- Hardcoded hex values: **0 found** ✓
- Duplicated className patterns: **0 found** ✓

## Screen Status

| # | Screen | Status | Notes |
|---|---|---|---|
| 01 | Landing | complete | VHS asset pending — video fallback in place |
