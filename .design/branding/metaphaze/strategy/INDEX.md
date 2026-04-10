# Strategy
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

| Chunk | File | ~Lines |
|-------|------|--------|
| Positioning | [positioning.md](./positioning.md) | ~130 |
| Archetype | [archetype.md](./archetype.md) | ~170 |
| Brand Platform | [brand-platform.md](./brand-platform.md) | ~200 |
| Voice & Tone | [voice-and-tone.md](./voice-and-tone.md) | ~310 |
| Messaging | [messaging.md](./messaging.md) | ~230 |

## Phase Summary

**The position is locked.** metaphaze takes the empty lower-right corner of the 2x2: brutalist aesthetics × first-party durability. The positioning statement anchors on The Senior Operator and names the mechanism (deterministic Rust state machine, disk-first `.mz/`, first-party `claude` binary) as the reason to believe. The architecture is the argument. Launch copy does not reference the April 4 crackdown — the safe play is to position on technical merits and let the competitive gap speak for itself.

**The archetype is Sage with a Creator undertone.** The Sage (truth-teller, knowledge-as-craft, refuses to dumb things down) is the primary — 75% of the brand. The Creator (craft visible in the joinery, methodology as moat) is the secondary — 25%. Hero and Ruler were rejected because metaphaze does not frame the Senior Operator as needing to be rescued and does not frame itself as the enterprise standard. The Sage's visual tendencies (clean typography, structured layouts, muted palettes) map cleanly onto the `terminal` style preset with the discover-phase token overrides (`#0a0a0a`, `#ededed`, `#5fb878`).

**The platform is five behavioral values, not aspirations.** We ship binaries, not promises. State lives on disk where you can read it. The first-party binary is the only surface. We refuse things in public. The README is the marketing. Each is testable against shipped code. The manifesto line is preserved exactly: "the orchestrator runs outside the loop. claude builds. mz drives."

**The voice is Precise · Spare · Lowercase.** Three attributes in priority order. Short sentences. No exclamation marks. Lowercase as a deliberate typographic position (not always — legal text and proper nouns follow their own rules). Em-dashes as structural markers. Bracketed navigation. References are htmx, suckless, and plan9 docs — not charmbracelet, linear, or vercel. A do/don't chart, a banned-words list, a preferred-words list, and nomenclature conventions for CLI commands, files, and branches make the voice reproducible by a second writer.

**The messaging is architecture-first.** Core message: "the orchestrator runs outside the loop." Three supporting messages with mechanisms and proof points: won't degrade (deterministic state machine), won't lose work (disk-first state), won't get banned (first-party binary). Three tagline directions, all testable against GSD 2.0's "one command, walk away." The elevator pitch is 30 seconds, mechanism-first, with no "agentic." The Senior Operator is the only audience — narrower is stronger — and the six questions they ask in the market-landscape chunk each map to a one-sentence, mechanism-backed answer.

## Handoff to Identity Phase

The identity phase should inherit these hard constraints from strategy:

1. **The base style preset is `terminal`.** Override the tokens to `#0a0a0a` / `#ededed` / `#8a8a8a` / `#2a2a2a` / `#5fb878`. Berkeley Mono first, JetBrains Mono as the free fallback. No secondary sans, ever.
2. **The archetype is Sage-first.** The logo, color, and typography decisions should read as clarity and structure — not as heroism, not as luxury, not as playfulness.
3. **The manifesto line is locked copy.** "the orchestrator runs outside the loop. claude builds. mz drives." Preserve spacing, preserve lowercase, preserve the period placement. Do not redesign the sentence.
4. **The voice is lowercase-leaning.** Any logo lockup that includes the wordmark should use lowercase `mz` and lowercase `metaphaze`. Never `MZ`, never `Metaphaze`, never `MetaPhaze`.
5. **No mascot, no illustration, no photograph.** The TUI is the hero asset. ASCII diagrams are the illustration system. Code blocks are the photography. Identity work should make the logo mark and the wordmark carry the entire visual identity without support from decorative imagery.
6. **The refusals list is a first-class brand asset.** Identity should treat it as visible content, not fine print. Layout should give it weight equal to the hero headline.
7. **Do not broaden the persona.** Strategy committed to The Senior Operator as the only audience. Identity should not design for "a wider range of developers." Narrow is the position.
8. **No gradients, no drop shadows, no rounded corners.** Solid fills only. Hard edges only. If a visual treatment would not survive being `cat`-ed in a terminal, it does not belong in the identity.
