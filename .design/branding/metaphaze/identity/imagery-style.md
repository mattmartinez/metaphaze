# Imagery Style

> Phase: identity | Brand: metaphaze | Generated: 2026-04-10

---

## TL;DR

No photography. No illustrations. No mascots. No stock art. No 3D renders. No gradient flourishes. The imagery system is the TUI, recorded. Everything that would normally be an image in a brand — hero banners, diagrams, icons, decorative flourishes — is replaced with a live terminal artifact or an ASCII structure.

**The rule:** if the brand asset cannot be rendered in a terminal, it is probably the wrong asset.

---

## The Alternative System

### 1. TUI Recordings as Hero Assets

The single most important visual artifact in the brand is a recording of the actual `mz` CLI running.

**Tools:**
- **VHS** (charmbracelet/vhs) — for autoplaying `.gif` / `.webm` embeds in the landing page and README
- **asciinema** — for interactive, scrubbable recordings in documentation (`.cast` files)

**Composition rules:**

- **Show real work, not demos.** The hero recording should do something a Senior Operator would actually do — "orchestrate a small Rust crate from spec to passing tests," not "hello world."
- **Full-width terminal, minimum 100 columns.** The monospace grid has to be visible.
- **Brand palette matches the website.** `#0a0a0a` bg, `#ededed` fg, `#5fb878` accent. No Gruvbox, no Dracula, no Catppuccin.
- **Default typeface.** Berkeley Mono or JetBrains Mono in the recording. The TUI matches the website and vice versa.
- **No typing animation.** Commands are pre-typed and executed instantly.
- **No CRT overlay, no scanlines, no phosphor glow.** The recording is a recording of a modern terminal, not a nostalgia filter.
- **Speed: honest real-time, slow sections trimmed.** Never speed-ramp. The brand does not fake speed.

### VHS Configuration (reference)

```
# demo.tape — canonical VHS setup for metaphaze recordings
Set Shell "bash"
Set FontSize 16
Set Width 1600
Set Height 900
Set Padding 24
Set Theme {
  "background": "#0a0a0a",
  "foreground": "#ededed",
  "black":      "#0a0a0a",
  "red":        "#b8860b",
  "green":      "#5fb878",
  "yellow":     "#d4a017",
  "blue":       "#ededed",
  "magenta":    "#ededed",
  "cyan":       "#ededed",
  "white":      "#ededed",
  "brightBlack":   "#8a8a8a",
  "brightRed":     "#b8860b",
  "brightGreen":   "#5fb878",
  "brightYellow":  "#d4a017",
  "brightBlue":    "#ededed",
  "brightMagenta": "#ededed",
  "brightCyan":    "#ededed",
  "brightWhite":   "#ededed",
  "selection":  "#2a2a2a",
  "cursor":     "#5fb878"
}
Set FontFamily "Berkeley Mono"
Set TypingSpeed 0ms
Set PlaybackSpeed 1.0
```

Every non-brand color (blue, magenta, cyan) maps to `#ededed` because the brand does not use them. The few semantic colors (red, yellow, green) map to the brand equivalents.

### 2. ASCII Diagrams as Illustrations

Diagrams drawn in monospace with Unicode box-drawing characters (U+2500–U+257F), not SVG.

**Aesthetic rules:**

- **Use single-line box characters by default:** `─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`
- **Double lines `═ ║ ╔ ╗ ╚ ╝`** reserved for emphasis only — the one thing the reader needs to see
- **One character per grid cell.** Respects the monospace grid. No half-widths.
- **No color inside diagrams.** Render in `mz.bone` on `mz.black`. Accent colors clutter them.
- **Lowercase labels.** Uppercase only for status codes inside diagram boxes.
- **Survives `cat`.** If the Senior Operator can't `cat README.md` and understand the diagram, it is too clever.

### 3. Code Blocks as Photography

The well-formatted code block with a meaningful command and its real output is the brand's photography.

**Formatting rules:**

- **Real commands only** — `cargo install mz`, `mz init`, `mz run`, `mz status`. All typeable.
- **Real output** — actual timestamps, exit codes, paths. Not sanitized marketing versions.
- **`mz.slate` background, `mz.bone` text, `mz.slate` border.** 1px border. No drop shadow. No rounded corners.
- **Prompt prefix `$` in `mz.dust`.** So the Senior Operator can tell input from output at a glance.
- **Highlighted element in `mz.signal`.** At most one character run per block — usually the command name (`mz`) or success indicator (`[OK]`, `●`).
- **No syntax highlighting theme.** No VS Code theme. Brand palette only. Rust code renders in `mz.bone` on `mz.slate` with no token colors.

### 4. The Phase Transition Screen as Signature Visual Moment

The `mz` CLI prints phase transitions as ASCII screens — bordered pane, headline, progress ratio, file tree. This is the brand's most authentic visual. The brand's job is to treat it as a first-class asset.

**Where it appears:**
- Landing page — static screenshot below the VHS hero, labeled "what a phase transition looks like"
- README — "what it looks like" section
- Social previews (see `brand-applications.md`)
- Release notes when a phase decomposition ships

The transition screen is not decorated or stylized. It is the actual output of the actual tool, cropped to its visible rectangle.

---

## Iconography

### No icon library

metaphaze does not use Lucide, Heroicons, Phosphor, Feather, Material Icons, or any other standard icon library. Traditional icon libraries are designed for GUI applications; metaphaze is not a GUI application.

### What replaces icons

**Unicode glyph inventory** — the canonical set. Every glyph is typeable, font-agnostic, survives being copied into a chat, and renders identically on the website and in the TUI.

| Glyph | U+ | Name | Use |
|---|---|---|---|
| `▌` | U+258C | LEFT HALF BLOCK | The logo cursor, the "waiting for input" indicator in the TUI |
| `─` | U+2500 | HORIZONTAL LINE | Diagram lines, dividers |
| `│` | U+2502 | VERTICAL LINE | Tree continuation, pane separators |
| `┌ ┐ └ ┘` | U+250C/U+2510/U+2514/U+2518 | BOX CORNERS | Pane corners |
| `├ ┤ ┬ ┴ ┼` | U+251C/U+2524/U+252C/U+2534/U+253C | BOX JUNCTIONS | Tree branches, grid junctions |
| `═ ║ ╔ ╗ ╚ ╝` | U+2550-U+255D | DOUBLE BOX | Emphasis diagrams only |
| `●` | U+25CF | BLACK CIRCLE | Active status (in `mz.signal`) |
| `○` | U+25CB | WHITE CIRCLE | Idle / pending status (in `mz.dust`) |
| `◆` | U+25C6 | BLACK DIAMOND | Complete status, phase markers (in `mz.signal`) |
| `◇` | U+25C7 | WHITE DIAMOND | Pending phase marker (in `mz.dust`) |
| `◈` | U+25C8 | WHITE DIAMOND IN DIAMOND | Active phase marker (in `mz.signal`) |
| `✓` | U+2713 | CHECK MARK | Success (in `mz.signal`) |
| `✗` | U+2717 | BALLOT X | Failure (in `mz.amber.deep`) |
| `…` | U+2026 | HORIZONTAL ELLIPSIS | Truncation, pending, "waiting" |
| `→` | U+2192 | RIGHTWARDS ARROW | "Then this" in flows, `mz init → mz plan` |
| `$` | U+0024 | DOLLAR SIGN | Shell prompt prefix |
| `>` | U+003E | GREATER-THAN | REPL / TUI prompt prefix |
| `~` | U+007E | TILDE | Home directory shorthand |

### Custom SVG direction (only if a brand asset genuinely requires vector)

When a Unicode glyph cannot carry the meaning — for example, a GitHub social preview that needs a vector wordmark — the brand ships hand-authored SVG, not icon-library exports.

**Rules for custom SVG:**

- **Paths only, no fills except flat `mz.signal` or `mz.bone`.** No gradients, no masks, no filters.
- **Stroke-width 2px at viewBox 24 24.** The brand stroke weight matches the Berkeley Mono semibold stem.
- **Corners are sharp (stroke-linejoin: miter).** No rounded joins — the rule is 0px radius everywhere.
- **Line caps are butt, not round.** Flat terminals match the monospace grid.
- **Exported via CLI, not Figma GUI.** The brand's SVG assets are authored as text files, committed to the repo, and rendered at build time. If Figma touches an SVG, the file is not canonical.
- **File naming:** `.design/branding/metaphaze/identity/svg/{name}.svg` — lowercase, kebab-case.

There is no icon library `import`. If a contributor reaches for one, the answer is: use a Unicode glyph, or author the SVG directly.

---

## Textures & Patterns

### Surface treatment — none

metaphaze does not use textures. No noise grain, no halftone dots, no CRT scanlines, no paper grain, no subtle overlays. The surface is flat.

This is a deliberate rejection of the terminal preset's default scanline overlay. Scanlines are the visual language of "terminal nostalgia"; metaphaze is a terminal present, not a terminal memory.

### Pattern motifs

The only repeating pattern allowed in the brand is the **box-drawing grid** used in ASCII diagrams. This is not a background texture — it is structural content that happens to repeat.

### Gradients — forbidden

No `linear-gradient()`, no `radial-gradient()`, no mesh, no conic. Solid fills only. This is the hard rule locked in discover/mood-board-direction.md.

The only place where two colors touch is the edge between a code block (`mz.slate` fill) and the surrounding page (`mz.black` fill). That edge is a 1px `mz.slate` border, not a gradient.

### The one exception: the `#5fb878` cursor blink

On live surfaces (landing page hero, TUI splash), the block cursor `▌` blinks on and off at 530ms intervals. This is not a gradient or a transition — it is a discrete state change. The cursor is fully visible or fully hidden, nothing in between.

```css
@keyframes mz-cursor-blink {
  0%, 49.99%   { opacity: 1; }
  50%, 100%    { opacity: 0; }
}

.mz-cursor {
  display: inline-block;
  color: var(--mz-signal);
  animation: mz-cursor-blink 1.06s step-end infinite;
}
```

`step-end` is the easing, not `ease` or `linear`. The cursor snaps. It does not fade.

---

## Image Treatments (CSS recipes)

### Code block treatment

```css
.mz-code {
  background: var(--mz-slate); /* #2a2a2a dark / #e0e0e0 light */
  color: var(--mz-fg);         /* #ededed / #0a0a0a */
  border: 1px solid var(--mz-slate);
  border-radius: 0;            /* never rounded */
  padding: 1rem 1.25rem;
  font-family: var(--font-mono);
  font-size: var(--mz-text-base);
  line-height: 1.6;
  box-shadow: none;            /* never shadowed */
  overflow-x: auto;
  white-space: pre;
}

.mz-code .mz-prompt {
  color: var(--mz-fg-muted);   /* $ and > prefixes */
}

.mz-code .mz-accent {
  color: var(--mz-signal);     /* single highlighted run per block */
}
```

### VHS embed treatment

```css
.mz-vhs {
  display: block;
  width: 100%;
  max-width: 960px;
  margin: 2rem auto;
  border: 1px solid var(--mz-border); /* #2a2a2a */
  border-radius: 0;
  background: var(--mz-bg);
  /* The recording itself carries the brand; no container decoration */
}
```

### Phase transition screen (static screenshot)

```css
.mz-phase-transition {
  display: block;
  width: 100%;
  max-width: 720px;
  border: 1px solid var(--mz-border);
  background: var(--mz-bg);
  padding: 1.5rem 2rem;
  font-family: var(--font-mono);
  font-size: var(--mz-text-sm);
  line-height: 1.6;
  white-space: pre;
  color: var(--mz-fg);
}
```

### Blend modes — forbidden

No `mix-blend-mode`, no `background-blend-mode`. The brand does not composite images. If an element needs to sit on top of another, it sits on top — no blending.

### Filters — forbidden

No `filter: blur()`, no `drop-shadow()`, no `grayscale()`, no `sepia()`, no `hue-rotate()`. The brand does not process assets at render time. If an asset needs treatment, it is baked in at export.

---

## Aspect Ratios

Standard aspect ratios for the handful of image-shaped assets the brand does produce:

| Use case | Ratio | Dimensions | Notes |
|---|---|---|---|
| VHS hero recording | 16:9 | 1600 × 900 | Landing page, README top |
| Phase transition screenshot | 5:3 | 720 × 432 | "What it looks like" sections |
| Social preview (OG image) | 1.91:1 | 1200 × 630 | Twitter, Facebook, LinkedIn cards |
| GitHub social preview | 2:1 | 1280 × 640 | GitHub repo OG card |
| Favicon square | 1:1 | 32 × 32, 64 × 64, 128 × 128 | Cursor-only icon variant |
| Documentation inline VHS | 16:9 | 960 × 540 | Smaller embeds in docs pages |
| Asciinema cast | auto | any size | Interactive, width-responsive |

All assets are delivered at 2x resolution minimum for retina displays.

---

## Responsive Strategy

### Art direction breakpoints

metaphaze has one art direction rule, applied to the VHS hero:

```html
<picture>
  <source
    media="(max-width: 640px)"
    srcset="/vhs/hero-mobile.webm"
    type="video/webm">
  <source
    media="(min-width: 641px)"
    srcset="/vhs/hero-desktop.webm"
    type="video/webm">
  <video
    src="/vhs/hero-desktop.webm"
    autoplay muted loop playsinline
    class="mz-vhs"
    aria-label="metaphaze CLI — full autonomous build"></video>
</picture>
```

The mobile variant is a shorter, tighter crop (100 columns minimum, maybe 30 lines). The desktop variant is the full hero (100+ columns, 50+ lines). Both are real recordings of the same tool, not reformats.

### object-fit strategy

```css
.mz-vhs video,
.mz-vhs img {
  width: 100%;
  height: auto;
  object-fit: contain; /* preserve the monospace grid, never crop */
  background: var(--mz-bg);
}
```

Never `object-fit: cover` on brand assets. Cropping the terminal grid destroys the visual language.

---

## Loading Strategy

### VHS hero

- **Format:** `.webm` (VP9) for size, `.gif` fallback for older browsers
- **Preload:** `preload="metadata"` — not `preload="auto"`. The hero is visible but not critical path.
- **Autoplay:** yes, muted, looped, playsinline
- **First frame placeholder:** a static screenshot of the phase transition screen (same color palette, inherits the page background). No blur-up, no skeleton — the placeholder IS an in-brand asset.

### Inline `img` and `video` elements

- **Loading attribute:** `loading="lazy"` for anything below the fold
- **No blur-up placeholder.** The brand does not use blur as a visual technique.
- **No skeleton loaders.** Static placeholders only.
- **Dominant color placeholder:** the brand background `#0a0a0a` is the dominant color of every asset by construction.

```css
.mz-vhs video,
.mz-vhs img {
  background: #0a0a0a;
  min-height: 540px;    /* prevents layout shift */
}
```

### Asciinema embeds

- Lazy-load the asciinema player script on user interaction (not on page load)
- Show the static VHS-generated preview until the user clicks play
- Bandwidth budget: asciinema `.cast` files are tiny (<10KB typical), safe to load inline

---

## Anti-Patterns

For the avoidance of doubt, these are banned:

- **Lucide icons, Heroicons, Phosphor, Feather, Material** — too generic, too SaaS-dashboard-shaped
- **Emoji** — wrong tone. No `:sparkles:`, no `:rocket:`, no `:fire:`. Exception: a literal Unicode character where no text substitute works (rare). Prefer `[WARN]` over `⚠`.
- **3D renders** — isometric vector scenes, 3D typography, cinema4D blobs, ambient glow
- **Gradient illustrations** — any illustration with a color gradient
- **Mascots** — no animal, no character, no branded figure. Charmbracelet owns that territory.
- **Stock photography** — of teams, of laptops, of screens, of "abstract tech"
- **Hand-drawn illustrations** — no sketchy marker-style diagrams, no Whimsical-style flowcharts
- **Hero videos of developers typing** — the Senior Operator does not want to watch a person type
- **"Trusted by" logo walls** — the brand refuses social proof in visual form
- **CRT scanline overlays** — terminal nostalgia, rejected
- **Phosphor text glow** — `text-shadow` glow effects, rejected
- **Typewriter reveal animations** — Hollywood hacker, rejected
- **Blur-up placeholders** — the brand does not use blur as a visual technique
- **`backdrop-filter: blur()`** — no glass, no frosted surfaces
- **`box-shadow` on brand assets** — drop shadows are forbidden
- **`border-radius > 0px`** — everything is sharp

---

## Related

- [logo-directions.md](./logo-directions.md)
- [color-system.md](./color-system.md)
- [typography.md](./typography.md)
- [brand-applications.md](./brand-applications.md)
