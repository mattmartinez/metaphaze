# Screen 01 — Landing Page (`/`)

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Purpose

The single surface of the metaphaze marketing site. Converts "curious senior dev who clicked a link" into "running `mz` on their machine." Target conversion time: under 90 seconds.

## User Flow Position

Entry point. There are no prior screens — the visitor arrives here cold.

Exit paths:
1. Runs `cargo install --git ...` from the hero or install section (primary conversion)
2. Clicks `[/docs]` to read more (secondary)
3. Clicks `[/source]` to inspect the code on GitHub (secondary)

---

## Layout

Single narrow column. `max-w-3xl` (`48rem`) centered on viewport. Background `var(--mz-bg)` (`#0a0a0a` dark, `#fafafa` light). All sections share this column. Section spacing `py-16` to `py-24`.

### Section sequence (document order)

```
┌─ nav bar (fixed, full-width) ──────────────────────────────────────────┐
│  mz▌   [/docs]  [/source]                                              │
└────────────────────────────────────────────────────────────────────────┘

┌─ max-w-3xl centered column ────────────────────────────────────────────┐
│                                                                        │
│  §1  HERO (id="hero")                                                  │
│  §2  WHAT IT DOES (id="what")                                          │
│  §3  WHY IT'S DIFFERENT (id="why")                                     │
│  §4  INSTALL (id="install")                                            │
│  §5  DOCS LINK                                                         │
│                                                                        │
├─ footer (full-width, inside max-w-3xl) ────────────────────────────────┤
│  [MIT]  [github.com/...]  ·  no accounts · no cloud · ...              │
│  the orchestrator runs outside the loop. claude builds. mz drives.     │
│  build: v0.1.0 · 2026-04-08                                            │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Section 1 — Hero

**Purpose:** Logo → manifesto → install teaser → proof (VHS + phase transition). All above or just below the fold on a 1280×800 viewport.

### Layout detail

```
                    mz▌

  the orchestrator runs outside the loop.
  claude builds. mz drives.

  $ cargo install --git https://github.com/mattmartinez/metaphaze  [ copy ]

  ┌───────────────────────────────────────────────────────────────────────┐
  │  ╌ mz auto ╌                                                          │
  │                                                                       │
  │  [VHS recording — autoplay, muted, looped .webm, 1600×900]            │
  │                                                                       │
  └───────────────────────────────────────────────────────────────────────┘

  ┌─ what a phase transition looks like ──────────────────────────────────┐
  │                                                                       │
  │  ╔═══════════════════════════════════════════════════════════╗        │
  │  ║  metaphaze — phase 3/6 complete                          ║        │
  │  ║                                                          ║        │
  │  ║  ◆ brief         complete   2026-04-08 14:23             ║        │
  │  ║  ◆ research      complete   2026-04-08 14:31             ║        │
  │  ║  ◆ design        complete   2026-04-08 15:12             ║        │
  │  ║  ◇ critique      pending    —                            ║        │
  │  ║  ◇ build         pending    —                            ║        │
  │  ║  ◇ review        pending    —                            ║        │
  │  ║                                                          ║        │
  │  ║  src/                                                    ║        │
  │  ║  ├── main.rs                              [OK]            ║        │
  │  ║  ├── orchestrator.rs                      [OK]            ║        │
  │  ║  └── tui/                                                ║        │
  │  ║      └── theme.rs                         [OK]            ║        │
  │  ╚═══════════════════════════════════════════════════════════╝        │
  │                                                                       │
  └───────────────────────────────────────────────────────────────────────┘
```

### Components

- `<Cursor />` — the `▌` glyph after `mz`, blink active. `font-size: --text-5xl`. Signal green.
- `<p>` — manifesto line in `--text-base`, `font-weight: 400`, `--mz-fg`
- `<CodeBlock prompt="$" copyable={true} highlight="metaphaze">` — install teaser
- `<Pane title="╌ mz auto ╌">` — wraps the `<video>` element. `variant="idle"`.
- `<video>` — `autoplay muted loop playsinline`, `src="/vhs/demo-desktop.webm"`. Wrapped in `<picture>` with mobile source swap.
- `<Pane title="what a phase transition looks like">` — wraps `<PhaseTransitionScreen />`
- `<PhaseTransitionScreen />` — static `<pre>` with hardcoded phase output. `aria-hidden`, outer `<figure aria-label="example phase transition screen from the metaphaze CLI">`.

### Typography

- Logo: `mz` in `--mz-fg` + `▌` in `--mz-signal`, `font-size: --text-5xl` (clamp-responsive), `font-weight: 700`
- Manifesto: `--text-base`, `font-weight: 400`, `line-height: 1.6`, `--mz-fg`
- Install command: monospace, `--text-sm`, `--mz-fg`. The command is pre-formatted text. No syntax highlighting.
- VHS pane title: `--text-xs`, `--mz-fg-muted`
- Phase transition content: `--text-xxs` to `--text-xs`, `white-space: pre`

### Image resources

- **VHS recording:** type `terminal-recording`, source is a `.webm` produced by `charmbracelet/vhs` using the `demo.tape` file in the repo. Brand palette: `#0a0a0a` bg, `#ededed` fg, `#5fb878` cursor. Dimensions: 1600×900 desktop, 1200×600 mobile. Target size: <500KB. No overlay, no filter, no border-radius.
- **Phase transition screen:** type `ASCII/text`, inline `<pre>` element. Not an image. Brand typography.

---

## Section 2 — What It Does

**Purpose:** Three-sentence functional description. Box-drawing diagram of the orchestration loop. Answers "but what does it actually do?" without marketing language.

### Layout detail

```
  what it does
  ─────────────────────────────────────

  metaphaze is a command-line orchestrator that drives claude code
  autonomously through multi-phase software projects. you define the
  project in a yaml brief. metaphaze runs the phases — research,
  design, build, review — returning only when the work is done.

  the orchestrator runs outside the loop so you can step away
  from the keyboard and come back to working code.

  ┌─ the loop ────────────────────────────────────────────────────────────┐
  │                                                                       │
  │       you                                                             │
  │        │                                                              │
  │        ▼                                                              │
  │   ┌─────────┐        ┌──────────────────────────────────┐            │
  │   │   mz    │──────▶│  claude code                     │            │
  │   │ drives  │        │  builds                          │            │
  │   └─────────┘◀──────┤  (one phase at a time)           │            │
  │        │             └──────────────────────────────────┘            │
  │        ▼                                                              │
  │   phase output                                                        │
  │   (committed to disk)                                                 │
  │                                                                       │
  └───────────────────────────────────────────────────────────────────────┘
```

### Components

- `<h2>` — `what it does`, `--text-2xl`, `font-weight: 700`, `--mz-fg`, lowercase
- `<hr>` styled as `─` box-drawing rule (or `border-bottom: 1px solid --mz-border`, full width)
- `<p>` × 2 — prose, `--text-base`, `--mz-fg`, `line-height: 1.6`, `max-w: --prose-max` (approx 66ch)
- `<Pane title="the loop">` — wraps the ASCII diagram
- `<figure aria-label="orchestration loop diagram showing mz driving claude code through phases">` — outer wrapper
- `<pre aria-hidden="true">` — the actual ASCII diagram

### ASCII diagram spec

Box-drawing: `─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼ →`. No color inside the diagram. Renders in `--mz-fg` on `--mz-bg`. Single-line box characters only. Labels lowercase. No double-line borders in the diagram itself — only the `<Pane>` wrapper uses single-line borders.

Design to fit within 66 characters maximum width (readable at `--text-sm` on a 320px viewport with overflow-x-auto fallback).

---

## Section 3 — Why It's Different

**Purpose:** Answers "why not just use [other tool]?" with concrete properties, not marketing claims. Bracketed-badge comparison table.

### Layout detail

```
  why it's different
  ─────────────────────────────────────

  ┌─ mz vs. other harnesses ──────────────────────────────────────────────┐
  │                                                                        │
  │  property         mz               other harnesses                    │
  │  ─────────────────────────────────────────────────────────────────    │
  │  api access       [FIRST-PARTY]     [THIRD-PARTY]                     │
  │  config file      [OK] none         [ERR] required                    │
  │  binary size      [OK] 3.2 MB       [WARN] 40–200 MB                  │
  │  license          [OK] MIT          [WARN] proprietary / mixed        │
  │  telemetry        [OK] none         [ERR] yes                         │
  │  runs offline     [OK] yes          [ERR] cloud-dependent             │
  │                                                                        │
  └────────────────────────────────────────────────────────────────────────┘

  [FIRST-PARTY] = direct anthropic api. no wrapper, no proxy, no rate-limit reroute.
  [THIRD-PARTY] = routes through a vendor api. pricing and availability not yours.
```

### Components

- `<h2>` — `why it's different`, same typography as `what it does`
- `<hr>` styled divider
- `<Pane title="mz vs. other harnesses">` — wraps the `<ComparisonTable />`
- `<ComparisonTable />` — `<table>` with `scope="col"` headers. `<thead>` + `<tbody>`.
  - Column 1: property name (plain text, `--mz-fg-muted`)
  - Column 2: mz value (StatusBadge + text)
  - Column 3: other harnesses value (StatusBadge + text)
- `<StatusBadge variant="first-party">` — renders `[FIRST-PARTY]` in `--mz-signal`
- `<StatusBadge variant="third-party">` — renders `[THIRD-PARTY]` in `--mz-error`
- `<StatusBadge variant="ok">` — `[OK]` in `--mz-signal`
- `<StatusBadge variant="warn">` — `[WARN]` in `--mz-warn`
- `<StatusBadge variant="error">` — `[ERR]` in `--mz-error`
- `<p>` × 2 — legend below the table explaining `[FIRST-PARTY]` and `[THIRD-PARTY]`

### Table behavior

Mobile: stacks to labeled rows. Per `responsive.md`. Each property becomes a two-row group:
```
api access
  mz:             [FIRST-PARTY]
  other:          [THIRD-PARTY]
```

Desktop: standard horizontal `<table>`, `border-collapse: collapse`, cell borders `1px solid --mz-border`. No alternating row colors — flat surface only.

---

## Section 4 — Install

**Purpose:** Complete installation instructions for readers who want the full setup, not just the one-liner in the hero.

### Layout detail

```
  install
  ─────────────────────────────────────

  prerequisites

  › cargo 1.75 or later
  › ANTHROPIC_API_KEY set in your environment

  ┌─ install ─────────────────────────────────────────────────── [ copy ] ┐
  │  $ cargo install --git https://github.com/mattmartinez/metaphaze      │
  └────────────────────────────────────────────────────────────────────────┘

  ┌─ verify ──────────────────────────────────────────────────────────────┐
  │  $ mz --version                                                       │
  │  metaphaze 0.1.0                                                      │
  └────────────────────────────────────────────────────────────────────────┘

  ┌─ first run ───────────────────────────────────────────────────────────┐
  │  $ mz status                                                          │
  │  [OK]  api key found                                                  │
  │  [OK]  rust toolchain: cargo 1.78.0                                   │
  │  [OK]  claude code: available                                         │
  │  ●     ready                                                          │
  └────────────────────────────────────────────────────────────────────────┘
```

### Components

- `<h2>` — `install`, same typography
- `<hr>` divider
- `<p>` — `prerequisites`, `--text-sm`, `font-weight: 600`
- `<ul>` — two list items, `list-style: none`, each line prefixed with `›` in `--mz-fg-muted`
- `<CodeBlock prompt="$" copyable={true} highlight="metaphaze">` — install command, `copyable={true}`, `pane title="install"`
- `<CodeBlock prompt="$" copyable={false}>` — verify command + output, `pane title="verify"`
- `<CodeBlock prompt="$" copyable={false}>` — first-run example, `pane title="first run"`. The `[OK]` tokens inside this code block should match the brand badge colors. Note: these are inline text inside a `<pre>`, not `<StatusBadge />` components — they share the color but not the HTML structure.

### Copy behavior

When the user clicks `[ copy ]`:
1. `navigator.clipboard.writeText("cargo install --git https://github.com/mattmartinez/metaphaze")`
2. Button label changes: `copy` → `copied`
3. After 1500ms: reverts to `copy`
4. No toast. No animation. Discrete state change.

---

## Section 5 — Docs Link

**Purpose:** Single exit point for readers who want more depth. Not a CTA — it's a reference.

### Layout detail

```
  ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─ · ─

  [/docs]  →  full documentation, api reference, and examples

```

### Components

- `<hr>` — em-dash rule (` — ` or `· — ·` repeat), or `border-top: 1px dashed --mz-border`
- `<p>` — `[/docs]` as a nav-style link (`<a>` with `.mz-nav a` class), then `→` separator, then description in `--mz-fg-muted`

This section is intentionally minimal. It does not have a heading.

---

## Footer

**Purpose:** License, source code link, the "refusals list" (positioning asset), and the manifesto as a closing statement.

### Layout detail

```
  ─────────────────────────────────────────────────────────────────────────

  [MIT]  ·  [github.com/mattmartinez/metaphaze]  ·  [generated with gsp]

  no accounts · no cloud · no telemetry · no permission slips ·
  no dashboards · no config files · no hallucinated toolchain

  the orchestrator runs outside the loop. claude builds. mz drives.

  build: v0.1.0 · 2026-04-08
```

### Components

- `<footer>` semantic element, `border-top: 1px solid --mz-border`, `py-12`
- Line 1: `<a>` links styled as nav items but without brackets on `MIT` — or use `<StatusBadge>` style `[MIT]`. GitHub link as bracketed text. `[generated with gsp]` links to the GSP repo.
- Line 2: refusals list as `<p>`, `--text-sm`, `--mz-fg-muted`. Separator: `·` with spaces.
- Line 3: manifesto in `--text-sm`, `font-weight: 400`, `--mz-fg-muted`
- Line 4: build line in `--text-xs`, `--mz-fg-muted`, `opacity: 0.7`

### Copy

Refusals list (verbatim): "no accounts · no cloud · no telemetry · no permission slips · no dashboards · no config files · no hallucinated toolchain"

This is positioning, not decoration. It tells the Senior Operator what they WON'T encounter. Do not abbreviate or reorder.

---

## States

### Default

As designed above. Dark mode. VHS autoplay. Cursor blinking.

### JS Disabled

- VHS `<video>` renders as a static first frame (browser default). The `<video>` element falls back to the `<source>` element's first-frame poster.
- `[ copy ]` button disappears (or renders without click handler — use `nojs:hidden` conditional in the server component).
- All other content renders identically — the page is static HTML+CSS.
- Nav links work. Section anchors work. All text is readable.

**Spec note:** The copy button is the only interactive element. Use conditional rendering: in Server Component mode, output only a `<kbd>` element showing the command; in Client Component mode, output the interactive button. Or: render the button always, but it degrades gracefully — clicking does nothing if JS is off, and the text is still copy-pasteable.

### Light Mode (`prefers-color-scheme: light`)

All colors invert per CSS custom properties. No layout change. VHS recording's background matches `#fafafa` in light mode — the recording itself will not match unless a light-mode variant is produced. Acceptable to ship with dark-only recording in v1.0 with a `[WARN]` noted in the design.

### `prefers-reduced-motion`

- Cursor blink stops (cursor visible, static)
- VHS autoplay removed — user clicks to play
- Copy button state change: instant, no transition

---

## Interactions

Per `shared/micro-interactions.md`:
- `cursor-blink` on logo and nav (ambient)
- `video-invert` on `[ copy ]` button hover
- `underline-reveal + > prefix` on nav links hover
- Copy feedback: text-swap `copy → copied` (1500ms)

No scroll-triggered effects. No entrance animations.

---

## Accessibility

### Semantic HTML

```html
<html lang="en">
  <head>
    <title>metaphaze — the orchestrator runs outside the loop</title>
    <meta name="description" content="mz drives claude code autonomously through multi-phase software projects. cargo install --git https://github.com/mattmartinez/metaphaze">
  </head>
  <body>
    <nav aria-label="primary navigation">...</nav>
    <main>
      <section id="hero" aria-label="hero">...</section>
      <section id="what" aria-label="what it does">...</section>
      <section id="why" aria-label="why it's different">...</section>
      <section id="install" aria-label="install">...</section>
    </main>
    <footer>...</footer>
  </body>
</html>
```

### Focus management

- Tab order: nav logo → nav links → hero install copy button → main content
- Skip link: `<a class="sr-only focus:not-sr-only" href="#main">skip to content</a>` as the first element in `<body>`
- All `<a>` and `<button>` elements: min 44px touch target (per Apple HIG). The bracketed button renders inline with text, so enforce `padding: 8px 0` minimum.
- Focus visible: `outline: 2px solid var(--mz-signal)` on all interactive elements. (Override the `--ring: transparent` for keyboard focus only — use `:focus-visible` not `:focus`.)

### ASCII diagrams and box-drawing

- All `<pre>` elements containing diagrams: `aria-hidden="true"`
- Wrapping `<figure>` has `aria-label` with a plain English description
- Phase transition screen: `<figure role="img" aria-label="example phase transition screen showing 6 project phases with 3 complete and 3 pending">`
- Comparison table: standard `<table>` with `<caption>`, `scope="col"` on headers

### VHS video

```html
<video
  aria-label="metaphaze CLI demonstration — autonomous Rust project build"
  autoplay muted loop playsinline
  width="1600" height="900">
  <source src="/vhs/demo-desktop.webm" type="video/webm">
  <p>Terminal recording of mz auto running through a Rust project.</p>
</video>
```

Autoplay is muted per spec. `playsinline` for iOS. Fallback text for users on unsupported browsers.

### Contrast

Body text `--mz-fg` (#ededed) on `--mz-bg` (#0a0a0a): ratio 16.97:1 — far exceeds WCAG AA (4.5:1) and AAA (7:1).

Muted text `--mz-fg-muted` (#8a8a8a) on `--mz-bg` (#0a0a0a): ratio 5.21:1 — passes AA for normal text.

Signal text `--mz-signal` (#5fb878) on `--mz-bg` (#0a0a0a): ratio 7.01:1 — passes AAA.

### Screen reader order

VoiceOver / NVDA will read the page in document order:
1. Navigation links
2. Logo heading
3. Manifesto
4. Install command (as a code element)
5. VHS video (announced as "video, metaphaze CLI demonstration")
6. Phase transition (skipped — `aria-hidden`) / figure label read
7. Section headings in order
8. Table with column headers for comparison
9. Code blocks as `<code>` elements (announced as code)
10. Footer links, refusals list, manifesto

---

## Related

- [shared/personas.md](./shared/personas.md)
- [shared/information-architecture.md](./shared/information-architecture.md)
- [shared/navigation.md](./shared/navigation.md)
- [shared/micro-interactions.md](./shared/micro-interactions.md)
- [shared/responsive.md](./shared/responsive.md)
- [shared/component-plan.md](./shared/component-plan.md)
- Brand cursor: `.design/branding/metaphaze/patterns/components/cursor.md`
- Brand pane: `.design/branding/metaphaze/patterns/components/pane.md`
- Brand button: `.design/branding/metaphaze/patterns/components/bracketed-button.md`
- Brand badge: `.design/branding/metaphaze/patterns/components/status-badge.md`
- Brand phase transition: `.design/branding/metaphaze/patterns/components/phase-transition.md`
