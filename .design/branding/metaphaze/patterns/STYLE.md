# metaphaze — STYLE.md

> Agent contract. Rendered from `metaphaze.yml`. If the .yml changes, regenerate this file.
> Generated: 2026-04-10

---

## Intensity

| Dial | Value | Meaning |
|------|-------|---------|
| Variance | **2 / 10** | rigid character grid, uniform pane structure, single narrow column. lower than the terminal preset (3) because metaphaze refuses tmux-style split panes in favor of a single reading-optimized column. |
| Motion | **1 / 10** | cursor blink and nothing else. no typewriter reveal, no glitch, no scanline-hum. lower than the terminal preset (4) because every other motion was rejected as hollywood-hacker nostalgia. |
| Density | **7 / 10** | dense monospace layout, tight information, long-form technical content. same as the terminal preset — the senior operator reads. |

---

## Philosophy

metaphaze is terminal *present*, not terminal *memory*. the senior operator reading a metaphaze surface is sitting in front of a modern terminal at 11pm, not watching a movie about hacking into a mainframe. every visual decision is an argument — the cursor is the running state of the tool, the monospace grid is the commitment, the narrow column is the reading discipline, the single accent color is the signal-versus-noise test.

the sage archetype refuses performance. the most restrained visual system available is the one that refuses to have ornament. monospace supremacy, lowercase voice, zero radius, no shadows, no gradients, one accent color used on less than 1% of pixels. the restraint is not a stylistic tic — it is the brand's statement that the signal is actually a signal.

the brand honors its dual target by committing the same tokens to two surfaces: rust constants for the ratatui TUI and CSS custom properties for the landing page. both are derived from the same canonical `.yml`. the senior operator who sees `mz▌` on the landing page is looking at the exact glyph they will see at the prompt five minutes later. there is no gap between brand and tool. the logo is a literal screenshot of the product.

brutally functional. high-contrast. authentically present. "the orchestrator runs outside the loop. claude builds. mz drives." — the manifesto is the brand and the brand is the manifesto. if a visual decision cannot be defended as a direct consequence of that sentence, it does not ship.

---

## Patterns

### Card
| Property | Rule |
|----------|------|
| border | `1px solid var(--mz-border)` |
| shadow | `none` |
| radius | `0` |
| background | `var(--mz-bg)` — flat, no elevated surface fill |
| header | box-drawing title bar (`┌─ title ──┐`) or inverted bar |

### Button (primary)
| Property | Rule |
|----------|------|
| background | `transparent` — no solid fill, bracketed text only |
| border | `none` |
| shadow | `none` |
| text | lowercase bracketed monospace `[ initiate ]` |
| radius | `0` |
| hover | video-invert — background fills `mz.bone`, text becomes `mz.black`, cursor `▌` appears after the label |

### Button (secondary)
| Property | Rule |
|----------|------|
| background | `transparent` |
| border | `none` |
| shadow | `none` |
| text | lowercase bracketed monospace `[ cancel ]` |
| radius | `0` |

### Input
| Property | Rule |
|----------|------|
| border | `none` — prompt-style (`$` or `>` prefix) |
| radius | `0` |
| background | `transparent` |
| focus | blinking block cursor `▌`, no ring, no outline |
| font | monospace always |

### Badge
| Property | Rule |
|----------|------|
| shape | `0` radius, bracketed |
| text | uppercase bracketed status codes `[OK]` `[ERR]` `[WARN]` |
| decoration | color matches status — signal green for `[OK]`, amber warn for `[WARN]`, deep amber for `[ERR]` (never red) |

### Navigation
| Property | Rule |
|----------|------|
| style | bracketed links `[/about]  [/docs]  [/source]` |
| background | `transparent` |
| border | `none` — no tab-bar rule |
| hover | `>` prefix slides in from the left, underline appears, color stays `mz.bone` |

### Code Block
| Property | Rule |
|----------|------|
| background | `var(--mz-border)` — the only place two colors touch |
| border | `1px solid var(--mz-border)` |
| radius | `0` |
| font | monospace always |
| prompt | `$` or `>` prefix in `mz.dust` |
| accent | at most one run in `mz.signal` per block (usually the command name) |

### Layout
| Property | Rule |
|----------|------|
| archetype | **monospace-editorial** |
| max-width | `max-w-3xl` — narrow, reading-optimized |
| section-spacing | `py-16` to `py-24` |
| grid-gap | `gap-4` to `gap-8` |
| surfaces | flat — no overlay, no scanlines, no phosphor |
| decoration | box-drawing diagrams, bracketed labels, status badges |
| dividers | em-dash rules (` — `) or box-drawing horizontal lines (`─`) |

---

## Constraints

### Never

- non-monospace fonts (Inter, Geist Sans, system-ui — any sans for any text)
- border-radius > 0 anywhere
- drop shadows or colored shadows
- background images or photography
- linear-gradient, radial-gradient, conic-gradient, mesh gradient
- pure black `#000000` or pure white `#ffffff`
- neon terminal green (`#00ff00`, `#33ff00`) — rejected from the terminal preset
- dev-tool blue (`#3b82f6`, `#007acc`, GitHub/Linear/Vercel blues)
- red for errors (use amber `#b8860b` — errors are recoverable states)
- CRT scanline overlay — rejected
- phosphor text glow — rejected
- typewriter reveal animations — rejected
- ALL CAPS headlines (caps reserved for `[BADGE]` codes only)
- italics (monospace italics are typographically weak)
- smart quotes (straight quotes only)
- emoji (text substitutes always — brand survives `LANG=C`)
- icon libraries (Lucide, Heroicons, Phosphor, Feather, Material)
- mascots, illustrations, 3D renders, stock photography
- backdrop-filter, mix-blend-mode, CSS filter
- marketing words: agentic, empower, transform, unlock, leverage, journey, ecosystem
- non-ASCII brackets (fullwidth, curly, lenticular) — only U+005B and U+005D

### Always

- monospace font on every element (Berkeley Mono or JetBrains Mono)
- lowercase for headlines, nav, body copy, labels, error messages
- bracketed navigation links (`[/about]  [/docs]  [/source]`)
- bracketed buttons (`[ initiate ]`)
- status code badges in brackets (`[OK]` `[ERR]` `[WARN]`)
- shell prompt prefixes on interactive elements (`$` `>` `~`)
- em-dash as structural divider (` — `), padded with spaces
- box-drawing characters for diagrams and borders (`─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`)
- real TUI recordings (VHS, asciinema) as the hero asset
- manifesto preserved verbatim: *the orchestrator runs outside the loop. claude builds. mz drives.*
- signal color (`#5fb878`) used on less than 1% of pixels per surface
- `NO_COLOR` environment variable respected — signal and amber drop out, brand still works in pure bone on black

---

## Effects

**Interaction vocabulary:** `cursor-blink`, `video-invert`, `underline-reveal`

### Hover
| Element | Technique | Description |
|---------|-----------|-------------|
| card | — | no hover state on static cards; border stays `mz.slate` |
| button | video-invert | background fills `mz.bone`, text becomes `mz.black` |
| link | underline-reveal | underline appears, `>` prefix slides in from the left |

### Active
| Element | Technique | Description |
|---------|-----------|-------------|
| button | cursor-append | block cursor `▌` appears after the label. no translate, no blink |

### Focus
| Element | Rule |
|---------|------|
| general | no ring — blinking cursor `▌` indicates focus |

### Transition
`0ms` or `100ms`, `step-end` only — no smooth easing. the cursor snaps, it does not fade.

### Ambient
- **cursor-blink** — block cursor `▌` blinks 530ms on / 530ms off on live surfaces only. never in static media (PDF, print, favicon, social preview).

---

## Bold Bets

1. **the cursor logo.** the mark is `mz▌` with a U+258C left half block in `#5fb878`. the logo is the literal glyph the TUI draws when the application is waiting for input. there is no gap between brand and product — the logo is a screenshot of the product, compressed to 2.5 monospace cells. on live surfaces the cursor blinks at the terminal default rate (530ms on / 530ms off). on static media it is frozen. no halo, no glow, no SVG reconstruction — it must be a real, typeable Unicode character.

2. **one typeface, everywhere, no concessions.** berkeley mono (paid, preferred) or jetbrains mono (free, fallback). headlines, body, nav, code, TUI output, README, the landing page — same face, weighted 400 / 600 / 700. no sans fallback. if both brand faces fail to load, the browser's default monospace is acceptable; a sans-serif fallback is not. using two typefaces is a performance. using one says "we committed."

3. **signal color as instrument, not decoration.** `#5fb878` appears on less than 1% of pixels on any surface. the cursor glyph in the logo. the numerator in `3/12` status lines. the `[OK]` badge content. the word `mz` in a `cargo install mz` callout. that is the entire budget. if the green grows bigger than that, it stops being a signal and starts being a color. this is the discipline that makes the system work.

4. **box-drawing characters as the illustration system.** diagrams drawn in monospace with `─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`. tables rendered as box-drawing tables. phase transition screens rendered with full-character borders. the entire inventory is 17 Unicode glyphs. every diagram is typeable, pasteable, and renders identically in any terminal, on any OS, in any font that has monospace coverage. if the diagram cannot survive `cat README.md`, it is the wrong diagram.

5. **dual target: rust constants AND css tokens from one yml.** the brand ships two deliverables from the same canonical source — `rust_bindings.color_ratatui` for the TUI, `web_bindings.css_custom_properties` for the landing page. consistency across surfaces is load-bearing because the TUI is the brand's most-used expression and the landing page is the brand's first impression. both are `#0a0a0a` / `#ededed` / `#5fb878`. both render the same logo, the same typography, the same narrow column. the senior operator switching between the two should see one brand.

---

## Implementation

### Rust (ratatui) bindings

```rust
use ratatui::style::Color;

// neutrals — the five anchors
pub const MZ_BLACK:  Color = Color::Rgb(0x0a, 0x0a, 0x0a);  // #0a0a0a  bg
pub const MZ_BONE:   Color = Color::Rgb(0xed, 0xed, 0xed);  // #ededed  fg
pub const MZ_DUST:   Color = Color::Rgb(0x8a, 0x8a, 0x8a);  // #8a8a8a  fg-muted
pub const MZ_SLATE:  Color = Color::Rgb(0x2a, 0x2a, 0x2a);  // #2a2a2a  border

// the one accent
pub const MZ_SIGNAL: Color = Color::Rgb(0x5f, 0xb8, 0x78);  // #5fb878  signal / success

// semantic amber (warnings and recoverable errors only — never red)
pub const MZ_WARN:   Color = Color::Rgb(0xd4, 0xa0, 0x17);  // #d4a017  [WARN]
pub const MZ_ERROR:  Color = Color::Rgb(0xb8, 0x86, 0x0b);  // #b8860b  [ERR]

// light-mode anchors (for when the TUI detects a light-background terminal)
pub const MZ_BG_LIGHT:     Color = Color::Rgb(0xfa, 0xfa, 0xfa);
pub const MZ_FG_LIGHT:     Color = Color::Rgb(0x0a, 0x0a, 0x0a);
pub const MZ_MUTED_LIGHT:  Color = Color::Rgb(0x6a, 0x6a, 0x6a);
pub const MZ_BORDER_LIGHT: Color = Color::Rgb(0xe0, 0xe0, 0xe0);
pub const MZ_SIGNAL_LIGHT: Color = Color::Rgb(0x2e, 0x8b, 0x57);
```

The TUI must respect `NO_COLOR`. When the variable is set, swap `MZ_SIGNAL`, `MZ_WARN`, and `MZ_ERROR` for `MZ_BONE` — the box-drawing survives, the brand still reads, and the accessibility rule holds.

Box-drawing glyphs for ratatui widgets:

```rust
pub const GLYPH_CURSOR:    &str = "\u{258C}";  // ▌
pub const GLYPH_H_LINE:    &str = "\u{2500}";  // ─
pub const GLYPH_V_LINE:    &str = "\u{2502}";  // │
pub const GLYPH_CORNER_TL: &str = "\u{250C}";  // ┌
pub const GLYPH_CORNER_TR: &str = "\u{2510}";  // ┐
pub const GLYPH_CORNER_BL: &str = "\u{2514}";  // └
pub const GLYPH_CORNER_BR: &str = "\u{2518}";  // ┘
pub const GLYPH_DOT_FULL:  &str = "\u{25CF}";  // ●
pub const GLYPH_DIAMOND_IN_DIAMOND: &str = "\u{25C8}";  // ◈
pub const GLYPH_CHECK:     &str = "\u{2713}";  // ✓
pub const GLYPH_CROSS_X:   &str = "\u{2717}";  // ✗
```

### CSS / Tailwind v4 bindings

```css
:root {
  --mz-bg:        #0a0a0a;
  --mz-fg:        #ededed;
  --mz-fg-muted:  #8a8a8a;
  --mz-border:    #2a2a2a;
  --mz-signal:    #5fb878;
  --mz-warn:      #d4a017;
  --mz-error:     #b8860b;

  --font-mono: "Berkeley Mono", "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;

  --mz-text-base: 1.00rem;
  --mz-leading-body: 1.6;
  --mz-radius: 0;
  --mz-border-w: 1px;
}

@media (prefers-color-scheme: light) {
  :root {
    --mz-bg:        #fafafa;
    --mz-fg:        #0a0a0a;
    --mz-fg-muted:  #6a6a6a;
    --mz-border:    #e0e0e0;
    --mz-signal:    #2e8b57;
    --mz-warn:      #b8860b;
    --mz-error:     #8b6508;
  }
}
```

Tailwind v4 `@theme`:

```css
@theme {
  --font-mono: "Berkeley Mono", "JetBrains Mono", ui-monospace, monospace;
  --font-sans: var(--font-mono);

  --color-mz-bg:       #0a0a0a;
  --color-mz-fg:       #ededed;
  --color-mz-fg-muted: #8a8a8a;
  --color-mz-border:   #2a2a2a;
  --color-mz-signal:   #5fb878;
  --color-mz-warn:     #d4a017;
  --color-mz-error:    #b8860b;

  --text-base: 1.00rem;
  --text-5xl:  clamp(2.4413rem, 1.95rem + 2.44vw, 3.0518rem);

  --font-weight-normal:   400;
  --font-weight-semibold: 600;
  --font-weight-bold:     700;

  --radius-sm: 0;
  --radius-md: 0;
  --radius-lg: 0;

  --shadow-sm: none;
  --shadow-md: none;
  --shadow-lg: none;
}
```

### Component code hints

```css
/* card — bordered pane, flat background */
.mz-card {
  border: 1px solid var(--mz-border);
  background: var(--mz-bg);
  border-radius: 0;
  box-shadow: none;
  padding: 1.5rem 2rem;
}

/* button-primary — bracketed text, video-invert on hover */
.mz-btn {
  background: transparent;
  border: none;
  color: var(--mz-fg);
  font-family: var(--font-mono);
  font-size: var(--mz-text-base);
  padding: 0;
  cursor: pointer;
  transition: none;
}
.mz-btn::before { content: "[ "; color: var(--mz-fg-muted); }
.mz-btn::after  { content: " ]"; color: var(--mz-fg-muted); }
.mz-btn:hover {
  background: var(--mz-fg);
  color: var(--mz-bg);
}
.mz-btn:hover::before,
.mz-btn:hover::after { color: var(--mz-bg); }

/* input — prompt style, no border */
.mz-input {
  border: none;
  background: transparent;
  font-family: var(--font-mono);
  color: var(--mz-fg);
  outline: none;
  caret-color: var(--mz-signal);
}
.mz-input-wrap::before {
  content: "$ ";
  color: var(--mz-fg-muted);
  font-family: var(--font-mono);
}

/* badge — bracketed uppercase status codes */
.mz-badge { font-family: var(--font-mono); }
.mz-badge::before { content: "["; color: var(--mz-fg); }
.mz-badge::after  { content: "]"; color: var(--mz-fg); }
.mz-badge-ok   { color: var(--mz-signal); }
.mz-badge-warn { color: var(--mz-warn); }
.mz-badge-err  { color: var(--mz-error); }

/* nav — bracketed links with > hover */
.mz-nav a {
  color: var(--mz-fg);
  text-decoration: none;
  font-family: var(--font-mono);
}
.mz-nav a::before { content: "["; color: var(--mz-fg); }
.mz-nav a::after  { content: "]"; color: var(--mz-fg); }
.mz-nav a:hover { text-decoration: underline; }
.mz-nav a:hover::before { content: "> ["; }

/* code-block — slate background, dust prompt, at most one signal run */
.mz-code {
  background: var(--mz-border);
  color: var(--mz-fg);
  border: 1px solid var(--mz-border);
  border-radius: 0;
  box-shadow: none;
  padding: 1rem 1.25rem;
  font-family: var(--font-mono);
  font-size: var(--mz-text-base);
  line-height: 1.6;
  white-space: pre;
  overflow-x: auto;
}
.mz-code .mz-prompt { color: var(--mz-fg-muted); }
.mz-code .mz-accent { color: var(--mz-signal); }
```

### Textures & surface recipes

none. metaphaze has no textures, no gradients, no surface treatments. the surface is flat. no noise grain, no halftone, no CRT scanlines, no paper grain, no subtle overlays, no backdrop-filter, no mix-blend-mode. if a designer reaches for a texture, the answer is no.

### Typography treatments

Berkeley Mono (preferred) self-hosted as WOFF2, weights 400 / 600 / 700 with `font-display: swap`:

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

JetBrains Mono fallback via Google Fonts:

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600;700&display=swap" rel="stylesheet">
```

Hierarchy is established by weight and whitespace, not by size alone. a 20px semibold heading next to 16px regular body reads as a clear hierarchy without a 3x size jump. no italics, no letter-spacing overrides on body copy, no text-stroke, no text-shadow.

### Animation recipes

the only animation in the brand:

```css
@keyframes mz-cursor-blink {
  0%, 49.99% { opacity: 1; }
  50%, 100%  { opacity: 0; }
}

.mz-cursor {
  display: inline-block;
  color: var(--mz-signal);
  animation: mz-cursor-blink 1.06s step-end infinite;
}
```

`step-end` is the easing. the cursor snaps on and off. it does not fade. 1.06s total cycle = 530ms on + 530ms off, matching the terminal default. on static media (PDF, print, social preview, favicon) the cursor does not blink — it is frozen in the "on" state.

---

## Related

- [metaphaze.yml](./metaphaze.yml) — source of truth (tokens, intensity, patterns, constraints, effects, rust_bindings, web_bindings)
- [guidelines.html](./guidelines.html) — visual brand guide (open in browser)
- [INDEX.md](./INDEX.md) — patterns phase index
