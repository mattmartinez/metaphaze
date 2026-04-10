# Logo Directions

> Phase: identity | Brand: metaphaze | Generated: 2026-04-10

---

## TL;DR

Three directions, all typographic, all set in the brand monospace (Berkeley Mono preferred, JetBrains Mono fallback). No orbs. No marks that could be confused with a generic dev-tool logo. No icon that could survive being separated from the wordmark — because the wordmark IS the mark.

**Recommendation: Direction B — The Cursor.** The only one of the three that carries the product thesis in the logo itself. The cursor is the TUI. The TUI is the product. The logo is the product.

---

## Direction A — The Wordmark

### Concept

`metaphaze` set in Berkeley Mono, lowercase, regular weight, tight tracking. Nothing else. No mark. No accent shape. The word is the logo. The single `#5fb878` treatment is reserved for the final `z` — the word trails off into the brand accent the way a status line ends in a green `OK`.

```
metaphaze
        ^ this character in #5fb878
```

The `mz` short form is the same treatment with the middle cropped: `mz` with the `z` in signal.

### Strategic rationale

- **Sage-first.** The Sage refuses performance. The most restrained logo available is the one that refuses to have a mark. "The architecture is the argument" applied to identity: the wordmark IS the argument.
- **Quiet competence.** Zero ornament. Cannot be mistaken for a startup. Looks like the name of a Unix utility, which is what metaphaze is.
- **Lowercase voice.** Enforces the voice rule at identity level. Any other treatment breaks the "lowercase is a position" commitment in `voice-and-tone.md`.
- **Single-binary signal.** One word, one face, one color accent. Mirrors `cargo install mz` — one command, one artifact.

### Mark type

Pure wordmark. No symbol, no lockup, no container.

### Construction

Set in Berkeley Mono Regular. Because the typeface is monospaced, all nine characters occupy an identical advance width. The baseline grid is the character grid — no kerning, no stylistic alternates, no manual letterspacing.

- **Advance width (1u):** the width of one monospace character at the current size.
- **Wordmark width:** exactly 9u for `metaphaze`, 2u for `mz`.
- **Cap height:** the default Berkeley Mono cap height at the chosen size.
- **x-height:** the default Berkeley Mono x-height — do not modify.
- **Tracking:** 0 (native monospace advance, no override).
- **Signal character:** the final `z` replaces the default glyph color with `#5fb878`. Same glyph, same position, different fill. No halo, no bold.

### Variations

| Variation | Lockup | Size envelope | Use |
|---|---|---|---|
| Primary | `metaphaze` lowercase, Berkeley Mono Regular, final `z` in `#5fb878` | ≥ 88px wide | README header, landing page, social preview body |
| Secondary | `mz` short form, same treatment | 28–88px wide | Tight horizontal slots, tab bars, compact nav |
| Icon | final `z` glyph alone in `#5fb878` on `#0a0a0a` | 16–48px favicon only | Favicons (16/32/48), never larger |
| Wordmark only | `metaphaze` in `#ededed` on `#0a0a0a`, no signal character | ≥ 88px wide | Print, single-color embroidery, fax |
| Monochrome | `metaphaze` in `#ededed` only (dark) or `#0a0a0a` only (light) | ≥ 88px wide | Applications where the green cannot survive |
| Reversed | `metaphaze` in `#0a0a0a` on `#ededed`, final `z` in `#2e8b57` (light-mode signal) | ≥ 88px wide | Light mode surfaces |

### Clear space

Minimum padding equal to **1x cap height** on all four sides. In monospace terms: the space occupied by one lowercase character at the wordmark's current size. Nothing may cross that envelope — no text, no rules, no other marks.

For the `mz` short form, clear space is **0.75x cap height** on all sides.

### Minimum size

- **Full lockup:** 88px wide for `metaphaze`, minimum 11px per character. Below 88px, switch to `mz` short form.
- **Short form:** 28px wide for `mz`. Below 28px, switch to icon (final `z` glyph alone).
- **Icon:** 16px is the floor. Below 16px, do not use the mark — omit it.
- **Print:** 18mm wide for the full wordmark, 6mm wide for `mz`.

### Don'ts

- Never capitalize. Not at the start of a sentence, not in a heading, not in a press release. "Metaphaze" and "METAPHAZE" are both wrong.
- Never add a tagline beneath the wordmark. The manifesto lives elsewhere.
- Never put the wordmark inside a box, pill, or container. It sits on the page directly.
- Never change the color of characters other than the final `z`. No rainbow `metaphaze`, no `#5fb878` treatment on multiple characters, no "m" in green.
- Never use the signal accent at sizes below 28px wide — a single green character becomes noise.
- Never stretch, condense, skew, or outline the wordmark.
- Never set the wordmark in a non-monospace fallback typeface. If Berkeley Mono and JetBrains Mono both fail to load, the browser default monospace is acceptable — a sans-serif fallback is not.
- Never animate the wordmark. It does not fade in, type in, glow, or pulse.

---

## Direction B — The Cursor  *(recommended)*

### Concept

`mz` set in Berkeley Mono Regular, immediately followed by a solid block cursor `▌` in `#5fb878`. The cursor is the U+258C "left half block" character — the same glyph the TUI draws when the application is waiting for input.

On static surfaces the cursor does not blink. On live surfaces (landing page hero, TUI splash screen) it blinks at the terminal default rate: 530ms on, 530ms off. That is the only animation in the brand.

```
mz▌
  ^ #5fb878 block cursor, U+258C
```

The long form is `metaphaze▌` with the cursor at the end of the word. Same glyph, same color, same rules.

### Strategic rationale

- **Sage-first.** The cursor is the state of a system waiting for input — the most honest depiction of a tool. No metaphor, no abstraction. The mark is literally what the product looks like when it is running.
- **Creator influence (the 25%).** The cursor is a crafted detail. The Creator in metaphaze uses the correct Unicode character (not a rectangle drawn in Illustrator). The Creator cares that the blink rate matches the terminal default.
- **Quiet competence positioning.** The cursor says "I am a tool" without shouting. It cannot be mistaken for a SaaS dashboard, a mobile app, or an enterprise platform. It is a terminal. That is the entire category frame.
- **Lowercase voice + TUI-native.** The cursor sits on the baseline and refuses to be loud. It is the typographic signature of the command line.
- **Single-binary signal.** The cursor appears at the end of a prompt waiting for a command. The command is `mz`. The logo is the prompt. Install-to-identity in one glyph.

### Mark type

Lettermark + symbol lockup. `mz` is the lettermark; `▌` is the symbol. They are inseparable in the primary mark.

### Construction

Set in Berkeley Mono Regular. The block cursor is U+258C ("LEFT HALF BLOCK"), which in a correctly rendered monospace occupies **0.5u** horizontally and the full vertical advance (ascender to descender) — the same rectangle a terminal draws in place.

- **Advance width — `mz▌`:** 2u for the letters + 0.5u for the cursor = **2.5u total**.
- **Advance width — `metaphaze▌`:** 9u + 0.5u = 9.5u total.
- **Cursor height:** full cell height (ascender to descender) of the current font size.
- **Cursor position:** baseline-aligned with the letters. No raised or lowered offset.
- **Gap between letters and cursor:** zero. The cursor sits immediately after the last letter, as a real terminal cursor would.
- **Baseline:** characters sit on the baseline, cursor rests with its bottom on the descender line — this is how terminals render U+258C.
- **Cursor color:** `#5fb878` on dark backgrounds, `#2e8b57` on light backgrounds. No other color is ever permitted.

### Variations

| Variation | Lockup | Size envelope | Use |
|---|---|---|---|
| Primary | `mz▌` with cursor in `#5fb878` | 32–∞ px wide | README header, GitHub avatar, landing page nav, social preview, default everywhere |
| Secondary | `metaphaze▌` long form | 96–∞ px wide | Title bars, social previews, contexts that need the full name |
| Icon (favicon) | `▌` alone in `#5fb878` on `#0a0a0a` | 16–48 px | Favicons only. The `mz` is unreadable at this size; the cursor carries the identity alone |
| Wordmark only | `mz` or `metaphaze` without cursor | 32–∞ px wide | Contexts where the cursor glyph cannot render (plaintext-only surfaces, line-drawn print) |
| Monochrome | `mz▌` with cursor in `#ededed` (dark) or `#0a0a0a` (light) — same shape, no signal | 32–∞ px wide | Single-color applications, embroidery, fax, monochrome print |
| Reversed | `mz▌` in `#0a0a0a` on `#ededed` with cursor in `#2e8b57` | 32–∞ px wide | Light mode surfaces |
| Motion | Cursor blinks 530ms on / 530ms off | Landing page hero, TUI splash only | Live surfaces only — never in static media |

### Clear space

Minimum padding equal to **1x cursor width (0.5u)** on all four sides — the width of one block cursor glyph at the mark's current size. Terminal-native: tight, honest, just enough breathing room to read.

For favicon use, clear space is **0** — the icon fills its container edge-to-edge.

### Minimum size

- **Full primary `mz▌`:** 32px wide minimum. Below 32px, switch to cursor-only icon.
- **Long form `metaphaze▌`:** 96px wide minimum. Below 96px, switch to `mz▌`.
- **Cursor-only icon:** 16px is the floor. Below 16px, do not use the mark.
- **Print:** 10mm wide for `mz▌`, 32mm wide for `metaphaze▌`.

### Don'ts

- Never use any color for the cursor other than `#5fb878` (dark mode) or `#2e8b57` (light mode). Not blue, not amber, not red. Not even in error states — the cursor is the logo, not a status indicator.
- Never blink the cursor in static media — PDFs, print, social previews, favicons, README images. Blink only on live surfaces that would naturally have a running process.
- Never replace U+258C with a rectangle drawn in SVG, an `<img>` tag, or a CSS pseudo-element. The cursor must be a real, typeable Unicode character.
- Never place text or symbols immediately after the cursor. The cursor is always the last glyph in the mark.
- Never use the cursor-only icon above 48px. At larger sizes it reads as a rectangle, not a cursor.
- Never stretch, condense, or skew the cursor glyph. It inherits the font's native aspect ratio.
- Never add a halo, glow, stroke, or outline to the cursor. No text-shadow. No phosphor effect.
- Never use a different blink rate. 530ms on / 530ms off matches the terminal default and nothing else.
- Never pair the cursor with another mark or symbol. It is self-contained.

---

## Direction C — The Bracket

### Concept

`[mz]` or `[metaphaze]` wrapped in square brackets, Berkeley Mono Regular, lowercase, monochrome. The brackets inherit from the bracketed navigation pattern already locked in `voice-and-tone.md` (`[/about]  [/docs]  [/source]`) and the htmx `</>` reference. The brackets are structural — they say "this is a label, this is a status, this is a unit of the interface."

```
[mz]
```

No signal color in the default treatment. The brackets are load-bearing enough that adding green would be decoration.

### Strategic rationale

- **Sage-first.** Brackets are the tightest way to say "this is a named thing" in monospace. The Sage labels precisely. `[mz]` reads like an error code or a log prefix — both of which are Sage artifacts.
- **Creator influence.** The bracket is an htmx homage, tying metaphaze to a lineage of brutalist dev tools that earned the Senior Operator's trust.
- **Quiet competence.** `[mz]` is the format of a log line. It is what the TUI prints in front of every status message (`[mz] phase 3/12 · track 2/4`). The logo is already in the product output.

### Mark type

Wordmark-in-container. The brackets are part of the mark — they are not a frame around it. Remove the brackets and the logo is broken.

### Construction

- **Advance width — `[mz]`:** 4u (bracket + two letters + bracket).
- **Advance width — `[metaphaze]`:** 11u.
- **Bracket glyphs:** U+005B `[` and U+005D `]` — the ASCII characters, never the fullwidth or curly variants.
- **Bracket color:** same as surrounding text (`#ededed` dark, `#0a0a0a` light).
- **Tracking:** 0 (native monospace advance).
- **Vertical position:** baseline-aligned with the letters. Brackets use their default font ascender and descender — no custom sizing.

### Variations

| Variation | Lockup | Size envelope | Use |
|---|---|---|---|
| Primary | `[mz]` in `#ededed` on `#0a0a0a`, Berkeley Mono Regular | 40–∞ px wide | README header, landing page, default |
| Secondary | `[metaphaze]` long form, same treatment | 96–∞ px wide | Title bars, social previews |
| Icon | `[ ]` empty brackets, centered | 16–48 px | Favicons only — less distinctive than B's cursor |
| Wordmark only | `mz` or `metaphaze` without brackets | 28–∞ px wide | Contexts where brackets conflict with surrounding UI |
| Monochrome | Default is already monochrome. Inverted for light mode: `#0a0a0a` on `#fafafa` | 40–∞ px wide | All applications |
| Reversed | `[mz]` in `#0a0a0a` on `#fafafa` | 40–∞ px wide | Light mode |
| Signal variant | Brackets themselves in `#5fb878` (letters stay `#ededed`) | Landing page `cargo install` callout only | The one approved signal treatment for this direction |

### Clear space

Minimum padding equal to **1x bracket width (1u)** on all four sides — one full monospace character of breathing room. Looser than Direction B because the brackets already create internal structure.

### Minimum size

- **Full `[mz]`:** 40px wide minimum. Below 40px, the brackets visually fuse with the letters.
- **Long form `[metaphaze]`:** 96px wide minimum.
- **Empty-bracket icon:** 16px floor at favicon sizes only.
- **Print:** 12mm wide for `[mz]`.

### Don'ts

- Never use non-ASCII brackets. No U+FF3B fullwidth, no U+3010 lenticular, no curly `{}`, no parentheses. Only U+005B and U+005D.
- Never nest the brackets in another container — no `[[mz]]`, no box around `[mz]`, no pill.
- Never stretch or condense the brackets. They inherit the monospace's native aspect ratio.
- Never apply the signal accent to letters. Only brackets can take the `#5fb878` treatment, and only in the install-callout variant.
- Never use the signal variant anywhere except the `cargo install mz` callout.
- Never draw the brackets as SVG paths. They must be set as text, in the brand typeface.
- Never animate the brackets.

---

## Recommendation

**Direction B — The Cursor.**

All three directions are defensible. Direction A is the safest and most restrained. Direction C is the most referential and inherits the strongest lineage. But B is the only one that does something none of the others do: **it makes the product's primary surface into the logo itself.**

The cursor is the TUI's state. The TUI is the product. When the Senior Operator sees `mz▌` on the landing page, they are looking at the exact glyph they will see at the prompt five minutes later. There is no gap between brand and tool. The logo is not a metaphor for the product — it is a literal screenshot of the product, compressed into two characters and a block cursor.

Direction A makes the wordmark beautiful. Direction C makes it clever. Direction B makes it honest. Honest wins, because honesty is the Sage's weapon and the Senior Operator's favorite flavor.

**Fallback ranking:**
1. Direction B — The Cursor
2. Direction A — The Wordmark
3. Direction C — The Bracket

Direction C drops to third not because it is worse than A, but because its strongest move (the bracket) is already doing heavy lifting in the navigation system (`[/about]  [/docs]`). Using brackets as both nav and logo risks over-signaling the htmx reference. Better to let the brackets own navigation and the cursor own identity.

---

## Related

- [color-system.md](./color-system.md)
- [typography.md](./typography.md)
- [imagery-style.md](./imagery-style.md)
- [brand-applications.md](./brand-applications.md)
