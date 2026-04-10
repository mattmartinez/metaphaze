# Design Critique — metaphaze landing page
> Phase: critique | Project: landing-page | Reviewer: GSP Design Critic | Date: 2026-04-08

---

## 1. Strategy Alignment

**Verdict: Excellent. The design solves the right problem.**

The stated primary goal — convert "curious senior dev" to `cargo install mz` in under 90 seconds — is executed with real conviction. The install command is above the fold. The manifesto is the first text after the logo. The VHS proof sits below without competing. The page would survive "open in lynx" because it is built around real words and real commands, not images or interactions.

**Audience match:** The Senior Operator persona is honoured in every section choice. No testimonials, no feature grids, no email capture, no "trusted by" logos — these absences are as deliberate as the inclusions, and each one is correct. The Curious Observer (secondary persona) is served adequately by the docs link and footer.

**Scope:** One page, six sections, correct. The brief scoped this exactly right and the design held to it. No scope creep, no added sections, no "wouldn't it be cool if" additions.

**Minor strategic note:** The `[SOON]` badge treatment on `[/docs]` in the nav (mentioned in navigation.md) needs a decision. Shipping with a disabled-looking nav item tells the Senior Operator the product is incomplete — consider making the link live to the GitHub wiki or removing the badge entirely rather than signaling incompleteness on first load.

---

## 2. Brand Contract (STYLE.md) — Score: 23/25

### Constraint Adherence — 5/5

No constraint violations found. Checked all items in the `never:` list:
- Monospace font only: confirmed (JetBrains Mono everywhere)
- Border-radius: 0 throughout: confirmed
- No shadows: confirmed (`box-shadow: none` in all CSS snippets)
- No gradients: confirmed
- No pure black/white: confirmed (`#0a0a0a` / `#ededed`)
- No neon green, no dev-tool blue: confirmed
- No red for errors (amber used): confirmed
- No CRT scanlines, no phosphor glow, no typewriter reveal: confirmed
- No ALL CAPS headlines: confirmed (headings are `what it does`, `why it's different`, `install`)
- No italics: confirmed
- No smart quotes: confirmed
- No emoji: confirmed
- No icon libraries: confirmed
- No marketing words: no "agentic", "empower", "transform" etc. in any copy

`always:` list check:
- Monospace on every element: confirmed
- Lowercase for headlines/nav/body: confirmed
- Bracketed navigation `[/docs] [/source]`: confirmed
- Bracketed buttons `[ copy ]`: confirmed
- Status code badges `[OK]` `[ERR]` `[WARN]`: confirmed
- Shell prompt prefix `$` on code blocks: confirmed
- Em-dash structural dividers: confirmed (` ─ · ─ · ─` in the docs link section and footer)
- Box-drawing diagrams: confirmed (the loop, comparison table borders)
- Real TUI recording as hero asset: confirmed (VHS webm specified)
- Manifesto preserved verbatim: confirmed ("the orchestrator runs outside the loop. claude builds. mz drives.")
- Signal color on <1% of pixels: confirmed (cursor only, [OK] badge only, `mz` word in install)

### Pattern Fidelity — 5/5

Card pattern: Pane components implement `1px solid var(--mz-border)`, flat background, box-drawing title bars (`┌─ title ──┐`). Correct.

Button pattern: `[ copy ]` button uses bracketed text, transparent background, no border, video-invert hover. Correct. Both `[MIT]` and `[github.com/...]` in footer use the badge/nav idiom correctly.

Badge pattern: `[OK]`, `[ERR]`, `[WARN]` with correct color assignments. The `[FIRST-PARTY]` / `[THIRD-PARTY]` project-local variants are documented in target-adaptations.md and correctly mapped to `--mz-signal` and `--mz-error`.

Navigation pattern: `[/docs] [/source]` with `>` hover prefix and underline-reveal. Correct.

Code block: `var(--mz-border)` background, `$` prompt in dust, single signal accent run. Correct.

### Effects Vocabulary — 5/5

Three and only three techniques used: `cursor-blink`, `video-invert`, `underline-reveal + > prefix`. All from the declared interaction vocabulary. No rogue animations introduced. `prefers-reduced-motion` handled correctly for all three.

### Intensity Calibration — 4/5

Variance (2/10): Single-column layout throughout, uniform pane structure, rigid grid. Correct. No asymmetric layouts, no variable column counts. One minor issue: the `mz vs. other harnesses` comparison table inside the pane introduces slightly more visual density than other sections — this is appropriate content density, not a variance issue, but worth watching on mobile where the stacked layout requires careful spacing.

Motion (1/10): Cursor blink only. Correct. The `video-invert` on the copy button and the nav's `> prefix` are hover states, not ambient motion — they don't violate the motion dial.

Density (7/10): The install section with three code blocks back-to-back is the densest section. It is correctly dense — the Senior Operator who wants the full setup path needs all three. No reduction recommended.

Score: 4 (not 5) because the `[SOON]` nav badge for `[/docs]` hasn't been resolved. If it ships as a grayed-out state, it adds a visual inconsistency not present in the effects vocabulary — hover state should still work, and the colour treatment needs explicit specification.

### Bold Bet Presence — 4/5

Bold Bet #1 (`mz▌` cursor logo): Present and correctly implemented. Hero logo at `--text-5xl`, nav logo at `--text-base`, both with signal-green U+258C blinking at 530ms. The cursor is the literal product glyph. Excellent.

Bold Bet #2 (one typeface everywhere): Present. JetBrains Mono for MVP, Berkeley Mono in the font stack. No sans-serif anywhere. The commitment holds.

Bold Bet #3 (signal as instrument <1%): Present. Cursor glyph only, `[OK]` badge content, `mz` word in the install command. The discipline is real.

Bold Bet #4 (box-drawing as illustration system): Present. The loop diagram, comparison table borders, Pane title bars. However, the design doesn't yet show the loop diagram at its full expressive potential — it's functional but not yet the signature moment it could be. The `[FIRST-PARTY]` / `[THIRD-PARTY]` badge innovation in the comparison table is a net-new Bold Bet not in the original four, and it's good.

Bold Bet #5 (dual target, one yml): Acknowledged in the design notes. Not directly visible in the landing page but the token alignment is specified.

Score: 4 because Bold Bet #4 is present but underexploited in the loop diagram — the diagram could push further into the box-drawing vocabulary (deeper nesting, more explicit data flow labels) without violating any constraint.

---

## 3. Usability — Score: 44/50

### H1 — Visibility of System Status: 5/5

The copy button provides clear state feedback (`copy` → `copied` for 1500ms). The page is static — there are no loading states, no async operations beyond the copy interaction. For a static marketing page, "no state" IS the state, and the design correctly doesn't introduce false states. The VHS video has `autoplay` — the user's only interaction with it is in `prefers-reduced-motion` mode where a play affordance is needed. That edge case is handled.

### H2 — Match Between System and Real World: 5/5

The design uses the Senior Operator's exact mental model language. `$` prompt prefix, bracketed status codes, box-drawing table borders, `cargo install` commands — these are not metaphors for the real world, they ARE the real world. The comparison table property names (`api access`, `config file`, `binary size`) match the vocabulary a developer would use in a HN comment. No jargon mismatch. Manifesto preserved verbatim.

### H3 — User Control and Freedom: 4/5

This is a read-only marketing page — control and freedom are limited by design. The copy button is the only interactive element, and it reverts cleanly. The `[ copy ]` button degrades gracefully with JS disabled (the command is still copy-pasteable from the `<pre>`). Score 4 rather than 5 because: (a) the VHS video has no user controls (no pause button — only `prefers-reduced-motion` stops it), and (b) the page has no scroll-to-top control. Neither is a usability failure on a short marketing page, but they represent minor gaps.

### H4 — Consistency and Standards: 5/5

One page, so internal consistency is easy to achieve — and it is achieved. Section headings use the same typography pattern. All code blocks use the same Pane + CodeBlock structure. All badge variants follow the same `[TEXT]` pattern. Nav links in the header, docs link in §5, and footer links all use the same `.mz-nav a` treatment. The `› cargo 1.75 or later` prefix in the install section is consistent with the `>` prefix pattern from the interaction vocabulary.

### H5 — Error Prevention: 4/5

This is a static content page — there are very few error paths. The copy button could fail silently if `navigator.clipboard` is unavailable (HTTPS-only API, or permissions denied). The design handles the happy path but doesn't specify a fallback for clipboard failure. Recommendation: if `navigator.clipboard.writeText()` rejects, keep the label as `copy` (don't show `copied`) and let the user copy manually from the `<pre>`. This is a Minor fix, not Critical.

### H6 — Recognition Over Recall: 5/5

Everything the user needs is visible. The comparison table has its legend below it (explaining `[FIRST-PARTY]` / `[THIRD-PARTY]`). The install section has prerequisites above the commands. The docs link has a text description (`→ full documentation, api reference, and examples`). No cross-page references. No "see also" that goes nowhere. The refusals list in the footer is self-explanatory.

### H7 — Flexibility and Efficiency: 4/5

The Senior Operator who wants to install immediately can: the install command is in the hero with a copy button. The senior operator who wants context can: the full install section, verification, and post-install output are below. Both paths exist. Score 4 (not 5) because there are no section anchor links in the nav. A user who arrives from a search result for "how to install metaphaze" and lands midpage has no quick way to jump to the install section. A `[/install]` anchor in the nav or a `[ jump to install ]` link near the hero manifesto would close this gap.

### H8 — Aesthetic and Minimalist Design: 5/5

This is the score the design deserves most confidently. Every section has a single purpose. The hero does not try to also be the explainer. The `why it's different` section doesn't redundantly repeat the `what it does` copy. The footer is four lines, not forty. The blank vertical space between sections (`py-16` to `py-24`) is load-bearing — it makes the density readable. No element competes with any other for the primary goal. The refusals list (a content element) serves double duty as design — it fills space that could have held a social proof section but instead removes marketing vocabulary from the page. This is Sage-level editorial restraint executed in pixels.

### H9 — Help with Errors: 4/5

No error states are possible on a static page except the copy failure. The VHS video fallback text is specified (`<p>Terminal recording of mz auto running through a Rust project.</p>`). The JS-disabled mode is specified and graceful. Score 4 because the copy failure state (clipboard API unavailable) lacks explicit handling in the design spec — the implementation will have to decide, and the design should have opined.

### H10 — Help and Documentation: 3/5

The `[/docs]` link is the entire help affordance. This is appropriate for a marketing page — it's not a product, it's a door. Score 3 rather than 4 because: (a) `[/docs]` is grayed out / SOON until the docs ship, which means the primary help path is broken at launch, and (b) the install section has prerequisites but no "what if this fails" path (what if cargo isn't installed? what if the API key isn't set?). The comparison table legend helps understand the badge vocabulary but that's content, not help.

---

## 4. Accessibility (Summary)

Full audit in `accessibility-audit.md`. Headline findings:

- **Contrast:** All primary text pairs exceed WCAG AAA. `#ededed` on `#0a0a0a` = 16.97:1. Signal text `#5fb878` on `#0a0a0a` = 8.17:1. Muted text `#8a8a8a` on `#0a0a0a` = 5.81:1. All pass AA and most pass AAA.
- **Light mode signal:** `#2e8b57` on `#fafafa` = 4.06:1 — passes AA for large text only. The cursor at minimum 16px is borderline. Documented constraint — acceptable for the block character but cannot be used for body text.
- **Screen reader structure:** Strong. `aria-hidden` on `<pre>` diagrams, `aria-label` on `<figure>` wrappers, semantic landmarks (`<nav>`, `<main>`, `<footer>`, `<section>`), descriptive heading hierarchy.
- **Focus management:** `outline: 2px solid var(--mz-signal)` on `:focus-visible` is specified. Skip link is specified. Tab order is logical.
- **Video:** `autoplay muted` — muted satisfies WCAG. `prefers-reduced-motion` stops autoplay. No captions specified for the VHS recording — this is a Minor issue since the recording shows terminal output (no speech, no narration).
- **Touch targets:** `[ copy ]` button minimum 44px touch target is noted. Bracketed nav links need `padding: 8px 0` minimum to reach the 44px threshold on mobile.

One notable gap: the comparison table's `[FIRST-PARTY]` / `[THIRD-PARTY]` status badges use color (`--mz-signal` / `--mz-error`) as the only differentiator. The badge text itself does differentiate (the words are different), but a colorblind user would still read `[FIRST-PARTY]` vs `[THIRD-PARTY]` correctly from the text alone — so this is not a violation.

---

## 5. Content Quality

**Verdict: Excellent. Real copy throughout.**

No Lorem Ipsum. No placeholder numbers. No "John Doe." No fake round percentages.

- Manifesto preserved verbatim: "the orchestrator runs outside the loop. claude builds. mz drives."
- Install command is the actual `cargo install --git` URL
- Refusals list is seven concrete, specific refusals, not a generic bullet list
- Comparison table uses real property names, real binary sizes (`3.2 MB`, `40–200 MB`), real verdicts
- Footer build line is a real version and date (`v0.1.0 · 2026-04-08`)
- Section headings are declarative, not marketing (`what it does`, `why it's different`, `install`) — Sage-grade voice

One content issue: the `what it does` section prose is not fully specified in the design chunk. The design shows the structure (3-4 sentences, box-drawing diagram) and the `what it does` heading, but the actual prose is in `content-strategy.md` (research phase). Ensure the build phase pulls the exact copy from there rather than rewriting from scratch.

---

## 6. Implementation Quality

**Verdict: Strong specification. One structural risk.**

The component plan is unusually complete for a design phase — file structure, Client Component isolation to `<CodeBlock>`, shadcn/ui install commands, and CSS override strategy are all specified. This is a design document that reads like a technical design document, which is correct for a `code` implementation target.

Strong choices:
- One `'use client'` component (the copy button) — everything else is Server Components
- `--radius: 0` override in `:root` kills all shadcn defaults at once
- `prefers-color-scheme` via CSS custom properties, not a JS toggle
- `prefers-reduced-motion` handled in CSS for the cursor and via React for the video
- Static export to GitHub Pages via `output: 'export'`

One structural risk: the design specifies the VHS demo as `src="/vhs/demo-desktop.webm"` but this file doesn't exist yet (it needs to be recorded with VHS). The build phase will need this asset before it can ship. The design correctly documents this as a dependency but the risk is that the page ships without it or with a placeholder. Recommend: make the build phase explicitly create a placeholder `demo-desktop.webm` (even a static poster frame) as the first implementation step.

The light-mode VHS recording is documented as acceptable to skip in v1.0 with a `[WARN]` — this is a reasonable product decision for an early ship.

---

## 7. Taste Assessment

**Verdict: High. This design is recognizable. It has a point of view.**

**Intentionality:** Every element earns its place. The refusals list is not decoration — it is the clearest positioning statement on the page. The phase transition `<pre>` is not a screenshot — it is a live brand artifact rendered in the same typography as the surrounding page. The empty space between sections is not dead air — it is the breathing room that makes the density of code blocks readable.

**Visual coherence:** One typeface. One color (plus two neutrals). One column. Five sections. No breaks in the system, no "let me try something different here" moments. The design chose a lane and drove straight down it.

**Confidence in constraints:** The constraints that would feel like restrictions on a generic SaaS (no gradients, no rounded corners, no icon libraries) are here the brand's most distinctive features. The design uses them as assets, not limitations. The comparison table is more memorable because it uses `[FIRST-PARTY]` bracketed badges instead of checkmarks.

**Craft in details:** The `╌ mz auto ╌` dashed title on the VHS pane is a correct choice — it differentiates the "content" pane from the structural panes (which use solid `─` lines). The `·` em-dot separators in the footer are visually lighter than pipes or slashes, which matches the muted treatment for that section. The `›` list prefix in the install prerequisites (vs `$` for commands) distinguishes prose list items from executable code items without adding a second visual system.

**Would someone ask "who designed this?"** Yes. The page looks like something a senior engineer shipped, not like something a landing page template produced. That is the goal.

**One taste note:** The `·` separator rhythm in `─ · ─ · ─ · ─` in the docs link section may feel slightly under-confident — it reads as decorative rather than structural compared to the solid `─────────` rule used at the footer top. Consider using a single `─────────` rule (matching the footer) or a full-width `·` repeat. The current dashed em-dot pattern is not wrong, but it is the weakest visual moment on the page.

---

## Overall Scores

| Dimension | Score | Weight |
|-----------|-------|--------|
| Nielsen Heuristics | **44/50** | primary |
| Brand Contract | **23/25** | secondary |
| Strategy Alignment | excellent | — |
| Content Quality | excellent | — |
| Implementation Quality | strong | — |
| Taste | high | — |

**Verdict: Conditional Pass**

Nielsen 44/50 exceeds the Pass threshold (≥40). Brand contract 23/25 exceeds the Pass threshold (≥20). No critical fixes required. Two Important fixes (H10 docs link broken at launch, H7 install anchor absent) and three Polish items. The design is ready to build with these noted.

The design is honest, specific, and committed. It would make the Senior Operator think "someone actually built this." That is the only thing that matters.
