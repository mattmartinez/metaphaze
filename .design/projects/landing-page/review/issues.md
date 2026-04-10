# Issues — metaphaze landing page QA Review

> Phase: review | Project: landing-page | Date: 2026-04-08

**Verdict:** Conditional Pass — 2 Major issues must be fixed before launch.

---

## Issues Table

| # | Issue | Severity | File | Line | Expected | Actual | Remediation |
|---|-------|----------|------|------|----------|--------|-------------|
| 1 | Light-mode signal text fails WCAG AA for small text | **Major** | `app/globals.css` | 67-74 | `--mz-signal-text: #1a6b3a` in light mode block for small text (`< 24px`) | Only `--mz-signal: #2e8b57` (4.06:1 — passes large text only) | Add `--mz-signal-text` token for light mode. Apply to badge text, code accents below 24px. See critique/accessibility-fixes.md Issue 1. |
| 2 | No video pause control (WCAG SC 2.2.2) | **Major** | `app/page.tsx` | 60-77 | `[ pause ]` / `[ play ]` button in VHS pane title bar | Video has `autoPlay muted loop` with no user control | Convert VHS pane to Client Component. Add videoRef + state toggle. Button uses `.mz-btn` class: `[ pause ]` / `[ play ]`. Position in pane header right side. See critique/accessibility-fixes.md Issue 2. |
| 3 | `scroll-padding-top` missing for fixed nav (WCAG SC 2.4.11) | Minor | `app/globals.css` | — (absent) | `html { scroll-padding-top: 48px; }` | Not set — focused elements near top may be obscured by 48px fixed nav | Add `scroll-padding-top: 48px` to `html` rule in globals.css (line ~98) |
| 4 | `aria-describedby` not wired in ComparisonTable | Minor | `components/brand/comparison-table.tsx` | 99, 107 | `<StatusBadge variant="first-party" aria-describedby="legend-first-party">` in table rows | Prop not passed — `StatusBadge` accepts it but `ComparisonTable` doesn't use it | In `comparison-table.tsx`, pass `aria-describedby="legend-first-party"` to first-party badge and `aria-describedby="legend-third-party"` to third-party badge in the first data row |
| 5 | Section IDs diverge from design spec | Minor | `app/page.tsx` | 87, 125 | `id="what"` and `id="why"` (per design spec) | `id="what-it-does"` and `id="why-different"` | No functional impact. No nav links point to these IDs. Accept as-is or align with spec if external deeplinks are anticipated. |
| 6 | PhaseTransitionScreen mixes `<StatusBadge>` inside `<pre>` | Minor | `components/brand/phase-transition.tsx` | 23-27 | Pure text `[OK]` inside `<pre>` with color via parent CSS context | `<StatusBadge>` React components embedded inside `<pre>` content | The `<pre>` has `aria-hidden="true"` so this is invisible to screen readers. Visually it renders correctly (inline `<span>` inside `<pre>`). Acceptable for v1.0 — note for future: `<pre>` with mixed React components can produce subtle whitespace artifacts. |

---

## Issue Details

### Issue 1 — Light-mode signal text (Major — must fix)

**WCAG:** SC 1.4.3 Contrast Minimum (AA) for normal text < 24px

`#2e8b57` on `#fafafa` = 4.06:1. Fails for normal text. The `[OK]` badge at `--text-xs` (0.8125rem, ~13px) fails.

**Fix in `app/globals.css`:**
```css
@media (prefers-color-scheme: light) {
  :root {
    --mz-bg:          #fafafa;
    --mz-fg:          #0a0a0a;
    --mz-fg-muted:    #6a6a6a;
    --mz-border:      #e0e0e0;
    --mz-signal:      #2e8b57;      /* large glyphs ≥ 24px: cursor ▌ */
    --mz-signal-text: #1a6b3a;      /* small text < 24px: [OK] badge, code accents */
    --mz-warn:        #b8860b;
    --mz-error:       #8b6508;
  }
}
```

Then update `.mz-badge-ok` and `.mz-code-accent` to use `--mz-signal-text` in light mode:
```css
@media (prefers-color-scheme: light) {
  .mz-badge-ok,
  .mz-badge-first-party,
  .mz-code-accent { color: var(--mz-signal-text); }
}
```

**Note:** This is a brand-level token change. If `--mz-signal-text` is adopted permanently, add it to `metaphaze.yml` and regenerate STYLE.md via `/gsp-brand-refine`.

---

### Issue 2 — Video pause button (Major — must fix)

**WCAG:** SC 2.2.2 Pause, Stop, Hide

`app/page.tsx` lines 57-83: VHS pane has no pause control.

**Fix — convert VHS pane to Client Component:**

Extract the hero VHS section to a `components/brand/vhs-pane.tsx` "use client" component:

```tsx
"use client";
import { useRef, useState, useEffect } from "react";
import { Pane } from "./pane";

export function VhsPane() {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [playing, setPlaying] = useState(true);
  const [reducedMotion, setReducedMotion] = useState(false);

  useEffect(() => {
    const mq = window.matchMedia("(prefers-reduced-motion: reduce)");
    if (mq.matches) {
      setPlaying(false);
      videoRef.current?.pause();
    }
    setReducedMotion(mq.matches);
    const handler = (e: MediaQueryListEvent) => {
      setReducedMotion(e.matches);
      if (e.matches) { videoRef.current?.pause(); setPlaying(false); }
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  const toggle = () => {
    if (playing) { videoRef.current?.pause(); setPlaying(false); }
    else { videoRef.current?.play(); setPlaying(true); }
  };

  return (
    <Pane
      title="╌ mz auto ╌"
      action={
        <button onClick={toggle} aria-label={playing ? "pause demo video" : "play demo video"} className="mz-btn text-[length:var(--text-xs)]">
          [ {playing ? "pause" : "play"} ]
        </button>
      }
    >
      <picture>
        {/* TODO: light-mode VHS variant needed — see issue #N */}
        <video
          ref={videoRef}
          autoPlay={!reducedMotion}
          muted loop playsInline
          aria-label="metaphaze auto mode demo, recorded terminal session"
          className="w-full block"
        >
          <source src="/vhs/demo-desktop.webm" type="video/webm" />
          <p className="font-mono text-[length:var(--text-sm)] text-[var(--mz-fg-muted)] p-4">
            your browser cannot play this video. the demo shows
            metaphaze driving claude code through an autonomous
            multi-phase project.
          </p>
        </video>
      </picture>
    </Pane>
  );
}
```

Also update `Pane` component to accept an `action` prop for title-bar button placement (right side of title bar).

---

### Issue 3 — scroll-padding-top (Minor)

**Fix in `app/globals.css`** — add to the `html` rule (around line 95):
```css
html {
  font-family: var(--font-mono);
  font-size: 16px;
  line-height: 1.6;
  scroll-padding-top: 48px; /* fixed nav height — prevents focus obscured by sticky bar */
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
```

---

### Issue 4 — aria-describedby wiring (Minor)

**Fix in `components/brand/comparison-table.tsx`** — pass `aria-describedby` to the first-party and third-party badges in the data rows:

```tsx
// In the desktop <table> body, api access row:
<StatusBadge variant={row.mzBadge} aria-describedby={row.mzBadge === "first-party" ? "legend-first-party" : undefined}>
  {mzLabel(row.mzBadge)}
</StatusBadge>
<StatusBadge variant={row.otherBadge} aria-describedby={row.otherBadge === "third-party" ? "legend-third-party" : undefined}>
  {otherLabel(row.otherBadge)}
</StatusBadge>
```

This only needs to apply in the first data row (api access) where first-party and third-party appear. Remaining rows use ok/warn/error variants which are self-explanatory.

---

## Non-Issues (Verified Acceptable)

| Item | Reason |
|------|--------|
| Border non-text contrast 1.38:1 | Intentional brand decision (STYLE.md "invisible box" aesthetic). Documented in critique/accessibility-audit.md as acceptable — Option B chosen. Not an implementation bug. |
| `BracketedButton` not used directly in page | Component exists and is complete. `CodeBlock` uses `.mz-btn` CSS class directly — consistent pattern. No functional gap. |
| `id` divergence (`what-it-does` vs `what`) | No nav links point to these IDs. Acceptable variant. |
| Nav changed from `[/docs] [/source]` to `[/install] [/source]` | Correctly addresses critique fix #2 (Important). `[/docs]` preserved in §5. |
| VHS recording absent | Known gap per BUILD-LOG.md. Fallback renders. Expected at launch. |
| Light-mode VHS mismatch | HTML comment in `<picture>` element per critique fix #6 (`TODO: light-mode VHS variant needed`). |

---

## Related

- Acceptance report: [acceptance-report.md](./acceptance-report.md)
- Design screen: [../design/screen-01-landing.md](../design/screen-01-landing.md)
- Prior a11y audit: [../critique/accessibility-audit.md](../critique/accessibility-audit.md)
- Critique fixes: [../critique/accessibility-fixes.md](../critique/accessibility-fixes.md)
- Brand STYLE.md: [../../branding/metaphaze/patterns/STYLE.md](../../branding/metaphaze/patterns/STYLE.md)
