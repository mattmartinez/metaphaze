# Pane

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the bordered box with a title bar — the primary container in every metaphaze surface. this is the component the senior operator looks at the most. custom to the brand — shadcn has no equivalent.

## Concept

a flat, single-column rectangle drawn with box-drawing characters, with an optional title padded into the top border and optional content and footer regions. no radius, no shadow, no elevation — the pane is drawn, not rendered. the pane is the brand's reading surface.

## Anatomy

```
┌─ tracks — phase-01 ───────────────────────────┐
│ ◆ discover    3/3  ● complete                 │
│ ◆ strategy    2/3  ▶ active                   │
│ ○ identity    0/3  — pending                  │
│                                               │
├───────────────────────────────────────────────┤
│  5 tracks · 8/12 steps · 2h17m                │
└───────────────────────────────────────────────┘
```

- **border:** `─ │ ┌ ┐ └ ┘` in `mz.slate` (idle) or `mz.signal` (active)
- **title bar:** top-left, padded with one space on each side, rendered in `mz.bone`
- **content:** interior, one cell of horizontal padding from the border
- **divider (optional):** `├───┤` horizontal rule separating content from footer
- **footer (optional):** status summary line in `mz.dust`

## States

| state | border | when |
|---|---|---|
| idle | `mz.slate` | pane is visible but not focused |
| active | `mz.signal` | pane has keyboard focus (tab-cycled) |
| completed | `mz.dust` | pane represents finished work (e.g. completed phase tracker) |
| blocked | `mz.amber.warn` | pane contains at least one blocked step — the only time amber touches a border |

no hover state on panes. panes are read, not clicked.

## Rust / ratatui

use the `brand_pane` and `brand_pane_active` helpers from `src/tui/theme.rs` (see [token-mapping.md](./token-mapping.md)). do not hand-roll `Block::default().borders(Borders::ALL)` in `src/tui.rs` — the helpers exist so the brand stays consistent.

```rust
use crate::tui::theme::{brand_pane, brand_pane_active, brand_text};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::text::{Line, Span};

// idle pane — render once
let body = Paragraph::new(vec![
    Line::from(vec![
        Span::styled("◆ discover    3/3  ", brand_text()),
        Span::styled("● complete", Style::default().fg(MZ_SIGNAL)),
    ]),
    Line::from("◆ strategy    2/3  ▶ active"),
    Line::from("○ identity    0/3  — pending"),
])
.block(brand_pane("tracks — phase-01"))
.wrap(Wrap { trim: false });

frame.render_widget(body, chunks[0]);

// active (focused) pane
let output = Paragraph::new(output_lines)
    .block(brand_pane_active("output"));
frame.render_widget(output, chunks[1]);
```

the existing `src/tui.rs` already constructs panes this way at line ~849 — the migration is mechanical: replace every inline `Block::default().title(...).borders(Borders::ALL).border_style(...)` with `brand_pane(title)` / `brand_pane_active(title)`.

## Web / Tailwind v4

a `<section>` with a 1px border and a dedicated `.mz-pane-title` line. box-drawing characters can be rendered inside the title, but on the web the common pattern is to render the title as a real `<h2>` above a normal bordered div.

```tsx
// components/mz-pane.tsx
type PaneProps = {
  title: string;
  children: React.ReactNode;
  active?: boolean;
  footer?: React.ReactNode;
};

export function MzPane({ title, children, active, footer }: PaneProps) {
  return (
    <section
      role="region"
      aria-label={title}
      className={[
        "border bg-mz-bg font-mono text-mz-fg",
        active ? "border-mz-signal" : "border-mz-border",
      ].join(" ")}
    >
      <header className="border-b border-mz-border px-4 py-2 text-sm">
        <span className="text-mz-fg-muted">─ </span>
        <span>{title}</span>
        <span className="text-mz-fg-muted"> ─</span>
      </header>
      <div className="p-4 leading-loose">{children}</div>
      {footer && (
        <footer className="border-t border-mz-border px-4 py-2 text-sm text-mz-fg-muted">
          {footer}
        </footer>
      )}
    </section>
  );
}
```

no `rounded-*`, no `shadow-*`, no `bg-card` — the global `*` override already forces radius and shadow to zero (see [token-mapping.md](./token-mapping.md)). the pane is flat by construction.

## Usage rules

- always flat. no `box-shadow`, no `border-radius`, no `bg-gradient-*`, no `backdrop-filter`.
- title is lowercase, padded with one space on each side.
- title may include an em-dash separator (` — `) for hierarchy: `tracks — phase-01`.
- box-drawing in the title is optional on the web (title bar is drawn with CSS border), mandatory in the TUI (the title is text inside a box-drawing border).
- never nest panes more than one level deep. the column is narrow — if you need a second level, use a horizontal rule (`├───┤`) instead.
- the pane footer is optional and reserved for a single status summary line. never more than one line.

## Accessibility

- `role="region"` on the `<section>`
- `aria-label` from the pane title (not `aria-labelledby` — the title bar is visually decorative box-drawing, and the semantic label should be the plain title text)
- focus ring is suppressed globally; the pane's `border-mz-signal` state is the focus indicator

## Related

- [token-mapping.md](./token-mapping.md)
- [phase-transition.md](./phase-transition.md) — panes are the containers phase-transition screens live in
- [../STYLE.md](../STYLE.md)
