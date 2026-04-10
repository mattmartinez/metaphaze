# Typography

> Phase: identity | Brand: metaphaze | Generated: 2026-04-10

---

## TL;DR

One typeface. Berkeley Mono (preferred) or JetBrains Mono (free fallback). Headlines, body, nav, code, TUI, README — same face, weighted differently. Three weights: 400 / 600 / 700. Modest Major Third scale (1.25), anchored at 16px, display caps at ~48px. Body line-height 1.6, display line-height 1.1. Lowercase everywhere. No italics.

---

## Primary Typeface

### Berkeley Mono — recommended

Berkeley Mono by Neil Panchal / U.S. Graphics Company. Paid license (~$75 personal, ~$300+ commercial), single-purchase, worth it for a brand that will outlive the purchase by a decade. It has the typographic refinement to carry a landing page, not just a code editor. The open counters, the assertive `g`, the mechanical but humane `a` — these are brand-grade letterforms. It is the font Berkeley Graphics uses to sell a typeface, which is the strongest possible argument for its marketing legibility.

### JetBrains Mono — acceptable fallback

JetBrains Mono, SIL Open Font License, free. If the license budget is $0, JetBrains Mono is the safest single-monospace choice. Excellent hinting, wide weight family, broad platform coverage, no brand baggage. Less distinctive than Berkeley, but it will never look cheap.

**Operational default:** ship Berkeley Mono if the maintainer buys the license, ship JetBrains Mono if they don't. Both produce a correct metaphaze.

---

## Font Loading

### CSS @font-face (self-hosted Berkeley Mono)

```css
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Regular.woff2") format("woff2");
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-SemiBold.woff2") format("woff2");
  font-weight: 600;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Bold.woff2") format("woff2");
  font-weight: 700;
  font-style: normal;
  font-display: swap;
}
```

### JetBrains Mono fallback (Google Fonts)

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600;700&display=swap" rel="stylesheet">
```

### Font stack (the one stack, everywhere)

```css
font-family:
  "Berkeley Mono",
  "JetBrains Mono",
  ui-monospace,
  SFMono-Regular,
  "SF Mono",
  Menlo,
  Consolas,
  monospace;
```

If both brand faces fail to load, the browser's monospace default is acceptable. A sans-serif fallback is not.

---

## Why One Typeface

The Sage archetype refuses performance. Using two typefaces is a performance. Using one typeface says "we committed" — and commitment is the whole personality.

If the metaphaze landing page renders the headline in Inter and the command examples in JetBrains Mono, the Senior Operator thinks "another Vercel-shaped dev tool." If everything is one monospace — headline, body, nav, code — the Senior Operator thinks "someone actually committed." The second reaction is what the brand is buying.

---

## Weight Strategy

Three weights. Predictable uses.

| Weight | When | Why |
|---|---|---|
| **Regular (400)** | Body, code, TUI output, nav, `--help` text, almost everything | The default. Precision lives here. |
| **Semibold (600)** | Section headings (H1–H3), status-line labels, the CLI command prefix in callouts | One step up. Hierarchy without volume. |
| **Bold (700)** | Display headline only. One per page max. | The loudest thing the brand says, and it is still a monospace at 48px. |

No light/thin weights. No black/extra-bold. No italics (see below).

---

## Type Scale

**Ratio:** Major Third (1.25). Balanced, functional, avoids poster drama.
**Base:** 16px (1rem).
**Range:** 11 steps from metadata (12.8px) to display (48.83px).

### Scale table

| Token | Role | Size (px) | Size (rem) | Weight | Line-height | Tracking | Use |
|---|---|---|---|---|---|---|---|
| `mz-text-xxs` | Metadata | 12.80 | 0.80 | 400 | 1.5 | 0.01em | Timestamps, footnote markers, legal footers |
| `mz-text-xs` | Small | 13.00 | 0.8125 | 400 | 1.5 | 0.01em | Flag descriptions, secondary UI text |
| `mz-text-sm` | Body compact | 14.40 | 0.90 | 400 | 1.55 | 0 | Dense tables, status lines |
| `mz-text-base` | Body | 16.00 | 1.00 | 400 | 1.6 | 0 | Default body, README, code blocks |
| `mz-text-lg` | Body lead | 18.00 | 1.125 | 400 | 1.55 | 0 | Subtitles, hero subhead, emphasized paragraphs |
| `mz-text-xl` | H3 | 20.00 | 1.25 | 600 | 1.4 | 0 | Subsection headings |
| `mz-text-2xl` | H2 | 25.00 | 1.5625 | 600 | 1.3 | 0 | Section headings |
| `mz-text-3xl` | H1 | 31.25 | 1.9531 | 600 | 1.2 | -0.005em | Page titles |
| `mz-text-4xl` | Display | 39.06 | 2.4413 | 700 | 1.15 | -0.01em | Hero headline (smaller contexts) |
| `mz-text-5xl` | Display large | 48.83 | 3.0518 | 700 | 1.1 | -0.015em | Hero headline (desktop), manifesto line |

Body and code live at nearly the same size because they ARE nearly the same thing — both are monospace, both are the brand voice. The distinction is the `mz.slate` background behind code blocks, not a typographic shift.

### Fluid type (clamp formulas)

Viewport range: 320px → 1280px.

```css
/* Body anchors — locked, no fluid behavior */
--mz-text-xxs:   0.80rem;   /* 12.80px */
--mz-text-xs:    0.8125rem; /* 13.00px */
--mz-text-sm:    0.90rem;   /* 14.40px */
--mz-text-base:  1.00rem;   /* 16.00px */
--mz-text-lg:    1.125rem;  /* 18.00px */

/* Headings — modest fluid behavior to keep monospace grid honest */
--mz-text-xl:    clamp(1.125rem, 0.96rem + 0.83vw, 1.25rem);   /* 18 → 20 */
--mz-text-2xl:   clamp(1.25rem,  1.04rem + 1.04vw, 1.5625rem); /* 20 → 25 */
--mz-text-3xl:   clamp(1.5625rem, 1.25rem + 1.56vw, 1.9531rem); /* 25 → 31.25 */
--mz-text-4xl:   clamp(1.9531rem, 1.56rem + 1.95vw, 2.4413rem); /* 31.25 → 39.06 */
--mz-text-5xl:   clamp(2.4413rem, 1.95rem + 2.44vw, 3.0518rem); /* 39.06 → 48.83 */
```

The body-range tokens are not fluid — monospace reading wants a fixed character width at every viewport. Only display sizes scale with viewport, and even then the range is narrow (48.83px → 39.06px on mobile, not 48 → 24). Brutalist does not mean dramatic.

---

## Scale Direction

**Modest scale. Brutalist does not mean giant — it means honest.**

- Body anchored at 16px. The Senior Operator reads a lot of text; body size has to survive a 40-minute README session without fatigue.
- Display caps at 48.83px. The hero headline is big enough to be the hero and small enough to not look like a landing-page cliché.
- Body and code live at nearly the same size (16px) because they are nearly the same thing.
- Small text (13px) exists for metadata only. The Sage respects the metadata layer but does not elevate it.
- The jump between sizes is modest — 1.25x ratio. This is the opposite of a poster aesthetic.

Hierarchy is established by **weight** and **whitespace** as much as by size. A 20px semibold heading next to 16px regular body reads as a clear hierarchy without needing a 3x size jump.

---

## Line-Height & Vertical Rhythm

Base line-height: **1.6** for body text (16px → 25.6px line-height). Generous enough for long-form reading in monospace.

Display line-height: **1.1** for 48.83px headlines (→ 53.7px line-height). Tight enough to make the manifesto feel like a block of statement, not a paragraph.

### Rhythm grid

Vertical rhythm anchored to an **8px baseline grid**. Every type size snaps to a multiple of 8px when possible.

| Size | Line-height × | Computed (px) | Snaps to 8px grid? |
|---|---|---|---|
| 12.80 | 1.5 | 19.20 | ~24 (adjust) |
| 16.00 | 1.6 | 25.60 | ~24 (close) |
| 25.00 | 1.3 | 32.50 | 32 ✓ |
| 48.83 | 1.1 | 53.71 | ~56 (round up) |

For interface use, round line-heights to the nearest multiple of 8px (e.g., body 16px → 24px line-height, not 25.6px). For long-form prose use the un-rounded values (16px → 25.6px). The TUI uses strict 8px/24px rhythm; the README and landing page use the prose rhythm.

---

## Lowercase Discipline

Lowercase is a typographic rule, not just a copy rule.

- **Headlines are lowercase.** Every H1, H2, H3, every display headline, every section label. Exceptions: legal text (license headers, security advisories) only.
- **CAPS are reserved for badges and status codes.** `[OK]`, `[ERR]`, `[WARN]`, `[INFO]`, `[ INITIATE ]`, `[ CONFIRM ]`. Uppercase means "this is a machine label, not a human sentence."
- **CAPS are never used for emphasis in body text.** Use `**bold**` or `` `inline code` `` instead.
- **CAPS are never used for headlines, nav, or marketing copy.** The brand is not a poster.

"precise · spare · lowercase" does not survive a capitalized headline. The brand becomes unrecognizable within one word.

---

## No Italics

Monospace italics are typographically weak — the angle fights the monospace grid. The brand does not use them.

Emphasis is achieved through:
- `inline code` with a `mz.slate` background — for terms, commands, file paths
- **bold** — for the rare moment that requires emphasis in prose
- Whitespace and short sentences — the Sage emphasizes by putting a thing on its own line

No italics in blockquotes, captions, footnotes, or "note" callouts. If the writer reaches for italics, the sentence needs to be rewritten.

---

## Rejected Alternatives

| Typeface | Why rejected |
|---|---|
| **Fira Code** | Ligatures. `=>` and `!=` ligatures are dated and opinionated. The Senior Operator either loves or hates them; the brand cannot pick a side. |
| **IBM Plex Mono** | Beautiful type, wrong brand association. The `a` alone tells the reader "IBM made this." |
| **Iosevka** | Too condensed for marketing headlines. At 48px display, the condensed letterforms read as cramped. |
| **Geist Mono** | Too Vercel. Any brand using it in 2026 reads as Vercel-adjacent within a second. metaphaze is positioned against that aesthetic. |
| **Commit Mono** | Defensible backup — strong free monospace, no brand baggage. Third choice because JetBrains Mono has broader pre-installed coverage. |
| **VT323** | Hollywood hacker typeface. The whole point of rejecting the terminal preset's defaults was to avoid this aesthetic. |
| **Monaco** | Apple system font. Reads as "I didn't pick a font." |
| **Courier** | No. |

---

## Typographic Details

Locked from discover/mood-board-direction.md, restated:

- **Em-dashes as structural dividers** inside sentences, padded with spaces: ` — `. Never `--` in prose (CLI flag territory only).
- **Bracketed navigation:** `[/about]  [/docs]  [/source]`. Brackets are part of the typography.
- **Footnotes use `[1]` inline markers**, not superscript. Plain-text-faithful.
- **Tables rendered as box-drawing characters** where possible (`─ │ ┌ ┐ └ ┘`). Website and TUI — one rendering system.
- **Status codes in brackets:** `[OK]`, `[ERR]`, `[WARN]`. The only uppercase in the brand.
- **Inline code in backticks:** `.mz/`, `mz run`, `cargo install mz`. Always.
- **No smart quotes.** Straight quotes `"` only. Smart quotes break when pasted into a terminal.

---

## Tailwind v4 Config

```css
@theme {
  --font-mono: "Berkeley Mono", "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  --font-sans: var(--font-mono); /* same face — no secondary sans */

  --text-xxs:  0.80rem;
  --text-xs:   0.8125rem;
  --text-sm:   0.90rem;
  --text-base: 1.00rem;
  --text-lg:   1.125rem;
  --text-xl:   clamp(1.125rem, 0.96rem + 0.83vw, 1.25rem);
  --text-2xl:  clamp(1.25rem, 1.04rem + 1.04vw, 1.5625rem);
  --text-3xl:  clamp(1.5625rem, 1.25rem + 1.56vw, 1.9531rem);
  --text-4xl:  clamp(1.9531rem, 1.56rem + 1.95vw, 2.4413rem);
  --text-5xl:  clamp(2.4413rem, 1.95rem + 2.44vw, 3.0518rem);

  --font-weight-normal:   400;
  --font-weight-semibold: 600;
  --font-weight-bold:     700;

  --leading-tight:  1.1;
  --leading-snug:   1.2;
  --leading-normal: 1.4;
  --leading-relaxed: 1.55;
  --leading-loose:   1.6;
}
```

---

## Related

- [logo-directions.md](./logo-directions.md)
- [color-system.md](./color-system.md)
- [imagery-style.md](./imagery-style.md)
- [brand-applications.md](./brand-applications.md)
