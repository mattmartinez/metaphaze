# Scope

> Phase: brief | Project: landing-page | Generated: 2026-04-10

---

## What we're building

One page. `metaphaze.dev` (or a GitHub Pages equivalent). The first thing a senior developer sees after clicking a link from HN, X, or a README reference. It renders in 500ms, communicates the whole product in 90 seconds, and ends in `cargo install --git https://github.com/mattmartinez/metaphaze`.

## Screen list

One screen. One page. No navigation. No sub-pages (yet).

| # | Screen | Priority | Purpose |
|---|---|---|---|
| 01 | Landing page (`/`) | P0 | Sole surface — hero, explanation, install, docs link, footer |

Sections within the single screen, in document order:

1. **Hero** — `mz▌` logo, manifesto, install command, VHS recording
2. **What it does** — 3-4 sentence explainer + one box-drawing diagram
3. **Why it's different** — bracketed-badge comparison row vs other harnesses
4. **Install** — copy-paste `cargo install --git` block with real post-install output
5. **Footer** — MIT · github link · the refusals list

## Component scope

Pulls from `{BRAND_PATH}/patterns/components/`:

| Component | Source | Why it's needed |
|---|---|---|
| `cursor.md` | brand | The hero logo — `mz▌` with blinking U+258C in signal green |
| `pane.md` | brand | The VHS hero frame, the install block wrapper, the "why it's different" comparison box |
| `bracketed-button.md` | brand | "[ copy ]" buttons next to code blocks |
| `prompt-input.md` | brand | None used directly, but the install command uses the `$` prompt prefix inherited from this pattern |
| `status-badge.md` | brand | The comparison row: `[FIRST-PARTY]`, `[THIRD-PARTY]`, `[OK]`, `[ERR]` |
| `phase-transition.md` | brand | Referenced as a static screenshot below the VHS — "what a phase transition looks like" |
| `token-mapping.md` | brand | Tailwind v4 `@theme` block + shadcn CSS variables, pasted into the Next.js `app/globals.css` |

## Out of scope

- Separate documentation pages
- Blog / changelog page
- Dashboards, login, interactive UI
- Analytics or tracking (brand forbids telemetry)
- Email capture or newsletter signup
- Social share widgets
- "Trusted by" logo walls
- Testimonials
- Cookie banner (no cookies → no banner)
- Video of a person talking
- JavaScript beyond what Next.js requires for rendering (zero custom JS)

## Success criteria

- **Load time:** under 500ms on a cold cache over a typical broadband connection
- **Contrast:** all text passes WCAG AA (brand palette already validates)
- **JS-disabled rendering:** the page reads correctly with JavaScript disabled (no interactive hover states break the layout)
- **`lynx` compatibility:** the page renders acceptably in `lynx` — the Senior Operator's text-only browser test
- **Scan time:** a senior developer landing on the page can identify what metaphaze is and how to install it within 2 seconds
- **No broken links:** every link works and is either internal or to a canonical source (github.com, docs.anthropic.com)

## Dependencies

- **Brand system** — fully specified in `.design/branding/metaphaze/`. No gaps.
- **VHS recording** — a real `mz auto` recording needs to be captured before launch. Captured with `charmbracelet/vhs` using the theme from `identity/imagery-style.md`. This is a content dependency, not a code dependency.
- **Berkeley Mono license** — optional. The site ships with JetBrains Mono from Google Fonts by default. If the maintainer buys a Berkeley Mono license, the `.woff2` files drop into `public/fonts/` and the `@font-face` declaration activates.
- **Domain** — optional. The site works on `mattmartinez.github.io/metaphaze` without a custom domain.

## Issue framing

This project is already scoped as **the tightest possible landing page**. Ship it as **one pull request** in the metaphaze repo (or a companion `metaphaze-www` repo — maintainer's call).

### Suggested bounded issues (if the maintainer wants to break it up)

| Issue | Deliverable |
|---|---|
| `landing: scaffold Next.js 14 project with brand tokens` | `app/globals.css` with the `@theme` block, `app/layout.tsx` with font loading, a placeholder `app/page.tsx` |
| `landing: hero section with cursor logo and manifesto` | The top of the page — logo, manifesto, install command. Zero scroll position. |
| `landing: VHS hero embed and "what a phase transition looks like" static` | Drop in the VHS `.webm` and a static phase transition image, wire up the `pane.md` frame |
| `landing: comparison row (first-party vs third-party)` | The "why it's different" section with bracketed status badges |
| `landing: install section with copy-to-clipboard button` | The install code block + bracketed copy button |
| `landing: footer with refusals list` | MIT license, github link, the "things metaphaze will never have" list |
| `landing: ship to vercel or github pages` | Deploy, verify WCAG, verify `lynx`, verify cold-cache load time |

**Recommendation:** ship as one PR if the maintainer has a 90-minute block of focused time. Split into the 7 issues above if landing-page work will be interleaved with `mz` development.

## Project boundaries

### What we're designing
- One Next.js page (`app/page.tsx`)
- Global CSS + Tailwind v4 theme (`app/globals.css`)
- Layout wrapper with font loading (`app/layout.tsx`)
- 7 React components that wrap the brand patterns for web use

### What we're NOT designing
- The metaphaze Rust CLI (that's a separate project — `mz-refresh`, P013 of the metaphaze tool)
- The GitHub README (separate project — `readme-redesign`)
- OG / social preview images (separate project — `og-images`)
- Any form of backend, API, or CMS

### What we're reusing from the brand system
Everything. The entire `patterns/` directory. The project phase does not invent new tokens or new patterns — it adapts existing ones for the single-page web context.
