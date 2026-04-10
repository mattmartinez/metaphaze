# Project Brief — metaphaze landing page

## Project
- **Name:** landing-page
- **Display name:** metaphaze landing page
- **Brand:** metaphaze (references `.design/branding/metaphaze/`)
- **Date:** 2026-04-10

## What we're building
A single-page marketing site for metaphaze, living at `metaphaze.dev` (or `mattmartinez.github.io/metaphaze`, whichever ships first). The page is the first thing a senior developer sees when they click a link from Hacker News, X, or a README reference.

The page is not a marketing funnel. It is the README rendered beautifully, with one hero asset (a real VHS recording of `mz auto` running) and the install command above the fold. The Senior Operator lands, scans, installs, closes the tab.

## Primary goal
Convert "curious senior dev" → "cargo install mz" in under 90 seconds.

## Persona
The Senior Operator (from brand BRIEF.md) — terminal-first, tired of hyped AI tools, wants to step away from the keyboard and come back to working code.

## Platforms
- Web only. Desktop-first (1280px), mobile-supported (640px). No native app.

## Tech stack
- Next.js 14 (app router)
- Tailwind v4 (using `metaphaze.yml` tokens)
- shadcn/ui with brand overrides (per `components/token-mapping.md`)
- JetBrains Mono from Google Fonts (Berkeley Mono self-hosted if/when licensed)
- Deployed on Vercel or GitHub Pages

## Implementation target
`code` — we're designing screens and producing a full Next.js project in this session's scope.

## Scope
The landing page is ONE page with these sections:
1. **Hero** — logo (`mz▌`), manifesto line, install command, VHS recording below
2. **What it does** — 3-4 sentences + one box-drawing diagram
3. **Why it's different** — bracketed-badge comparison vs other harnesses
4. **Install** — copy-paste `cargo install --git` block with real output
5. **Docs link** — single bracketed nav item `[/docs]`
6. **Footer** — MIT · github · the refusals list

No separate pricing page. No "features" grid with 12 cards. No testimonials. No newsletter signup. No cookie banner (no tracking = no cookies needed).

## Out of scope
- Documentation pages (a separate project)
- Blog or changelog page
- Any kind of dashboard, login, or interactive UI
- Analytics tracking (the brand forbids telemetry)
- Email capture
- Social media widgets
- "Trusted by" logo walls
- Video of a person talking

## Success criteria
- The page loads in under 500ms on a cold cache
- All text passes WCAG AA contrast (brand palette already validates)
- The page is self-contained HTML+CSS+one Google Fonts link and zero JavaScript (aspirational — Next.js may require JS, but the rendered output should work with JS disabled)
- The page reads identically in a desktop Chrome and in `lynx` (text-only browser)
- A senior dev landing here can tell in 2 seconds that metaphaze was made by someone who actually writes code

## Constraints
- Monospace-only typography
- `#0a0a0a` / `#ededed` / `#5fb878` palette, signal used on <1% of pixels
- No photos, no illustrations, no icon libraries
- Bracketed navigation (`[/about]`, `[/docs]`, `[/source]`)
- Manifesto preserved verbatim
- Lowercase voice throughout

## Notes
The brand is fully specified. This project inherits everything — tokens, typography, components, voice, constraints. The work here is: pick screens, identify gaps, adapt a handful of brand components for web-specific contexts, produce an install manifest for shadcn/ui primitives, and plan how the Next.js code maps to the brand system.
