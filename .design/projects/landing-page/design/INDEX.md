# Design

> Phase: design | Project: landing-page | Generated: 2026-04-08

## Screens

| # | Screen | File | Components Used |
|---|--------|------|-----------------|
| 01 | Landing Page (`/`) | [screen-01-landing.md](./screen-01-landing.md) | Cursor, Pane, CodeBlock, BracketedButton, StatusBadge, PhaseTransitionScreen, ComparisonTable, Footer |

One screen. One page. Five sections (Hero, What it does, Why different, Install, Docs link) + Footer.

## Shared

| Chunk | File | ~Lines |
|-------|------|--------|
| Personas | [personas.md](./shared/personas.md) | ~70 |
| Information Architecture | [information-architecture.md](./shared/information-architecture.md) | ~85 |
| Navigation | [navigation.md](./shared/navigation.md) | ~90 |
| Micro-interactions | [micro-interactions.md](./shared/micro-interactions.md) | ~100 |
| Responsive | [responsive.md](./shared/responsive.md) | ~110 |
| Component Plan | [component-plan.md](./shared/component-plan.md) | ~120 |

## Preview

[preview.html](./preview.html) — self-contained wireframe preview. Open in browser.

## Key Design Decisions

All six research-phase decisions resolved:

1. **Hero composition:** A — logo → manifesto → install teaser → VHS recording (install-first)
2. **Demo asset:** A — `.webm` via `<video autoplay muted loop>`, `<picture>` art direction for mobile
3. **Nav style:** A — bracketed top nav `[/docs] [/source]`, 2 items
4. **Footer content:** A + build line — `[MIT] · [github] · refusals list · manifesto · build: v0.1.0`
5. **Install placement:** C — teaser in hero (one-liner, copyable), full block in §4 install section
6. **Typography:** A — JetBrains Mono via `next/font/google` for MVP

## Brand Alignment Notes

- STYLE.md constraints observed: zero border-radius, no shadows, no gradients, no icon libraries, no marketing words
- STYLE.md patterns implemented: `<Pane>` with box-drawing title bars, `[ bracketed ]` buttons, `[STATUS]` badges, `$` prompt prefixes, monospace-editorial column layout
- STYLE.md effects: `cursor-blink` (logo, nav), `video-invert` (copy button hover), `underline-reveal + > prefix` (nav links)
- Bold Bet #1 implemented: `mz▌` logo with blinking signal-green U+258C glyph
- Bold Bet #3 implemented: signal color `#5fb878` used on cursor glyph only — well under 1% of pixels
- Bold Bet #4 implemented: phase transition screen as static `<pre>` with box-drawing, comparison table as ASCII-style bordered table

## Component inventory

| Component | Source | Status |
|-----------|--------|--------|
| `<Cursor />` | brand `cursor.md` | adapt (React wrapper) |
| `<Pane />` | brand `pane.md` | adapt (React wrapper) |
| `<BracketedButton />` | brand `bracketed-button.md` | adapt (shadcn override) |
| `<StatusBadge />` | brand `status-badge.md` | adapt + extend (first-party/third-party variants) |
| `<CodeBlock />` | project-new | new shared (candidate for brand promotion) |
| `<PhaseTransitionScreen />` | brand `phase-transition.md` | adapt (static `<pre>`) |
| `<ComparisonTable />` | project-new | new local |
| `<Footer />` | project-new | new local |

## Related

- Brand STYLE.md: `.design/branding/metaphaze/patterns/STYLE.md`
- Brand components: `.design/branding/metaphaze/patterns/components/`
- Project BRIEF.md: `../BRIEF.md`
- Brief scope: `../brief/scope.md`
- Brief adaptations: `../brief/target-adaptations.md`
- Research recommendations: `../research/recommendations.md`
