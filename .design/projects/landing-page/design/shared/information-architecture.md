# Information Architecture

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Site Structure

One page. One URL. No sub-pages in this phase.

```
/
└── landing page (app/page.tsx)
    ├── <header>  — nav bar
    ├── <main>
    │   ├── #hero         — logo, manifesto, install teaser, VHS recording
    │   ├── #what         — what it does (3-4 sentences + box-drawing diagram)
    │   ├── #why          — why it's different (comparison table)
    │   ├── #install      — full install block with verification
    │   └── #docs-link    — single bracketed nav item
    └── <footer>          — license, github, refusals list
```

Linked pages (exist elsewhere, not in scope):
- `[/docs]` — future docs site
- `[/source]` — GitHub repository

---

## Content Hierarchy

### Priority order (information scent for the Senior Operator)

1. **What is it + how do I install it** — hero section resolves both in one viewport
2. **What does it actually do** — #what section provides technical specificity
3. **Why not just use X** — #why section resolves the "vs other tools" question that will be in every reader's head
4. **Full install instructions + verification** — #install section for readers who want the complete setup flow
5. **Where to go next** — footer + docs link

### Grouping rationale

- **Hero above the fold** — the Senior Operator should not scroll before seeing the install command
- **What before Why** — explain function before positioning; avoids the "this sounds like marketing" trigger
- **Install section separate from hero teaser** — hero install is one-liner (quick copy), install section has full context (prerequisites, verification, post-install output). Serves both "I know what this is" and "walk me through it" visitors
- **Docs link and footer at the bottom** — readers who want more will scroll; readers who want to install immediately don't need to pass through docs to get there

---

## Content Inventory

### Hero
- Logo: `mz▌` — `mz` in `--mz-fg`, `▌` in `--mz-signal`, blink animation active
- Headline: `metaphaze` (logo serves as headline; no separate h1 text needed)
- Manifesto: "the orchestrator runs outside the loop. claude builds. mz drives."
- Install teaser: `$ cargo install --git https://github.com/mattmartinez/metaphaze` (copyable)
- VHS recording: `<video>` embed, autoplay, muted, loop, 1200×600, `<500KB`
- Phase transition screenshot: static `<pre>` below the video

### What it does
- Short description: 3-4 sentences. No marketing words. Functional language only.
- Box-drawing diagram: the orchestration loop in ASCII — shows mz as the driver, claude as the builder, the loop they form

### Why it's different
- Comparison table: mz vs. other harnesses. Bracketed badges (`[FIRST-PARTY]`, `[THIRD-PARTY]`, `[OK]`, `[ERR]`).
- Concrete properties compared: API access method, config file required, binary size, license, telemetry

### Install section
- Full install command (copyable)
- Prerequisites note: `cargo 1.75+`, `ANTHROPIC_API_KEY` set
- Verification: `mz --version` expected output
- Post-install: example of `mz status` output (real terminal output, not mock)

### Footer
- License: `[MIT]`
- Source: `[github.com/mattmartinez/metaphaze]`
- Refusals list: "no accounts · no cloud · no telemetry · no permission slips · no dashboards · no config files · no hallucinated toolchain"
- Manifesto (small, muted): "the orchestrator runs outside the loop. claude builds. mz drives."
- Build line: `build: v0.1.0 · 2026-04-08`

---

## Related

- [personas.md](./personas.md)
- [navigation.md](./navigation.md)
- [../screen-01-landing.md](../screen-01-landing.md)
