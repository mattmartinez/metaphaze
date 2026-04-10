# Voice & Tone
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

Three voice attributes, in priority order: **Precise · Spare · Lowercase**. Short sentences. No exclamation marks. Lowercase as a deliberate (not always) choice. Em-dashes as structure. Bracket navigation (`[/about]  [/docs]`). References: htmx, suckless, plan9 docs. Two writers following these rules should produce copy that sounds like the same person wrote it.

---

## Voice Principle

Voice is the personality of the writer — the thing that stays the same whether the content is a README headline, an error message, or a status line in the TUI. For metaphaze, the writer is a senior engineer explaining their own tool to another senior engineer in a tmux pane at 11pm. Precise enough to trust. Spare enough to respect the reader's time. Lowercase because capitalization is a choice, and the brand has thought about it.

The three attributes are ordered. **Precise** is the non-negotiable. **Spare** is how precision stays readable. **Lowercase** is the typographic signature that makes the first two legible as a voice, not just a writing style.

---

## Voice Attributes

### 1. Precise

**Means:** Every claim is falsifiable, every term is the specific one, every number has a mechanism behind it. The brand does not say "fast" when it could say "single binary, no runtime." It does not say "robust" when it could say "survives crashes because state lives in `.mz/` on disk."

**Doesn't mean:** Verbose, academic, jargon-stuffed, or hedged into uselessness. Precision is about picking the right word, not using more words. The brand is not trying to sound smart — it is trying to be correct.

**Examples:**

| Do | Don't |
|---|---|
| `mz` drives claude code from outside the llm loop. | metaphaze is an AI-powered agentic framework that leverages Claude. |
| state lives in `.mz/`. kill the process, restart, the plan resumes. | robust state management with enterprise-grade reliability. |
| single binary. mit. no accounts. | streamlined, frictionless, user-centric experience. |
| calls the `claude` binary directly. no sdk. | built on industry-standard APIs for maximum compatibility. |
| 12 phases. 97 steps. rust. | full-featured, production-ready AI orchestration platform. |

**The precision test:** If a senior engineer reading the sentence could ask "wait, what do you actually mean by that?" — the sentence fails. Rewrite until the mechanism is visible.

### 2. Spare

**Means:** Short sentences. Short paragraphs. No filler clauses. No throat-clearing. No "we believe that, at the end of the day, what really matters is..." If a word can come out without changing the meaning, it comes out. The sentence is done when the point is made.

**Doesn't mean:** Cryptic, terse to the point of rudeness, or so compressed that the reader has to decode it. Spare writing is still complete. It just refuses to pad. Short sentences carry more weight because they do not dilute themselves.

**Examples:**

| Do | Don't |
|---|---|
| state on disk. crashes don't matter. | Our sophisticated state management architecture ensures that in the unlikely event of a crash, your work is preserved. |
| cargo install mz. that's the install. | Getting started with metaphaze is simple — just run our cargo install command and you'll be up and running in no time. |
| 90 minutes, hands off, working code. | In as little as 90 minutes, depending on the complexity of your task, you can walk away from your keyboard and return to find that metaphaze has delivered working code. |
| it's rust. one binary. no node. | Built with performance and reliability in mind, metaphaze is a Rust-based tool that ships as a single binary without the overhead of Node.js. |

**The spare test:** Read the sentence aloud. Remove any word that is not carrying meaning. Read it again. Still sounds like the brand? Keep going. Stop when removing another word breaks the meaning.

### 3. Lowercase

**Means:** Sentence-initial letters are lowercase in casual surfaces — the README, the landing page, the TUI, the CLI help text, the error messages, the status outputs. Proper nouns stay capitalized when they refer to external things (`Rust`, `Claude`, `Anthropic`, `MIT`). The product name `mz` is **always** lowercase. The brand name `metaphaze` is **always** lowercase. This is not a stylistic tic — it is a typographic position. Lowercase says "this is not a press release. this is a tool."

**Doesn't mean:** Lowercase always, in every context, with no judgment. Headings may be lowercase. Code comments may be lowercase. Section labels may be lowercase. But legal text, citations, and external-facing formal documents (license files, security disclosures) follow standard capitalization because those contexts call for it. Lowercase is a deliberate choice, not a rule to enforce against common sense.

**Also doesn't mean:** e.e. cummings. It doesn't mean lowercasing proper nouns (`Rust` is still `Rust`, not `rust` — the language is a proper noun). It doesn't mean lowercasing acronyms (`MIT`, `CLI`, `TUI`, `LLM` stay uppercase). Lowercase applies to sentence starts, headings, labels, and the brand name. Not to everything.

**Examples:**

| Do | Don't |
|---|---|
| the orchestrator runs outside the loop. | The Orchestrator Runs Outside The Loop. |
| cargo install mz | Cargo Install Metaphaze |
| state lives in `.mz/`. | State lives in `.mz/`. |
| mz drives. claude builds. | MZ Drives. Claude Builds. |
| hands-off autonomous runs that survive long sessions. | Hands-Off Autonomous Runs That Survive Long Sessions. |
| built in rust. calls the claude binary. mit-licensed. | Built in Rust. Calls the Claude binary. MIT-licensed. |

Wait — some of those "do" examples break the rule. `rust` should be `Rust`. `claude` is the name of a binary (lowercase, because that is how it is installed), but the product is `Claude` (proper noun). This is where the rule needs judgment.

**Lowercase conventions, for the avoidance of doubt:**

- `mz` — always lowercase. The binary name, the brand, the subject of sentences.
- `metaphaze` — always lowercase.
- `claude` — lowercase when referring to the binary (`the claude binary`, `shells out to claude`). Uppercase `Claude` when referring to the model, the product, or the company's work.
- `Rust` — always capitalized. It is a proper noun.
- `Anthropic` — always capitalized.
- `MIT`, `CLI`, `TUI`, `LLM`, `SDK`, `API` — acronyms, always uppercase.
- Section headings in marketing surfaces (README, landing) — lowercase preferred.
- Code comments — lowercase preferred.
- Error messages in the TUI — lowercase preferred.
- Legal text, license headers, security advisories — standard capitalization.

**The lowercase test:** Would writing it in sentence case make the brand sound like a press release or a corporate announcement? If yes, lowercase it. Would writing it lowercase make it look sloppy or confusing (legal text, error codes)? If yes, use standard case.

---

## Tone Spectrum

Voice does not change. Tone does. metaphaze's default tone is plotted here, along with the shifts for each context.

### Default position

```
Formal        1 ─── 2 ─── 3 ─── 4 ─── 5    Casual
                          ●
Serious       1 ─── 2 ─── 3 ─── 4 ─── 5    Playful
                    ●
Authoritative 1 ─── 2 ─── 3 ─── 4 ─── 5    Friendly
                    ●
Technical     1 ─── 2 ─── 3 ─── 4 ─── 5    Simple
              ●
Reserved      1 ─── 2 ─── 3 ─── 4 ─── 5    Enthusiastic
                    ●
```

The default is: moderately casual but not chatty, mostly serious but willing to be dry, authoritative-leaning, technically explicit, reserved. The brand is not cold — it is focused. Short sentences do not mean unfriendly sentences. They mean sentences that do not waste the reader's time.

### Context shifts

| Context | Formal↔Casual | Serious↔Playful | Auth↔Friendly | Notes |
|---|:---:|:---:|:---:|---|
| README hero | 3 | 2 | 2 | declarative. short. the manifesto lives here. |
| README install section | 3 | 2 | 3 | instructions. precise. friendly enough to reduce friction. |
| landing page | 3 | 2 | 2 | matches the README exactly. no extra polish. |
| TUI status output | 4 | 2 | 3 | terse. informative. green dot and a verb. |
| TUI error message | 3 | 2 | 4 | helpful. blame-free. states what happened and what to do. never punishes. |
| CLI --help text | 3 | 2 | 3 | precise. complete. no jokes. |
| github issue response | 3 | 2 | 4 | direct. answers the question. warmer when explaining a refusal. |
| refusals list | 3 | 2 | 2 | declarative negatives. no apologies. |
| release notes | 3 | 2 | 3 | what changed, why, and any breaking-change callouts. |
| legal / license | 1 | 1 | 2 | mit license text verbatim. no voice overlay. |

### Tone examples by context

**README hero (formal 3 / serious 2 / auth 2):**
```
the orchestrator runs outside the loop.
claude builds. mz drives.

single binary. rust. mit. no accounts.
```

**TUI status output (formal 4 / serious 2 / friendly 3):**
```
[mz] phase 3/12 · track 2/4 · step 17/97 · claude running · 04:12
```

**TUI error message (formal 3 / serious 2 / friendly 4):**
```
[mz] step 17 failed: claude exited with status 1.

stderr captured to .mz/phase-3/track-2/step-17/stderr.log
retry with: mz resume
```

**CLI --help text (formal 3 / serious 2 / auth 3):**
```
mz run — execute a plan from .mz/plan.toml

usage: mz run [--from PHASE] [--only TRACK] [--dry-run]

  --from PHASE   start at phase PHASE instead of the beginning
  --only TRACK   run only TRACK and its dependencies
  --dry-run      print the plan without calling claude

exit codes:
  0  run completed
  1  run halted by user
  2  step failed, state saved, resume with `mz resume`
```

**GitHub issue response — declining a feature (friendly 4):**
```
thanks for the write-up. adding a dashboard isn't on the roadmap and
won't be — the refusals list on the readme covers why. the short
version: `.mz/` on disk is already the dashboard, and `less .mz/state.toml`
is the ui. if there's a specific thing `less` doesn't show you well,
i'm interested in that as a separate issue.
```

---

## Do / Don't Chart

Paired writing samples for the three attributes combined. Each "do" sounds like metaphaze. Each "don't" sounds like something else.

| Do | Don't | What's wrong |
|---|---|---|
| mz drives claude code from outside the llm loop. | Metaphaze is an agentic AI framework powered by Claude. | "agentic" is banned. "framework" is category fuzz. capitalization and length both wrong. |
| state lives in `.mz/`. | We leverage a robust state management solution. | "leverage" is a banned word. no mechanism. |
| cargo install mz | Get started today with our simple installer! | exclamation mark. marketing voice. no actual command. |
| no accounts. no telemetry. no dashboard. | We respect your privacy with our no-tracking philosophy. | values-as-aspiration instead of values-as-refusal. describes, does not declare. |
| 97 steps shipped. | Built with industry-leading reliability. | vague claim vs specific number. |
| the orchestrator runs outside the loop. | Our intelligent orchestration runs at the edge of the LLM. | "intelligent" is filler. "at the edge" is marketing poetry. |
| rust. one binary. no node. | Built with Rust for maximum performance and efficiency. | filler adjectives. no specificity. |
| walk away for 90 minutes. come back to working code. | Transform your development workflow with autonomous AI. | "transform" is banned. "workflow" is filler. no mechanism. |
| kill `mz`. restart. the plan resumes. | Enjoy peace of mind with our crash-resilient architecture. | abstract claim vs concrete demonstration. |
| the readme is the marketing. | Check out our comprehensive documentation and tutorials. | marketing voice. patronizing. |

---

## Style Rules

Concrete rules the brand follows every time. These are the mechanical details that make the voice reproducible.

### Punctuation

- **Exclamation marks:** never. Not in TUI success states, not in release notes, not anywhere. If something is exciting, the brand says so declaratively.
- **Em-dashes:** use liberally as structural markers. Padded with spaces: ` — `. Never double hyphens `--` in prose (they are for CLI flags only).
- **Oxford comma:** yes, always.
- **Contractions:** yes. `don't`, `won't`, `it's`. Contractions make short sentences feel like speech, not telegraphese.
- **Smart quotes:** no. Straight quotes only. `"` not `"`. The brand must survive being pasted into a terminal.
- **Ellipses:** only for literal truncation (code output). Never for dramatic pause.
- **Semicolons:** rare. If you reach for a semicolon, consider two sentences instead.

### Capitalization

- Sentence-initial lowercase in casual surfaces (README, landing, TUI, CLI help, errors, status).
- Standard case in formal surfaces (license, legal, security advisories).
- Proper nouns always capitalized: `Rust`, `Anthropic`, `MIT`, `Claude` (the product), `Ratatui`, `Linear`.
- The binary name `claude` is lowercase when referring to the executable.
- The product name is always `mz` (lowercase) or `metaphaze` (lowercase).

### Formatting

- **No italics.** Monospace italics are typographically weak. Use backticks for code, `**bold**` for emphasis.
- **Inline code with backticks:** `.mz/`, `mz run`, `cargo install mz`. Always.
- **Bracketed navigation:** `[/about]  [/docs]  [/source]`. The brackets are part of the design, not ornament.
- **Tables as layout:** use markdown tables or `column -t`-style plain-text tables. Not `<div>`-grid cards.
- **Lists with `-` not `*` or `•`:** hyphens read more terminal-native.
- **Headings in lowercase** on marketing surfaces. Sentence case for headings in formal documents.

### Banned words

Never use these words or phrases in any metaphaze-authored copy:

- agentic
- empower
- transform
- unlock
- leverage
- journey
- ecosystem
- cutting-edge
- next-generation / next-gen
- powered by
- intelligent (as adjective for software)
- seamless
- frictionless
- robust (without a mechanism)
- enterprise-grade
- best-in-class
- industry-leading
- revolutionary
- game-changing
- solution (as substitute for "tool")
- platform (unless actually a platform, which metaphaze is not)

### Preferred words

Use these. They sound like the brand.

- orchestrator
- driver
- runner
- state machine
- binary
- loop (as in "outside the loop")
- pipeline
- disk
- precise
- deterministic
- first-party
- shells out to
- calls directly
- survives (crashes, restarts, long runs)
- resumes
- refuses
- ships

---

## Nomenclature

Consistent naming for the things the brand and the tool talk about.

### Product naming

- The tool is **`mz`** when referring to the binary, the command, or the subject of a sentence.
- The tool is **`metaphaze`** when referring to the project, the brand, or the full name on the landing page.
- Never `MZ`, `Mz`, `Metaphaze`, `MetaPhaze`, `metaPhaze`. Always fully lowercase.

### CLI command naming

All commands are lowercase verbs, no hyphens unless multi-word, short where possible. Existing commands:

- `mz init` — create a plan from a spec
- `mz run` — execute the plan
- `mz resume` — pick up from the last saved state
- `mz status` — show where the run is
- `mz plan` — print the plan without running
- `mz clean` — reset `.mz/` state

Never `mz startExecution`, `mz getStatus`, `mz initializePlan`. Unix-style short verbs.

### File and directory naming

- **`.mz/`** — the state directory. Hidden, like `.git/`. Lowercase. The trailing slash is part of the name in copy.
- **`.mz/plan.toml`** — the authoritative plan.
- **`.mz/phase-{n}/`** — phase subdirectories, hyphenated, numeric.
- **`.mz/phase-{n}/track-{n}/`** — tracks within phases.
- **`.mz/phase-{n}/track-{n}/step-{n}/`** — steps within tracks.
- TOML for structured config. Plain text for logs. No JSON unless a tool demands it.

### Branch naming in the project repo

- `main` — the default branch.
- `phase/{n}-{slug}` — feature branches follow the product's phase decomposition.
- No `feature/`, `bugfix/`, `hotfix/` Git-flow prefixes. The phase is the unit of work.

### Reference terminology (for documentation)

- **phase** — a top-level block of the plan. 12 of them shipped.
- **track** — a parallel lane within a phase.
- **step** — an atomic unit of work. Gets its own clean context window and its own `.mz/phase/track/step/` directory.
- **run** — a single invocation of `mz run`.
- **resume** — picking up after a crash, halt, or interrupt.
- **the loop** — shorthand for the LLM's context window. The orchestrator runs *outside* the loop.

---

## Reference Voices (for calibration)

When in doubt, read these sites and match their cadence. Do not copy their content; absorb their rhythm.

- **htmx.org** — declarative, slightly dry, willing to make a joke if the joke is correct. Bracketed navigation. Monospace throughout. Confident without being loud.
- **suckless.org** — philosophy as voice. Refuses to perform. Text-first, structure-first, position-first. The voice closest to metaphaze's `refusals` section.
- **plan9 from user space / plan9 docs** — terse, precise, man-page-inflected. Every sentence does a job. No filler. Lowercase section headings.
- **berkeleygraphics.com** — commercial brutalism done with typographic craft. Same cadence metaphaze wants, slightly more decorative because they sell a typeface.

**Not** these references:
- charmbracelet (too warm, too mascot-driven for metaphaze)
- linear (too polished, too product-led-growth)
- vercel (too neutral)
- github default (too bland)
