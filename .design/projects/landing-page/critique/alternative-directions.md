# Alternative Directions — metaphaze landing page
> Phase: critique | Project: landing-page | Date: 2026-04-08

These are not corrections to the current design — the current design passes. These are genuinely different approaches that could have been taken and may be worth revisiting if the current design doesn't convert as expected.

---

## Direction A — "The README, Rendered"

**Concept:** Abandon the five-section marketing structure and render the actual GitHub README as the landing page. The README becomes the single source of truth for both surfaces. No separate sections — just the README with brand typography, color, and the VHS embed.

**How it's different from the current design:**
- No separate `why it's different` comparison table — that information lives in the README comparison block
- No explicit section headings (`what it does`, `why it's different`, `install`) — the README's own headings are the navigation
- The page IS the README — nothing exists on the landing page that isn't in the README

**Why this is compelling:**
- Eliminates the synchronization problem (landing page can become stale vs README)
- Maximally honest — a Senior Operator who reads the landing and then reads the README sees identical content in two different skins
- Brand Applications note (section 2) explicitly says the landing page should be "the README rendered beautifully, with two additions: real brand colors and real brand typography"
- Zero marketing copy to maintain

**Trade-offs:**
- Loses the visual structure of explicitly-labeled sections (`install`, `why it's different`) — the current design makes it easier to scan
- README sections may not map cleanly to the landing page's section rhythm
- Requires a documentation-to-web pipeline (auto-sync README changes → landing page) which adds build complexity
- The comparison table's bracket badge design (`[FIRST-PARTY]` / `[THIRD-PARTY]`) is richer than a typical README table — would need a markdown extension or custom rendering

**When to try this:** If the current design requires ongoing copy maintenance that creates drift between the README and the landing page, or if the conversion rate suggests users want more depth (more like a README, less like a marketing page).

---

## Direction B — "The Phase Transition as Hero"

**Concept:** Lead with the phase transition screen as the first visual element — above the manifesto, before the VHS recording. The terminal ASCII output becomes the page's opening statement. The hero is the product's most authentic visual moment, not a logo and a blurb.

**Layout variant:**

```
┌──────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│  ╔════════════════════════════════════════════════════════╗              │
│  ║  metaphaze — phase 4/6 complete                        ║              │
│  ║                                                        ║              │
│  ║  ◆ brief       complete   2026-04-08 14:23             ║              │
│  ║  ◆ research    complete   2026-04-08 14:31             ║              │
│  ║  ◆ design      complete   2026-04-08 15:12             ║              │
│  ║  ◆ critique    complete   2026-04-08 17:44             ║              │
│  ║  ◇ build       pending    —                            ║              │
│  ║  ◇ review      pending    —                            ║              │
│  ║                                                        ║              │
│  ╚════════════════════════════════════════════════════════╝              │
│                                                                          │
│  mz▌                                                                     │
│  the orchestrator runs outside the loop. claude builds. mz drives.      │
│                                                                          │
│  $ cargo install --git https://github.com/mattmartinez/metaphaze [ copy]│
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**How it's different from the current design:**
- Phase transition screen is the first element (currently below the VHS)
- Logo, manifesto, and install follow the phase transition
- VHS recording moves to section 2 or is removed (the phase transition IS the proof)
- Page begins with product output, not brand identity

**Why this is compelling:**
- More aggressive self-selection: a visitor who doesn't understand the phase transition ASCII output immediately knows this product is for terminal-native developers and filters themselves in or out in 2 seconds
- "Show, don't tell" at the opening frame — the first thing you see is the product working, not a claim about the product
- Doubles down on the Bold Bet #4 (box-drawing as illustration) — makes the phase transition the literal opening image of the brand
- The double-line `╔═╗` border on the phase transition screen creates strong visual contrast against the single-line `┌─┐` pane borders used elsewhere — a natural focal point

**Trade-offs:**
- Higher cognitive load on first load — the visitor must understand the ASCII output before they understand what they're looking at
- The `mz▌` logo identity beat is delayed — the logo no longer owns the first impression
- Visitors arriving without terminal context (Zara, the secondary persona) are more likely to bounce
- The phase transition screen needs to be more polished than the current design spec — it becomes the hero, not a supporting element

**When to try this:** If conversion analytics show that the current page over-indexes on people who already know what `mz` is (returning visitors, direct GitHub referrals) but under-converts on cold HN traffic who need the "oh, I get it" moment to happen faster.
