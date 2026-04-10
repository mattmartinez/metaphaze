# screen-01-landing build log

## screen
- **id**: screen-01-landing
- **path**: `app/page.tsx`
- **target**: `~/Documents/bluehelixlab/metaphaze-www/app/page.tsx`
- **build**: `npx next build` — compiled cleanly (Next.js 16.2.3, Turbopack)

## approach
Built the full landing page as a single server component at `app/page.tsx`,
composing the brand components that already exist under
`components/brand/`. No new components were created; foundations were read
first to confirm props, semantics, and CSS classes.

The page is a single narrow column (`max-w-3xl`, `px-6`), with five
sections matching the design spec exactly.

## sections implemented
1. **Hero** (`#hero`)
   - `<h1>` "mz" at `--text-5xl` / `font-weight: 700`, followed by
     `<Cursor blink={true} />`.
   - Two-line manifesto at `--text-base` in `--mz-fg`.
   - `<CodeBlock prompt="$" copyable highlight="metaphaze" paneTitle="install">`
     containing `cargo install --git https://github.com/mattmartinez/metaphaze`.
   - `<Pane title="╌ mz auto ╌">` wrapping a `<picture>` / `<video>` that
     autoplays, loops, is muted, and plays inline. The `<picture>`
     contains an HTML comment flagging the light-mode VHS TODO. A
     fallback `<p>` inside `<video>` is shown if the source is missing
     (which it currently is — `/vhs/demo-desktop.webm` is a placeholder).
   - `<Pane title="what a phase transition looks like">` wrapping
     `<PhaseTransitionScreen />`.

2. **What It Does** (`#what-it-does`)
   - `<h2>` "what it does" using `.mz-section-heading`.
   - `<hr>` divider with `border-top: 1px solid var(--mz-border)`.
   - Two `<p>` elements carrying the spec copy inside a `.mz-prose`
     container.
   - `<figure role="img" aria-label="...">` wrapping
     `<Pane title="the loop">` wrapping `<pre aria-hidden="true">` with
     the exact 66-char ASCII loop diagram.

3. **Why It's Different** (`#why-different`)
   - `<h2>` "why it's different" + `<hr>`.
   - `<Pane title="mz vs. other harnesses">` wrapping
     `<ComparisonTable />` (already has caption + scoped headers).
   - Two legend paragraphs using `<StatusBadge variant="first-party">`
     and `<StatusBadge variant="third-party">`. Each paragraph has an
     id (`legend-first-party`, `legend-third-party`) so the badges
     elsewhere could pick it up via `aria-describedby` (the component
     already accepts the prop).

4. **Install** (`#install`)
   - `<h2>` "install" + `<hr>`.
   - Prerequisites: `<p>` "prerequisites" in `--text-sm`, `font-weight: 600`,
     then a `<ul>` where each `<li>` is prefixed with a muted `›`.
   - `<CodeBlock paneTitle="install" copyable highlight="metaphaze">` for
     the cargo install command.
   - `<CodeBlock paneTitle="verify">` with multiline `mz --version\nmetaphaze 0.1.0`.
   - **first run** is rendered as a `<Pane title="first run">` with a
     custom `<pre>` inside, because the spec calls for inline colored
     `[OK]` / `●` markers that cannot be achieved via `<CodeBlock>`
     (which only accepts a string child). Each marker is wrapped in a
     `<span className="mz-code-accent">` which applies `var(--mz-signal)`.

5. **Docs Link** (`#docs-link`)
   - Dashed `<hr>` (inline `style={{ borderTop: "1px dashed ..." }}`
     because Tailwind v4 here doesn't expose a short utility for dashed
     border-top colour via `--mz-border`).
   - `<p>` containing `[/docs]` linking to
     `https://github.com/mattmartinez/metaphaze#readme` (`mz-nav-link pl-5`)
     followed by `" →  full documentation, api reference, and examples"`
     in `--mz-fg-muted`.

## critique fixes applied
1. `[/docs]` → `https://github.com/mattmartinez/metaphaze#readme` (not
   a coming-soon page).
2. Clipboard failure: handled inside `<CodeBlock>` via try/catch (already
   there).
3. Section 5 separator: dashed `<hr>` via inline style (no decorative
   `─ · ─`).
4. Comparison table caption: already in `<ComparisonTable>`.
5. Light-mode VHS mismatch: HTML comment placed inside `<picture>`
   element as a reminder.
6. `aria-describedby`: legend `<p>` elements have ids
   (`legend-first-party`, `legend-third-party`) for future wiring; the
   `StatusBadge` component already accepts the prop.

## accessibility
- Skip link lives in `layout.tsx`.
- Sections each have `id` and `aria-label`.
- ASCII diagrams are wrapped in `<figure role="img" aria-label>` with
  inner `<pre aria-hidden="true">`.
- `<video>` has `aria-label`, plus a readable fallback `<p>` for when
  the source is missing.
- `<Cursor blink={true} />` handles `prefers-reduced-motion` internally.
- `prefers-reduced-motion` is also handled globally via `.mz-cursor`
  animation rules in `globals.css`.
- `focus-visible` outlines are in `globals.css`.

## build verification
```
▲ Next.js 16.2.3 (Turbopack)
✓ Compiled successfully in 2.9s
  Running TypeScript ... Finished TypeScript in 4.0s
✓ Generating static pages using 5 workers (4/4)

Route (app)
┌ ○ /
└ ○ /_not-found
```

No TypeScript errors. No ESLint failures. All routes are prerendered
as static content. The only warning is the multiple-lockfile workspace
warning, which is unrelated to this screen.

## files touched
- `app/page.tsx` (rewritten)

## files NOT touched
- Any component under `components/brand/` (all foundations used as-is)
- `app/layout.tsx`
- `app/globals.css`

## known placeholders
- `/public/vhs/demo-desktop.webm` does not exist yet. The `<video>`
  element shows a text fallback until the file lands. This is
  acceptable for v1.0 per the spec.
