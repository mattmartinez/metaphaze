# Synthesis and Recommendations

Research synthesis across the six research chunks. Organized as Adopt / Adapt / Avoid, followed by the key decisions the design phase must make first.

## Adopt (Use Directly)

These patterns should be used with no modification, because they match metaphaze's brand and constraints:

1. **htmx.org's four-part hero composition** — logo, manifesto, code example, install command. Source: `ux-patterns.md`, `competitor-ux.md`. The cleanest hero in the reference set; matches metaphaze's section plan exactly.

2. **charm.sh's VHS recording as primary demo asset** — VHS was literally invented by the people who ship terminal-polish tooling; `charm.sh` proves that an animated TUI in the hero is the single best asset for a CLI landing page. Source: `competitor-ux.md`, `reference-specs.md`.

3. **Berkeley Graphics' monospace-everywhere typography** — headlines, body, nav, code, all in one typeface. usgraphics.com is the canonical commercial reference. Source: `ux-patterns.md`.

4. **Tailwind v4 `@theme` pattern with zero JS config** — no `tailwind.config.js`, all tokens in CSS variables, shadcn overrides at `:root`. Source: `technical-research.md`.

5. **Next.js App Router with Server Components by default** — one `'use client'` directive total (copy button), everything else static. Bundle stays under 100KB. Source: `technical-research.md`.

6. **`next/font/google` for JetBrains Mono** — auto self-hosts, zero Google runtime calls, privacy-clean. Source: `technical-research.md`, `reference-specs.md`.

7. **Static export to GitHub Pages** — matches the "no third-party infra on the critical path" principle; the landing page lives in the same repo as the Rust CLI. Source: `technical-research.md`.

8. **WCAG AA-first approach with 16.97:1 body contrast** — already validated; no additional accessibility work needed for contrast. Source: `accessibility-patterns.md`.

9. **`aria-hidden` on decorative ASCII + `aria-label` on the container** — the only correct way to make box-drawing diagrams screen-reader-friendly. Source: `accessibility-patterns.md`.

10. **README voice, verbatim on the page** — no rewrite from README voice to "landing page voice." One source of truth. Source: `content-strategy.md`.

11. **Bracketed nav and bracketed buttons as the brand signature** — `[/docs]`, `[ copy ]`. Matches Berkeley Graphics precedent; reinforces the terminal metaphor. Source: `ux-patterns.md`, `content-strategy.md`.

12. **Concrete specs comparison table (oxide pattern) for "why it's different"** — numeric, bracketed, no adjectives. Source: `competitor-ux.md`, `content-strategy.md`.

## Adapt (Modify for Project)

These patterns need adjustment to fit metaphaze's specific constraints:

1. **suckless.org's refusal to sell, BUT with entry-level accessibility** — suckless is hostile to newcomers; metaphaze can be terse without being hostile. Adapt: keep the refusal-to-sell energy but include a 16-word manifesto that explains what it is.

2. **Oxide's concrete-specs band, BUT without the enterprise framing** — oxide shows "$X/kWh" and "12x efficiency"; metaphaze should show "3MB binary," "MIT license," "no config file." Same pattern, different specs. Source: `competitor-ux.md`.

3. **htmx's single-sentence explainer, BUT lowercase** — htmx uses title case ("high power tools for HTML"); metaphaze is lowercase across the board. Source: `content-strategy.md`.

4. **Berkeley Graphics' bracketed nav, BUT with leading-slash URL pattern** — Berkeley Graphics uses `[ products ]`; metaphaze uses `[/docs]` (file-path style) to reinforce the CLI metaphor. Source: `ux-patterns.md`.

5. **VHS recording at hero position, BUT short and autoplay-friendly** — charm.sh has 20+ second TUI demos; metaphaze's should be <10 seconds, 1200x600, <500KB, autoplay loop. Source: `reference-specs.md`.

6. **shadcn/ui Button and Badge, BUT heavily restyled via CSS overrides** — don't fight shadcn's accessibility (keep semantic HTML), but override all visual tokens to zero. Source: `technical-research.md`, `accessibility-patterns.md`.

## Avoid (Anti-Patterns)

These patterns appear in the reference set but are wrong for metaphaze:

1. **Tabbed hero with rotating use cases (tailscale.com)** — breaks single-voice principle, feels like marketing, hides the actual content. Source: `competitor-ux.md`.

2. **GUI screenshots instead of terminal recordings (tailscale.com)** — contradicts metaphaze's TUI-first principle. Source: `competitor-ux.md`.

3. **"Developer approved" testimonials / customer logo walls (tailscale, oxide)** — banned by brand; Senior Operators don't trust social proof on landing pages. Source: `competitor-ux.md`, `content-strategy.md`.

4. **Newsletter signup or email capture (charm.sh)** — explicitly out of scope per brief. Source: `competitor-ux.md`.

5. **Mascot or illustrated character (charm.sh)** — charming for charm, wrong for metaphaze's Sage archetype. Source: `competitor-ux.md`.

6. **Multi-viewport modular storytelling (oxide.computer)** — too long for metaphaze; 3-4 viewports is the target. Source: `competitor-ux.md`.

7. **Gradient hero backgrounds (every SaaS)** — banned by brand. Source: brand BRIEF.md.

8. **Backdrop-filter / glassmorphism** — banned by brand. Source: brand BRIEF.md.

9. **Border-radius on any element** — `--radius: 0` in CSS variables; hard rule. Source: `technical-research.md`.

10. **Marketing verbs (agentic, empower, transform, unlock, leverage, journey, ecosystem)** — banned by brand; additional banlist in `content-strategy.md`. Source: `content-strategy.md`.

11. **First-person "we believe" voice (htmx.org)** — metaphaze is imperative/declarative, not confessional. Source: `content-strategy.md`.

12. **Cookie banner, analytics widget, chat bubble** — explicitly out of scope per brief. Source: project brief.

13. **Hero illustrations of abstract shapes, gradient meshes, 3D renders (every modern SaaS)** — banned by brand. The VHS recording is the only visual. Source: brand BRIEF.md, `competitor-ux.md`.

14. **CDN tab pattern for install (htmx.org)** — metaphaze has one install path (`cargo install`), no need for tabs. Source: `ux-patterns.md`.

15. **Scroll-jacking, parallax, animated section reveals** — violates "nothing fancy" principle. The only animation is the cursor blink. Source: brand BRIEF.md, `accessibility-patterns.md`.

## Key Decisions for the Design Phase

The five decisions that will most affect the design output. These should be resolved first in the design phase before any layout work begins.

### Decision 1: Hero Composition

**Options:**
- A. Logo → manifesto → install command → VHS recording (install-first)
- B. Logo → manifesto → VHS recording → install command (demo-first)
- C. Logo → VHS recording → manifesto → install command (demo-first, demo-dominant)

**Recommendation: A.** Install-first matches the Senior Operator persona — they already know what they want, the VHS is proof, the install command is the conversion. Option B is acceptable if the demo is <8 seconds. Option C is wrong because it buries the manifesto.

### Decision 2: Demo Asset

**Options:**
- A. Static `.webm` via `<video autoplay loop muted>`
- B. Interactive asciinema player
- C. Animated GIF
- D. Static screenshot + link to video

**Recommendation: A.** Matches the stack (no JS), smallest bundle impact, respects `prefers-reduced-motion` via CSS. Asciinema is overkill; GIF is too large; static screenshot is weak. Target: 1200x600, 30fps, <500KB.

### Decision 3: Nav Style

**Options:**
- A. Top bar with bracketed items `[/docs] [/source] [/changelog]`
- B. No nav (single page, scroll to sections)
- C. Footer-only nav

**Recommendation: A.** Even though metaphaze is single-page, a top nav to `[/docs]` (the actual docs site) and `[/source]` (GitHub) is expected by visitors. Keep it to 2-3 items max. The bracket style is the brand signature.

### Decision 4: Footer Content

**Options:**
- A. MIT license, GitHub link, refusals list (per brief)
- B. A + build info (version, commit hash, build time)
- C. A + contact email

**Recommendation: A with a single concession — include a small "build: v0.1.0 · 2026-04-08" text.** The refusals list is the main footer asset; the license and GitHub link frame it. Build info reinforces "this is an engineering artifact." No contact email.

### Decision 5: Install Command Placement

**Options:**
- A. In the hero, above the fold
- B. In its own section, one scroll down
- C. In both places

**Recommendation: C.** Short install teaser in the hero (`cargo install metaphaze` with a `[ copy ]` button), then a full install section further down with prerequisites, verification command, and post-install output. This serves both "I already know what this is" and "I want the full instructions" visitors.

### Decision 6: Typography System

**Options:**
- A. JetBrains Mono only (free, via `next/font/google`)
- B. Berkeley Mono (purchased, self-hosted)
- C. Berkeley Mono with JetBrains Mono fallback

**Recommendation: A for MVP, C for v1.1.** Ship JetBrains Mono first to avoid the purchase blocker; upgrade to Berkeley Mono when the site has proven itself. Both are sharp, both are monospace, both work. Berkeley is the aesthetic upgrade; JetBrains is the pragmatic ship.

## Summary

Research converges on a clear picture: the metaphaze landing page is a 3-4 viewport, monospace-everywhere, dark-background, VHS-hero, install-first, single-source-of-truth page that reads like its own README. Build with Next.js 14 App Router + Tailwind v4 + shadcn/ui (heavily restyled), static-export to GitHub Pages, ship under 150KB excluding the VHS. No marketing voice, no social proof, no illustrations beyond box-drawing, no animations beyond the cursor blink.

The design phase should start by resolving the six key decisions above and producing a single-page mockup in Figma or directly in code (given the brief's implementation target is `code`, consider skipping Figma and designing in the browser).
