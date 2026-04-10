# Component Plan

> Phase: design | Project: landing-page | Generated: 2026-04-08
> Implementation target: `code` (Next.js 14 + Tailwind v4 + shadcn/ui)

---

## Overview

Seven components needed. Three are direct brand adaptations. Two override shadcn primitives. Two are net-new and project-specific. Only one component requires `'use client'`.

---

## Reuse (as-is from brand patterns)

These components exist in the brand system and can be used directly with no modification beyond wrapping in React.

| Component | Source | Screens Used | Notes |
|-----------|--------|--------------|-------|
| `<Cursor />` | `.design/branding/metaphaze/patterns/components/cursor.md` | Hero, Nav | React `<span>` wrapping `▌` U+258C. Props: `blink?: boolean`. The CSS animation is already defined in `token-mapping.md`. |
| `<StatusBadge />` | `.design/branding/metaphaze/patterns/components/status-badge.md` | Why Different section | Renders `[TEXT]` with status color. Needs `first-party` and `third-party` variant extensions (see New/Local below). |

---

## Refactor (brand component needs web adaptation)

| Component | Source | Changes Needed | Screens Used |
|-----------|--------|----------------|--------------|
| `<Pane />` | `.design/branding/metaphaze/patterns/components/pane.md` | TUI pane → React `<section>` or `<div>` with `1px solid --mz-border`, optional title bar. Box-drawing title (`┌─ title ──┐`) is rendered as text, not pseudo-elements. Props: `title?: string`, `variant?: 'idle' \| 'active' \| 'completed'`, `children`. | VHS hero frame, Install section, Comparison table |
| `<BracketedButton />` | `.design/branding/metaphaze/patterns/components/bracketed-button.md` | Wraps shadcn `<Button>` with the brand CSS override. The shadcn `<Button>` provides accessibility (keyboard, ARIA role). Override removes all visual defaults: `--radius: 0`, no border, no shadow, no fill. `[ label ]` text via CSS `::before` / `::after`. | Copy button in Install section |

---

## New — Shared (reusable across project)

| Component | Purpose | Screens Used | Notes |
|-----------|---------|--------------|-------|
| `<CodeBlock />` | `<pre><code>` wrapper with brand styling. `--mz-border` background, `$` prompt prefix in `--mz-fg-muted`, optional signal-colored highlight run, optional `<BracketedButton>[ copy ]</BracketedButton>` in top-right. | Hero (install teaser), Install section, What It Does section | Not in brand component library yet — add as `code-block.md` post-ship. Props: `prompt?: '$' \| '>' \| null`, `highlight?: string`, `copyable?: boolean`, `children: string`. |
| `<PhaseTransitionScreen />` | Static `<pre>` rendering a hardcoded phase transition ASCII example. Not a live component — shows what the real TUI output looks like. Wrapped in a `<Pane>` with `aria-label`. | Hero section | One-off component. No props. Content is hardcoded. `aria-hidden` on the `<pre>`, `aria-label` on the `<figure>` wrapper. |

---

## New — Local (single-screen, not reused)

| Component | Screen | Purpose | Notes |
|-----------|--------|---------|-------|
| `<StatusBadge variant="first-party" \| "third-party" />` | Why Different | Project-specific badge variants not in the brand system. `first-party` maps to `--mz-signal`, `third-party` maps to `--mz-error`. Extends the base `<StatusBadge />`. | Documented deviation — see `target-adaptations.md`. |
| `<Footer />` | Landing page | Project-specific footer composed from brand voice + refusals list. Three blocks: license+github, refusals list, manifesto+build. No brand `footer.md` exists — this is the prototype for it. | Candidate for promotion to brand component library. |
| `<ComparisonTable />` | Why Different | Bordered table inside a `<Pane>` showing mz vs. other harnesses. Uses `<StatusBadge />` for each cell. Renders as a proper `<table>` with `scope="col"` headers. Mobile: stacks to labeled rows. | Local to this page. |
| `<DocsLink />` | Docs link section | A prominent `[/docs]` link styled as a nav item, centered in its own section, with a short label. Not the footer — this is a mid-page CTA. | Simple component — could just be a styled `<a>` tag. |

---

## shadcn/ui Installation

```bash
npx shadcn@latest init
# Select: New York style, zinc base, CSS variables: yes, Tailwind v4: yes

npx shadcn@latest add button badge
# Adds:
# components/ui/button.tsx
# components/ui/badge.tsx
```

**Override strategy:** The shadcn components are used for their semantic HTML and accessibility (ARIA, keyboard). All visual tokens are overridden in `app/globals.css` via `:root` variable mapping. Do not edit `components/ui/*.tsx` for visual purposes — only override CSS variables.

Key overrides in `app/globals.css`:
```css
:root {
  --radius: 0;          /* kill all border-radius */
  --ring: transparent;  /* kill focus ring — cursor indicates focus */
  /* ... all mz color tokens mapped to shadcn vars */
}
```

---

## File Structure

```
app/
├── layout.tsx            (font loading, metadata, <html lang="en">)
├── page.tsx              (root page — imports all section components)
├── globals.css           (Tailwind v4 @theme + brand tokens + shadcn overrides)
└── components/
    ├── ui/
    │   ├── button.tsx    (shadcn — visual override via globals.css)
    │   └── badge.tsx     (shadcn — visual override via globals.css)
    ├── cursor.tsx        (brand: <Cursor /> — the ▌ glyph)
    ├── pane.tsx          (brand: <Pane /> — bordered container)
    ├── code-block.tsx    (project: <CodeBlock />)
    ├── bracketed-button.tsx  (brand: <BracketedButton />)
    ├── status-badge.tsx  (brand: <StatusBadge /> + local variants)
    ├── phase-transition-screen.tsx  (project: static ASCII)
    ├── comparison-table.tsx  (project: local)
    └── footer.tsx        (project: local)
```

---

## Client Components

Only one component needs `'use client'`:
- `<CodeBlock />` — when `copyable={true}`, the copy-to-clipboard action needs `navigator.clipboard.writeText()`. All other components are Server Components.

---

## Related

- [responsive.md](./responsive.md)
- [../screen-01-landing.md](../screen-01-landing.md)
- Brand: `.design/branding/metaphaze/patterns/components/`
- Project brief: `target-adaptations.md`
