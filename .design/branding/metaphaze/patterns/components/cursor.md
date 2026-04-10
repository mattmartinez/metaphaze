# Cursor

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the blinking `▌` logo cursor — U+258C LEFT HALF BLOCK in `mz.signal`, 530ms on / 530ms off, `step-end` easing. the cursor is the single most important glyph in the brand. it is the logo's primary element and the interaction vocabulary's primary technique. it is a literal Unicode character, always — never an SVG rectangle, never a CSS box drawn with a pseudo-element shape.

## Concept

the cursor is the running state of the tool made typographic. when the metaphaze TUI is waiting for input, a cursor blinks at the prompt. when the landing page shows the `mz▌` logo, the same cursor blinks. there is no gap between brand and product — the logo is a screenshot of the product, compressed to 2.5 monospace cells. the senior operator seeing `mz▌` on the landing page is looking at the exact glyph they will see at the prompt five minutes later.

## Variants

| variant | use | animation |
|---|---|---|
| static | PDF, print, favicon, social preview, screenshots, any medium that cannot animate | frozen in the "on" state, full opacity |
| blinking | live TUI, live web, live README rendering in a browser | 530ms on / 530ms off, step-end easing, 1.06s total cycle |

no other variants. no "fast blink," no "slow blink," no "pulse," no "halo," no "glow." the cursor is 530ms on / 530ms off, always.

## The character

```
U+258C  ▌  LEFT HALF BLOCK
```

- always this character — never `█` (U+2588 full block), never `|` (ASCII pipe), never a rendered rectangle
- always colored `mz.signal` (`#5fb878` dark / `#2e8b57` light)
- never any other color
- never with a halo, glow, text-shadow, or drop-shadow
- never italicized, never transformed (no scale, no rotate, no translate)

## Rust / ratatui

the cursor is a `Span` with the `▌` glyph and the `brand_cursor()` style. the blink is driven by the app's tick rate (toggle a `blink_on` boolean every 530ms) — do not rely on `Modifier::SLOW_BLINK` because terminal support is inconsistent.

```rust
use crate::tui::theme::{brand_cursor, brand_text};
use ratatui::text::{Line, Span};
use std::time::{Duration, Instant};

/// Render the cursor glyph. Returns an empty span when the blink is "off".
pub fn cursor_span(blink_on: bool) -> Span<'static> {
    if blink_on {
        Span::styled("\u{258C}", brand_cursor())
    } else {
        Span::raw(" ")  // width-preserving off-state
    }
}

// ── driving the blink from the app tick loop ────────────────────────────────
pub struct CursorBlink {
    on: bool,
    last: Instant,
}

impl CursorBlink {
    pub fn new() -> Self {
        Self { on: true, last: Instant::now() }
    }

    /// Call on every tick. Toggles the blink state if 530ms has elapsed.
    pub fn tick(&mut self) {
        if self.last.elapsed() >= Duration::from_millis(530) {
            self.on = !self.on;
            self.last = Instant::now();
        }
    }

    pub fn on(&self) -> bool { self.on }
}

// ── usage: the mz▌ logo line ─────────────────────────────────────────────────
let blink = app.cursor_blink.on();  // CursorBlink state on the app
let logo = Line::from(vec![
    Span::styled("mz", brand_text()),
    cursor_span(blink),
]);
```

the `cursor_span` returns `Span::raw(" ")` in the off state rather than an empty string, so the grid column stays stable (ratatui won't collapse the row when the cursor disappears). the whole blink is driven by `CursorBlink::tick()` called once per app tick — no timer thread, no interval callback.

if `std::env::var("NO_COLOR").is_ok()`, swap `brand_cursor()` for `brand_text()` — the cursor glyph still appears but in `mz.bone` instead of `mz.signal`. the brand still reads.

## Web / HTML / CSS

the cursor is a `<span>` containing the literal `▌` character, animated with a CSS keyframe using `step-end` easing. no JavaScript, no `setInterval`, no canvas — just a CSS class.

```css
/* app/globals.css — already present in token-mapping.md */
@keyframes mz-cursor-blink {
  0%, 49.99% { opacity: 1; }
  50%, 100%  { opacity: 0; }
}

.mz-cursor {
  display: inline-block;
  color: var(--color-mz-signal);
  font-weight: 400;
  animation: mz-cursor-blink 1.06s step-end infinite;
}

/* respect prefers-reduced-motion — freeze in the "on" state */
@media (prefers-reduced-motion: reduce) {
  .mz-cursor { animation: none; opacity: 1; }
}

/* static variant — never animated, always visible */
.mz-cursor--static { animation: none; opacity: 1; }
```

```tsx
// components/mz-cursor.tsx
type Props = { static?: boolean };

export function MzCursor({ static: isStatic }: Props) {
  return (
    <span
      className={isStatic ? "mz-cursor mz-cursor--static" : "mz-cursor"}
      aria-hidden="true"
    >
      {"\u258C"}
    </span>
  );
}

// usage — the mz▌ logo lockup
<span className="font-mono text-mz-fg">
  mz<MzCursor />
</span>

// usage — a favicon or social preview (static)
<MzCursor static />
```

the `1.06s` duration is `530ms on + 530ms off = 1060ms total`. `step-end` makes the opacity flip instantaneous — no fade. the cursor snaps.

## Usage rules

- **always the literal U+258C character.** never an SVG `<rect>`, never a CSS `::after` content with `▌` unless the content is the glyph string, never a Lottie file, never an image.
- **always `mz.signal` color.** dark mode `#5fb878`, light mode `#2e8b57`. no other colors ever. not even for "themed" variants.
- **always the same blink rate.** 530ms on, 530ms off. this matches the default terminal cursor blink and is the physically-correct rate for the brand.
- **always `step-end` easing.** the cursor does not fade. it snaps.
- **static on static media.** PDF, print, favicon, social preview images, and any screenshot used as a hero asset → the cursor is frozen in the "on" state.
- **blinking on live surfaces only.** the TUI, the landing page, and the README when rendered in a browser. never in an email signature, never in a static image.
- **`aria-hidden="true"` on the web.** the cursor is decorative — it does not carry information that screen readers need to hear.
- **`prefers-reduced-motion: reduce`** freezes the cursor in the "on" state. this is non-negotiable.
- **never compose with other modifiers.** no bold cursor, no italic cursor, no underlined cursor, no cursor with a drop-shadow.

## Accessibility

- `aria-hidden="true"` on the span — the cursor is visual only
- `prefers-reduced-motion: reduce` is respected — cursor freezes, brand still reads
- `NO_COLOR` is respected in the TUI — cursor becomes `mz.bone` (still visible, still the `▌` glyph)
- the cursor never carries unique information — anything it would convey is also carried in the focus semantics of the focused element

## Related

- [token-mapping.md](./token-mapping.md) — `brand_cursor()` preset
- [prompt-input.md](./prompt-input.md) — the cursor follows the input caret
- [bracketed-button.md](./bracketed-button.md) — the cursor appears after the `]` on active/focused buttons
- [phase-transition.md](./phase-transition.md) — phase transition screens do NOT blink (static render)
- [../STYLE.md](../STYLE.md) — bold bet #1, the cursor logo
