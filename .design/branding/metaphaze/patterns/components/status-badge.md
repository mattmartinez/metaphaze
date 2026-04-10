# Status Badge

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the bracketed uppercase status code — `[OK]` `[ERR]` `[WARN]` `[INFO]`. this is the only place UPPERCASE appears in the entire brand. custom to metaphaze — shadcn's `<Badge>` primitive is explicitly not used.

## Concept

a three-to-five character uppercase status code inside square brackets, colored to match its semantic meaning. inline with text, never wrapped in a box, never filled. the whole token — brackets and label — is colored together. brackets are part of the typography.

## Variants

| variant | content | color (dark) | color (light) | use |
|---|---|---|---|---|
| ok | `[OK]` | `mz.signal` `#5fb878` | `#2e8b57` | successful operation, verified state |
| warn | `[WARN]` | `mz.amber.warn` `#d4a017` | `#b8860b` | non-blocking issue, attention required |
| err | `[ERR]` | `mz.amber.deep` `#b8860b` | `#8b6508` | recoverable failure (never red) |
| info | `[INFO]` | `mz.dust` `#8a8a8a` | `#6a6a6a` | annotation, not signal |
| pending | `[…]` | `mz.dust` | `mz.dust` | in-progress or awaiting input |

`[INFO]` uses `mz.dust` because info is not signal — if you want attention, use `[WARN]`.

## Anatomy

```
[OK]     [WARN]     [ERR]     [INFO]     [...]
↑  ↑     ↑    ↑     ↑   ↑     ↑    ↑     ↑  ↑
│  │     │    │     │   │     │    │     │  │
└──┴── brackets are same color as label, always
```

- no padding inside the brackets (`[OK]`, not `[ OK ]`)
- uppercase only — lowercase loses the "machine label" signal
- ASCII brackets only — U+005B `[` and U+005D `]`, never fullwidth or lenticular
- no emoji, no icons, no unicode dots

## Rust / ratatui

helper function that returns a `Span` with the right style. import the style presets from `src/tui/theme.rs`.

```rust
use crate::tui::theme::{brand_badge_ok, brand_badge_warn, brand_badge_err, brand_muted};
use ratatui::text::Span;

pub enum BadgeKind { Ok, Warn, Err, Info, Pending }

pub fn badge(kind: BadgeKind) -> Span<'static> {
    match kind {
        BadgeKind::Ok      => Span::styled("[OK]",   brand_badge_ok()),
        BadgeKind::Warn    => Span::styled("[WARN]", brand_badge_warn()),
        BadgeKind::Err     => Span::styled("[ERR]",  brand_badge_err()),
        BadgeKind::Info    => Span::styled("[INFO]", brand_muted()),
        BadgeKind::Pending => Span::styled("[...]",  brand_muted()),
    }
}

// usage in a ratatui Line
use ratatui::text::Line;
use crate::tui::theme::brand_text;

let line = Line::from(vec![
    Span::styled("phase 3/12 complete  ", brand_text()),
    badge(BadgeKind::Ok),
]);
```

never construct a badge by string concatenation inline — always use `badge(kind)`. this keeps the brand color assignment in one place.

## Web / Tailwind v4

a `<span>` with semantic classes. CSS applies the color to the whole span (brackets and label together).

```css
/* app/globals.css — inside @layer components, or as plain CSS */
.mz-badge {
  font-family: var(--font-mono);
  font-weight: 400;
  white-space: nowrap;
}
.mz-badge-ok   { color: var(--color-mz-signal); }
.mz-badge-warn { color: var(--color-mz-warn); }
.mz-badge-err  { color: var(--color-mz-error); }
.mz-badge-info { color: var(--color-mz-fg-muted); }
```

```tsx
// components/mz-badge.tsx
type BadgeKind = "ok" | "warn" | "err" | "info" | "pending";

const labels: Record<BadgeKind, string> = {
  ok: "[OK]",
  warn: "[WARN]",
  err: "[ERR]",
  info: "[INFO]",
  pending: "[...]",
};

const ariaLabels: Record<BadgeKind, string> = {
  ok: "status ok",
  warn: "status warning",
  err: "status error",
  info: "status info",
  pending: "status pending",
};

export function MzBadge({ kind }: { kind: BadgeKind }) {
  return (
    <span
      className={`mz-badge mz-badge-${kind === "pending" ? "info" : kind}`}
      role="status"
      aria-label={ariaLabels[kind]}
    >
      {labels[kind]}
    </span>
  );
}
```

use inline with text: `<p>phase 3/12 complete <MzBadge kind="ok" /></p>`.

## Usage rules

- **inline only.** never place a badge in a filled pill or rounded container.
- **caps only.** `[OK]`, not `[Ok]` or `[ok]`. caps are the whole point — they signal "machine label."
- **one badge per line.** multiple badges on one line read as a lint output, not a status line.
- **place at the end of the line** when summarizing a row of data: `phase 3/12 complete <SPACE> [OK]`.
- **lowercase surrounding text.** a badge at the end of a sentence should be preceded by lowercase words. mixing title case with caps badges reads as decoration.
- **never in body copy prose.** badges belong in status lines, CLI output, and README tables — not in paragraphs.
- **no red.** `[ERR]` is `mz.amber.deep`. metaphaze errors are recoverable states, not catastrophes.
- **no emoji substitute.** never `✅` or `❌` or `⚠️` — the brand survives `LANG=C`, so the badges must be plain ASCII plus ANSI color only.

## Accessibility

- `role="status"` on the span (web) — screen readers announce status changes
- `aria-label` describes the status in full words (`"status ok"`, not `"OK"`) — sighted users see the bracketed code, assistive tech hears the semantic meaning
- color is never the only signal — the `[OK]` / `[ERR]` / `[WARN]` text is readable in monochrome, so `NO_COLOR` terminals and color-blind users get the same information

## Related

- [token-mapping.md](./token-mapping.md)
- [../STYLE.md](../STYLE.md) — patterns > badge
- [../../identity/color-system.md](../../identity/color-system.md) — semantic token assignments
