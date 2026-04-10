# Target Adaptations

> Phase: brief | Project: landing-page | Generated: 2026-04-10

---

## Overview

How the brand system is adapted for the single-page Next.js 14 landing context. The brand is already dual-target (Rust + Web), so most adaptation is selection and composition, not invention. This document captures the project-specific decisions that sit on top of the brand.

## Token overrides

**None.** The project inherits all tokens from `.design/branding/metaphaze/patterns/metaphaze.yml` without modification.

The CSS custom properties block from `token-mapping.md` is copied directly into `app/globals.css`:

```css
@import "tailwindcss";

@theme {
  --color-mz-bg:        #0a0a0a;
  --color-mz-fg:        #ededed;
  --color-mz-fg-muted:  #8a8a8a;
  --color-mz-border:    #2a2a2a;
  --color-mz-signal:    #5fb878;
  --color-mz-warn:      #d4a017;
  --color-mz-error:     #b8860b;

  --font-mono: "Berkeley Mono", "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  --font-sans: var(--font-mono);

  --text-xxs:  0.80rem;
  --text-xs:   0.8125rem;
  --text-sm:   0.90rem;
  --text-base: 1.00rem;
  --text-lg:   1.125rem;
  --text-xl:   clamp(1.125rem, 0.96rem + 0.83vw, 1.25rem);
  --text-2xl:  clamp(1.25rem, 1.04rem + 1.04vw, 1.5625rem);
  --text-3xl:  clamp(1.5625rem, 1.25rem + 1.56vw, 1.9531rem);
  --text-4xl:  clamp(1.9531rem, 1.56rem + 1.95vw, 2.4413rem);
  --text-5xl:  clamp(2.4413rem, 1.95rem + 2.44vw, 3.0518rem);
}

@media (prefers-color-scheme: light) {
  :root {
    --color-mz-bg:       #fafafa;
    --color-mz-fg:       #0a0a0a;
    --color-mz-fg-muted: #6a6a6a;
    --color-mz-border:   #e0e0e0;
    --color-mz-signal:   #2e8b57;
    --color-mz-warn:     #b8860b;
    --color-mz-error:    #8b6508;
  }
}
```

## Component adaptations

Seven brand patterns → seven React components. Each wraps its brand equivalent, applied to the Next.js App Router context.

### 1. `<Cursor />` — from `cursor.md`

**Source:** `{BRAND_PATH}/patterns/components/cursor.md`
**Adaptation:** React component that renders a `<span>` with the U+258C character, CSS-animated via `@keyframes mz-cursor-blink` from the brand. Honors `prefers-reduced-motion: reduce` — when the user prefers reduced motion, the cursor is visible but does not blink.

**Props:**
- `blink?: boolean` — default `true`, set to `false` for static media contexts (e.g., footer)

**Usage on the landing page:**
- Hero: next to `mz` in the logo lockup
- Install section: at the end of the "run this" line

### 2. `<Pane />` — from `pane.md`

**Source:** `{BRAND_PATH}/patterns/components/pane.md`
**Adaptation:** React component that renders a `<section>` with a 1px `--color-mz-border` border, optional title bar with box-drawing characters (`┌─ title ──┐`), and children as content.

**Props:**
- `title?: string` — optional title bar
- `variant?: 'idle' | 'active' | 'completed' | 'blocked'` — controls border color per `pane.md` spec
- `children: ReactNode`

**Usage on the landing page:**
- VHS hero frame (variant=`idle`, title=`╌ mz auto ╌`)
- Install code block wrapper (variant=`idle`)
- "Why it's different" comparison table wrapper

### 3. `<BracketedButton />` — from `bracketed-button.md`

**Source:** `{BRAND_PATH}/patterns/components/bracketed-button.md`
**Adaptation:** Wraps the shadcn `<Button>` with the override CSS that removes borders, shadows, and radius. Renders children inside `[ ... ]` brackets. `onClick` handler for the copy-to-clipboard action on the install block.

**Props:**
- `onClick?: () => void`
- `aria-label?: string`
- `variant?: 'primary' | 'secondary'`
- `children: ReactNode` — rendered as `[ {children} ]`

**Usage on the landing page:**
- `[ copy ]` button next to the install command (onClick copies to clipboard)

### 4. `<StatusBadge />` — from `status-badge.md`

**Source:** `{BRAND_PATH}/patterns/components/status-badge.md`
**Adaptation:** Renders a `<span class="mz-badge mz-badge-{variant}">[TEXT]</span>` with the color applied to the whole token. For the landing page comparison row, additional variants needed beyond the brand defaults.

**Props:**
- `variant: 'ok' | 'warn' | 'error' | 'info' | 'pending'` — standard brand variants
- `customColor?: 'first-party' | 'third-party'` — **project-specific extension** for the comparison row (maps to `--color-mz-signal` for first-party, `--color-mz-error` for third-party)
- `children: string` — renders inside brackets, auto-uppercased

**Project-specific extension (documented deviation):**
The comparison row needs badges that aren't strictly status codes — they're positioning labels. The extension adds `first-party` and `third-party` as custom variants. These reuse the brand signal and error colors but with semantic labels specific to this page. The extension is documented here and referenced in `gap-analysis.md`.

**Usage on the landing page:**
- `[FIRST-PARTY]` (signal green) and `[THIRD-PARTY]` (amber error) in the comparison row
- `[OK]` and `[ERR]` in the "before/after April 4" comparison (standard variants)

### 5. `<CodeBlock />` — from the brand `.yml` `patterns.code-block`

**Source:** `{BRAND_PATH}/patterns/metaphaze.yml` → `patterns.code-block`
**Adaptation:** Not a component spec in the brand system, so this project produces it. A React component with `<pre><code>` inside a styled wrapper — `--color-mz-border` background, `$` or `>` prompt prefix in `--color-mz-fg-muted`, optional signal-colored accent run.

**Props:**
- `prompt?: '$' | '>' | '~' | null` — default `$`, `null` for no prompt
- `children: string` — the code
- `highlight?: string` — substring to render in `--color-mz-signal` (at most one per block)
- `copyable?: boolean` — default `false`, when `true` renders a `<BracketedButton>[ copy ]</BracketedButton>` in the top-right

**Note:** This component is NOT in the brand's `components/` directory. It should be added there later as `code-block.md`, but for this project we're building it inline. See `gap-analysis.md`.

**Usage on the landing page:**
- Install command block (`copyable={true}`, `highlight="mz"`)
- `mz status` example in the "what it does" section (no highlight)
- `mz --help` example in the "what it does" section (no highlight)

### 6. `<PhaseTransitionScreen />` — from `phase-transition.md`

**Source:** `{BRAND_PATH}/patterns/components/phase-transition.md`
**Adaptation:** Static `<pre>` element rendering a hardcoded phase transition example. NOT a live component — the actual transitions are rendered by the Rust TUI. The landing page shows a **screenshot in text form** that matches the real output format.

**Props:**
- None. The content is hardcoded for the landing page.

**Usage on the landing page:**
- Below the VHS hero, labeled "what a phase transition looks like"

### 7. `<Footer />` — composed from multiple brand pieces

**Source:** no single brand component. Composed from:
- `voice-and-tone.md` for the copy voice
- The refusals list concept from `positioning.md`
- Bracketed nav from the typography rules

**Adaptation:** A `<footer>` with three blocks:
1. `[MIT]` · `[github.com/mattmartinez/metaphaze]` · `[generated by gsp]`
2. The refusals list: "no accounts · no cloud · no telemetry · no permission slips"
3. The manifesto line in `--color-mz-fg-muted`

**Project-specific:** The footer layout is web-specific. The brand system does not have a "footer" component because READMEs don't have footers. This is invented here and should be promoted to the brand system as `footer.md` in a later phase if reused.

## Platform considerations

**Desktop-first.** Design and test at 1280px first. The Senior Operator is a desktop user.

**Mobile-supported.** At 640px and below, the hero VHS switches to the mobile variant (100 cols, tighter crop), the comparison table stacks vertically, the install command code block gets horizontal scroll with a `visible scrollbar: auto` style. The footer links stack.

**No tablet-specific breakpoint.** The brand's breakpoints are binary: desktop (≥641px) or mobile (<641px). Tablets render the desktop layout.

**Dark mode is default.** The site respects `prefers-color-scheme: light` and inverts via the CSS custom properties. No manual theme toggle — the user's OS preference is the source of truth.

## Implementation target mapping

Implementation target is `code` — we produce actual Next.js source files.

| Design component | Target primitive | Notes |
|---|---|---|
| `<Cursor />` | custom React component | No shadcn equivalent |
| `<Pane />` | custom React component | No shadcn equivalent (shadcn `<Card>` has border-radius and shadow, can't be overridden cleanly) |
| `<BracketedButton />` | shadcn `<Button>` + override CSS | Use the shadcn Button as a base, override via `className` + the global CSS from `token-mapping.md` |
| `<StatusBadge />` | shadcn `<Badge>` + override CSS | Similar pattern |
| `<CodeBlock />` | custom React component | No shadcn equivalent |
| `<PhaseTransitionScreen />` | static `<pre>` in a `<Pane>` | Just HTML |
| `<Footer />` | custom React component | Composed |

**Libraries to install:**
- `next@14` — framework
- `react@18`, `react-dom@18` — runtime
- `tailwindcss@4` — styling (v4 uses CSS `@theme`, not tailwind.config.js)
- `@radix-ui/react-slot` — required by shadcn Button
- Install shadcn components via `npx shadcn@latest add button badge` — see `install-manifest.md`

No React Query, no form libraries, no animation libraries (no animation beyond the cursor blink), no state management. The page is static.

## Cross-references

- Brand components: `.design/branding/metaphaze/patterns/components/`
- Token mapping: `.design/branding/metaphaze/patterns/components/token-mapping.md`
- Brand `.yml`: `.design/branding/metaphaze/patterns/metaphaze.yml`
- Brand guidelines.html (for visual reference): `.design/branding/metaphaze/patterns/guidelines.html`
