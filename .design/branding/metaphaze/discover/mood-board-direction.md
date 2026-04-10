# Mood Board Direction
> Phase: discover | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

**Color:** Warm off-black background, warm off-white text, one signal color — muted terminal green. No dev-tool blue. No gradients.

**Type:** Berkeley Mono as the voice (if license allows) or JetBrains Mono as the free alternative. One typeface. Everything. Headlines, body, navigation, code blocks, TUI — same face, different weights and sizes.

**Imagery:** None, in the traditional sense. The TUI itself, recorded with VHS or asciinema, is the hero asset. ASCII diagrams are the illustrations. Code blocks are the photography. No photos. No illustrations. No mascots.

**Feel:** If suckless and Linear had a baby and raised it on Berkeley Graphics' typographic discipline. Brutalist in structure, crafted in detail, terminal-native in execution.

---

## Color Direction

### Primary Palette

| Role | Hex | Name | Usage |
|---|---|---|---|
| Background | `#0a0a0a` | `mz.black` | Everything. Site, README dark preview, TUI. Not pure `#000000` — a warm, barely-off-black so eyes don't fatigue on long reads. |
| Foreground | `#ededed` | `mz.bone` | Primary text. Warm off-white. Not `#ffffff` because pure white on `#0a0a0a` is too high a contrast ratio for comfortable long-form reading. |
| Mid-gray | `#8a8a8a` | `mz.dust` | Secondary text, timestamps, metadata. The "this is context, not content" layer. |
| Faint gray | `#2a2a2a` | `mz.slate` | Borders, table dividers, code block backgrounds. Structural only. |
| Signal | `#5fb878` | `mz.signal` | The one accent. Used for status indicators, success states, the logo mark, and nothing else. |

### Why these choices

**`#0a0a0a` over pure black.** Pure `#000000` is the Nothing Phone move, and it's the right move for OLED hardware brands — but on a laptop LCD at reading distance, pure black is hard on the eyes after a few minutes. `#0a0a0a` reads as black to most viewers but is measurably easier to scan long-form text against. It is the Linear and Vercel choice, and it is correct.

**`#ededed` over pure white.** Same logic inverted. Pure `#ffffff` foreground on near-black is 19:1 contrast, which is overkill and reads as harsh. `#ededed` comes in at around 16:1 — still comfortably above WCAG AAA for body text but less "staring into a flashlight" in a dimly lit room at 11pm, which is when the Senior Operator is actually reading the README.

**Muted terminal green `#5fb878` as the signal color.** This is the most opinionated decision in the whole brand. Options considered:

- **Bright terminal green `#00ff88`** — too synthetic. Reads as "I am a hacker movie."
- **Nothing Phone red `#ff0000`** — would be distinctive but wrong personality. metaphaze is not "signal + danger." It is "build + verify."
- **Linear purple `#5e6ad2`** — too close to existing brand equity. metaphaze would read as a Linear clone.
- **Vercel magenta `#ff0080`** — same problem.
- **Amber `#d4a017`** — strong candidate. Reads as "caution/warning" too naturally. Save for error states.
- **Muted terminal green `#5fb878`** — the chosen direction. Reads as "build succeeded" / "the agent is running" / "green means go." Naturally conveys the "hands-off, walk away, come back to working code" promise. Desaturated enough to avoid the "hacker aesthetic" trap, vivid enough to function as a real signal. Sits well next to `#0a0a0a` and `#ededed` without fighting either.

Use the signal color for **less than 1% of pixels on any page**. The logo mark. Status dots in the TUI. The `cargo install mz` code block highlight. That's it. Never for buttons, never for headlines, never for "hover" states. Restraint is the whole point.

### Light Mode Counterpart

Light mode exists because some Senior Operators use light terminals. It should be a literal inversion, not a softened version.

| Role | Hex | Name |
|---|---|---|
| Background | `#fafafa` | `mz.bone` |
| Foreground | `#0a0a0a` | `mz.black` |
| Mid-gray | `#6a6a6a` | `mz.dust` |
| Faint gray | `#e0e0e0` | `mz.slate` |
| Signal | `#2e8b57` | `mz.signal` (darker to hold contrast on light bg) |

### Anti-Palette (what metaphaze will never use)

- **Dev-tool blue** — `#3b82f6`, `#007acc`, any of the Atlassian/Linear/GitHub blues. Overused. Signals "generic SaaS." Banned.
- **Gradients** — any interpolation between two colors. Banned outright. Solid fills only.
- **Pastels** — nothing in the `#f7c` or `#c7f` family. Reads as consumer SaaS.
- **Warm whites** — beige, cream, or ivory backgrounds in light mode. Reads as editorial, not technical.

## Typography

### Primary Typeface: Berkeley Mono (recommended) or JetBrains Mono (fallback)

**Berkeley Mono** by Neil Panchal / U.S. Graphics Company is the strongest choice. It has the typographic refinement to carry a marketing site, not just a code editor. The letterforms are distinctive enough to function as brand. Paid license (~$75 for personal, ~$300+ for commercial), but single-purchase and worth it for a brand that will outlive the purchase by years. ([Berkeley Mono](https://www.featuredtype.com/typefaces/berkeley-mono))

**JetBrains Mono** is the free alternative if the license budget is $0. It is the safest single-monospace choice for 2026, with excellent hinting and wide family support. ([JetBrains Mono](https://www.jetbrains.com/lp/mono/)) Less distinctive than Berkeley, but still strong. SIL Open Font License. Acceptable without compromise.

**Rejected alternatives:**
- **Fira Code** — ligatures are dated and opinionated. Hard pass.
- **IBM Plex Mono** — beautiful but carries IBM brand association that metaphaze does not want.
- **Iosevka** — too condensed for marketing headlines; wonderful in the editor, awkward at 72pt.
- **Commit Mono** — close second to JetBrains Mono among free options; would be defensible.
- **Geist Mono** — too associated with Vercel; would read as a clone.

**Pick one. Use it for everything.** No secondary sans. No accent serif. No exceptions. The monospace IS the brand voice. The moment you introduce a sans-serif for navigation "because monospace nav looks weird" — that is the moment the brand breaks. Commit.

### Type Scale

Anchored to a 16px base with a modest scale. Brutalist does not mean giant — it means honest.

| Role | Size | Weight | Tracking | Usage |
|---|---|---|---|---|
| Display | 48px / 3rem | 700 | -0.02em | Hero headline only. One per page max. |
| H1 | 32px / 2rem | 600 | -0.01em | Page titles |
| H2 | 24px / 1.5rem | 600 | 0 | Section headings |
| H3 | 18px / 1.125rem | 600 | 0 | Subsections |
| Body | 16px / 1rem | 400 | 0 | Default. Line-height 1.6. |
| Small | 13px / 0.8125rem | 400 | 0.01em | Metadata, timestamps, footnotes |
| Code | 15px / 0.9375rem | 400 | 0 | Inline and block code. Same face as body. |

Note that body and code are nearly the same size because they ARE nearly the same thing — it is all monospace. The distinction is the `mz.slate` background behind code blocks, not the typeface.

### Typographic Details

- **Navigation is bracketed.** `[/about]  [/docs]  [/source]` — the brackets are part of the design, not decoration. This is an htmx reference and it is correct for this brand.
- **Em-dashes as structure.** Use ` — ` (padded em-dash) as a typographic divider within sentences. The Senior Operator reads a lot; they notice.
- **Tables rendered as `column -t` output** whenever possible. ASCII borders using `─ │ ┌ ┐ └ ┘` box-drawing characters. This applies on the website AND in the TUI — same aesthetic, same system.
- **Footnotes use `[1]` inline markers**, not superscript. Plain-text-faithful.
- **No italics.** Monospace italics are usually weak. Use **bold** or `inline code` for emphasis instead.

## Imagery Direction

**No photography.** Not of teams. Not of screens. Not of abstract textures. Not of "code on a laptop in a dark room." The brief is explicit and the brief is right.

**No illustrations.** Not mascots. Not hand-drawn diagrams. Not isometric vector scenes. Not abstract 3D renders. Charmbracelet does illustrations brilliantly and metaphaze should not try to compete — different personality.

**What replaces imagery:**

1. **Live TUI recordings.** asciinema for interactive demos, VHS for GIF-like embeddable recordings. The hero of the landing page should be a real, running `mz run` session captured with VHS, autoplaying in a `<pre>` block. It should show the actual TUI, with the actual output, doing something real like "orchestrate a Rust crate from spec to shipped." ([Charmbracelet VHS](https://github.com/charmbracelet/vhs))

2. **ASCII diagrams as the illustration system.** The phase → track → step decomposition should be rendered as a box-drawing diagram in monospace, not as an SVG flowchart. Same typography as everything else. Same color system. Integrates visually with the TUI because it IS the TUI's aesthetic.

3. **Code blocks as the photography.** A well-formatted code block with a meaningful command and its real output is the most honest marketing asset a developer tool can have. Use them liberally. `mz init`, `mz run`, `mz status` — show the real output.

4. **The logo mark as the only "visual" element.** One mark. Used sparingly. Details in the identity phase, but the concept direction: the `mz` letterform itself, set in the brand typeface, possibly with a single `#5fb878` accent element (a status dot, a cursor, a bracket). Never an orb. Never a generic geometric mark.

5. **The phase transition screen is the hero moment.** The actual `mz` CLI prints phase transitions in ASCII — that is the brand's most authentic visual. Screenshots of it belong everywhere the brand exists.

## Overall Feel

**"If suckless and Linear had a baby and raised it on Berkeley Graphics' typographic discipline."**

- **suckless** provides the philosophical spine — refusal as design language, text as brand, function over form.
- **Linear** provides the polish ceiling — the obsessive micro-typography, the restraint, the "every decision was intentional" feel. But stripped of the gradients and the cinematic flourishes.
- **Berkeley Graphics / berkeleygraphics.com** provides the typographic discipline — monospace-everywhere done without compromise, proving that a marketing site set in a single typeface can be beautiful.
- **Charmbracelet** provides the TUI craft reference — the understanding that terminal output is a first-class design surface. metaphaze inherits the craft without inheriting the mascots.
- **htmx** provides the voice — witty, direct, slightly irreverent, willing to reference itself in the brand.

Specifically NOT:
- **Not Nothing Phone.** Too hardware, too consumer, too cold. metaphaze is warmer and more textual than dot-matrix industrialism.
- **Not Vercel.** Too polished, too neutral, too "we want to be acquired." metaphaze is more opinionated.
- **Not GitHub dark.** That is the competitor aesthetic (literally — GSD 2.0 uses GitHub's default dark). metaphaze has to look visibly different from a GitHub README even though it will mostly BE a GitHub README.

The test: **If a senior dev sees the metaphaze landing page in a tab next to a `claude-flow` landing page, they should be able to tell in under 2 seconds that metaphaze was made by someone who actually writes code.**

## Style Affinity

From `/gsp-style/styles/INDEX.yml`, ranked by fit to this research:

### 1. `terminal` — **STRONG match, primary recommendation**

Tags: `[developer, monospace, dark, minimal, technical]` — "Your favorite code editor as a design system"

This is the closest preset to what the research points to. It covers the core of metaphaze: monospace everywhere, dark mode by default, developer-facing, minimal, technical. Use this as the base preset and override the specific tokens (`mz.black`, `mz.bone`, `mz.signal`) defined above.

**Why it's the right base:** The brief and the trend analysis both converge here. The Senior Operator already lives in a terminal aesthetic. Starting from `terminal` means the defaults are already mostly correct and the work is in specificity, not reinvention.

### 2. `monochrome` — **STRONG secondary, use for structure**

Tags: `[black-white, monochrome, high-contrast, editorial, minimal]` — "Pure black and white — typographic depth, zero decoration"

Layer this on top of `terminal` for the typographic discipline it enforces: high contrast, editorial structure, zero decoration. `terminal` gives you the color system and the monospace; `monochrome` gives you the editorial rigor that keeps the brand from looking like a default VS Code theme.

**Why it belongs here:** metaphaze's content is text-heavy (READMEs, documentation, TUI output). Editorial typographic structure is load-bearing. `monochrome` enforces the rhythm and discipline the content needs.

### 3. `nothing` — **Partial match, borrow selectively**

Tags: `[monochrome, industrial, dark, minimal, technical, instrument, mechanical, swiss]` — "Nothing Phone DNA — OLED black, dot-matrix Doto headlines, red signal accent"

This is a partial match, not a primary direction. Borrow: the single signal color discipline, the "instrument / mechanical" framing, the OLED-black rigor. Do NOT borrow: the dot-matrix Doto headlines (too consumer hardware), the red accent (wrong personality for metaphaze), the industrial product-design heritage (metaphaze is not a physical object).

**Why only partial:** `nothing` is the right reference for "one signal color, used with restraint" and "the brand is an instrument, not decoration." It is the wrong reference for "how does this read in a terminal pane." Take the discipline, leave the aesthetic.

### Rejected presets (for the record)

- **`modern-dark`** — Linear/Vercel direction with ambient blobs and mouse spotlights. Too cinematic for brutalist. The research explicitly calls for the stripped version of this.
- **`bold-typography`** — poster aesthetic with massive headlines. metaphaze's typography is confident but not theatrical. Brutalist restraint is not poster energy.
- **`neubrutalism`** — thick borders, hard offset shadows, playful-flat. Too playful. Neubrutalism has been absorbed into consumer SaaS ("funky 90s throwback") and is the wrong signal for the Senior Operator in 2026.

### Final stack

**Base: `terminal`** — provides the core system.
**Layer: `monochrome`** — provides the editorial typographic discipline.
**Accent discipline from: `nothing`** — one signal color, used with restraint, as a brand instrument.

Token overrides to apply after the base preset loads:
- `background: #0a0a0a`
- `foreground: #ededed`
- `muted: #8a8a8a`
- `border: #2a2a2a`
- `accent: #5fb878`
- `font-family: "Berkeley Mono", "JetBrains Mono", ui-monospace, monospace`

---

## Sources

- [Berkeley Mono on Featured Type](https://www.featuredtype.com/typefaces/berkeley-mono)
- [JetBrains Mono](https://www.jetbrains.com/lp/mono/)
- [Best Coding Fonts 2026 — Made Good Designs](https://madegooddesigns.com/coding-fonts/)
- [Vercel Design System Breakdown — SeedFlip](https://seedflip.co/blog/vercel-design-system)
- [Ratatui docs](https://ratatui.rs/)
- [suckless.org](https://suckless.org/rocks/)
