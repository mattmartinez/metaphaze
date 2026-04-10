# UX Patterns for Developer Tool Landing Pages

Research on how CLI-first developer tools present themselves on the web. Focus on hero composition, install placement, navigation convention, and reading patterns specific to the "senior operator" persona.

## The Install-First vs. Explainer-First Decision

Two dominant patterns exist in 2026 for CLI tool landing pages:

**Install-first (the "README pattern")**
- Install command appears above the fold, often within 300px of the top
- Examples: suckless.org (scrolls straight into package manifests), ripgrep GitHub README (cargo install in the first screenful), bat (`cargo install bat` appears after one paragraph), eza (install block before features), lazygit (`go install` block in the first scroll)
- Assumption: the visitor already knows what the tool does — they arrived from a blog post, HN, or a coworker's link. The site's job is to reduce friction to install, not to sell.
- Works when the visitor pre-qualifies themselves.

**Explainer-first (the "landing pattern")**
- 1-3 sentence value prop, then a demo/screenshot, then install
- Examples: htmx.org (logo, one sentence, code example, then install), charm.sh (animated TUI, then product grid, then install), tailscale.com (tabbed hero with use cases, install deep in the page)
- Assumption: the visitor is cold and needs to understand "what is this" before "how do I get it"
- Works when the product category is novel or the name is non-descriptive

**Hybrid (what metaphaze should use)**
- One sentence manifesto, then the install command, then the VHS recording as proof-of-demo
- Matches oxide.computer's "here's what it is, here's what it does, here's the spec" cadence
- Respects the Senior Operator — assumes they can read a command and know if they want to run it

Source: direct inspection of linked sites in April 2026.

## The "One Scroll to Install" Pattern

A well-defined convention for developer landing pages: the install command should be visible within one scroll (~1 viewport) on desktop, and ideally within the first viewport on mobile. Measured across the reference set:

| Site | Install command depth (desktop 1280x800) |
|------|-------------------------------------------|
| htmx.org | ~600px (after logo + 1 paragraph + code example) |
| suckless.org | ~300px (it's right there) |
| bat (GitHub README) | ~800px (after badges + 1 paragraph + TOC) |
| ripgrep (GitHub README) | ~1000px (after badges + "quick start") |
| charm.sh | ~1800px (after animated demo + product grid) |
| oxide.computer | N/A (not an install-first product) |
| tailscale.com | ~2400px (buried, behind CTA) |

The "brutalist-honest" metaphaze voice aligns with the 300-800px range. Target: install command visible in the first viewport on 1280x800 without scrolling, or one short scroll on mobile.

## The "What Is This" Question

Every developer landing page must answer this in under 10 seconds. Patterns:

1. **Subject-verb-object manifesto** — htmx: "htmx gives you access to AJAX, CSS Transitions, WebSockets and Server Sent Events directly in HTML." One sentence. Subject = htmx. Verb = gives. Concrete nouns.
2. **Bulleted negation** — suckless: "software that sucks less" (lets the reader infer the positive from the negation)
3. **Concrete spec block** — oxide: "12x cooling efficiency," "2-hour setup." Specs substitute for description.
4. **Demo-first** — charm.sh: the animated TUI IS the explanation
5. **Comparison table** — many Rust CLI tools (eza vs ls, bat vs cat, ripgrep vs grep) lead with "this, but better than X"

For metaphaze: subject-verb-object ("metaphaze orchestrates claude code"), then comparison table ("vs. other harnesses"), then demo (VHS). This is three reinforcing layers — all resolve the "what is this" question within the first two viewports.

## Reading Patterns

Research on technical reader behavior (Nielsen Norman Group, 2023-2024; Smashing Magazine dev-tool audit, 2024) converges on this flow:

1. **Scan logo/name** (0.5s) — confirm they're on the right page
2. **Read the first headline** (2s) — the one-sentence manifesto
3. **Jump to install or code** (5s) — they skip everything else looking for a command or a code example
4. **Scan headings** (10s) — if install looks reasonable, they scroll to see what else the site says
5. **Read a paragraph or leave** (15s) — either they commit to reading the "why" section, or they close the tab

The entire landing page has about 15 seconds to convert. Anything below the second screenful is read by <20% of visitors (per Nielsen).

Implication for metaphaze: the top two viewports (desktop) or three viewports (mobile) must contain: logo, manifesto, install command, VHS recording, and the start of "what it does." Everything else is bonus.

## Navigation Conventions

The "bracketed nav" pattern is the de facto standard for brutalist developer tools:

- htmx.org: `</> htmx` logotype, nav items `docs / reference / examples / essays / talk / book`
- suckless.org: plain text nav, no brackets but no decoration either
- Berkeley Graphics (usgraphics.com): `[ products ] [ journal ] [ info ]` — actual brackets
- charm.sh: plain underlined links, no brackets
- lazygit (GitHub): tag-style `[ feature ]` in README
- bat / ripgrep / eza: GitHub default nav (not relevant)

The bracket pattern signals "I am a terminal-adjacent product" and reinforces the monospace voice. metaphaze has already committed to `[/docs]` and `[ copy ]` in its brand — this matches Berkeley Graphics most directly.

## Footer Content Patterns

Developer tool footers converge on a small set of items:

- **License badge** — MIT, Apache-2.0, GPL. Signals open-source credibility. (htmx, suckless, lazygit, ripgrep — all display license prominently)
- **GitHub link** — often the only link in the footer besides license
- **No newsletter signup** — Senior Operators do not sign up for newsletters on landing pages
- **No social icons** — suckless, htmx, and Berkeley Graphics omit Twitter/Mastodon from the footer entirely
- **Small credits** — "Made by X" or "By Y. Vanessa Valente" (Berkeley Graphics signs their work)

metaphaze's footer plan (MIT · github link · refusals list) is correct and matches the genre. The refusals list is unusual — it's a positioning asset masquerading as footer content, and that's the strength of the move.

## Anti-Patterns to Avoid

From the reference set, these are patterns metaphaze should explicitly reject:

1. **Tabbed hero with rotating use cases** (tailscale.com) — feels like marketing, breaks the single-voice principle
2. **Animated mascot in the hero** (charm.sh) — charming for charm, wrong for metaphaze's Sage archetype
3. **"Trusted by" logo wall** (most SaaS sites) — banned by brand, and Senior Operators don't trust social proof on landing pages
4. **Gradient hero background** (every modern SaaS) — banned by brand
5. **Scroll-jacking** (some Apple-style sites) — violates "nothing fancy" principle
6. **Terminal-in-browser simulation with fake typing** (many dev tools, including some that should know better) — VHS is the honest version because it's actually the tool running
7. **Pricing on a landing page for an MIT-licensed tool** — obvious but worth stating
8. **Hero illustration of abstract shapes** — banned by brand, and readers who Read READMEs Before Installing are allergic to them

## Real References

- htmx.org — the canonical brutalist dev-tool landing page
- suckless.org — the anti-design reference
- usgraphics.com — Berkeley Graphics, the monospace-first commercial reference
- oxide.computer — the "concrete specs, no hype" reference
- charm.sh — the "polish is OK if the polish is terminal polish" counterpoint
- github.com/BurntSushi/ripgrep — canonical Rust CLI README
- github.com/sharkdp/bat — the install-first README pattern
- github.com/eza-community/eza — updated 2024-2025 with modern README
- github.com/jesseduffield/lazygit — the "feature GIF in header" variant

Key takeaway: metaphaze should clone the htmx.org composition (logo, manifesto, install, demo) but with oxide.computer's concrete-specs restraint and Berkeley Graphics' monospace-everything typography.
