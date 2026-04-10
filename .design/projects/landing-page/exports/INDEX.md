# GSP Design Exports

> Load this file first, then load only the chunks needed for your task.

## Usage

This file is the entry point for coding agents consuming GSP design output.

1. Read this file to find chunk paths for your task
2. Load only the chunks relevant to your current screen or component
3. Each chunk is self-contained — follow `## Related` links for cross-references

## Quick Reference

- Building a screen? → Design table → load screen chunk + referenced components
- Need a component spec? → Components table (in brand system)
- Need color/type/spacing? → Foundations table (in brand system)
- Need project scope? → Brief table
- Need UX patterns or reference specs? → Research table

## Design System (Brand-Level)

<!-- BEGIN:system -->
| Section | Chunk | Lines |
|---------|-------|-------|
| _(populated by /gsp-brand-system — lives in brand directory)_ | | |

### Foundations

| Foundation | File | Tokens |
|------------|------|--------|
| Style + tokens | [STYLE.md](../../branding/metaphaze/patterns/STYLE.md) | all |
| Token mapping | [token-mapping.md](../../branding/metaphaze/patterns/components/token-mapping.md) | CSS + Rust |

### Components

| Component | File | States | Variants |
|-----------|------|--------|----------|
| Cursor | [cursor.md](../../branding/metaphaze/patterns/components/cursor.md) | blink / static | — |
| Pane | [pane.md](../../branding/metaphaze/patterns/components/pane.md) | idle / active / completed / blocked | — |
| Bracketed Button | [bracketed-button.md](../../branding/metaphaze/patterns/components/bracketed-button.md) | default / hover / active | primary / secondary |
| Status Badge | [status-badge.md](../../branding/metaphaze/patterns/components/status-badge.md) | — | ok / warn / error |
| Phase Transition | [phase-transition.md](../../branding/metaphaze/patterns/components/phase-transition.md) | — | — |
<!-- END:system -->

## Project Brief

<!-- BEGIN:brief -->
| Section | File |
|---------|------|
| Brief | [BRIEF.md](../BRIEF.md) |
| Scope | [brief/scope.md](../brief/scope.md) |
| Target Adaptations | [brief/target-adaptations.md](../brief/target-adaptations.md) |
| Install Manifest | [brief/install-manifest.md](../brief/install-manifest.md) |
| Gap Analysis | [brief/gap-analysis.md](../brief/gap-analysis.md) |
<!-- END:brief -->

## Project Research

<!-- BEGIN:research -->
| Section | File |
|---------|------|
| UX Patterns | [research/ux-patterns.md](../research/ux-patterns.md) |
| Competitor UX | [research/competitor-ux.md](../research/competitor-ux.md) |
| Technical Research | [research/technical-research.md](../research/technical-research.md) |
| Accessibility Patterns | [research/accessibility-patterns.md](../research/accessibility-patterns.md) |
| Content Strategy | [research/content-strategy.md](../research/content-strategy.md) |
| Reference Specs | [research/reference-specs.md](../research/reference-specs.md) |
| Recommendations | [research/recommendations.md](../research/recommendations.md) |
<!-- END:research -->

## Design

<!-- BEGIN:design -->
### Screens

| # | Screen | File | Components Used |
|---|--------|------|-----------------|
| 01 | Landing Page (`/`) | [screen-01-landing.md](../design/screen-01-landing.md) | Cursor, Pane, CodeBlock, BracketedButton, StatusBadge, PhaseTransitionScreen, ComparisonTable, Footer |

### Shared

| Section | File |
|---------|------|
| Personas | [personas.md](../design/shared/personas.md) |
| Information Architecture | [information-architecture.md](../design/shared/information-architecture.md) |
| Navigation | [navigation.md](../design/shared/navigation.md) |
| Micro-interactions | [micro-interactions.md](../design/shared/micro-interactions.md) |
| Responsive | [responsive.md](../design/shared/responsive.md) |
| Component Plan | [component-plan.md](../design/shared/component-plan.md) |

### Preview

| File | Description |
|------|-------------|
| [preview.html](../design/preview.html) | Self-contained wireframe preview — open in browser |
<!-- END:design -->

## Design Critique

<!-- BEGIN:critique -->
| Section | File |
|---------|------|
| Critique | [critique.md](../critique/critique.md) |
| Prioritized Fixes | [prioritized-fixes.md](../critique/prioritized-fixes.md) |
| Alternative Directions | [alternative-directions.md](../critique/alternative-directions.md) |
| Strengths | [strengths.md](../critique/strengths.md) |
| Accessibility Audit | [accessibility-audit.md](../critique/accessibility-audit.md) |
| Accessibility Fixes | [accessibility-fixes.md](../critique/accessibility-fixes.md) |
<!-- END:critique -->

## Build

<!-- BEGIN:build -->
| Section | File |
|---------|------|
| _(populated by /gsp-project-build)_ | |
<!-- END:build -->

## QA Review

<!-- BEGIN:review -->
| Section | File |
|---------|------|
| Acceptance Report | [acceptance-report.md](../review/acceptance-report.md) |
| Issues | [issues.md](../review/issues.md) |
<!-- END:review -->
