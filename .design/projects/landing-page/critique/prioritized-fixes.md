# Prioritized Fixes — metaphaze landing page
> Phase: critique | Project: landing-page | Date: 2026-04-08

See [critique.md](./critique.md) for full evaluation. See [accessibility-fixes.md](./accessibility-fixes.md) for accessibility-specific remediation.

---

## Critical (Must Fix Before Ship)

None. No Critical fixes identified.

The design has no constraint violations, no Lorem Ipsum, no broken flows, no WCAG failures at the AA level for the primary content.

---

## Important (High Priority — Fix Before Launch)

### 1. Resolve the `[/docs]` nav state before shipping

**Location:** nav bar, `[/docs]` link  
**Issue (H10 — 3/5):** The design notes that `[/docs]` may ship with a `[SOON]` badge or link to the GitHub wiki. Shipping a broken or disabled navigation item tells the first visitor that the product is incomplete. This is a first-impression problem.  
**Fix:**  
Option A (recommended): Link `[/docs]` to the GitHub README or GitHub wiki directly. The GitHub page is the real documentation until a proper docs site ships. No badge, no disabled state.  
Option B: Remove `[/docs]` from the nav and add it only when the docs site exists. The nav can have one item (`[/source]`) until then. Simpler, cleaner.  
Option C: If `[SOON]` must ship, implement it as a tooltip on hover rather than a badge. `aria-label="documentation — coming soon"` on the link, with the link pointing to an issue tracker URL or a placeholder page. Do not gray out the link (breaks hover state) — use full opacity with a `title` attribute.  

### 2. Add install section anchor to the nav or hero

**Location:** nav bar and/or hero section  
**Issue (H7 — 4/5):** Users arriving at the page via a link about installation have no shortcut to the install section. They must scroll through the hero and two content sections before reaching the full install instructions.  
**Fix:**  
Option A (recommended): Add `[/install]` as a third nav item linking to `#install`. Keeps the nav minimal while serving the common task. At mobile sizes (where nav collapses), this can be removed since the page is short enough to scroll.  
Option B: Below the hero manifesto and before the VHS pane, add a subtle `[ jump to install ↓ ]` link using the `.mz-nav a` treatment. This serves the user without adding to the top nav.  
**Note:** The nav currently has `[/docs] [/source]` — adding `[/install]` may make three items, which is the stated maximum. Remove `[/docs]` or keep `[/install]` mobile-only.

### 3. Specify clipboard API failure behavior in `<CodeBlock>`

**Location:** `<CodeBlock copyable={true}>` component  
**Issue (H5 — 4/5):** `navigator.clipboard.writeText()` throws if the page is served over HTTP (not HTTPS), if the user denies clipboard permission, or if the browser is unsupported. The design doesn't specify the failure mode.  
**Fix:** In the `handleCopy` function, wrap the clipboard call in try/catch:
```tsx
const handleCopy = async () => {
  try {
    await navigator.clipboard.writeText(INSTALL_CMD);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  } catch {
    // Clipboard unavailable — leave label as "copy", let user copy from <pre>
    // Optionally: briefly flash the pre/code element (outline: 2px solid --mz-signal for 300ms)
  }
};
```
The silent fallback (leave label as `copy`) is correct. Do not add a toast or error message — the Senior Operator can see the command in the `<pre>` and select-copy it. The `<pre>` is always selectable.

---

## Polish (If Time Allows)

### 4. Tighten the `─ · ─ · ─` separator in the docs link section

**Location:** section 5 (docs link), horizontal rule above `[/docs]`  
**Issue (Taste):** The alternating `─ · ─ · ─` rhythm is the weakest visual moment on the page. It reads as decorative rather than structural compared to the solid `─────────────` rule at the footer.  
**Fix:** Use a full-width `─` repeat rule (`border-top: 1px solid var(--mz-border)`) matching the footer divider. This unifies the divider vocabulary. The mid-page divider should not be visually lighter than the footer divider — it is separating a content section, not providing a secondary visual treatment.

### 5. Clarify screen reader label for the comparison table `<caption>`

**Location:** `<ComparisonTable />`, `<caption>` element  
**Issue (Minor accessibility):** The design specifies `<caption>` for the comparison table but doesn't provide the caption text. A screen reader user navigating tables will hear the caption before the data.  
**Fix:** Add `<caption>mz versus other ai coding harnesses — key properties compared</caption>` (lowercase, sentence-style). This is already implied by the `<Pane title="mz vs. other harnesses">` wrapper, but the `<table>` element itself needs its own `<caption>` for screen readers that navigate directly to tables.

### 6. Document the light-mode VHS mismatch as a known issue

**Location:** `design/screen-01-landing.md`, States section  
**Issue (Polish):** The design notes "Acceptable to ship with dark-only recording in v1.0 with a `[WARN]` noted in the design." This is correct but the `[WARN]` note should be in the HTML as a CSS comment or in a KNOWN_ISSUES.md, not only in the design document.  
**Fix:** Add an HTML comment in the `<picture>` element: `<!-- TODO: light-mode VHS variant needed for prefers-color-scheme: light -- https://github.com/mattmartinez/metaphaze/issues/N -->`. This keeps the technical debt visible to future contributors.

### 7. Specify `[FIRST-PARTY]` / `[THIRD-PARTY]` badge ARIA labels

**Location:** `<StatusBadge variant="first-party">` and `<StatusBadge variant="third-party">`  
**Issue (Minor accessibility):** These are project-local badge variants not in the brand system. A screen reader announces the text content (`[FIRST-PARTY]`) which is accurate but could be improved. The legend below the table already explains the terms.  
**Fix:** No change needed to the badge text. Add `aria-describedby` pointing to the legend paragraph IDs so screen readers can optionally surface the full explanation. Example:
```html
<span class="mz-badge mz-badge-first-party" aria-describedby="legend-first-party">[FIRST-PARTY]</span>
```

---

## [STYLE] Notes

No STYLE-level issues found. All constraint violations checked — none present. All brand patterns correctly applied. Effects vocabulary limited to the declared three techniques.

If future screen additions are needed, the following STYLE.md gaps should be addressed via `/gsp-brand-refine`:
- The `[SOON]` badge variant has no official spec in the brand system. If this pattern is used in multiple places, define it in `status-badge.md` as `variant="pending"` with `--mz-fg-muted` color and a documented use case.
- The `╌ mz auto ╌` dashed separator in the Pane title bar is not in the standard Pane component spec. If this pattern extends to other contexts, document it in `pane.md` as an optional `variant="recording"` title treatment.
