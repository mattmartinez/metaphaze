# Brand Applications
> Phase: identity | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

Seven touchpoints. Each one is a place where the brand actually lives, not a theoretical surface. The README is the primary touchpoint because it is the product's first impression and the brand's manifesto. The landing page is the second because some readers come from HN before they hit GitHub. The TUI itself is the third — and the most important — because the product is the brand and every invocation of `mz` is a brand expression. The other four are supporting surfaces that have to match.

Nothing here is a billboard. Nothing here is a launch event. The brand lives in text files and terminal panes, which is exactly what the Senior Operator wants.

---

## 1. The README (Primary Touchpoint)

The GitHub README is metaphaze's first, last, and most important brand surface. It is where 90% of the audience encounters the brand. Value #5 in the platform says it plainly: **the README is the marketing.**

### Visual direction

- **Pure markdown.** Renders correctly on GitHub's dark theme and `cat README.md` in a terminal. No HTML, no `<img>` tags except for the VHS hero gif and the logo image.
- **Hero block.** First screen shows three things in order:
  1. The logo (`mz▌` wordmark variant) as a PNG, 320px wide, centered.
  2. The manifesto line in an H1: `the orchestrator runs outside the loop. claude builds. mz drives.`
  3. A single line below: `single binary. rust. mit. no accounts.`
- **Install before explanation.** The `cargo install mz` command is the first code block, inside the first screen. The Senior Operator can install before they read anything else. This defeats the Sage-shadow "six paragraphs of theory first" failure mode.
- **VHS recording.** Below install, one autoplaying `.gif` generated with VHS showing a real `mz run` session. Embedded as `<img src="docs/vhs/hero.gif" alt="mz run session" />`.
- **State machine diagram.** An ASCII diagram of phase → track → step, rendered with box-drawing characters. Drawn once, used on every surface.
- **The refusals list.** The "things metaphaze will never have" list, as a plain-text bulleted list. This is as visible as the features — per positioning, refusal is a first-class brand tool.
- **Headings are lowercase.** Every `## heading` in lowercase. The brand's voice rule applies to the README.
- **Code blocks in `mz.slate`.** GitHub renders code blocks in its own theme, but the accompanying landing page uses the brand palette. The README-to-landing consistency is handled by the landing page, not by fighting GitHub.

### Specific brand decisions

- README headline is the manifesto, verbatim. Not a variation. The manifesto is locked.
- The install command is the hero, not the marketing copy.
- No "badges row" with 47 shields.io badges. One or two at most — MIT license, crates.io version. No "built with Rust" badge. No "PRs welcome" badge.
- No table of contents above the fold. The reader scrolls.

---

## 2. The Landing Page (github.io or custom domain)

If metaphaze has a landing page beyond the README, it is a single HTML file that renders the README content with brand-accurate styling. Not a marketing site. The landing page is the README, with two additions: real brand colors and real brand typography.

### Visual direction

- **One HTML file.** Static site, no JavaScript frameworks. Possibly one JavaScript file for the VHS embed and the blinking cursor — nothing else.
- **Monospace everywhere.** Berkeley Mono (if licensed) or JetBrains Mono (if not). Loaded as a web font. One face, three weights (400, 600, 700).
- **Warm off-black background** (`#0a0a0a`), warm off-white text (`#ededed`). Dark mode by default. Light mode available via `prefers-color-scheme` but dark is the hero.
- **Bracketed navigation at the top.** `[/about]  [/docs]  [/source]` — three nav items, no more. No dropdown menus, no mega-nav. The brackets are the design.
- **Hero is a VHS recording.** Autoplaying, muted, looped. Real `mz run` session. Above the fold.
- **Manifesto below the hero.** Display weight (700), 48px, lowercase, left-aligned. Signal color on the word `mz` only.
- **The rest of the page is the README.** Same content, same structure, rendered in the brand's type system. 1px `mz.slate` borders around code blocks, `mz.slate` background fills, `mz.dust` metadata.
- **No footer beyond a single line.** The footer is one line of text: `mit · source · [/source]`. That is all.

### Specific brand decisions

- The landing page matches the README. Anything on the landing that is not in the README is a violation of Value #5.
- No cookie banner. No analytics that would require a cookie banner. No telemetry at all (Value #4).
- No email capture. No newsletter signup. No "join the waitlist."
- No hero illustration. No background pattern. No ambient animation. The only motion on the page is the block cursor blinking in the logo and the autoplay VHS.

---

## 3. The TUI Itself (The Product IS a Touchpoint)

The TUI is the brand's most-used surface. Every time a Senior Operator runs `mz run`, they are looking at the brand. The TUI has to carry the identity without breaking the product.

### Visual direction

- **Brand palette as the TUI color set.** `#0a0a0a` background, `#ededed` foreground, `#8a8a8a` for metadata, `#2a2a2a` for pane borders, `#5fb878` for success states and status dots. `#d4a017` for warnings. Never dev-tool blue.
- **Box-drawing panes.** The TUI is rendered in Ratatui or a Rust equivalent using `─ │ ┌ ┐ └ ┘` box-drawing characters. 1px borders. No rounded corners. No drop shadows — terminals don't have drop shadows.
- **Status line in `mz.dust` with signal-colored numerators.** A line like `phase 3/12 · track 2/4 · step 17/97 · claude running · 04:12` where the numerators (`3`, `2`, `17`) are in `#5fb878` and the rest is `mz.dust`. The glance test: the Senior Operator looks at the TUI for half a second and knows where they are in the plan.
- **Status dots for state.** `●` in `mz.signal` for active/running, `○` in `mz.dust` for idle, `✓` in `mz.signal` for complete, `✗` in `mz.amber.deep` for failed.
- **The block cursor `▌`** wherever the TUI is waiting for user input. The same glyph as the logo. Same color. Same meaning.
- **Bracketed buttons** where the TUI has interactive elements: `[ INITIATE ]`, `[ RESUME ]`, `[ ABORT ]`. Uppercase inside the brackets only — this is the status-code exception to the lowercase rule.
- **Phase transition screen as the signature moment.** When `mz` finishes a phase and moves to the next, the TUI prints a full-screen ASCII banner: a bordered pane with the completed phase name, the step count, a compact file tree of produced artifacts, and a progress bar in `mz.signal`. This is the brand's most-photographed artifact.

### Specific brand decisions

- No emoji in TUI output, ever. The brand survives `LANG=C`.
- No CRT scanline effect. No phosphor glow. No typewriter animation. The TUI is a modern terminal application, not a nostalgia piece.
- The signal color appears in less than 1% of the TUI's pixels at any moment. If the whole screen turns green during a success state, the brand is broken.
- The TUI respects `NO_COLOR`. If `NO_COLOR` is set, the signal color and the amber drop out and the TUI renders in pure `mz.bone` on `mz.black`. The box-drawing still works.

---

## 4. The CLI `--help` Output (`mz --help` as a Branded Artifact)

The `--help` output is the brand's smallest and most-used surface. Every Senior Operator who runs `mz --help` sees it. It has to earn the brand.

### Visual direction

- **Lowercase throughout.** `mz run — execute a plan from .mz/plan.toml`. Not title-cased. Not sentence-cased mid-description. Lowercase.
- **Em-dash separating command and description.** `mz run — execute a plan from .mz/plan.toml`. The padded em-dash is the brand's signature structural marker.
- **Column-aligned flag descriptions.** Rendered as if by `column -t`. The monospace grid matters. Flags in `mz.bone`, descriptions in `mz.dust` if the terminal supports color.
- **Exit codes section.** Every `--help` includes an `exit codes:` section with numeric codes and lowercase descriptions. This is the Sage's move — explaining the mechanism, not just the command.
- **Short sentences.** The `--help` text is the voice guide applied literally. No throat-clearing. No "this command allows you to..." — just the verb.

### Specific brand decisions

- The first line of any `--help` output is always `mz {subcommand} — {one-line lowercase description}`. No exceptions.
- The usage line uses `[]` for optional args and `<>` for required args. Standard Unix convention, brand-aligned because brackets are already part of the design language.
- No ASCII art header in `--help`. The reader asked for help, not a logo.

---

## 5. Social Preview / OG Image (Simple, Monospace, Manifesto Quote)

The Open Graph image that renders when someone shares a metaphaze URL on X, Mastodon, Discord, or Slack. 1200x630 standard OG dimensions.

### Visual direction

- **Pure brand palette.** `#0a0a0a` background, full bleed. No gradient, no texture.
- **Manifesto line as the hero.** Lowercase, Berkeley Mono Bold, 48–56px, left-aligned, roughly centered vertically. Signal color on the word `mz` only.
- **Logo in the bottom-left corner.** `mz▌` wordmark, small (~32px), in `mz.bone` with the cursor in `mz.signal`.
- **One-line URL in the bottom-right.** `metaphaze.dev` or `github.com/user/metaphaze` in `mz.dust`.
- **Nothing else.** No decorative elements, no graphics, no gradients. The OG image should look like a screenshot of a terminal running a single-command `figlet`-less output.

### Specific brand decisions

- The OG image is generated programmatically, not designed in Figma. A small Rust or Node script reads the manifesto and outputs a PNG. The brand is consistent because it is generated, not because someone remembered the rules.
- No variant OG images for different pages. One image, one manifesto, one URL. Simplicity is the brand.

---

## 6. GitHub Social Preview (1280x640 PNG)

GitHub's repository social preview is a specific size and a specific moment — it renders in the repo's social card and in GitHub-aware embeds. Distinct from the general OG image above because it gets its own crop.

### Visual direction

- **Same palette, different composition.** `#0a0a0a` background. `#ededed` text. `#5fb878` accent glyph only.
- **Left half: the logo and the one-line positioning.** `metaphaze▌` wordmark at 96px, lowercase, left-aligned. Below it, one line: `single-binary rust orchestrator for claude code`. In `mz.dust`, 24px.
- **Right half: a cropped ASCII phase transition.** A static capture of a phase transition screen from the TUI, rendered inside a `mz.slate` border. This is the only "screenshot" on the card and it is the product's most photogenic moment.
- **Bottom strip: the manifesto line, compressed.** Across the bottom of the 640px height, one line in 20px Berkeley Mono Regular: `the orchestrator runs outside the loop. claude builds. mz drives.` Padded with em-dashes on either end for structure.

### Specific brand decisions

- The phase transition screenshot is real, not mocked. It is a capture of an actual `mz` run.
- The GitHub social preview replaces GitHub's default (a generic Octocat scene). It has to be distinctive at thumbnail size in a tweet embed.
- No GitHub logo. No "view on GitHub" button. The URL is implicit because this IS the GitHub preview.

---

## 7. The `cargo install mz` Moment

The install output itself is a brand surface. `cargo install mz` prints a lot of text. That text is a metaphaze experience, and the brand has opinions about it.

### Visual direction

- **No control over cargo's output.** The brand cannot change what `cargo` prints during the install. What it can control is the `[package]` metadata in `Cargo.toml`, the post-install message (if any), and the first line the user sees when they run `mz --version`.
- **`Cargo.toml` description field.** The description field in `Cargo.toml` is: `single-binary rust orchestrator for claude code`. Lowercase, no marketing words. This is what shows up on crates.io and in `cargo search`.
- **`mz --version` output.** Three lines:
  ```
  mz 0.1.0
  the orchestrator runs outside the loop.
  https://github.com/user/metaphaze
  ```
  The manifesto half-line appears in the version output. The Senior Operator's first post-install command is `mz --version`, and the brand greets them with a one-line statement.
- **First-run `mz init` output.** When the user runs `mz init` for the first time in a project, the TUI prints a brief banner — the logo, a one-line welcome, and the prompt to run `mz run`. The welcome line is: `ready. run \`mz run\` to start a phase.` Lowercase, short, useful.

### Specific brand decisions

- No "Thanks for installing!" post-install script. The Senior Operator did not ask for gratitude.
- No "Join our Discord!" message. There is no Discord.
- No telemetry prompt. There is no telemetry.
- The install is silent about the brand beyond the `Cargo.toml` description and the `--version` output. Restraint is the brand, and silence is the ultimate restraint.

---

## Applications Summary

The brand lives in seven places. Five of them are text files (README, landing, `--help`, `Cargo.toml`, `--version`). One is a terminal application (the TUI). One is a pair of PNGs (social previews). That is the whole brand footprint. No print collateral, no swag, no conference booth, no t-shirt, no event banner, no business card. metaphaze is not a company. It is a tool, and the brand is the tool's surface.

Anything beyond this list — if it appears — is a discretionary addition by whoever is maintaining the project. The brand does not require it and the Senior Operator does not expect it. Less is the position.
