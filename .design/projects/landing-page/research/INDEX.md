# Research Phase — metaphaze Landing Page

Phase output from `/gsp-project-research`. Seven research chunks + this index, covering UX patterns, competitor product analysis, technical stack specifics, accessibility, content strategy, reference specs, and synthesis.

## Phase Summary

This research builds on the brand-level work already completed in `.design/branding/metaphaze/discover/` (competitive-audit, market-landscape, mood-board-direction, trend-analysis). Where brand discovery asked "who are we and who are we for," this project research asks "what specifically do we build, how do we build it, and what mistakes do we avoid."

Focus areas:
- Developer landing page UX patterns, drawn from 9+ real reference sites
- Five product-level competitor deep-dives (htmx, suckless, charm.sh, oxide, tailscale) with pattern comparison matrix
- Next.js 14 + Tailwind v4 + shadcn/ui technical specifics (not generic React)
- WCAG 2.1 AA for a monospace-heavy dark-background page
- Microcopy and banned-words list for maintaining the Sage / brutalist voice
- Collected reference specs that the build phase will need
- Synthesis: adopt / adapt / avoid + six key design decisions

The research converges on a clear picture: a 3-4 viewport, monospace-everywhere, install-first, VHS-demo, single-page landing page that reads like a README. Built static, deployed to GitHub Pages, shipped under 150KB excluding media.

## Chunks

| # | File | What it covers | Lines |
|---|---|---|---|
| 1 | `ux-patterns.md` | Dev-tool landing page UX: install-first vs explainer-first, one-scroll-to-install, reading patterns, nav conventions, footer patterns, anti-patterns. Cites htmx, suckless, charm, oxide, tailscale, ripgrep, bat, eza, lazygit. | ~160 |
| 2 | `competitor-ux.md` | Product-level deep-dives of htmx.org, suckless.org, charm.sh, oxide.computer, tailscale.com. Pattern comparison matrix, strengths/weaknesses for each, "what metaphaze will do differently" section. | ~170 |
| 3 | `technical-research.md` | Next.js 14 App Router, Tailwind v4 `@theme` pattern, shadcn/ui CSS variable overrides, `next/font/google` for JetBrains Mono, server-component-only architecture, static export to GitHub Pages, bundle size targets. | ~180 |
| 4 | `accessibility-patterns.md` | WCAG 2.1 AA specifics: contrast ratios (validated 16.97:1 body), monospace readability, screen reader handling of ASCII box-drawing, keyboard nav, reduced-motion, color-scheme, semantic code markup, shadcn a11y preservation. | ~170 |
| 5 | `content-strategy.md` | README-as-marketing principle, headline rules (lowercase, concrete, short), manifesto drafts, install command context, refusals list as positioning asset, banned words, tone calibration, section copy drafts. | ~200 |
| 6 | `reference-specs.md` | Collected build-phase references: Next.js docs, Tailwind v4, shadcn/ui, next/font, VHS, asciinema, WCAG, Google Fonts, Berkeley Mono, prefers-reduced-motion, GitHub Pages deploy with Next.js. Each with URL + key takeaways + how it applies. | ~240 |
| 7 | `recommendations.md` | Synthesis. Adopt (12 patterns), Adapt (6 patterns), Avoid (15 anti-patterns), and six Key Decisions for the design phase (hero composition, demo asset, nav style, footer content, install placement, typography). | ~160 |

## Key Findings (TL;DR)

1. **Hero composition**: logo → manifesto → install → VHS recording, in that order, all above the second viewport. Mirrors htmx.org's proven four-part hero.
2. **Tech stack**: Next.js 14 App Router + Tailwind v4 (`@theme` in CSS, no config file) + shadcn/ui Button and Badge heavily restyled via CSS variable overrides (`--radius: 0` in particular).
3. **Fonts**: JetBrains Mono via `next/font/google` for MVP; Berkeley Mono via `next/font/local` for v1.1 if budget permits.
4. **Deploy target**: Static export to GitHub Pages, via `output: 'export'` in `next.config.js`.
5. **Accessibility**: WCAG AA comfortably met on contrast; ASCII box-drawing needs `aria-hidden` on inner `<pre>` + `aria-label` on wrapping `<figure>`.
6. **Content voice**: README is the marketing. One voice, shared between README and landing page. No rewrite. Banlist includes 40+ marketing words.
7. **VHS recording**: Output `.webm`, 1200x600, 30fps, <500KB, <10 seconds, autoplay-loop-muted. Theme colors match brand palette.
8. **Bundle target**: <150KB total excluding VHS media.

## Key Decisions for Design Phase

These six must be resolved first:

1. **Hero composition**: install-first (A) / demo-first (B) / demo-dominant (C). Recommend A.
2. **Demo asset format**: `.webm` (A) / asciinema (B) / GIF (C) / static (D). Recommend A.
3. **Nav style**: bracketed top nav (A) / no nav (B) / footer-only (C). Recommend A with 2-3 items.
4. **Footer content**: minimal (A) / +build info (B) / +contact (C). Recommend A with small build line.
5. **Install command placement**: hero (A) / dedicated section (B) / both (C). Recommend C.
6. **Typography system**: JetBrains only (A) / Berkeley only (B) / Berkeley+JetBrains (C). Recommend A for MVP, C for v1.1.

## Handoff Notes for Design Phase

1. The research deliberately includes rough copy drafts in `content-strategy.md` — treat them as seed material, not final text. The design phase can refine or replace them, but should stay within the voice constraints.
2. The "refusals list" is a brief-defined footer element but it's also the single most distinctive positioning move on the page. Do not treat it as decoration. It needs care in typography and copy.
3. Because the brief's implementation target is `code` (producing actual Next.js source files), the design phase can skip Figma and design directly in the browser. Consider setting up the Next.js scaffold first and iterating on the mockup in real CSS.
4. The VHS recording needs its own production step — it's not just a screenshot. Build a `.tape` file, iterate on the recording, commit the `.webm`. This work should start in parallel with design.
5. All six key decisions above should be answered before layout work begins — they're not design questions, they're product decisions that constrain the design.
6. Cross-reference `.design/branding/metaphaze/discover/mood-board-direction.md` for visual direction already locked in brand discovery. This research does not re-derive those visual choices.
7. The pre-fetched competitor research at the top of the task brief (Oxide + Tailscale summary) is correctly captured in `competitor-ux.md` — no gaps.

## Phase Status

Research phase: COMPLETE.

Next phase: `design` — produce a single-page mockup (Figma or direct Next.js code) using the six key decisions above as starting constraints.
