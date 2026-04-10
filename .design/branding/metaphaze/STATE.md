# Brand State

## Brand: metaphaze
**Started:** 2026-04-10
**Mode:** new
**Current Phase:** 4 (Patterns complete) — BRANDING DIAMOND COMPLETE
**Prettiness Level:** 100%

---

## Phase Progress

| # | Phase | Status | Started | Completed |
|---|-------|--------|---------|-----------|
| 0 | Audit | skipped | — | — |
| 1 | Discover | complete | 2026-04-10 | 2026-04-10 |
| 2 | Strategy | complete | 2026-04-10 | 2026-04-10 |
| 3 | Identity | complete | 2026-04-10 | 2026-04-10 |
| 4 | Patterns | complete | 2026-04-10 | 2026-04-10 |

## Status Values
<!-- pending | in-progress | complete | needs-revision | skipped -->

## Decisions
- **Brand mode:** new — no existing brand to evolve
- **Personality direction:** Brutalist · Honest · Technical
- **Anti-direction:** Enterprise dev tool aesthetics
- **Inspiration mix:** suckless + Linear + Charmbracelet
- **Constraint:** brand must read correctly in monospace / terminal-first
- **Discover phase (Phase 1):** Brutalist × first-party 2x2 white space identified. April 4 2026 Anthropic crackdown is the launch context. Mood board: `#0a0a0a` / `#ededed` / `#5fb878` signal green, Berkeley Mono primary, no photography, TUI recordings as hero asset. Style affinity: terminal (primary) + monochrome (editorial discipline) + nothing (single-signal restraint).
- **Strategy phase (Phase 2):** Archetype = The Sage (primary) + Creator (secondary). Positioning = safe play (quiet competence, no crackdown reference). Voice = Precise · Spare · Lowercase. Style base = `terminal` preset with discover-phase token overrides. Manifesto locked: "the orchestrator runs outside the loop. claude builds. mz drives."
- **Identity phase (Phase 3):** Visual direction = selective (restrained terminal). Logo = The Cursor (`mz▌` with U+258C in `#5fb878`). Colors = 5 anchors (black `#0a0a0a`, bone `#ededed`, dust `#8a8a8a`, slate `#2a2a2a`, signal `#5fb878`) + amber for warn/error. Composition = Neutral + Single Accent with Terminal/ANSI discipline. Typography = Berkeley Mono (preferred) / JetBrains Mono (fallback), Major Third scale at 16px base, 3 weights (400/600/700). Imagery = no photos/illustrations/mascots, TUI recordings as hero, ASCII diagrams as illustrations, 17-glyph Unicode inventory, no gradients/shadows/blur.
- **Guidelines phase (Phase 4):** Operational artifacts complete. Canonical `metaphaze.yml` extends `terminal` preset with documented overrides. Dual-target: Rust `ratatui` constants + Tailwind v4 / shadcn/ui CSS variables, both derived from one source. `STYLE.md` renders from the .yml. `guidelines.html` is a self-contained visual brand guide (open in browser). 7 component specs: token-mapping, pane, status-badge, bracketed-button, prompt-input, phase-transition, cursor — all with Rust AND Web implementations. Intensity: variance 2/10 · motion 1/10 · density 7/10. Bold bets: cursor logo, one typeface, signal as instrument, box-drawing illustrations, dual target.

## Notes
- Product is already built across 12 phases (97 steps complete) — brand follows the product, not the other way around
- TUI is the primary surface — every visual decision tested in monospace before approval
