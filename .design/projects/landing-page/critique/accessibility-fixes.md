# Accessibility Fixes — metaphaze landing page
> Phase: critique | Project: landing-page | Auditor: GSP Accessibility Auditor | Date: 2026-04-08

Full audit: [accessibility-audit.md](./accessibility-audit.md)  
Design fixes: [prioritized-fixes.md](./prioritized-fixes.md)  
Reference screen: [screen-01-landing.md](../design/screen-01-landing.md)

---

## Violations

| # | Issue | Severity | WCAG Criterion | Remediation |
|---|-------|----------|----------------|-------------|
| 1 | Light-mode signal text too small | **Major** | SC 1.4.3 (Contrast Minimum) | See below |
| 2 | No explicit video pause control | **Major** | SC 2.2.2 (Pause, Stop, Hide) | See below |
| 3 | Border non-text contrast | **Major** | SC 1.4.11 (Non-text Contrast) | See below |
| 4 | Table caption text unspecified | Minor | SC 1.3.1 (Info and Relationships) | Add `<caption>mz versus other ai coding harnesses — key properties compared</caption>` |
| 5 | Focus not obscured by sticky nav | Minor | SC 2.4.11 (Focus Not Obscured) | Add `scroll-padding-top: 48px` on `html` element |
| 6 | Nav link touch targets | Minor | SC 2.5.8 (Target Size) | Ensure nav links have `min-height: 44px` or `padding: 12px 0` |
| 7 | `:focus-visible` specificity vs `--ring: transparent` | Minor | SC 2.4.7 (Focus Visible) | Verify specificity; use `:focus-visible` on interactive elements at sufficient specificity to override `--ring: transparent` |

---

## Issue 1 — Light-mode signal text too small

**Severity:** Major  
**WCAG:** SC 1.4.3 Contrast Minimum (AA)  
**Location:** All surfaces with `--mz-signal` in light mode, specifically `[OK]` badge text at `--text-sm` or smaller

**Problem:** `--mz-signal` in light mode is `#2e8b57` on `--mz-bg` `#fafafa`, ratio 4.06:1. This passes AA for large text (≥24px / ≥18.66px bold) but fails for normal text. The `[OK]` badge in the comparison table renders at `--text-sm` (14.4px), which fails.

**Remediation:**

Option A (recommended — no visual change): Apply a darker signal green for small text in light mode only. Add a new token `--mz-signal-text` for light mode:
```css
@media (prefers-color-scheme: light) {
  :root {
    --mz-signal: #2e8b57;        /* large glyphs, cursor (≥24px) */
    --mz-signal-text: #1a6b3a;   /* small text, badges (< 24px) — ratio ~7:1 on #fafafa */
  }
}
```
Apply `--mz-signal-text` to `[OK]` badge content, inline code highlights, and any signal-colored text below 24px in light mode. Keep `--mz-signal` for the cursor (block character, visually "large").

Option B (conservative): In light mode, render `[OK]` badge in `--mz-fg` (`#0a0a0a`) with no color — sacrifice the green signal in light mode for full contrast. Simple but loses brand signal in light mode.

**Build note:** This is a constraint the design system has already documented as known. The build phase must enforce `--mz-signal-text` for small text in light mode. Do not use `--mz-signal` for text smaller than 24px in light mode.

---

## Issue 2 — No explicit video pause control

**Severity:** Major  
**WCAG:** SC 2.2.2 Pause, Stop, Hide  
**Location:** `<video autoplay muted loop>` in [screen-01-landing.md](../design/screen-01-landing.md) — Hero section

**Problem:** WCAG SC 2.2.2 requires that auto-playing, looping content can be paused or stopped. The `prefers-reduced-motion` handling stops autoplay for users who declare that preference, but users without that OS preference set have no way to pause the video without right-clicking for browser native controls — which many users don't know about.

**Remediation:**

Add a minimal `[ pause ]` / `[ play ]` toggle button to the VHS pane title bar or as an overlay:

```tsx
// In the Pane title bar for the VHS pane
const [playing, setPlaying] = useState(true);

<Pane title={`╌ mz auto ╌`} action={
  <button
    onClick={() => {
      videoRef.current[playing ? 'pause' : 'play']();
      setPlaying(!playing);
    }}
    aria-label={playing ? 'pause demo video' : 'play demo video'}
    className="mz-btn"
  >
    {playing ? 'pause' : 'play'}
  </button>
}>
```

The button uses the brand's `[ label ]` pattern. Position it in the top-right corner of the pane header (matching the `[ copy ]` button position in the install pane). This requires the VHS pane to become a Client Component, but the state is self-contained.

Alternatively: add `controls` attribute to the `<video>` element. Browser-native controls are less on-brand but require zero additional JavaScript.

---

## Issue 3 — Border non-text contrast

**Severity:** Major  
**WCAG:** SC 1.4.11 Non-text Contrast  
**Location:** All `<Pane>` components, `<CodeBlock>` components, `<ComparisonTable>`

**Problem:** Pane borders use `--mz-border` (`#2a2a2a` dark / `#e0e0e0` light) against `--mz-bg` (`#0a0a0a` dark / `#fafafa` light). Ratios: dark mode 1.38:1, light mode 1.13:1. WCAG SC 1.4.11 requires non-text UI component contrast of 3:1 against adjacent colors.

**Severity note:** This is a known design constraint. The `--mz-border` / `--mz-bg` pair has never had sufficient non-text contrast — the palette was designed for text legibility, not UI component borders. The brand's color system documents `--mz-slate` as "Structural only. Never a text color" — the designers know it's low contrast.

**Remediation options:**

Option A (recommended — minimal visual change): Replace `--mz-border` token values with slightly brighter versions that meet 3:1 while staying within the monochrome aesthetic:
- Dark mode: `--mz-border: #4a4a4a` (ratio: 2.65:1 on `#0a0a0a` — still fails)
- Dark mode: `--mz-border: #5a5a5a` (ratio: 3.05:1 on `#0a0a0a` — passes 3:1)
- Light mode: `--mz-border: #767676` (ratio: 4.54:1 on `#fafafa` — passes 4.5:1)

Visual impact: Pane borders would be slightly more visible. This does not violate the brand's "flat, no elevation" principle — it makes the structural boundaries more perceptible.

Option B (STYLE.md update needed): If the brand's intent is specifically low-contrast borders (the "invisible box" aesthetic), document this as a WCAG 2.2 AA exception in the accessibility statement and note it in STYLE.md as an intentional design decision. This is a legitimate product choice, but it must be explicit.

**Recommendation:** Option B is more honest for this brand. The `--mz-slate` border is intentionally subtle — it implies structure without announcing it. Brightening it changes the visual character. Document as an intentional exception.

**Note:** This issue should be tracked as a `/gsp-brand-refine` candidate — it requires updating the `.yml` source tokens, not just the landing page CSS.
