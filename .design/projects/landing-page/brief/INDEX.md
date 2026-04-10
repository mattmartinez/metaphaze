# Brief
> Phase: brief | Project: landing-page | Generated: 2026-04-10

## Scoping

| Chunk | File | ~Lines |
|-------|------|--------|
| Scope | [scope.md](./scope.md) | ~140 |
| Target Adaptations | [target-adaptations.md](./target-adaptations.md) | ~180 |
| Install Manifest | [install-manifest.md](./install-manifest.md) | ~180 |
| Gap Analysis | [gap-analysis.md](./gap-analysis.md) | ~110 |

## Phase Summary

**One page. One PR. Seven sections.** The landing page is scoped as the tightest possible surface — hero, explainer, comparison, install, docs link, footer — with zero custom JavaScript, zero tracking, and zero dependencies beyond Next.js 14 + Tailwind v4 + two shadcn primitives.

**No blocking gaps.** The brand system covers 95% of the project's needs. The remaining 5% is project-specific (custom badge variants for the comparison row, a project-local footer component, an inline code-block component). All three are documented in `gap-analysis.md`.

**Implementation target:** `code` — produces actual Next.js source files. Install is 10 copy-paste commands.

**Ready for research phase.**
