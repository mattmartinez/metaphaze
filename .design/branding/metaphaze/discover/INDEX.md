# Discover
> Phase: discover | Brand: metaphaze | Generated: 2026-04-10

| Chunk | File | ~Lines |
|-------|------|--------|
| Market Landscape | [market-landscape.md](./market-landscape.md) | ~120 |
| Competitive Audit | [competitive-audit.md](./competitive-audit.md) | ~140 |
| Trend Analysis | [trend-analysis.md](./trend-analysis.md) | ~180 |
| Mood Board Direction | [mood-board-direction.md](./mood-board-direction.md) | ~210 |

## Phase Summary

**The market just reset.** Four days before this research (April 4, 2026), Anthropic cut off third-party harness access to Claude subscriptions. Every major competitor — claude-flow/Ruflo, OpenClaw, Pi, GSD 2.0 — was built on the surface that just got restricted. metaphaze's technical choice to call the first-party `claude` binary directly is now a hard differentiator with a clean marketing story.

**The position is bottom-right of a 2x2:** brutalist × first-party. The corner is empty. Own it.

**The aesthetic direction compounds five trends:** brutalist dev-tool revival, anti-SaaS positioning, TUI craft, monospace-everywhere marketing, and stripped-down restrained dark mode. Each alone is getting copied. The combination is still rare enough to be a durable brand signal for 6-12 months.

**The typographic commitment:** one monospace typeface, everywhere. Berkeley Mono if budget allows, JetBrains Mono as the free default. No secondary sans. No exceptions.

**The color commitment:** warm off-black (`#0a0a0a`), warm off-white (`#ededed`), one signal color — muted terminal green (`#5fb878`), used for less than 1% of pixels on any page.

**The imagery commitment:** no photos, no illustrations, no mascots. The TUI itself, recorded with VHS or asciinema, is the hero asset. ASCII diagrams are the illustrations. Code blocks are the photography.

**The brand test:** if a senior dev sees the metaphaze landing page in a tab next to a `claude-flow` landing page, they should be able to tell in under 2 seconds that metaphaze was made by someone who actually writes code.

## Recommended Style Presets

1. **`terminal`** (primary base) — developer, monospace, dark, minimal, technical
2. **`monochrome`** (secondary layer) — editorial typographic discipline
3. **`nothing`** (accent discipline only) — single signal color, instrument-grade restraint

See [mood-board-direction.md](./mood-board-direction.md#style-affinity) for rationale and token overrides.

## Handoff to Strategy Phase

The strategy phase should inherit these hard constraints from discovery:

1. **Position against the April 4 moment explicitly.** The launch narrative has to reference the crackdown — "the orchestrator that calls `claude` directly" is not a feature, it is the brand.
2. **Anchor voice in The Senior Operator's frustration.** Direct, lowercase where it fits, short sentences, code over prose. The README is the marketing. No "agentic" language ever.
3. **Manifesto line is already right:** "The orchestrator runs outside the loop. Claude builds. mz drives." Strategy should build around this, not replace it.
4. **Refusals as positioning.** A "Things metaphaze will never have" list — no accounts, no cloud, no dashboard, no telemetry, no signup wall — is a strong candidate for the brand's second-most-visible asset after the hero headline.
5. **Do not soften to appeal to juniors.** The Senior Operator is a narrower market than "all developers using Claude Code." Narrower is stronger here. Resist any pressure in later phases to broaden the voice.
