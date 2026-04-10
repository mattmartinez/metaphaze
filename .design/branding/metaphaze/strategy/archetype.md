# Archetype
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

**Primary archetype:** The Sage — knowledge as craft, truth-teller, technically right, refuses to dumb things down.
**Secondary archetype:** The Creator (~25% influence) — the orchestrator is a made thing, shaped by a craftsman, and craft is visible in every corner.

metaphaze is Sage-first because the whole product is an argument about what is true — deterministic orchestration survives, fragile harnesses don't. Creator is the secondary because the methodology (phase → track → step) is hand-built, not discovered, and the brand should honor the making.

---

## Primary: The Sage

### Core desire
Truth and understanding — specifically, the truth about how AI coding orchestration actually works when nobody is watching. The Sage is willing to be technically right even when it is commercially inconvenient. metaphaze's entire product thesis is a Sage claim: the orchestrator has to run outside the loop, and anyone telling you otherwise is selling something.

### Brand promise
**The architecture is the argument.** If the reader understands how metaphaze actually works — Rust state machine, disk-first `.mz/`, first-party `claude` — they will reach the same conclusion metaphaze did. No persuasion needed. Just show the work.

### Sage traits that metaphaze owns

- **Knowledgeable without performance.** The brand knows more than it says on any given page. The README does not list every feature; it explains the shape of the thing and lets the source code be the footnote.
- **Analytical.** Every claim has a mechanism behind it. "Won't degrade over long runs" is not a marketing line — it is a consequence of the orchestrator running outside the LLM context window. The brand explains the mechanism whenever there is room.
- **Thoughtful.** The brand takes positions. "The orchestrator runs outside the loop" is a position, not a hedge. Refusing to dumb things down is a position. Lowercase voice is a position.
- **Informed by first principles.** The Sage does not appeal to authority. metaphaze does not cite "trusted by 500+ teams" because that is an argument from social proof. The Sage argument is "here is how it works; verify it yourself."

### Why The Sage fits The Senior Operator

The Senior Operator's frustration list from BRIEF.md reads like a bill of particulars against everyone who is not a Sage:

- "Agent harnesses that look great in demos but fall apart on long runs" — the opposite of analytical rigor.
- "Marketing copy that says 'agentic' forty times and explains nothing" — the opposite of knowledge-as-craft.
- "SaaS dashboards for things that should be a CLI" — the opposite of thoughtful architecture.
- "Tools that get banned by providers a month after launch" — the opposite of first-principles durability.

The Senior Operator is not looking for a Hero brand that will fight alongside them, a Creator brand that will inspire them, or a Jester brand that will entertain them. They are looking for a Sage that will **tell them the truth about the tool**, in enough detail that they can trust it, and then get out of the way. metaphaze's job is to be that Sage.

### Sage communication style, mapped to metaphaze

| Sage trait | metaphaze expression |
|---|---|
| Data-driven | The README leads with the state machine diagram, not the tagline. |
| Educational | Every command has a `--help` that explains the why, not just the what. |
| Thought leadership | The manifesto line earns its place by being technically precise, not inspirational. |
| Nuanced | Edge cases are documented. Known limitations are listed above the fold. Caveats are first-class content. |
| Refuses to dumb down | No "simple English" mode. The reader is assumed to be a senior engineer. The writing meets them at their level. |

### Sage visual tendencies

The Sage's default visual language in Mark & Pearson's taxonomy — clean typography, structured layouts, muted palettes, data visualization — maps cleanly onto the `terminal` preset from `discover/mood-board-direction.md`. Not by coincidence.

**The Sage's clarity aligns with terminal's monospace discipline — both refuse decoration in favor of structure.** The visual identity will inherit terminal's developer-native defaults and override colors with the discover-phase tokens (`#0a0a0a`, `#ededed`, `#5fb878`). Berkeley Mono (or JetBrains Mono as the free fallback) is the voice. ASCII diagrams are the illustrations. The TUI itself, recorded with VHS, is the hero asset. Every visual decision is legible as an argument.

What Sage visuals look like in metaphaze:

- **Structure over ornament.** ASCII box-drawing characters as the grid system. Tables as tables. `<pre>` blocks as layout.
- **Muted signal color.** One green (`#5fb878`), used for less than 1% of pixels. The restraint is the point.
- **Data-dense without being busy.** Long-form technical content, rendered in monospace with tight vertical rhythm, organized by typographic hierarchy alone.
- **No decoration.** No gradients, no drop shadows, no rounded corners, no mascots, no illustrations, no photos.

---

## Secondary: The Creator (~25%)

### Why Creator, not Hero or Ruler

The obvious second-choice archetype for a dev tool is Hero ("overcome the odds") or Ruler ("premium, authoritative, market leader"). Both are wrong for metaphaze.

- **Hero is wrong** because metaphaze does not frame the Senior Operator as someone who needs to fight through adversity. It frames them as someone who already knows what they want and is looking for the tool that respects them. No villain arc, no triumphant montage.
- **Ruler is wrong** because metaphaze is MIT-licensed, solo-built, and explicitly anti-enterprise. Ruler language ("the industry standard," "trusted by market leaders") is exactly the voice the brief says to never use.
- **Creator is right** because metaphaze is a made thing, deliberately shaped, and the making is visible in every decision. The methodology (phase → track → step decomposition with disk-based state) is the craft. The single-binary constraint is the craft. The typography commitment is the craft. The Creator honors the making without demanding the Hero's drama.

### Creator traits that metaphaze inherits

- **Craft visible in the details.** The `.mz/` directory is inspectable because the Creator wants you to see the joinery.
- **Inventive within constraints.** Single binary, no runtime, no accounts — constraints are where craft happens. The Creator does not chafe at limitations; the Creator uses them as the brief.
- **Enduring over trendy.** The methodology is meant to outlive the model, the provider, and the language. "Tools built on this foundation outlive their dependencies." That is a Creator's time horizon.

### How Creator shows up (without overwhelming the Sage)

The Creator influence is roughly one-in-four decisions. It shows up in:

- **Pride in the phase → track → step decomposition as methodology**, not just as code. The Creator cares that the shape is right.
- **Micro-typographic obsession.** Tracking, kerning, vertical rhythm, em-dashes as structural markers. The Creator notices.
- **TUI craft.** The TUI is a designed surface, not just a text output. Charmbracelet-grade care, without the mascots.
- **The manifesto line itself.** "The orchestrator runs outside the loop. Claude builds. mz drives." is a crafted sentence, not a generated one. The rhythm is deliberate.

### Where the Creator must stay quiet

The Creator archetype can easily drift into self-indulgence — "look at my craft" instead of "here is a useful tool." metaphaze keeps the Creator influence subordinate by these rules:

- Never lead with the craft. Lead with the mechanism.
- Never call attention to the typography, the ASCII, or the TUI in copy. Let the reader notice.
- Never celebrate the brand. Celebrate the thing the brand makes possible (walking away from the keyboard).

---

## Shadow Traits to Avoid

Every archetype has a shadow — the dysfunctional expression of its motivation. metaphaze has to guard against both.

### Sage shadow: ivory tower elitism
The Sage, left unchecked, becomes the brand that makes the reader feel stupid. It lectures, it hedges, it buries the useful information under six paragraphs of theory, it name-drops obscure references, it rolls its eyes at simple questions.

**How metaphaze avoids it:**
- The README shows the `cargo install mz` command in the first screen, not after a philosophical preamble.
- No "you should already know..." phrasing. Assume the reader is a senior engineer, but do not punish them for asking.
- No gratuitous Rust-ism or jargon-flexing. Every technical term earns its place.
- The brand refuses to dumb things down, but it does not refuse to be clear. Those are different things.
- Sage with range, not Sage with a chip on its shoulder.

### Creator shadow: perfectionism paralysis
The Creator, left unchecked, becomes the brand that never ships because the details are never quite right, or the brand that is so obsessed with its own craft that the actual user gets lost.

**How metaphaze avoids it:**
- "We ship binaries, not promises." Version 0.1.0 is already better than a polished 1.0 that doesn't exist.
- The landing page is the README. No separate marketing site that has to be perfect before launch.
- Craft serves the Senior Operator's workflow, not the maker's portfolio.

---

## Communication Style (Archetype-Derived)

From the Sage + Creator blend, the brand's communication style resolves to four rules:

1. **Explain the mechanism.** When making a claim, show how it works. "Won't degrade" is a Sage claim; "because the orchestrator runs outside the LLM context window" is the Sage proof.
2. **Respect the reader.** Assume a senior engineer is reading. Do not over-explain, but do not under-explain either. Meet them where they live (in the terminal, with three panes open).
3. **Show the craft by doing it, not by describing it.** Monospace typography, inspectable state, precise CLI help text — the craft is visible in the doing. The brand never says "we care about craft."
4. **Take positions, own them.** The Sage has opinions and defends them with reasoning. The brand does not hedge. "The orchestrator runs outside the loop" is not "we think orchestrators should probably run outside the loop."

---

## Archetype Test

Before approving any copy, visual, or decision, run it through the archetype test.

- **Is it Sage-first?** Does it tell the reader something true, in enough detail to verify? Or does it ask them to trust the brand on vibes?
- **Is the Creator quiet enough?** Is the craft in service of the user, or in service of the maker's portfolio?
- **Does it stay out of the shadows?** No lecturing (Sage shadow), no perfectionism theater (Creator shadow).

If any answer is no, rewrite from the Sage premise.
