# Component Log — all
> Phase: build/components | Generated: 2026-04-08

## Components Built
| Component | File | Classification |
|---|---|---|
| Cursor | components/brand/cursor.tsx | custom |
| Pane | components/brand/pane.tsx | custom |
| StatusBadge | components/brand/status-badge.tsx | library-customize |
| CodeBlock | components/brand/code-block.tsx | custom |
| PhaseTransitionScreen | components/brand/phase-transition.tsx | custom |
| ComparisonTable | components/brand/comparison-table.tsx | custom |
| BracketedButton | components/brand/bracketed-button.tsx | library-customize |

## Files Created
- components/brand/cursor.tsx
- components/brand/pane.tsx
- components/brand/status-badge.tsx
- components/brand/code-block.tsx
- components/brand/phase-transition.tsx
- components/brand/comparison-table.tsx
- components/brand/bracketed-button.tsx

## Notes
- Fixed TypeScript narrowing error in comparison-table.tsx: extracted `mzLabel()` and `otherLabel()` helpers so the `string` type flows through rather than a narrowed literal union exhausted to `never`.
- Build passes clean: `npx next build` — ✓ Compiled, ✓ TypeScript, ✓ Static pages generated.
