# Phase Transition

> Phase: guidelines | Brand: metaphaze | Generated: 2026-04-10

the ASCII phase transition screen — a bordered pane containing a pipeline progress line, a completion banner, and a file tree of newly-written outputs. this is metaphaze's signature visual moment. when a phase completes, this screen is what the senior operator sees. if anything in the brand has to be unforgettable, it's this.

## Concept

phase transitions are rendered as plain monospace text inside a brand pane. no animation, no spinners, no progress bars. the screen is drawn once when the phase finishes and stays on screen until the user presses a key. the entire screen is typeable, pasteable, and renders identically in any terminal. if the screen cannot survive `cat transition.txt`, it's the wrong screen.

## Anatomy

```
┌─ metaphaze — phase complete ─────────────────────────────┐
│                                                          │
│  ◆ discover  ─── ◆ strategy  ─── ◆ identity  ─── ◈ guidelines │
│                                                          │
│  ◆ identity complete — 5 files written                   │
│                                                          │
│  .design/branding/metaphaze/identity/                    │
│  ├── color-system.md                                     │
│  ├── typography.md                                       │
│  ├── logo-directions.md                                  │
│  ├── imagery-style.md                                    │
│  └── brand-applications.md                               │
│                                                          │
│  next: guidelines                                        │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

three regions, stacked with one blank row between each:

1. **pipeline line** — phase names separated by `─── `, with `◆` (completed / current) and `◈` (in-progress / next) diamond glyphs
2. **completion banner** — `◆ {phase} complete — {message}` in `mz.signal` bold
3. **file tree** — the output files for the just-completed phase, drawn with `├── ` `└── ` box-drawing

optionally, a fourth line at the bottom: `next: {phase}` or `done` in `mz.dust`.

## Colors

| element | color | modifier |
|---|---|---|
| pane border | `mz.slate` | — |
| pipeline line (completed phases) | `mz.signal` | BOLD |
| pipeline line (current phase) | `mz.signal` | BOLD |
| pipeline line (pending phases) | `mz.dust` | — |
| pipeline separators (`─── `) | `mz.dust` | — |
| completion banner diamond `◆` | `mz.signal` | BOLD |
| completion banner text | `mz.bone` | — |
| completion banner message (after em-dash) | `mz.bone` | — |
| file tree root directory | `mz.dust` | — |
| file tree branches (`├── └──`) | `mz.dust` | — |
| file tree file names | `mz.bone` | — |
| `next:` line | `mz.dust` | — |

`mz.signal` appears exactly three times in this screen: the completed-phase diamonds in the pipeline, the completion banner diamond, and nowhere else. that's the entire signal budget for the transition. it stays well under the 1% rule.

## Rust / ratatui

phase transition rendering already exists in `src/tui.rs` (phase completion screen). the refactor is to extract the rendering into `src/tui/phase_transition.rs` that uses the brand theme presets and box-drawing glyphs from `metaphaze.yml`.

```rust
use crate::tui::theme::{brand_pane, brand_text, brand_muted, brand_signal, MZ_SIGNAL};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub struct PhaseTransition {
    pub phases:  Vec<(String, PhaseState)>,  // (name, state)
    pub current: usize,
    pub message: String,                     // e.g. "5 files written"
    pub files:   Vec<String>,                // relative paths
    pub root:    String,                     // e.g. ".design/branding/metaphaze/identity/"
    pub next:    Option<String>,             // next phase name, or None if done
}

pub enum PhaseState { Done, Current, Pending }

pub fn render_phase_transition(t: &PhaseTransition) -> Paragraph<'_> {
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    // ── pipeline line ───────────────────────────────────────────────────────
    let mut pipeline_spans = Vec::new();
    for (i, (name, state)) in t.phases.iter().enumerate() {
        let glyph = match state {
            PhaseState::Done    => "\u{25C6}",  // ◆
            PhaseState::Current => "\u{25C6}",  // ◆
            PhaseState::Pending => "\u{25C8}",  // ◈
        };
        let style = match state {
            PhaseState::Done | PhaseState::Current => brand_signal(),
            PhaseState::Pending => brand_muted(),
        };
        pipeline_spans.push(Span::styled(format!("{} {}", glyph, name), style));
        if i + 1 < t.phases.len() {
            pipeline_spans.push(Span::styled("  \u{2500}\u{2500}\u{2500}  ", brand_muted()));
        }
    }
    lines.push(Line::from(pipeline_spans));
    lines.push(Line::from(""));

    // ── completion banner ───────────────────────────────────────────────────
    let phase_name = &t.phases[t.current].0;
    lines.push(Line::from(vec![
        Span::styled("\u{25C6} ", brand_signal()),
        Span::styled(format!("{} complete", phase_name), brand_text()),
        Span::styled(format!(" \u{2014} {}", t.message), brand_text()),
    ]));
    lines.push(Line::from(""));

    // ── file tree ───────────────────────────────────────────────────────────
    lines.push(Line::from(Span::styled(t.root.clone(), brand_muted())));
    for (i, file) in t.files.iter().enumerate() {
        let branch = if i + 1 == t.files.len() { "\u{2514}\u{2500}\u{2500} " } else { "\u{251C}\u{2500}\u{2500} " };
        lines.push(Line::from(vec![
            Span::styled(branch, brand_muted()),
            Span::styled(file.clone(), brand_text()),
        ]));
    }
    lines.push(Line::from(""));

    // ── next line ───────────────────────────────────────────────────────────
    let next_text = match &t.next {
        Some(n) => format!("next: {}", n),
        None    => "done".to_string(),
    };
    lines.push(Line::from(Span::styled(next_text, brand_muted())));

    Paragraph::new(lines).block(brand_pane("metaphaze — phase complete"))
}
```

the pane is drawn with `brand_pane()` so the border is `mz.slate`. the signal color appears only on completed-phase diamonds and the completion banner diamond — three instances total.

## Web / Tailwind v4

on the web, the phase transition is a static `<pre>` element. no animation, no JavaScript — just the raw text wrapped in a brand pane.

```tsx
// components/mz-phase-transition.tsx
type Props = {
  phases: Array<{ name: string; state: "done" | "current" | "pending" }>;
  message: string;
  root: string;
  files: string[];
  next?: string;
};

export function MzPhaseTransition({ phases, message, root, files, next }: Props) {
  const pipeline = phases
    .map((p) => {
      const glyph = p.state === "pending" ? "\u25C8" : "\u25C6";
      return `${glyph} ${p.name}`;
    })
    .join("  \u2500\u2500\u2500  ");

  const current = phases.find((p) => p.state === "current")?.name ?? "";
  const tree = files
    .map((f, i) => {
      const branch = i + 1 === files.length ? "\u2514\u2500\u2500 " : "\u251C\u2500\u2500 ";
      return branch + f;
    })
    .join("\n");

  return (
    <section
      role="region"
      aria-label="phase transition"
      className="border border-mz-border bg-mz-bg p-6 font-mono text-mz-fg"
    >
      <pre className="leading-loose whitespace-pre">
        <span className="text-mz-signal font-semibold">{pipeline}</span>
        {"\n\n"}
        <span className="text-mz-signal font-semibold">{"\u25C6"} </span>
        <span>{current} complete {"\u2014"} {message}</span>
        {"\n\n"}
        <span className="text-mz-fg-muted">{root}</span>
        {"\n"}
        <span className="text-mz-fg-muted">{tree}</span>
        {"\n\n"}
        <span className="text-mz-fg-muted">{next ? `next: ${next}` : "done"}</span>
      </pre>
    </section>
  );
}
```

the `<pre>` preserves whitespace, which preserves the box-drawing alignment. `font-mono` locks the grid. the transition is a literal screenshot of what the TUI renders.

## Usage rules

- **never animated.** no typewriter reveal, no fade-in, no diamond pulse, no progress sweep. it appears instantly.
- **never screenshotted with UI chrome.** when this screen appears on the landing page or a social card, it is the only thing visible in the frame. no browser chrome, no terminal-app title bar, no cropping.
- **never decorated.** no icons, no glyph substitutions, no emoji diamonds. the glyphs are U+25C6 `◆` and U+25C8 `◈` always.
- **never more than 6 phases in the pipeline line.** if the project has more phases, split into two pipeline rows or show only the current + adjacent phases.
- **file list is capped at ~8 entries.** if the phase wrote more, show the first 7 and a `└── … N more` line.
- **the completion banner uses an em-dash (U+2014)**, padded with spaces. never two hyphens.
- **this is the brand's most-used asset on social surfaces.** treat every rendering like a logo lockup.

## Existing implementation

the metaphaze codebase already renders phase transition screens in `src/tui.rs` (see the phase completion logic around the `finished` field on `DashboardState`). when adopting this component spec, the refactor is:

1. extract phase transition rendering into `src/tui/phase_transition.rs`
2. replace inline `Color::Green` / `Color::DarkGray` calls with the `brand_signal()` / `brand_muted()` presets from `src/tui/theme.rs`
3. keep the file tree format identical — it's already brand-correct

## Related

- [pane.md](./pane.md) — the pane the transition renders inside
- [token-mapping.md](./token-mapping.md)
- [../STYLE.md](../STYLE.md) — bold bet #4, box-drawing as illustration system
