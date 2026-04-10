# Guidelines
> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

## Core

| File | Description |
|------|-------------|
| [metaphaze.yml](./metaphaze.yml) | Style preset — single source of truth (tokens, patterns, constraints, effects, rust_bindings, web_bindings) |
| [STYLE.md](./STYLE.md) | Agent contract rendered from the .yml |
| [guidelines.html](./guidelines.html) | Visual brand guide (open in browser) |

## Components

| File | Type | Description |
|------|------|-------------|
| [token-mapping.md](./components/token-mapping.md) | mapping | Brand tokens → ratatui (Rust) + Tailwind v4 / shadcn/ui (web) |
| [pane.md](./components/pane.md) | custom | The TUI pane — bordered box with title bar |
| [status-badge.md](./components/status-badge.md) | custom | `[OK]` `[ERR]` `[WARN]` bracketed status codes |
| [bracketed-button.md](./components/bracketed-button.md) | override | `[ initiate ]` button (shadcn Button override + ratatui pattern) |
| [prompt-input.md](./components/prompt-input.md) | override | `$` prefixed input (shadcn Input override + ratatui pattern) |
| [phase-transition.md](./components/phase-transition.md) | custom | The ASCII phase transition screen — signature visual moment |
| [cursor.md](./components/cursor.md) | custom | The blinking `▌` logo cursor (Rust + CSS implementations) |

All components mapped. brand complete.
