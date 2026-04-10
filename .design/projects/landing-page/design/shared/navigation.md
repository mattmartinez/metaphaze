# Navigation

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Pattern

**Type:** Fixed top bar with bracketed links. Single-page — no router navigation except anchor jumps within the page and external links.

This is a minimal nav bar, not a mega-menu or sidebar. The Senior Operator reads README files; the nav bar is the "Related" section at the top.

---

## Primary Nav

Items (desktop, left-to-right after the logo):

```
mz▌    [/docs]  [/source]
```

- **Logo:** `mz▌` — same glyph as the hero logo, but smaller. Links to `#hero` (page top / anchor scroll). The `▌` cursor blinks on the nav logo too.
- **`[/docs]`** — links to the future docs site (external link, `target="_blank"`, `rel="noopener"`). Grayed out with a `[SOON]` badge or simply present and linking to the GitHub wiki until docs ship.
- **`[/source]`** — links to `https://github.com/mattmartinez/metaphaze` (external, `target="_blank"`, `rel="noopener"`).

Items are bracketed per the brand pattern: `[/docs]` not `docs`. The slash prefix makes them look like file paths, which reinforces the terminal metaphor.

---

## Secondary Nav

None. No hamburger. No dropdown. No sub-navigation.

The page has no sections that warrant their own top-level nav item. Users scroll. The page is short enough that a smooth scroll + anchor link on the install command is sufficient.

---

## Nav Bar Specifications

| Property | Value |
|----------|-------|
| Position | `fixed` top, full width |
| Background | `var(--mz-bg)` — opaque, no blur |
| Border-bottom | `1px solid var(--mz-border)` |
| Height | `48px` desktop, `40px` mobile |
| Max-width | `max-w-3xl` (`48rem`), centered |
| Logo font-size | `--text-base` |
| Nav link font-size | `--text-sm` |
| Nav link color | `var(--mz-fg)` default, `var(--mz-fg)` hover with `>` prefix |
| Padding | `px-4` on mobile, `px-0` inside max-width container on desktop |

---

## Hover State

Per `STYLE.md` `nav` pattern:
- Default: `[/docs]`
- Hover: `> [/docs]` — a `>` character slides in from the left (implemented via `::before` pseudo with `content: "> "` and `opacity: 0 → 1` at `transition: none` / `step-end`), underline appears

```css
.mz-nav a::before {
  content: "> ";
  color: var(--mz-fg);
  opacity: 0;
}
.mz-nav a:hover::before {
  opacity: 1;
}
.mz-nav a:hover {
  text-decoration: underline;
}
```

Transition: `0ms` or `100ms step-end` per brand. No fade. No slide animation.

---

## Mobile Nav

At `< 640px`:
- Same three items, but logo glyph only (no label text)
- Nav links collapse to two items maximum. If three items, consider hiding `[/docs]` if docs aren't live
- No hamburger menu — the nav stays inline, items stack if needed
- Font size: `--text-xs` for links

---

## Accessibility

- `<nav aria-label="primary navigation">` wrapping element
- Each link has descriptive `aria-label` beyond the bracket text: `aria-label="documentation"`, `aria-label="source code on GitHub"`
- Tab order: logo → docs → source
- Logo cursor animation: `prefers-reduced-motion: reduce` stops the blink

---

## Related

- [personas.md](./personas.md)
- [information-architecture.md](./information-architecture.md)
- [micro-interactions.md](./micro-interactions.md)
- Brand component: `.design/branding/metaphaze/patterns/components/cursor.md`
