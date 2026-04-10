# Token Mapping

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

Brand tokens → ratatui (Rust) + Tailwind v4 / shadcn/ui (web). copy-paste-ready for both targets.

## Overview

metaphaze is a dual-target brand. the canonical source is `metaphaze.yml` — five neutrals, one accent, two amber semantics, one typeface. this file is the translation layer that turns those tokens into real code on the two surfaces the brand has to exist on: the `ratatui` TUI (primary, already shipping in `src/tui.rs`) and the future web landing page built on tailwind v4 + shadcn/ui. every value below derives from the `.yml` — if you change the `.yml`, you regenerate this file.

---

## Rust / ratatui

### File layout

put everything in `src/tui/theme.rs` (new file, one module). every `ratatui` call in `src/tui.rs` and anywhere else that styles cells imports from this module and never writes a raw `Color::Rgb(...)` inline.

```
src/
  tui.rs          -- existing render loop, imports from tui/theme
  tui/
    theme.rs      -- color constants, style presets, block presets
```

### Color constants

```rust
// src/tui/theme.rs
use ratatui::style::Color;

// ── neutrals — the five anchors ──────────────────────────────────────────────
pub const MZ_BLACK:  Color = Color::Rgb(0x0a, 0x0a, 0x0a);  // #0a0a0a  mz.black  bg
pub const MZ_BONE:   Color = Color::Rgb(0xed, 0xed, 0xed);  // #ededed  mz.bone   fg
pub const MZ_DUST:   Color = Color::Rgb(0x8a, 0x8a, 0x8a);  // #8a8a8a  mz.dust   fg-muted
pub const MZ_SLATE:  Color = Color::Rgb(0x2a, 0x2a, 0x2a);  // #2a2a2a  mz.slate  border

// ── the one accent ───────────────────────────────────────────────────────────
pub const MZ_SIGNAL: Color = Color::Rgb(0x5f, 0xb8, 0x78);  // #5fb878  mz.signal

// ── semantic amber (warning + recoverable error — never red) ─────────────────
pub const MZ_WARN:   Color = Color::Rgb(0xd4, 0xa0, 0x17);  // #d4a017  mz.amber.warn
pub const MZ_ERROR:  Color = Color::Rgb(0xb8, 0x86, 0x0b);  // #b8860b  mz.amber.deep

// ── light-mode anchors (when the TUI detects a light terminal) ───────────────
pub const MZ_BG_LIGHT:     Color = Color::Rgb(0xfa, 0xfa, 0xfa);
pub const MZ_FG_LIGHT:     Color = Color::Rgb(0x0a, 0x0a, 0x0a);
pub const MZ_MUTED_LIGHT:  Color = Color::Rgb(0x6a, 0x6a, 0x6a);
pub const MZ_BORDER_LIGHT: Color = Color::Rgb(0xe0, 0xe0, 0xe0);
pub const MZ_SIGNAL_LIGHT: Color = Color::Rgb(0x2e, 0x8b, 0x57);
pub const MZ_WARN_LIGHT:   Color = Color::Rgb(0xb8, 0x86, 0x0b);
pub const MZ_ERROR_LIGHT:  Color = Color::Rgb(0x8b, 0x65, 0x08);
```

### Style presets

higher-level `Style` builders on top of the constants. every widget in the TUI should use one of these, not a hand-rolled `Style::default().fg(...)`.

```rust
// src/tui/theme.rs (continued)
use ratatui::style::{Modifier, Style};

// base text
pub fn brand_text() -> Style {
    Style::default().fg(MZ_BONE).bg(MZ_BLACK)
}

// muted secondary text (metadata, timestamps, flag descriptions)
pub fn brand_muted() -> Style {
    Style::default().fg(MZ_DUST).bg(MZ_BLACK)
}

// signal — the single accent, BOLD so it lands at < 1% of pixels
pub fn brand_signal() -> Style {
    Style::default()
        .fg(MZ_SIGNAL)
        .bg(MZ_BLACK)
        .add_modifier(Modifier::BOLD)
}

// border color for idle panes
pub fn brand_border() -> Style {
    Style::default().fg(MZ_SLATE)
}

// border for the currently-focused pane
pub fn brand_border_active() -> Style {
    Style::default().fg(MZ_SIGNAL)
}

// ── badge styles (for [OK] [WARN] [ERR] inline tokens) ───────────────────────
pub fn brand_badge_ok() -> Style {
    Style::default().fg(MZ_SIGNAL).bg(MZ_BLACK)
}

pub fn brand_badge_warn() -> Style {
    Style::default().fg(MZ_WARN).bg(MZ_BLACK)
}

pub fn brand_badge_err() -> Style {
    Style::default().fg(MZ_ERROR).bg(MZ_BLACK)
}

// ── cursor — the ▌ glyph, signal-colored, bold ───────────────────────────────
pub fn brand_cursor() -> Style {
    Style::default().fg(MZ_SIGNAL).add_modifier(Modifier::BOLD)
}

// ── video-invert (button hover, code-block inverse) ──────────────────────────
pub fn brand_invert() -> Style {
    Style::default().fg(MZ_BLACK).bg(MZ_BONE)
}
```

### Block presets

reusable `Block` builders for pane borders. use these instead of hand-rolling a `Block::default().borders(Borders::ALL)` on every pane.

```rust
// src/tui/theme.rs (continued)
use ratatui::widgets::{Block, BorderType, Borders};

/// A flat bordered pane with a title bar. the primary container in the TUI.
/// title is padded with spaces and rendered in the brand's bone color.
pub fn brand_pane(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(brand_border())
        .title(format!(" {} ", title))
        .title_style(brand_text())
}

/// A brand pane with a signal-colored border for the focused / active pane.
pub fn brand_pane_active(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(brand_border_active())
        .title(format!(" {} ", title))
        .title_style(brand_text())
}

/// A code-block presentation — no border, slate background.
/// this is the one place two colors touch in the brand (mz.slate on mz.black).
pub fn brand_code_block() -> Block<'static> {
    Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(MZ_SLATE).fg(MZ_BONE))
}
```

### Modifier usage rules

- `Modifier::BOLD` — reserved for signal-bearing text (the accent color, display headings, command names in code blocks). never used for body copy.
- `Modifier::REVERSED` — inverted-video on button hover and the "this line is selected" pattern. matches the `video-invert` effect in `metaphaze.yml`.
- `Modifier::UNDERLINED` — link hover and the underline-reveal interaction.
- `Modifier::SLOW_BLINK` — cursor only. on surfaces that don't honor `SLOW_BLINK`, drive the blink manually at the app tick rate (see [cursor.md](./cursor.md)).
- `Modifier::ITALIC` — **banned**. monospace italics are typographically weak. the brand does not use them. never.
- `Modifier::DIM` — acceptable for disabled states when `mz.dust` already appears elsewhere in the frame (use sparingly — prefer an explicit `MZ_DUST` color).

### `NO_COLOR` discipline

when the `NO_COLOR` environment variable is set, the theme module should swap `MZ_SIGNAL`, `MZ_WARN`, and `MZ_ERROR` for `MZ_BONE`. the box-drawing survives. the brand still reads in pure `mz.bone` on `mz.black`.

---

## Web / Tailwind v4

### `app/globals.css`

one file. `@theme` block + CSS custom properties + shadcn variable mapping + font-face. paste this as `app/globals.css` in a Next.js 15 + tailwind v4 project.

```css
@import "tailwindcss";

/* ── tailwind v4 theme — derived from metaphaze.yml ──────────────────────── */
@theme {
  /* families — the one face, no secondary sans */
  --font-mono: "Berkeley Mono", "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  --font-sans: var(--font-mono);

  /* colors */
  --color-mz-bg:        #0a0a0a;
  --color-mz-fg:        #ededed;
  --color-mz-fg-muted:  #8a8a8a;
  --color-mz-border:    #2a2a2a;
  --color-mz-signal:    #5fb878;
  --color-mz-warn:      #d4a017;
  --color-mz-error:     #b8860b;

  /* type scale — major third, 1.25, 16px base */
  --text-xxs:  0.80rem;
  --text-xs:   0.8125rem;
  --text-sm:   0.90rem;
  --text-base: 1.00rem;
  --text-lg:   1.125rem;
  --text-xl:   clamp(1.125rem, 0.96rem + 0.83vw, 1.25rem);
  --text-2xl:  clamp(1.25rem,  1.04rem + 1.04vw, 1.5625rem);
  --text-3xl:  clamp(1.5625rem, 1.25rem + 1.56vw, 1.9531rem);
  --text-4xl:  clamp(1.9531rem, 1.56rem + 1.95vw, 2.4413rem);
  --text-5xl:  clamp(2.4413rem, 1.95rem + 2.44vw, 3.0518rem);

  /* weights */
  --font-weight-normal:   400;
  --font-weight-semibold: 600;
  --font-weight-bold:     700;

  /* line-heights */
  --leading-tight:   1.1;
  --leading-snug:    1.2;
  --leading-normal:  1.4;
  --leading-relaxed: 1.55;
  --leading-loose:   1.6;

  /* shape — zero radius everywhere */
  --radius-sm: 0;
  --radius-md: 0;
  --radius-lg: 0;

  /* elevation — no shadows */
  --shadow-sm: none;
  --shadow-md: none;
  --shadow-lg: none;
  --shadow-xl: none;
}

/* ── shadcn/ui CSS variable mapping — radix-compatible names ─────────────── */
:root {
  --background:          var(--color-mz-bg);
  --foreground:          var(--color-mz-fg);
  --card:                var(--color-mz-bg);
  --card-foreground:     var(--color-mz-fg);
  --popover:             var(--color-mz-bg);
  --popover-foreground:  var(--color-mz-fg);
  --primary:             var(--color-mz-signal);
  --primary-foreground:  var(--color-mz-bg);
  --secondary:           var(--color-mz-fg-muted);
  --secondary-foreground: var(--color-mz-bg);
  --muted:               var(--color-mz-fg-muted);
  --muted-foreground:    var(--color-mz-fg-muted);
  --accent:              var(--color-mz-signal);
  --accent-foreground:   var(--color-mz-bg);
  --destructive:         var(--color-mz-error);
  --destructive-foreground: var(--color-mz-bg);
  --border:              var(--color-mz-border);
  --input:               var(--color-mz-border);
  --ring:                transparent;  /* no focus ring — cursor indicates focus */
  --radius:              0;
}

/* ── light mode — literal inversion ──────────────────────────────────────── */
@media (prefers-color-scheme: light) {
  @theme {
    --color-mz-bg:       #fafafa;
    --color-mz-fg:       #0a0a0a;
    --color-mz-fg-muted: #6a6a6a;
    --color-mz-border:   #e0e0e0;
    --color-mz-signal:   #2e8b57;
    --color-mz-warn:     #b8860b;
    --color-mz-error:    #8b6508;
  }
}

/* ── global element defaults — lock monospace on everything ──────────────── */
html, body, button, input, select, textarea, code, pre, kbd, samp {
  font-family: var(--font-mono);
  font-feature-settings: normal;
}

body {
  background: var(--color-mz-bg);
  color: var(--color-mz-fg);
  font-size: var(--text-base);
  line-height: var(--leading-loose);
  -webkit-font-smoothing: antialiased;
}

/* kill radius and shadow on every shadcn primitive by default */
*, *::before, *::after {
  border-radius: 0 !important;
  box-shadow: none !important;
}

/* the only animation in the brand */
@keyframes mz-cursor-blink {
  0%, 49.99% { opacity: 1; }
  50%, 100%  { opacity: 0; }
}
```

### shadcn variable rationale

| shadcn var | metaphaze token | why |
|---|---|---|
| `--background` | `mz.black` / `mz.bg-light` | root surface |
| `--foreground` | `mz.bone` / `mz.fg-light` | body text |
| `--primary` | `mz.signal` | the one accent — any shadcn "primary" surface reads as signal |
| `--primary-foreground` | `mz.black` | video-invert text on signal background |
| `--secondary` | `mz.dust` | annotation layer |
| `--muted` / `--muted-foreground` | `mz.dust` | info-not-signal |
| `--border` / `--input` | `mz.slate` | structural only |
| `--ring` | `transparent` | no focus rings — cursor indicates focus |
| `--destructive` | `mz.amber.deep` | recoverable error, never red |
| `--card`, `--popover` | `mz.black` | flat, no elevated surface fill |
| `--radius` | `0` | sharp corners, always |

### Font loading

self-hosted Berkeley Mono (preferred) — place WOFF2 files in `public/fonts/` and add to `globals.css`:

```css
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Regular.woff2") format("woff2");
  font-weight: 400;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-SemiBold.woff2") format("woff2");
  font-weight: 600;
  font-display: swap;
}
@font-face {
  font-family: "Berkeley Mono";
  src: url("/fonts/BerkeleyMono-Bold.woff2") format("woff2");
  font-weight: 700;
  font-display: swap;
}
```

JetBrains Mono fallback via `next/font/google` (for projects without a Berkeley Mono license):

```ts
// app/layout.tsx
import { JetBrains_Mono } from "next/font/google";

const jetbrains = JetBrains_Mono({
  subsets: ["latin"],
  weight: ["400", "600", "700"],
  variable: "--font-mono",
  display: "swap",
});

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={jetbrains.variable}>
      <body>{children}</body>
    </html>
  );
}
```

the `--font-mono` CSS variable is the single font variable. `--font-sans` aliases to it — no second family, ever.

### Tailwind utility shortcuts

tailwind v4 auto-generates utility classes from `@theme` variables. the color tokens above produce real classes:

- `font-mono` / `font-sans` — both resolve to the same face
- `text-mz-fg`, `text-mz-fg-muted`, `text-mz-signal`, `text-mz-warn`, `text-mz-error`
- `bg-mz-bg`, `bg-mz-border`
- `border-mz-border`

no custom tailwind plugin is needed — the `--color-mz-*` vars in `@theme` generate `text-mz-*` and `bg-mz-*` utilities automatically in v4. radius and shadow utilities all resolve to `0` / `none` because every radius / shadow token is overridden in `@theme`.

---

## Dual-target discipline

### Single source of truth

`metaphaze.yml` is canonical. `src/tui/theme.rs` and `app/globals.css` are both derived from it. if you change `mz.signal` in the `.yml`, you regenerate both files.

### Automated sync strategy

recommend writing a small build script (`scripts/sync-theme.rs` or `scripts/sync-theme.py`) that reads `metaphaze.yml` and emits both outputs:

```python
# scripts/sync_theme.py — aspirational, ~30 lines
import yaml, pathlib, textwrap

src = pathlib.Path(".design/branding/metaphaze/patterns/metaphaze.yml")
data = yaml.safe_load(src.read_text())

colors = {
    "MZ_BLACK":  data["dark_mode"]["color"]["background"],
    "MZ_BONE":   data["dark_mode"]["color"]["on-background"],
    "MZ_DUST":   data["tokens"]["color"]["muted"],
    "MZ_SLATE":  data["dark_mode"]["color"]["border"],
    "MZ_SIGNAL": data["dark_mode"]["color"]["primary"],
    "MZ_WARN":   data["dark_mode"]["color"]["warning"],
    "MZ_ERROR":  data["dark_mode"]["color"]["error"],
}

# emit src/tui/theme.rs constants
rs = ["use ratatui::style::Color;", ""]
for name, hex in colors.items():
    r, g, b = int(hex[1:3], 16), int(hex[3:5], 16), int(hex[5:7], 16)
    rs.append(f"pub const {name}: Color = Color::Rgb(0x{r:02x}, 0x{g:02x}, 0x{b:02x});")
pathlib.Path("src/tui/theme_generated.rs").write_text("\n".join(rs))

# emit app/globals.css @theme color block
css = ["@theme {"]
for name, hex in colors.items():
    css.append(f"  --color-{name.lower().replace('_', '-')}: {hex};")
css.append("}")
pathlib.Path("app/theme-generated.css").write_text("\n".join(css))
```

run as `python scripts/sync_theme.py` on pre-commit or in CI. the generated files are committed. the brand stays in sync automatically.

### Test harness

both environments should render the same seven brand colors identically. minimal smoke test:

- **rust:** add a `theme_preview()` function to `src/tui/theme.rs` that renders `[BLACK] [BONE] [DUST] [SLATE] [SIGNAL] [WARN] [ERROR]` as inline spans in a debug screen.
- **web:** add `/brand/preview` page that renders the same seven colors as `<span>` elements with matching labels.
- **visual comparison:** screenshot both at the same terminal size / viewport width and verify the hex values render identically.

---

## Related

- [../metaphaze.yml](../metaphaze.yml)
- [../STYLE.md](../STYLE.md)
- [pane.md](./pane.md)
- [status-badge.md](./status-badge.md)
- [bracketed-button.md](./bracketed-button.md)
- [prompt-input.md](./prompt-input.md)
- [phase-transition.md](./phase-transition.md)
- [cursor.md](./cursor.md)
