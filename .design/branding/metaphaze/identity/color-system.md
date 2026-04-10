# Color System

> Phase: identity | Brand: metaphaze | Generated: 2026-04-10

---

## TL;DR

Five colors. That is the whole system. Warm off-black, warm off-white, two grays for structure, one muted green signal. No dev-tool blue, no gradients, no pastels. Dark mode is the default because the Senior Operator lives in a dark terminal. Light mode is a literal inversion.

The signal color — muted terminal green `#5fb878` — is used for less than 1% of pixels on any surface. Restraint is not a stylistic tic; it is the brand's statement that the signal is actually a signal.

**Composition strategy:** Neutral + Single Accent, layered with Terminal/ANSI discipline. Near-monochrome neutral base with one accent that carries all semantic weight. The accent appears only where it means something.

---

## Primary Palette (Dark Mode, Default)

| Role | Hex | Token | Usage | Rationale |
|---|---|---|---|---|
| Background | `#0a0a0a` | `mz.black` | Site, README dark preview, TUI chrome, code block fills | Warm off-black. Pure `#000000` is hard on eyes during long reads — `#0a0a0a` reads as black but is measurably more comfortable at minute 90. Linear/Vercel choice. |
| Foreground | `#ededed` | `mz.bone` | Primary text, headlines, body copy, code, TUI output | Warm off-white. `#ffffff` on `#0a0a0a` hits 19:1 — harsh. `#ededed` at 16.97:1 is AAA for body and legibly softer. |
| Mid-gray | `#8a8a8a` | `mz.dust` | Secondary text, timestamps, metadata, flag descriptions | The "this is context, not content" layer. The Sage distinguishes the argument from the annotation. |
| Faint gray | `#2a2a2a` | `mz.slate` | Borders of panes and tables, code block backgrounds | Structural only. Never a text color. Draws the box around the TUI pane and the `1px solid` around code blocks. |
| Signal | `#5fb878` | `mz.signal` | Logo cursor, success states, status dots, `cargo install mz` highlight | The single opinionated choice. Muted terminal green. Reads as "build succeeded." Desaturated enough to avoid the Hollywood hacker trap. |

### Why muted green and not anything else

The decision was forced in `discover/mood-board-direction.md` and is locked. Reasoning restated:

- **Bright terminal `#00ff88`** — reads as "I am a hacker movie." metaphaze is a real tool, not a movie prop.
- **Red** — metaphaze's emotional promise is relief, not urgency.
- **Linear purple `#5e6ad2`** and **Vercel magenta `#ff0080`** — would read as a clone.
- **Amber `#d4a017`** — strong, but belongs in the error/warning slot.
- **Muted green `#5fb878`** — maps onto the product's emotional target: "walk away, come back to working code."

---

## OKLCH Palette Scales

Full 11-stop scales generated via tints.dev. The brand-canonical mid-stop is listed where it differs from the scale-computed `500` value — tokens should resolve to the canonical, not the computed, for primary signal use.

### `mz.black` scale (neutral, dominant)

| Stop | Hex |
|---|---|
| 50 | `#e2e2e2` |
| 100 | `#c9c9c9` |
| 200 | `#939393` |
| 300 | `#636363` |
| 400 | `#353535` |
| **500 (canonical)** | **`#0a0a0a`** |
| 600 | `#070707` |
| 700 | `#070707` |
| 800 | `#040404` |
| 900 | `#040404` |
| 950 | `#000000` |

### `mz.bone` scale (neutral, foreground)

| Stop | Hex |
|---|---|
| 50 | `#fcfcfc` |
| 100 | `#fcfcfc` |
| 200 | `#f6f6f6` |
| 300 | `#f3f3f3` |
| 400 | `#f1f1f1` |
| **500 (canonical)** | **`#ededed`** |
| 600 | `#b9b9b9` |
| 700 | `#868686` |
| 800 | `#575757` |
| 900 | `#2e2e2e` |
| 950 | `#191919` |

### `mz.signal` scale (accent, the one color)

| Stop | Hex |
|---|---|
| 50 | `#f7fdf9` |
| 100 | `#eef9f3` |
| 200 | `#d9f1e5` |
| 300 | `#c4e9d7` |
| 400 | `#add7c3` |
| **500 (canonical)** | **`#5fb878`** |
| 600 | `#4a9b60` |
| 700 | `#367d4b` |
| 800 | `#2e8b57` (light-mode brand stop) |
| 900 | `#1d2622` |
| 950 | `#131a15` |

The canonical `#5fb878` is the dark-mode brand anchor and `#2e8b57` is the light-mode anchor. The tints.dev-computed `500` returns a more desaturated green (`#96c5af`) which the brand explicitly rejects — the scale exists for supporting shades, not for overriding the canonical anchors.

### `mz.amber` scale (warning + error only)

| Stop | Hex |
|---|---|
| 50 | `#fef9f0` |
| 100 | `#fef3e0` |
| 200 | `#fdd9b5` |
| 300 | `#fcbb80` |
| 400 | `#faa855` |
| **500 (canonical: warning)** | **`#d4a017`** |
| **600 (canonical: error, dark mode)** | **`#b8860b`** |
| 700 | `#c85a14` |
| **800 (canonical: error, light mode)** | **`#8b6508`** |
| 900 | `#6b2305` |
| 950 | `#4a1501` |

The canonical warning/error anchors override the generated scale at the 500/600/800 stops. The scale exists for supporting shades only.

---

## WCAG Contrast Audit

Calculated per WCAG 2.1 relative-luminance formula. All primary text pairs are flagged for AA (4.5:1 normal / 3:1 large) and AAA (7:1 normal / 4.5:1 large).

### Dark mode pairs

| Foreground | Background | Ratio | AA normal | AA large | AAA normal | Use |
|---|---|---|---|---|---|---|
| `#ededed` (bone) | `#0a0a0a` (black) | **16.97:1** | ✓ | ✓ | ✓ | Body, headlines, code |
| `#8a8a8a` (dust) | `#0a0a0a` (black) | **5.81:1** | ✓ | ✓ | — | Secondary text, metadata, flags |
| `#5fb878` (signal) | `#0a0a0a` (black) | **8.17:1** | ✓ | ✓ | ✓ | Signal-bearing text and glyphs |
| `#d4a017` (amber warn) | `#0a0a0a` (black) | **8.48:1** | ✓ | ✓ | ✓ | `[WARN]` badge, warning states |
| `#b8860b` (amber error) | `#0a0a0a` (black) | **6.15:1** | ✓ | ✓ | — | `[ERR]` badge, error states |
| `#2a2a2a` (slate) | `#0a0a0a` (black) | **1.38:1** | — | — | — | Structural border only — never text |

### Light mode pairs

| Foreground | Background | Ratio | AA normal | AA large | AAA normal | Use |
|---|---|---|---|---|---|---|
| `#0a0a0a` (black) | `#fafafa` (bone) | **18.95:1** | ✓ | ✓ | ✓ | Body, headlines, code |
| `#6a6a6a` (dust) | `#fafafa` (bone) | **5.18:1** | ✓ | ✓ | — | Secondary text, metadata |
| `#2e8b57` (signal) | `#fafafa` (bone) | **4.06:1** | ✗ | ✓ | ✗ | **Large glyphs only** — cursor, status dots (≥24px). Never body text. |
| `#8b6508` (amber error) | `#fafafa` (bone) | **6.80:1** | ✓ | ✓ | — | Error badge, error states |
| `#e0e0e0` (slate) | `#fafafa` (bone) | **1.13:1** | — | — | — | Structural border only — never text |

### Light-mode signal constraint (enforced rule)

`#2e8b57` on `#fafafa` passes AA only for large text (≥24px or ≥18.66px bold). It must never be applied to body copy, flag descriptions, nav links, or any text smaller than 24px on light mode surfaces. For body-sized signal use, fall back to `#367d4b` (the 700 stop, ~6.1:1) as the only permitted alternative.

The cursor glyph at minimum 16px is technically borderline — but the U+258C block character is a high-stroke-weight shape and reads as "large" in visual weight. It passes the spirit of the rule. The rule exists to prevent thin-stroke body text, not block elements.

---

## Semantic Token Assignments

| Token | Light value | Dark value | Rationale |
|---|---|---|---|
| `--mz-bg` | `#fafafa` (bone) | `#0a0a0a` (black) | Page, surface, pane |
| `--mz-fg` | `#0a0a0a` (black) | `#ededed` (bone) | Body text, headlines, code |
| `--mz-fg-muted` | `#6a6a6a` (dust) | `#8a8a8a` (dust) | Metadata, timestamps, flag descriptions |
| `--mz-border` | `#e0e0e0` (slate) | `#2a2a2a` (slate) | Pane borders, code block outlines |
| `--mz-signal` | `#2e8b57` (signal) | `#5fb878` (signal) | Cursor, success states, the `OK` |
| `--mz-warn` | `#b8860b` (amber warn) | `#d4a017` (amber warn) | `[WARN]` badge |
| `--mz-error` | `#8b6508` (amber error) | `#b8860b` (amber error) | `[ERR]` badge — amber, never red |
| `--mz-disabled` | `#6a6a6a` (dust) | `#8a8a8a` (dust) | Disabled flags, aged timestamps |

No `--mz-info` token. Info states use `--mz-fg-muted`. Info is not a signal — it is annotation.

---

## Anti-Palette

| Color family | Specific offenders | Why it is banned |
|---|---|---|
| Dev-tool blue | `#3b82f6`, `#007acc`, GitHub blue, Atlassian blue, Linear blue, any blue in the `#1d4ed8–#60a5fa` range | Every agent harness in the category is already wearing this. Wearing it too would erase the positioning. |
| Gradients | any `linear-gradient()`, any radial fade, any ambient blob | Solid fills only. Gradients are the language of "polished SaaS" — the quadrant metaphaze explicitly does not sit in. |
| Pastels | `#f7c3d4`, `#c7d4f7`, baby blue, millennial pink | Reads as consumer SaaS, D2C startup. Wrong audience. |
| Warm editorial whites | cream, beige, ivory (`#fdf6e3`, `#f5e6d3`) in light mode | Reads as editorial or literary, not technical. Light mode is `#fafafa`, cool and technical. |
| Neon terminal green | `#00ff00`, `#33ff00`, phosphor greens | Hollywood hacker aesthetic. Rejected in favor of `#5fb878`. |
| Red | any red-family hue | metaphaze errors are recoverable states, not catastrophes. Error uses amber. |
| Rainbow | any use of multiple accent hues | The whole point of the signal is that it is the only color. |

---

## Color Usage Discipline

**The rule that makes this system work:** the signal color appears on less than 1% of pixels on any given surface. If the green is bigger than that, it stops being a signal and starts being a color.

### Where you WILL see `#5fb878`

- The `▌` cursor in the logo (every surface that shows the logo)
- Status dots in the TUI pane (`●` next to "claude running," "step complete")
- The `[OK]` badge content (not the brackets themselves)
- The `cargo install mz` command highlight in the README hero — the word `mz` only
- The phase-transition progress bar when a phase completes
- Status lines: `phase 3/12 · step 17/97` — numerator (`3`, `17`) in signal, denominator in dust
- The GitHub social preview's one-glyph accent

### Where you will NEVER see `#5fb878`

- Button backgrounds (there are no button backgrounds; buttons are bracketed text in `mz.bone`)
- Link colors in body copy (links are `mz.bone` with underline)
- Headline colors (headlines are `mz.bone`)
- Navigation items (nav is `mz.bone` in brackets)
- Hover states on the website (hover is underline-reveal, not color-change)
- Code block borders (borders are always `mz.slate`)
- Icons (there are no icons)
- The background of any surface (the signal is foreground only)

### Where amber appears

- `[WARN]` badge content
- `[ERR]` badge content (in the deeper `mz.amber.deep`)
- `mz status` output when a step has failed and is awaiting `mz resume`

Amber is rarer than green because metaphaze ships binaries, not errors.

---

## Token Naming Rationale

Every token is prefixed `mz.` because the binary is `mz` and the brand's one job is to stay internally consistent. Token names match the locked mood board (`mz.black`, `mz.bone`, `mz.dust`, `mz.slate`, `mz.signal`) with semantic aliases on top (`mz.amber`, `mz.amber.deep`).

The `mz.` prefix is insurance against a name collision in a multi-brand design system — which will never exist, because this brand refuses to be a platform.

---

## Related

- [logo-directions.md](./logo-directions.md)
- [typography.md](./typography.md)
- [imagery-style.md](./imagery-style.md)
- [brand-applications.md](./brand-applications.md)
- [palettes.json](./palettes.json)
