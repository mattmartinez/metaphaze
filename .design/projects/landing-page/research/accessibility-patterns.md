# Accessibility Patterns for Monospace-Heavy Landing Pages

WCAG 2.1 AA conformance research specific to a landing page that commits to monospace everywhere and uses ASCII box-drawing as illustration. Focus on realistic, measurable criteria rather than generic a11y checklists.

## Contrast Ratios

metaphaze's core palette meets WCAG AA comfortably:

| Foreground | Background | Ratio | AA Normal | AA Large | AAA |
|---|---|---|---|---|---|
| `#ededed` | `#0a0a0a` | **16.97:1** | pass | pass | pass |
| `#5fb878` signal | `#0a0a0a` | **8.06:1** | pass | pass | pass |
| `#d4a017` amber | `#0a0a0a` | **8.95:1** | pass | pass | pass |
| `#8a8a8a` muted | `#0a0a0a` | **5.61:1** | pass | pass | fail (AAA) |
| `#2a2a2a` border | `#0a0a0a` | **1.41:1** | N/A (non-text) | N/A | N/A |

Notes:
- WCAG AA requires 4.5:1 for normal text, 3:1 for large text (18pt+ or 14pt+ bold)
- WCAG AAA requires 7:1 for normal text, 4.5:1 for large text
- The muted `#8a8a8a` passes AA but fails AAA — acceptable for metadata, timestamps, secondary nav, but NOT for body copy
- Border `#2a2a2a` does not need to meet text contrast since it's a non-text UI element (WCAG 1.4.11 requires 3:1 for UI elements, and border vs background is 1.41:1, which is fine because borders don't convey essential info)

Reference: contrast ratios computed via `npx @cfpb/color-contrast` or the WebAIM Contrast Checker (webaim.org/resources/contrastchecker).

## Monospace Readability Concerns

Monospace fonts have measurable disadvantages for long-form reading:

1. **Lower x-height** — JetBrains Mono has an x-height of ~52% of em, versus ~54% for Inter. Berkeley Mono has ~50%. Smaller x-height means lower perceived size at the same point size.
2. **Uniform character width** — every character occupies the same horizontal space, so 'i' and 'm' look the same width. This reduces word-shape recognition, which is a key component of fast reading.
3. **Slower reading speed** — research (Bernard et al., 2002; later confirmed by Nielsen) suggests monospace reads ~10-15% slower than proportional fonts for long-form text.

**Mitigations metaphaze must apply:**

- **Larger base size**: use `16px` minimum, ideally `17-18px`, for body copy. Most sans-serif sites use `16px`; monospace needs more.
- **Generous line-height**: use `1.6-1.7` instead of the typical `1.5`. Monospace needs more vertical breathing room because characters are denser.
- **Shorter line length**: target 66-80 characters max per line. The standard "60-75 characters for sans-serif" applies in characters, not em — and since monospace characters are narrower than average proportional characters, 66-80 monospace chars is a similar visual line length to 60-75 proportional chars.
- **Higher contrast**: `#ededed` on `#0a0a0a` is 16.97:1, which is comfortably above the AAA threshold and compensates for the reading-speed cost.

## Line Length Recommendations

For a monospace-first page at 17px base:

- Body prose: `max-width: 66ch` (66 characters) — tight, reads like a README
- Code blocks: `max-width: 80ch` — the classic terminal width
- Hero manifesto: `max-width: 40ch` — short, punchy, hero-appropriate
- Footer fine print: `max-width: 80ch` — dense but acceptable

Use Tailwind's `max-w-[66ch]` utility or set CSS variables:

```css
:root {
  --prose-width: 66ch;
  --code-width: 80ch;
  --hero-width: 40ch;
}
```

Note: the `ch` unit in CSS is defined as the width of the "0" character in the current font. In a monospace font, `1ch` equals the width of ANY character. In Tailwind v4 with monospace as the default font, `max-w-[66ch]` behaves correctly.

## ASCII Box-Drawing and Screen Readers

This is the single most important a11y question for metaphaze: will screen readers correctly read the ASCII diagrams?

**The problem:** box-drawing characters (`─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`) are technically Unicode characters with names like "BOX DRAWINGS LIGHT HORIZONTAL." Some screen readers announce them literally ("box drawings light horizontal, box drawings light horizontal, box drawings light horizontal..." — nightmare).

**Testing results** (based on NV Access, VoiceOver, and JAWS documentation 2023-2024):
- **NVDA (Windows)**: by default, announces box-drawing characters as their Unicode names. Catastrophic for any ASCII diagram.
- **VoiceOver (macOS/iOS)**: handles box-drawing more gracefully — either skips them or announces "horizontal line." Still not ideal.
- **JAWS (Windows)**: configurable; default behavior varies by punctuation level.
- **TalkBack (Android)**: similar to VoiceOver, varies.

**The fix:**

Wrap ASCII diagrams in a container with:
```html
<figure role="img" aria-label="diagram: metaphaze orchestrates claude code sessions">
  <pre aria-hidden="true">
┌──────────┐    ┌───────────┐
│ metaphaze│───▶│claude code│
└──────────┘    └───────────┘
  </pre>
</figure>
```

The `aria-hidden="true"` on the `<pre>` hides the raw ASCII from screen readers, and the `aria-label` on the `<figure>` provides an accurate text description. This is the same pattern Oxide uses for its hardware rack diagrams.

For the `mz▌` logo:
```html
<h1>
  mz<span aria-hidden="true">▌</span>
  <span class="sr-only">metaphaze</span>
</h1>
```

The `▌` is decorative (it's a blinking cursor); `aria-hidden` hides it; `sr-only` provides the readable brand name.

## Keyboard Navigation

For a single-page site, keyboard nav is simple:

1. **Tab order must be logical**: logo → nav items → hero CTAs (if any) → install copy button → github link → footer links. This is the default DOM order for a well-structured page.
2. **Skip link**: add a `<a href="#main" class="sr-only focus:not-sr-only">skip to main content</a>` at the top. WCAG 2.4.1 requires a bypass mechanism for repetitive content. For a landing page with minimal nav, this is still required.
3. **Focus indicators**: the default browser focus ring is insufficient on a brutalist site — add a custom focus ring using `focus-visible:outline-2 focus-visible:outline-signal focus-visible:outline-offset-2`. The green signal color provides 8.06:1 contrast against the black background.
4. **Skip gaming**: don't trap focus anywhere. No modals, no popovers, no hover-only menus.

## Reduced Motion

metaphaze will have ONE animation: the blinking cursor glyph `▌` in the logo. Respect `prefers-reduced-motion`:

```css
@keyframes blink {
  50% { opacity: 0; }
}

.cursor-blink {
  animation: blink 1s step-end infinite;
}

@media (prefers-reduced-motion: reduce) {
  .cursor-blink {
    animation: none;
    opacity: 1;  /* stay visible, don't blink */
  }
}
```

For the VHS recording:

```tsx
<video autoPlay={!prefersReducedMotion} loop muted playsInline>
```

Detecting `prefers-reduced-motion` in a Server Component is not possible, so either:
- Use CSS media query + poster image, and let the video show a static frame when motion is reduced
- Or use a small Client Component that checks `window.matchMedia('(prefers-reduced-motion: reduce)')`

Recommended: CSS-only approach — set a poster frame and let the browser handle the media query.

## prefers-color-scheme Handling

metaphaze's brand IS dark mode. But some users set `prefers-color-scheme: light` and the OS forces an inversion. metaphaze should:

1. **Not auto-invert** — the brand is dark. Light mode would break the `#5fb878` signal's contrast and the `#0a0a0a` ground.
2. **Use a persistent dark palette** — declare `color-scheme: dark` in `<html>` to tell the browser not to auto-style form controls for light mode.
3. **Optionally offer a "system" fallback** — but metaphaze's brand is opinionated enough that a single dark theme is acceptable.

```html
<html lang="en" class="dark">
```

```css
:root {
  color-scheme: dark;
  background: #0a0a0a;
  color: #ededed;
}
```

This prevents the browser from auto-styling scrollbars, form controls, and focus rings with light-mode defaults.

## Semantic HTML for Code

WCAG and HTML spec disagree sometimes on semantic code markup. The correct combinations:

- **Inline code**: `<code>cargo install metaphaze</code>` — single command inline in prose
- **Code block**: `<pre><code>cargo install metaphaze</code></pre>` — the `<pre>` preserves whitespace, the `<code>` marks it as code
- **Keyboard input**: `<kbd>ctrl</kbd> + <kbd>c</kbd>` — for key bindings
- **Sample output**: `<samp>[DONE] installed metaphaze v0.1.0</samp>` — for expected output from a program

metaphaze's install block needs:
```html
<pre><code class="language-shell">cargo install metaphaze</code></pre>
<samp>installed metaphaze v0.1.0 in ~/.cargo/bin</samp>
```

This is semantically correct and allows screen readers to announce "sample output" for the expected post-install response.

## shadcn/ui Button + Badge Accessibility

When heavily restyling shadcn components, the built-in a11y doesn't break as long as you preserve:

1. The underlying HTML element (`<button>` for Button, `<span>` or `<div>` for Badge)
2. Focus-visible states (shadcn Button has `focus-visible:ring-*` by default)
3. `aria-*` attributes the component consumes
4. Keyboard activation (buttons should activate on Space and Enter — shadcn handles this)

For metaphaze's bracketed button style, use CSS `::before` and `::after` to add `[` and `]` — these are purely decorative and screen readers ignore them by default. Do NOT put real `[` and `]` characters in the DOM text — they'd be announced as "open bracket, open bracket."

## Language Attribute

`<html lang="en">` is required by WCAG 3.1.1. Do not forget this. Next.js 14 will NOT set it for you — you must explicitly set it in `app/layout.tsx`:

```tsx
export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
```

## WCAG 2.1 AA Checklist (Minimum Viable)

The specific success criteria that apply to the metaphaze landing page:

- **1.1.1 Non-text Content** — VHS recording has `aria-label`, logo cursor is `aria-hidden`, ASCII diagrams have `aria-label` + `aria-hidden` inner
- **1.3.1 Info and Relationships** — use semantic HTML (`<header>`, `<main>`, `<section>`, `<footer>`, headings in order)
- **1.4.3 Contrast (Minimum)** — all text ≥4.5:1, all large text ≥3:1 (validated above)
- **1.4.4 Resize Text** — all sizes in `rem`, no fixed `px` for text
- **1.4.10 Reflow** — page must work at 320px width without horizontal scroll (mobile-first CSS)
- **1.4.12 Text Spacing** — no `line-height < 1.5`, no `letter-spacing < 0.12em` restrictions (monospace is fine)
- **2.1.1 Keyboard** — every interactive element reachable by Tab, activatable by Enter/Space
- **2.4.1 Bypass Blocks** — skip link required
- **2.4.2 Page Titled** — `<title>` in metadata
- **2.4.6 Headings and Labels** — descriptive headings
- **2.4.7 Focus Visible** — custom focus ring with signal color
- **3.1.1 Language of Page** — `<html lang="en">`
- **4.1.2 Name, Role, Value** — all buttons have accessible names

Omit: 1.2.x (Time-based media) — no audio/video requiring captions; the VHS recording has no audio track and is decorative. Omit: 1.4.6 (Enhanced contrast) — that's AAA, not required.

## Testing Plan

Before ship:
- Run Lighthouse accessibility audit (target: 100)
- Run axe DevTools (target: 0 violations)
- Test with VoiceOver (macOS) — tab through, verify announcements
- Test with NVDA (Windows VM) — tab through, verify announcements
- Test at 200% zoom — verify no horizontal scroll
- Test at 320px width — verify reflow
- Test with `prefers-reduced-motion` forced on — verify cursor stops blinking
