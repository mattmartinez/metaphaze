# Bracketed Button

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the `[ initiate ]` button — text-only, square brackets, lowercase label, video-invert hover. this is the brand's primary call-to-action pattern. on the web it's an override of shadcn's `<Button>` that strips the default styling entirely. in the TUI it's a `Paragraph` at the app level.

## Concept

a button is a lowercase word wrapped in square brackets with one space of padding inside the brackets. no background fill, no border, no radius, no shadow, no icon. the only affordance is the brackets themselves — they say "this is clickable" without needing a box.

## Anatomy

```
[ initiate ]    [ cancel ]    [ confirm ]    [ resume ]
↑ ↑         ↑    ↑ ↑      ↑
│ │         │    │ │      │
│ └─ label ─┘    │ └──────┘ label
│                │
└── bracket + one space of padding on each side
```

- label: lowercase, one or two words maximum (`initiate`, `cancel`, `resume`, `mark done`)
- brackets: ASCII `[` and `]`, with exactly one space between bracket and label
- color (idle): `mz.bone` on transparent background
- no icon, no emoji, no divider, no chevron

## States

| state | visual | brand rule |
|---|---|---|
| default | `[ initiate ]` in `mz.bone` on transparent | bracketed text, nothing else |
| hover | `[ initiate ]` in `mz.black` on `mz.bone` (video-invert) | `video-invert` interaction vocabulary |
| active | `[ initiate ]▌` with cursor appended after the closing bracket | `cursor-append` interaction vocabulary |
| disabled | `[ initiate ]` in `mz.dust` on transparent | muted, no hover state |

no focus ring — if the button has keyboard focus, the cursor `▌` appears after the closing bracket (same as active state). no transition — video-invert is `step-end` (instant).

## Rust / ratatui

ratatui doesn't have a "button" widget — buttons are rendered as `Paragraph` spans and click/key events are handled at the app level (match on the focused element in the input handler).

```rust
use crate::tui::theme::{brand_text, brand_muted, brand_invert, brand_cursor};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub enum ButtonState { Idle, Hovered, Active, Disabled }

pub fn bracketed_button(label: &str, state: ButtonState) -> Paragraph<'_> {
    let text = format!("[ {} ]", label);
    let line = match state {
        ButtonState::Idle => {
            Line::from(Span::styled(text, brand_text()))
        }
        ButtonState::Hovered => {
            // video-invert — reverse fg/bg
            Line::from(Span::styled(text, brand_invert()))
        }
        ButtonState::Active => {
            // cursor appended after the closing bracket
            Line::from(vec![
                Span::styled(text, brand_text()),
                Span::styled("\u{258C}", brand_cursor()),
            ])
        }
        ButtonState::Disabled => {
            Line::from(Span::styled(text, brand_muted()))
        }
    };
    Paragraph::new(line)
}

// usage
let btn = bracketed_button("initiate", ButtonState::Idle);
frame.render_widget(btn, button_area);
```

the app's input handler tracks which button (if any) is focused and re-renders with `ButtonState::Active` when the user presses `Tab` to it. on `Enter`, the handler dispatches the button's action.

## Web / Tailwind v4 (shadcn override)

the shadcn `<Button>` ships with a default variant that has a `rounded-md` radius, a background color, a hover shadow, and a focus ring — all four of which are banned by the brand. the override pattern is to either pass `className` to replace every default style, or wrap shadcn's `Button` with our own.

```css
/* app/globals.css */
.mz-btn {
  background: transparent;
  border: none;
  padding: 0;
  color: var(--color-mz-fg);
  font-family: var(--font-mono);
  font-size: var(--text-base);
  line-height: var(--leading-loose);
  cursor: pointer;
  transition: none;  /* step-end, no fade */
}
.mz-btn::before { content: "[ "; color: var(--color-mz-fg); }
.mz-btn::after  { content: " ]"; color: var(--color-mz-fg); }
.mz-btn:hover {
  background: var(--color-mz-fg);
  color: var(--color-mz-bg);
}
.mz-btn:hover::before,
.mz-btn:hover::after { color: var(--color-mz-bg); }
.mz-btn:focus-visible { outline: none; }  /* cursor indicates focus */
.mz-btn:focus-visible::after { content: " ]\u258C"; color: var(--color-mz-signal); }
.mz-btn[disabled] {
  color: var(--color-mz-fg-muted);
  cursor: default;
}
.mz-btn[disabled]:hover {
  background: transparent;
  color: var(--color-mz-fg-muted);
}
```

```tsx
// components/mz-button.tsx — shadcn override wrapper
import { Button as ShadcnButton, type ButtonProps } from "@/components/ui/button";

export function MzButton({ children, className, ...props }: ButtonProps) {
  return (
    <ShadcnButton
      {...props}
      className={`mz-btn ${className ?? ""}`}
      variant="ghost"  // strips shadcn's default bg, but we override everything anyway
    >
      {children}
    </ShadcnButton>
  );
}

// usage — label must be a plain lowercase string, no JSX children
<MzButton onClick={initiate}>initiate</MzButton>
<MzButton onClick={cancel}>cancel</MzButton>
```

the `::before` / `::after` pseudo-elements add the brackets via CSS `content`, which keeps the JSX clean and ensures brackets can never drift out of sync with the button. the bracket spacing is baked into the content string (`[ ` and ` ]`).

## Usage rules

- **always bracketed.** never `initiate` alone, never `Initiate` with title case, never `INITIATE` in caps.
- **always lowercase.** uppercase is reserved for status badges only.
- **one or two words max.** `initiate`, `cancel`, `mark done`. if you need three words, the action isn't bracketed button territory — use a link or a prompt.
- **never with an icon.** no chevrons, no arrows, no loading spinners. if the button is busy, swap the label for `[ running... ]`.
- **never in a row of more than three.** a row of brackets reads as a menu, not a CTA.
- **primary + secondary are visually identical.** the only difference is intent. there is no "outline" variant or "ghost" variant or "destructive" variant — bracketed text handles all cases.

## Accessibility

- semantic `<button>` element (web) or real focusable widget (TUI)
- `aria-label` if the label is cryptic (e.g. `[ init ]` should carry `aria-label="initiate phase"`)
- `aria-disabled` when `ButtonState::Disabled`
- the `▌` cursor on active/focus is `aria-hidden="true"` — it's a visual indicator only, screen readers use the native focus semantics

## Related

- [token-mapping.md](./token-mapping.md)
- [cursor.md](./cursor.md) — the `▌` appended on active state
- [../STYLE.md](../STYLE.md) — patterns > button-primary, button-secondary
