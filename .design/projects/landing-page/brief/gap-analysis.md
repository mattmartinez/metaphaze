# Gap Analysis

> Phase: brief | Project: landing-page | Generated: 2026-04-10

---

## Overview

What's in the brand system vs. what the landing page actually needs. The codebase is `greenfield` (no existing `metaphaze-www` code yet), so this analysis compares the brand system against project requirements, not against an existing codebase.

## Components in brand, used directly

These exist in `.design/branding/metaphaze/patterns/components/` and are used without modification (just wrapped as React components):

| Brand component | Project usage | Status |
|---|---|---|
| `cursor.md` | `<Cursor />` — hero logo + install line | ✓ ready |
| `pane.md` | `<Pane />` — VHS frame, code wrappers | ✓ ready |
| `bracketed-button.md` | `<BracketedButton />` — copy button | ✓ ready |
| `prompt-input.md` | Not used as a component, but the `$` prefix pattern is used | ✓ ready |
| `phase-transition.md` | `<PhaseTransitionScreen />` — static below VHS | ✓ ready |
| `token-mapping.md` | Pasted directly into `app/globals.css` | ✓ ready |

## Components in brand, needing extension

### `status-badge.md` — extension needed

The brand spec defines variants `ok`, `warn`, `error`, `info`, `pending`. The landing page comparison row needs two more **project-specific custom variants** that don't belong in the brand spec but are documented here:

| Custom variant | Color | Semantic use |
|---|---|---|
| `first-party` | `var(--color-mz-signal)` | Label for technologies that call the Claude CLI directly |
| `third-party` | `var(--color-mz-error)` | Label for technologies that use third-party SDK layers |

**Decision:** These extensions live in the project's `<StatusBadge />` component only. They are NOT promoted to the brand system because the semantic meaning ("first-party/third-party") is specific to metaphaze's competitive positioning and would not generalize to other projects using the brand.

## Components NOT in brand, needed by the project

### `code-block` — missing from brand

The brand `.yml` has `patterns.code-block` defined (background, border, prompt prefix, signal accent rules) but there is **no `code-block.md` in `patterns/components/`**.

**Why it was skipped in the brand phase:** The brand pattern is declared in the `.yml` as a composition rule, not a full component spec. The landing page is the first surface that needs an actual reusable component.

**Action:** The project creates `components/brand/code-block.tsx` inline and uses it. After the landing page ships, the `code-block.md` spec should be promoted to the brand system (as a follow-up task, not part of this project).

### `footer` — missing from brand

READMEs don't have footers, and the brand was designed README-first. The landing page needs a footer.

**Why it was skipped in the brand phase:** Correctly — the brand is primarily a README + TUI, neither of which has a footer.

**Action:** Invent `<Footer />` inline for this project. The component is project-specific (includes project-specific content like the "refusals list" copy). It does not need to be promoted back to the brand system unless a second web project reuses it.

## Tokens used

All seven brand color tokens plus all nine type-scale tokens are used. Nothing is missing.

| Token | Used for |
|---|---|
| `--color-mz-bg` | Page background, code block foreground on hover |
| `--color-mz-fg` | All body text, headlines |
| `--color-mz-fg-muted` | Secondary text, footer copy, code block prompts |
| `--color-mz-border` | All pane borders, code block backgrounds, dividers |
| `--color-mz-signal` | Cursor, install command highlight, `[FIRST-PARTY]` badge |
| `--color-mz-warn` | Not used on this page (no warnings) |
| `--color-mz-error` | `[THIRD-PARTY]` badge, `[ERR]` badges |
| `--text-xxs` through `--text-5xl` | All 11 type scale steps used across the page sections |

**Unused tokens:** `--color-mz-warn` is not used on the landing page (no warning states). This is correct — the page is informational, not interactive. Warnings would only appear in an error state that the landing page does not have.

## Brand patterns applied

From `metaphaze.yml` → `patterns:`, each of the 8 patterns is applied on the landing page:

| Pattern | Where it's applied |
|---|---|
| `card` | VHS hero pane, install code block pane, comparison pane |
| `button-primary` | `[ copy ]` button next to install command |
| `button-secondary` | Not used (the page only has one button) |
| `input` | Not used (no interactive input — the install command is display-only) |
| `badge` | `[OK]`, `[ERR]`, `[FIRST-PARTY]`, `[THIRD-PARTY]` in the comparison row |
| `nav` | Single `[/docs]` link in the hero area |
| `code-block` | Install command, `mz status` example, `mz --help` example |
| `layout` | Overall page structure with `max-w-3xl` reading column |

## Brand constraints applied

All 21 `never` constraints and 11 `always` constraints from `metaphaze.yml` apply to the project. No exemptions, no local overrides.

**Specifically relevant:**
- Zero border-radius throughout
- No box-shadow anywhere
- No gradients, no backdrop-filter, no CSS filters
- No italics, no smart quotes, no emoji
- Lowercase voice in all copy
- Bracketed navigation and badges
- Em-dash as structural divider

The landing page is the first external-facing metaphaze surface. The constraints exist to keep it consistent with the brand even under deadline pressure.

## Gaps summary

| Gap | Impact | Resolution |
|---|---|---|
| `code-block.md` spec missing from brand | Low — the project builds it inline | Promote to brand after landing page ships |
| `footer.md` spec missing from brand | Low — project-specific, may not reuse | Keep as project-local, do not promote |
| `status-badge.md` needs custom variants | Low — documented extension | Keep variants project-local |
| `--color-mz-warn` unused | None — correct behavior | Leave unused |

**No blocking gaps.** The brand system covers 95%+ of the project's needs. The remaining 5% is project-specific web composition (footer, custom badge variants) that was correctly left out of the brand phase.

## Cross-references

- Brand components: `../../../branding/metaphaze/patterns/components/`
- Brand `.yml`: `../../../branding/metaphaze/patterns/metaphaze.yml`
- Target adaptations: `./target-adaptations.md`
- Install manifest: `./install-manifest.md`
