# Prompt Input

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the `$` / `>` / `~` prefixed input — no border, no background, no focus ring. just a shell-style prompt prefix and the input text followed by a blinking cursor. override of shadcn's `<Input>` that strips every default affordance.

## Concept

the brand does not draw input boxes. an input is a line of text on an otherwise empty row, prefixed by a prompt character that announces which kind of input this is: `$` for commands, `>` for a TUI/REPL prompt, `~` for file path input. the cursor is the focus indicator. no ring, no outline, no background fill.

## Anatomy

```
$ cargo install mz▌
> initiate phase-01▌
~ /Users/matt/workspace/metaphaze/▌
```

- **prefix:** single character (`$`, `>`, `~`) in `mz.dust`
- **separator:** one space after the prefix (brand rule — never tight)
- **value:** user input in `mz.bone`
- **cursor:** `▌` in `mz.signal`, blinking on live surfaces
- no border, no background, no padding except the one space after the prefix
- the whole input occupies a single row

## States

| state | visual | rule |
|---|---|---|
| empty | `$ ▌` (prefix + space + blinking cursor) | placeholder text is NEVER used — the prompt prefix is the affordance |
| filled | `$ cargo install mz` (no cursor shown when blurred) | cursor only visible when focused |
| focused | `$ cargo install mz▌` (cursor at caret position) | cursor drawn in signal, blinks at 530ms cycle |
| disabled | `$ cargo install mz` in `mz.dust` | dimmed, no cursor, no interaction |

no error state on the input itself — errors are reported in a status line below the prompt, badged with `[ERR]`.

## Prefix semantics

the prefix is not decoration. it tells the reader what category of input this is, just like a real shell:

- `$ ` — shell-style command (`$ cargo install mz`). use for anything the user might paste into a terminal.
- `> ` — REPL or TUI prompt (`> initiate`). use for in-app commands the user types at the metaphaze TUI.
- `~ ` — file path or home-rooted location (`~ /src/tui.rs`). use for path inputs.

pick one and stick with it per input — never mix prefixes in the same form.

## Rust / ratatui

render the prompt as a `Paragraph` with three spans: prefix, value, cursor. the cursor span is only added when the input is focused, and the blink state is driven by the app's tick rate.

```rust
use crate::tui::theme::{brand_text, brand_muted, brand_cursor};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub struct PromptInput {
    pub prefix: char,       // '$', '>', or '~'
    pub value: String,
    pub focused: bool,
    pub blink_on: bool,     // toggled by app tick at 530ms
}

pub fn render_prompt(input: &PromptInput) -> Paragraph<'_> {
    let prefix = format!("{} ", input.prefix);
    let mut spans = vec![
        Span::styled(prefix, brand_muted()),
        Span::styled(input.value.clone(), brand_text()),
    ];
    if input.focused && input.blink_on {
        spans.push(Span::styled("\u{258C}", brand_cursor()));
    }
    Paragraph::new(Line::from(spans))
}

// in the app tick handler
if last_blink.elapsed() >= Duration::from_millis(530) {
    prompt.blink_on = !prompt.blink_on;
    last_blink = Instant::now();
}
```

no `Block` wrapping — the prompt stands alone on its row. if it needs a label ("enter command"), put the label on a preceding row in `mz.dust`, not as a placeholder or border title.

## Web / Tailwind v4 (shadcn override)

shadcn's `<Input>` ships with `border`, `bg-background`, `ring-offset`, `focus-visible:ring-2`, and `rounded-md` — all five banned. override via CSS class and render the prefix as a sibling span or a `::before` pseudo.

```css
/* app/globals.css */
.mz-input-wrap {
  display: flex;
  align-items: baseline;
  gap: 0.5ch;
  font-family: var(--font-mono);
  font-size: var(--text-base);
  line-height: var(--leading-loose);
}
.mz-input-wrap > .mz-prompt {
  color: var(--color-mz-fg-muted);
  user-select: none;
  flex-shrink: 0;
}
.mz-input-wrap > input {
  flex: 1;
  background: transparent;
  border: none;
  padding: 0;
  color: var(--color-mz-fg);
  font: inherit;
  caret-color: var(--color-mz-signal);
  outline: none;
}
.mz-input-wrap > input::placeholder { color: transparent; }
.mz-input-wrap > input:focus { outline: none; }
.mz-input-wrap > input:disabled {
  color: var(--color-mz-fg-muted);
  caret-color: transparent;
}
```

```tsx
// components/mz-input.tsx
type PromptKind = "shell" | "repl" | "path";

const prefixes: Record<PromptKind, string> = {
  shell: "$",
  repl: ">",
  path: "~",
};

type Props = React.InputHTMLAttributes<HTMLInputElement> & {
  kind?: PromptKind;
  label?: string;
};

export function MzInput({ kind = "shell", label, id, ...props }: Props) {
  const prefix = prefixes[kind];
  return (
    <div className="mz-input-wrap">
      <span className="mz-prompt" aria-hidden="true">{prefix}</span>
      <input
        id={id}
        aria-label={label ?? `${kind} input`}
        {...props}
      />
    </div>
  );
}

// usage
<MzInput kind="shell" label="install command" defaultValue="cargo install mz" />
```

the native browser caret is set to `var(--color-mz-signal)` via `caret-color`, which gives a reasonable approximation of the `▌` cursor. for a pixel-perfect blinking block cursor, see [cursor.md](./cursor.md) — the web implementation hides the native caret and renders a CSS-animated `▌` span after the last character.

## Usage rules

- **no placeholder text.** the prompt prefix IS the affordance. "enter your email" in gray is banned.
- **no border.** the input does not live inside a rounded box. it lives on a row.
- **no label above.** if you must label the input, use a short line of `mz.dust` text on the row above. no `<label>` inside a `<div>` inside a `<form>` stack.
- **no icon inside.** no magnifying glass, no calendar icon, no eye icon for passwords. for passwords, use `type="password"` and trust the browser's masking.
- **never more than one input per row.** two inputs side-by-side read as a form widget. stack them.
- **prefix is required.** every prompt input MUST have a prefix character. a bare input with no prefix is not a metaphaze input.
- **prefix is semantic.** `$` = shell, `>` = repl, `~` = path. never mix.

## Accessibility

- `<input>` is a real semantic input — `type`, `name`, `aria-label` all work as expected
- prefix span is `aria-hidden="true"` — decorative only, the aria label carries the full meaning
- no focus ring because the blinking caret / cursor serves as the focus indicator — this relies on the browser's `caret-color` being honored, which is robust across modern browsers
- if `prefers-reduced-motion: reduce` is set, the cursor freezes in its "on" state (see [cursor.md](./cursor.md))

## Related

- [token-mapping.md](./token-mapping.md)
- [cursor.md](./cursor.md) — the `▌` blink at the caret
- [../STYLE.md](../STYLE.md) — patterns > input
