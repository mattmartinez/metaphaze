# Acceptance Report — metaphaze landing page

> Phase: review | Project: landing-page | Reviewer: GSP QA Reviewer | Date: 2026-04-08

---

## Verdict

**Conditional Pass**

The landing page is substantially complete and production-shippable with two items addressed before launch. All five sections implemented, all seven brand components built, all design tokens used correctly, zero hardcoded hex values in component or page files. Two accessibility issues must be fixed before launch (no video pause control, light-mode signal text contrast for small text). Three minor issues are polish.

---

## Implementation Checklist

| Screen | Design File | Codebase File | Status |
|--------|------------|---------------|--------|
| Landing Page (`/`) | [screen-01-landing.md](../design/screen-01-landing.md) | `app/page.tsx` | complete |

### Sections (within Landing Page)

| Section | Status | Notes |
|---------|--------|-------|
| Nav bar — `mz▌ [/install] [/source]` | complete | Design specified `[/docs]` but critique fix #2 recommended `[/install]` — builder correctly resolved this. `[/docs]` surfaced in §5 as content link. |
| §1 Hero — logo, manifesto, install teaser, VHS pane, phase transition | complete | VHS asset pending (known gap, fallback text in place). Video has no pause button (Major accessibility issue). |
| §2 What It Does — prose + loop diagram | complete | `id="what-it-does"` (design spec said `id="what"` — minor discrepancy, not a functional issue) |
| §3 Why It's Different — comparison table | complete | |
| §4 Install — prerequisites, three code blocks, first-run | complete | |
| §5 Docs Link | complete | |
| Footer — MIT, github, gsp links, refusals list, manifesto, build line | complete | Refusals list verbatim. Manifesto verbatim. |

---

## Screen Coverage

| Designed Screens | Implemented Screens | Coverage |
|-----------------|---------------------|----------|
| 1 (Landing `/`) | 1 (Landing `/`) | 100% |

---

## Component Coverage

| Design Component | Codebase File | Status | Notes |
|---|---|---|---|
| `<Cursor />` | `components/brand/cursor.tsx` | complete | `prefers-reduced-motion` handled via JS client hook |
| `<Pane />` | `components/brand/pane.tsx` | complete | All four variants (idle/active/completed/blocked) |
| `<StatusBadge />` | `components/brand/status-badge.tsx` | complete | All variants including first-party/third-party. `aria-describedby` prop implemented |
| `<CodeBlock />` | `components/brand/code-block.tsx` | complete | Copy button with try/catch (critique fix #3 addressed), 1500ms revert |
| `<PhaseTransitionScreen />` | `components/brand/phase-transition.tsx` | complete | `figure[role=img]` + `aria-hidden` pre |
| `<ComparisonTable />` | `components/brand/comparison-table.tsx` | complete | Desktop table + mobile stacked. `<caption>` (sr-only) + `<table>` with `scope="col"` |
| `<BracketedButton />` | `components/brand/bracketed-button.tsx` | complete | Built but not used directly in page (CodeBlock uses inline `mz-btn` class) |
| Footer (local) | `app/layout.tsx` | complete | |

---

## Token Audit

**Result: PASS — zero hardcoded hex values found in component or page files.**

| Check | Result |
|-------|--------|
| Hardcoded hex in `components/brand/*.tsx` | 0 found |
| Hardcoded hex in `app/page.tsx` | 0 found |
| `--mz-bg` token applied | `body { background: var(--mz-bg) }` in globals.css ✓ |
| `--mz-fg` token applied | body text, all foreground uses ✓ |
| `--mz-fg-muted` token applied | secondary text, prompts, metadata ✓ |
| `--mz-border` token applied | pane borders, code blocks, table cells ✓ |
| `--mz-signal` token applied | cursor, [OK] badge, copy button focus ring ✓ |
| `--mz-warn` token applied | [WARN] badge ✓ |
| `--mz-error` token applied | [ERR] badge, third-party badge ✓ |
| `--font-mono` stack | `"Berkeley Mono", "JetBrains Mono", ui-monospace...` — matches brand spec exactly ✓ |
| `--text-5xl` clamp on logo | applied via `text-[length:var(--text-5xl)]` ✓ |
| `border-radius: 0 !important` global reset | `globals.css:80` ✓ |
| `box-shadow: none !important` global reset | `globals.css:81` ✓ |
| Light-mode token inversion | `@media (prefers-color-scheme: light)` in globals.css ✓ |
| `--mz-signal-text` light-mode small text token | **MISSING** — see issues.md #1 |

### Font Variable Wiring

`layout.tsx` defines `variable: "--font-jetbrains-mono"` but `globals.css` references `"JetBrains Mono"` as a string in the `--font-mono` stack. This is correct: `next/font/google` injects an `@font-face` rule for the font-family name "JetBrains Mono" automatically — the string name in the CSS stack resolves to the loaded font. No breakage.

---

## Accessibility Compliance

Prior accessibility audit: `critique/accessibility-audit.md` (2026-04-08)

| Issue | Severity | Status in Code |
|-------|----------|----------------|
| Light-mode `--mz-signal` at `--text-sm` fails AA (4.06:1) | **Major** | NOT FIXED — `--mz-signal-text` token absent from globals.css |
| No explicit video pause control (WCAG SC 2.2.2) | **Major** | NOT FIXED — video has `autoPlay` with no pause button |
| Border non-text contrast 1.38:1 (WCAG SC 1.4.11) | Major | Intentional design decision — documented in critique as acceptable; consistent with brand aesthetic |
| Table caption text unspecified | Minor | FIXED — `<caption className="sr-only">mz versus other ai coding harnesses...</caption>` in comparison-table.tsx |
| Focus not obscured by sticky nav (SC 2.4.11) | Minor | NOT FIXED — `scroll-padding-top: 48px` absent from globals.css |
| Nav link touch targets (SC 2.5.8) | Minor | Meets 24px minimum (32px actual via padding). Below 44px recommended — acceptable for AA |
| `:focus-visible` specificity over `--ring: transparent` | Minor | FIXED — `*:focus-visible { outline: 2px solid var(--mz-signal) }` applied globally at `globals.css:91-94`. Ring transparency set but overridden correctly by `:focus-visible` |
| `aria-describedby` on FIRST-PARTY / THIRD-PARTY badges | Minor | PARTIALLY FIXED — `StatusBadge` accepts `aria-describedby` prop, legend `p` elements have `id="legend-first-party"` / `id="legend-third-party"` in page.tsx, but `ComparisonTable` does not pass `aria-describedby` to the badges it renders. The prop wiring stops at the component boundary. |

### Skip Link

`layout.tsx` — present as first element in body, uses `sr-only focus:not-sr-only` pattern. Points to `#main`. `<main id="main">` exists. PASS.

### Semantic HTML Structure

- `<html lang="en">` ✓
- `<nav aria-label="primary navigation">` ✓
- `<main id="main">` ✓
- `<section aria-label="...">` on all five sections ✓
- `<footer role="contentinfo">` ✓
- `<figure role="img" aria-label="...">` on loop diagram and phase transition ✓
- `<pre aria-hidden="true">` on all ASCII content ✓

### Cursor Accessibility

`cursor.tsx` uses `"use client"` + `useEffect` to detect `prefers-reduced-motion` at runtime. Correct behavior: animation paused when motion preference is set. `aria-hidden="true"` on cursor span ✓.

---

## Responsive Verification

| Requirement | Implementation | Status |
|-------------|----------------|--------|
| Single column `max-w-3xl` centered | `<div className="mx-auto max-w-3xl px-6">` in page.tsx | PASS |
| Mobile stacked comparison table | `hidden sm:table` (desktop) + `sm:hidden` (mobile stack) | PASS |
| ASCII diagrams with `overflow-x: auto` | `overflow-x-auto` on all `<pre>` elements | PASS |
| Section spacing `py-16` to `py-24` | `className="py-16 sm:py-24"` on all sections | PASS |
| Nav fixed full-width | `fixed top-0 left-0 right-0 z-40` in layout.tsx | PASS |
| `pt-16` main offset for fixed nav | `<main className="pt-16">` | PASS |

---

## Effects Vocabulary

| Effect | Implementation | Status |
|--------|----------------|--------|
| `cursor-blink` | `@keyframes mz-cursor-blink` in globals.css, `.mz-cursor` class | PASS |
| `video-invert` | `.mz-btn:hover { background: var(--mz-fg); color: var(--mz-bg) }` | PASS |
| `underline-reveal + > prefix` | `.mz-nav-link` with `::before` and `::after` pseudo-elements | PASS |
| `step-end` easing only | `animation: mz-cursor-blink 1.06s step-end infinite` | PASS |
| `prefers-reduced-motion` cursor | CSS `animation: none` fallback + JS runtime check in Cursor component | PASS |

---

## Design Constraint Verification

| Constraint | Status |
|-----------|--------|
| No non-monospace fonts | PASS — `--font-sans: var(--font-mono)` globally |
| No border-radius > 0 | PASS — `border-radius: 0 !important` global reset |
| No shadows | PASS — `box-shadow: none !important` global reset |
| No hardcoded hex | PASS — 0 found in component/page files |
| No gradients | PASS — none found |
| No icon libraries | PASS — none imported |
| No emoji | PASS — none found |
| No marketing words | PASS — content reviewed |
| Lowercase headlines | PASS — `text-transform: lowercase` on `.mz-section-heading`, h1 content is `mz` (already lowercase) |
| Manifesto verbatim | PASS — "the orchestrator runs outside the loop. claude builds. mz drives." in footer and hero |
| Signal color < 1% pixels | PASS — used only on cursor glyph and `[OK]` badge text |
| Bold Bet #1 (cursor logo) | PASS — `mz▌` with `mz-cursor` class, signal green, blink active |
| Bold Bet #3 (signal restraint) | PASS — signal only on cursor and [OK] badge |
| Bold Bet #4 (box-drawing) | PASS — phase transition and loop diagram use box-drawing chars |

---

## Known Gaps (Accepted)

These gaps were acknowledged in BUILD-LOG.md and do not affect verdict:

| Gap | Resolution |
|-----|------------|
| `/public/vhs/demo-desktop.webm` | VHS recording pending. `<video>` fallback text renders. Acceptable for v1.0. |
| `/public/vhs/demo-mobile.webm` | Same as above. `<picture>` source swap wired, file pending. |
| Berkeley Mono typeface | JetBrains Mono (Google Fonts) active. Berkeley Mono activates when `.woff2` files added to `public/fonts/`. |
| Favicon | Default Next.js favicon. Replace `app/favicon.ico` with `mz` logomark. |

---

## Related

- Design screen: [../design/screen-01-landing.md](../design/screen-01-landing.md)
- Issues: [issues.md](./issues.md)
- Prior a11y audit: [../critique/accessibility-audit.md](../critique/accessibility-audit.md)
- Brand STYLE.md: [../../branding/metaphaze/patterns/STYLE.md](../../branding/metaphaze/patterns/STYLE.md)
