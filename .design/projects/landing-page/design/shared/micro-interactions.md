# Micro-interactions

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Interaction Vocabulary

This page uses exactly three interaction techniques from `STYLE.md`'s effects vocabulary:
`cursor-blink`, `video-invert`, `underline-reveal`.

No spring physics. No slide transitions. No scroll-triggered reveals. No parallax. The cursor snaps; it does not fade.

---

## Interaction Table

| Element | Trigger | Technique | Duration | Easing | Description |
|---------|---------|-----------|----------|--------|-------------|
| `mz▌` logo (hero) | ambient (always on) | cursor-blink | 1060ms cycle | `step-end` | `▌` block cursor blinks 530ms on / 530ms off. Signal green `#5fb878`. |
| `mz▌` logo (nav) | ambient | cursor-blink | 1060ms cycle | `step-end` | Same blink as hero. Smaller size. |
| `[ copy ]` button | hover | video-invert | instant | `step-end` | Background fills `--mz-fg`, text becomes `--mz-bg`. |
| `[ copy ]` button | active (click) | cursor-append | instant | none | Block cursor `▌` appears after the label text while pressed. No translate, no scale. |
| Nav links `[/docs]` `[/source]` | hover | underline-reveal + `>` prefix | instant | `step-end` | `> ` prefix appears before the `[`, underline activates. No slide or fade. |
| Logo link (nav) | hover | underline-reveal | instant | `step-end` | Underline on `mz` text only (not the cursor glyph). |
| `[ copy ]` button | click (success) | text-swap | instant | none | Label changes from `copy` to `copied` for 1500ms then resets. No animation, discrete state change. Requires `'use client'`. |
| VHS `<video>` | `prefers-reduced-motion: reduce` | stop autoplay | n/a | n/a | When user prefers reduced motion, video `autoplay` is removed; user must click to play. |
| Cursor (`▌`) | `prefers-reduced-motion: reduce` | freeze | n/a | n/a | Animation stops; cursor frozen in visible (on) state. |

---

## CSS Implementations

### Cursor blink

```css
@keyframes mz-cursor-blink {
  0%, 49.99% { opacity: 1; }
  50%, 100%  { opacity: 0; }
}

.mz-cursor {
  display: inline-block;
  color: var(--mz-signal);
  animation: mz-cursor-blink 1.06s step-end infinite;
}

@media (prefers-reduced-motion: reduce) {
  .mz-cursor {
    animation: none;
    opacity: 1;
  }
}
```

### Video-invert (button hover)

```css
.mz-btn:hover {
  background: var(--mz-fg);
  color: var(--mz-bg);
  transition: none;
}
.mz-btn:hover::before,
.mz-btn:hover::after {
  color: var(--mz-bg);
}
```

### Underline-reveal with `>` prefix (nav links)

```css
.mz-nav a {
  text-decoration: none;
  color: var(--mz-fg);
}
.mz-nav a::before {
  content: "";
  color: var(--mz-fg);
}
.mz-nav a:hover {
  text-decoration: underline;
}
.mz-nav a:hover::before {
  content: "> ";
}
```

Transition: `none` — this is a snap, not a slide.

### Copy button state swap (React/JS)

```tsx
// 'use client' — the only client component on the page
const [copied, setCopied] = useState(false);

const handleCopy = () => {
  navigator.clipboard.writeText(INSTALL_CMD);
  setCopied(true);
  setTimeout(() => setCopied(false), 1500);
};

// Label renders as: [ copy ] → [ copied ]
```

---

## What's NOT Here

Per brand constraints — these techniques are banned:

- No typewriter reveal on hero text
- No section entrance animations (no fade-in on scroll)
- No parallax or scroll-jacking
- No hover lift/shadow on cards
- No toast notifications
- No loading skeletons
- No smooth CSS transitions (other than instant step-end)

---

## Related

- [navigation.md](./navigation.md)
- [responsive.md](./responsive.md)
- Brand STYLE.md — `## Effects` section
- Brand component: `.design/branding/metaphaze/patterns/components/cursor.md`
- Brand component: `.design/branding/metaphaze/patterns/components/bracketed-button.md`
