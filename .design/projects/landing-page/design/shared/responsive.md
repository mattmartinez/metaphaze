# Responsive Behavior

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Breakpoints

The brand uses a binary breakpoint system. No tablet-specific treatment.

| Breakpoint | Value | Target | Layout |
|------------|-------|--------|--------|
| mobile | `< 640px` | smartphone portrait | single column, stacked |
| desktop | `>= 640px` | laptop/desktop/tablet | max-width narrow column, centered |

Tailwind v4 tokens used: `sm:` prefix for `>= 640px`.

The overall layout is always a single reading column. The brand archetype is `monospace-editorial` — never a grid of cards, never a multi-column split. Responsive changes are adjustments within that column, not layout swaps.

---

## Global Adaptive Behaviors

| Property | Mobile (`< 640px`) | Desktop (`>= 640px`) |
|----------|--------------------|----------------------|
| Max-width | `w-full` + `px-4` gutters | `max-w-3xl` (`48rem`) centered, `px-0` |
| Body font-size | `--text-sm` (`0.9rem`) | `--text-base` (`1rem`) |
| Section spacing | `py-12` | `py-16` to `py-24` |
| Nav height | `40px` | `48px` |
| Nav font-size | `--text-xs` | `--text-sm` |

---

## Section-by-Section

### Nav bar

| | Mobile | Desktop |
|---|---|---|
| Logo | `mz▌` glyph only, `--text-base` | `mz▌` glyph, `--text-base` |
| Nav links | `[/docs] [/source]` inline, `--text-xs` | `[/docs] [/source]` inline, `--text-sm` |
| Layout | `flex justify-between` | same inside `max-w-3xl` container |

### Hero

| | Mobile | Desktop |
|---|---|---|
| Logo | `mz▌` at `--text-4xl` (clamp-responsive) | `mz▌` at `--text-5xl` |
| Manifesto | `--text-sm`, line-height 1.6, wraps naturally | `--text-base`, wider line |
| Install teaser | full width, `overflow-x: auto` on code block | full `max-w-3xl` width |
| VHS `<video>` | `<picture>` src switches to mobile variant (1200×600 crop, tighter) | desktop variant (1600×900) |
| Phase transition screenshot | `font-size: --text-xxs` to fit in column | `--text-sm` |
| Padding-top | `pt-16` (clears fixed nav) | `pt-20` |

### What it does

| | Mobile | Desktop |
|---|---|---|
| Prose | full width, `--text-sm` | `max-w-2xl` inside column |
| Box-drawing diagram | `overflow-x: auto`, `white-space: pre`, horizontal scroll if needed | `white-space: pre`, no scroll needed at `48rem` |

The diagram is designed to fit within 66 characters (a conservative terminal column count). At `--text-sm` on a 320px viewport this is approximately 55 characters visible — add `overflow-x: auto` on the containing `<pre>` for sub-400px screens.

### Why it's different

| | Mobile | Desktop |
|---|---|---|
| Comparison table | stacks vertically: feature name row + badge row per property | horizontal table, bordered pane |
| Badge layout | block-level per row | inline in table cells |

Mobile layout for comparison table:
```
┌─ comparison ──────────────┐
│ api access                │
│   mz:          [FIRST-PARTY]  │
│   other tools: [THIRD-PARTY] │
│                           │
│ config file               │
│   mz:       [OK]          │
│   ...                     │
└───────────────────────────┘
```

Desktop layout: standard bordered `<table>` inside a `<Pane>`.

### Install section

| | Mobile | Desktop |
|---|---|---|
| Code block | full width, `overflow-x: auto`, visible scrollbar | full `max-w-3xl` width, no scroll |
| `[ copy ]` button | below the code block (stacked) | top-right corner of the pane |

### Footer

| | Mobile | Desktop |
|---|---|---|
| License + GitHub | stacked vertically | inline with `·` separators |
| Refusals list | stacked: one item per line | single line with `·` separators, wraps |
| Manifesto | visible, `--text-xs`, `--mz-fg-muted` | same |

---

## Art Direction (VHS video)

```html
<picture>
  <source
    media="(max-width: 639px)"
    srcset="/vhs/demo-mobile.webm"
    type="video/webm">
  <video
    src="/vhs/demo-desktop.webm"
    autoplay muted loop playsinline
    class="mz-vhs"
    aria-label="metaphaze CLI — autonomous Rust build demo"
    width="1600" height="900">
  </video>
</picture>
```

Both variants share the brand theme (bg `#0a0a0a`, fg `#ededed`, signal `#5fb878`). Mobile variant is cropped to show the key part of the phase transition without the surrounding shell chrome.

---

## `prefers-color-scheme`

The CSS custom properties in `:root` flip automatically at `@media (prefers-color-scheme: light)`. No manual theme toggle. No `.dark` class. OS preference is the only input.

When in light mode:
- `--mz-bg: #fafafa` / `--mz-fg: #0a0a0a` — inverted
- `--mz-signal: #2e8b57` — darker signal for AA contrast on white
- All other tokens adjust per `token-mapping.md`

No layout changes in light mode — the column, spacing, and component structure stay identical.

---

## Related

- [navigation.md](./navigation.md)
- [micro-interactions.md](./micro-interactions.md)
- [../screen-01-landing.md](../screen-01-landing.md)
