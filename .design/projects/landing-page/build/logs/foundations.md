# Foundations Log
> Phase: build/foundations | Project: landing-page | Generated: 2026-04-08

## Files Created
| File | Purpose |
|------|---------|
| `public/vhs/.gitkeep` | Placeholder so VHS directory exists in repo; actual `.webm` recording added separately |

## Files Modified
| File | Changes |
|------|---------|
| `app/globals.css` | Full replacement: removed all shadcn oklch defaults, integrated metaphaze brand tokens, global reset, Tailwind v4 @theme block, all three effects vocabulary classes, component classes (badge, pane, code-block, rule, prose, section-heading), skip-link sr-only utility |
| `app/layout.tsx` | Full replacement: JetBrains Mono font loading via next/font/google, metaphaze metadata, skip link, fixed nav shell (logo + [/install] + [/source]), footer shell (license + refusals + manifesto + build line) |
| `app/page.tsx` | Replaced boilerplate with minimal brand placeholder; comment notes screen phase pending |

## Tokens Integrated
All `--mz-*` brand tokens defined in `:root` with dark-mode defaults (#0a0a0a bg, #ededed fg, #5fb878 signal, etc.). Light mode override via `@media (prefers-color-scheme: light)`. Shadcn variable bridge maps `--background`, `--foreground`, `--primary`, `--border`, `--destructive`, `--ring`, `--card`, `--popover`, `--secondary`, `--muted` to brand equivalents. `--radius: 0` kills all shadcn border-radius. Full type scale (--text-xxs through --text-5xl) and spacing scale (--space-1 through --space-24) defined. Tailwind v4 `@theme` block registers `--color-mz-*` utilities.

## Effects Implemented
- **cursor-blink**: `@keyframes mz-cursor-blink` with `step-end` easing at 1.06s (530ms on/off). Applied to `.mz-cursor` with `--mz-signal` color. Reduced-motion: animation disabled, opacity forced 1.
- **video-invert**: `.mz-btn` hover fills `--mz-fg` background and inverts text to `--mz-bg`. No transition (instant per brand spec).
- **underline-reveal + > prefix**: `.mz-nav-link` uses `::before` for `>` prefix (slides in on hover via opacity 0→1) and `::after` for underline (width 0→100%). Both instant (no transition per brand spec).

## Layout
Nav shell fixed at top with `z-40`, max-w-3xl centered, `border-b border-[--mz-border]`, dark background. Logo: `mz▌` with `.mz-cursor` blink on the block character. Nav links: `[/install]` (anchor to #install section) and `[/source]` (external GitHub link), both using `.mz-nav-link` with `pl-5` to make room for `>` prefix. Footer: `border-t`, `mt-24`, contains license/source/gsp links, refusals list, manifesto line, and build version stamp at `--text-xxs` opacity-70.

## Notes
- Build compiles cleanly with Next.js 16.2.3 / Turbopack (2.8s). TypeScript passes. Static generation completes.
- `@import "shadcn/tailwind.css"` was present in old globals.css and removed — it was pulling in the oklch variable definitions that conflicted with brand tokens.
- JetBrains Mono loaded as CSS variable `--font-jetbrains-mono`; `--font-mono` in `:root` still lists the full stack (Berkeley Mono first) so sites with Berkeley Mono installed get that preference.
- `public/vhs/` placeholder ready for VHS `.webm` recording to be dropped in during screen phase.
- No components/ui/ files were modified.
