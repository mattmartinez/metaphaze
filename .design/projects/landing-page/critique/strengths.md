# Strengths — metaphaze landing page
> Phase: critique | Project: landing-page | Date: 2026-04-08

These are the elements that work and must be preserved through the build phase. Changes to these without strong reason would make the design worse.

---

## 1. The manifesto as the second thing you read

After the `mz▌` logo, the first text on the page is:

> the orchestrator runs outside the loop. claude builds. mz drives.

This is not a tagline, not a USP, not a "value proposition." It is a sentence that describes the thing accurately without explaining it. The Senior Operator reads it and either gets it (installs) or doesn't (leaves). That is the correct filter. The install command follows immediately, completing the hero before the reader needs to scroll.

The manifesto's placement — after the logo but before any explanation — says "the name and the claim are the same thing." Preserve this order absolutely.

---

## 2. The refusals list as the footer's primary content

The footer's primary visual element is:

> no accounts · no cloud · no telemetry · no permission slips · no dashboards · no config files · no hallucinated toolchain

This is the page's most distinctive positioning move. Every other tool in the category has a footer with copyright and links. metaphaze's footer has a list of things it will never have. This is not decoration — it is a product promise, stated in the place traditionally reserved for boilerplate.

The `·` separators and the lowercase voice make the list feel like terminal output rather than marketing copy. Keep every item. Keep the exact order. Do not abbreviate.

---

## 3. The comparison table's bracketed badge vocabulary

Using `[FIRST-PARTY]`, `[THIRD-PARTY]`, `[OK]`, `[ERR]`, `[WARN]` in the comparison table instead of checkmarks, crosses, or icons is the correct choice. It does three things simultaneously:
- Uses the brand's existing status badge vocabulary (no new visual system introduced)
- Is more precise than binary checkmarks (the distinction between `[FIRST-PARTY]` and `[THIRD-PARTY]` carries semantic weight a checkmark can't)
- Is terminal-native — a developer reading the table in a text browser or a `cat`-ed version of the README still understands the comparison

The legend below the table (`[FIRST-PARTY] = direct anthropic api...`) is exactly the right treatment — it explains without patronising. Keep the legend at its current position.

---

## 4. The VHS pane's dashed title `╌ mz auto ╌`

The decision to use `╌` (dashed horizontal) instead of `─` (solid horizontal) in the VHS Pane's title bar is a subtle, correct craft decision. It differentiates the "this is a recording of the running product" pane from the "this is a structured content block" panes. The dashed title signals liveness, impermanence, process — exactly what a terminal recording conveys.

This detail will likely not be noticed, but it would be noticed if it were wrong (the pane title looking identical to the static ASCII diagram panes would reduce clarity). Keep the dashed title treatment for the VHS pane.

---

## 5. The Single Client Component discipline

The design's decision to make `<CodeBlock>` the only `'use client'` component is a structural strength, not just a technical preference. It means:
- The page renders completely without JavaScript
- The only runtime dependency is the copy button
- Everything else — headings, nav, comparison table, install commands, footer — renders as static HTML

This is the "works in lynx" guarantee made real. The copy button gracefully degrades (the `<pre>` is still selectable). The VHS video still renders (no JS required for `<video autoplay muted loop>`). The cursor still blinks (pure CSS animation).

Keep this discipline in the build phase. Any feature that requires adding a second `'use client'` component should be challenged.

---

## 6. The three-code-block install section structure

The install section's three panes — `install`, `verify`, `first run` — answer three different user questions:
- `install`: what's the command? (copyable)
- `verify`: how do I know it worked? (`mz --version` output)
- `first run`: what's the first thing I do? (`mz status` with `[OK]` badges)

The three-pane structure serves users at different points in the install journey without being redundant. A user who already has cargo will skip `install` and read `verify`. A user who installed yesterday and wants the quick-start will read `first run`.

The inline `[OK]` text in the `first run` code block (matching badge colors but rendered as `<pre>` text, not `<StatusBadge>` components) is the correct implementation choice — it keeps the code block as literal terminal output, not a styled component grid.

---

## 7. The "works in lynx" principle as a design constraint

The brief states "The page reads identically in a desktop Chrome and in `lynx` (text-only browser)." This constraint, held throughout the design, has produced a page that degrades gracefully everywhere:
- All content is semantic HTML
- All diagrams are `<pre>` elements with text equivalents
- All interactive elements have keyboard equivalents
- The page conveys its full value proposition without any images, videos, or scripts

This is not just accessibility compliance — it is a positioning statement. A tool made by someone who writes code would survive `lynx`. This constraint must be preserved as a build-phase testing criterion, not just a design aspiration.
