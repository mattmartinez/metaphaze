# Messaging
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

- **Core message:** the orchestrator runs outside the loop.
- **Three supporting messages:** deterministic state machine (won't degrade), disk-first state (won't lose work), first-party binary (won't get banned).
- **Elevator pitch:** 30 seconds, mechanism-first, no "agentic."
- **Tagline directions:** three options, all testable next to GSD 2.0's "one command, walk away."
- **Audience:** one persona — The Senior Operator — because the BRIEF.md only defines one, and narrower is stronger.

---

## Core Message

**the orchestrator runs outside the loop.**

This is the one sentence metaphaze says in every external expression. It is the headline of the README. It is the first line of the landing page. It is the subject of the manifesto. It is the answer to "what is this thing." Every other message supports it.

### Why this as the core

- **It is technically precise.** "Outside the loop" is a specific architectural claim: the orchestrator runs outside the LLM's context window, not inside it. A senior engineer reads that and knows exactly what is being claimed.
- **It is a category position.** It names where metaphaze sits relative to every other agent harness on the market. Every competitor runs the orchestrator *inside* the loop (or pretends the distinction does not matter). metaphaze is the one that does not.
- **It is a refusal as much as a claim.** "Outside the loop" implies the critique of "inside the loop" without having to name names. The Senior Operator understands the critique because they have lived it.
- **It survives being shouted.** Lowercase, short, rhythmic. Fits in a terminal pane, a README hero, a tweet, a tab title. Works at any size.

---

## Three Supporting Messages

Each supporting message has a claim, a mechanism, and a proof point. The claim is the headline, the mechanism is the reason it is true, the proof point is the falsifiable artifact a skeptic can verify.

### Supporting Message 1 — Won't degrade over long runs

**Claim:** mz runs for hours without losing the plot.

**Mechanism:** the orchestrator is a deterministic rust state machine. the plan lives in `.mz/plan.toml`, not in claude's context window. each step gets a clean context with exactly the inputs it needs pre-inlined. context rot cannot accumulate because no single claude invocation holds more than one step's worth of state.

**Proof point:**
- 12 phases shipped. 97 steps complete. the product itself was built by mz running on mz.
- disk-first architecture means the context assembled for step 17 is a file you can `cat .mz/phase-3/track-2/step-17/context.md` and read. the inputs are inspectable.
- resume after a crash does not require re-assembling history from claude — the plan is already on disk.

**Talk track:** when the senior operator says "every agent harness i've tried falls apart after 90 minutes," the answer is: *the ones that fall apart are the ones where the llm is the orchestrator. in those, context accumulates until the model forgets what it was doing. mz's orchestrator is rust. the rust state machine does not forget.*

### Supporting Message 2 — Won't lose your work

**Claim:** kill the process, restart, the plan resumes.

**Mechanism:** every unit of state lives on disk in `.mz/`. phases, tracks, steps, contexts, outputs — all files. the filesystem is the api. `rm -rf .mz/` is a full reset and always works. there is no in-memory state that matters.

**Proof point:**
- `.mz/` is inspectable with `cat`, `less`, `jq`, `grep`. no custom tools.
- a user who closes their laptop mid-run and reopens it tomorrow runs `mz resume` and the plan picks up at the last completed step.
- a user who wants to understand what mz was thinking reads `.mz/phase-3/track-2/step-17/context.md` and sees exactly what got handed to claude.

**Talk track:** when the senior operator says "i lost a three-hour run last week because of an oom kill," the answer is: *everything is on disk. the oom kill would have interrupted the claude call, but the orchestrator state was already on disk at the last checkpoint, and `mz resume` would have restarted from there.*

### Supporting Message 3 — Won't get banned

**Claim:** mz calls the first-party `claude` binary directly.

**Mechanism:** no reverse-engineered sdk. no scraped endpoints. no auth hacks. mz shells out to the `claude` binary exactly as anthropic ships it. if `claude` works on your machine, mz works on your machine. there is no third-party surface to crack down on.

**Proof point:**
- the source has no http client pointed at a claude api endpoint. the dependency is a child process named `claude`. anyone can verify this by reading `Cargo.toml` and the source.
- provider policy changes do not affect mz's subscription access because mz is not the subscriber — the user's own `claude` login is.
- every competitor built on the reverse-engineered claude agent sdk has to choose between rebuilding on the first-party surface or accepting permanent pay-as-you-go pricing. mz does not have that choice because it was never on the third-party surface.

**Talk track:** when the senior operator says "i'm done with harness tools that break every time anthropic changes their mind," the answer is: *mz cannot break that way. the thing it calls is the same thing you call when you type `claude` into your terminal. there is nothing between mz and the first-party binary.*

---

## Elevator Pitch (30 seconds)

**Spoken / monospace version:**

> mz is a rust cli that drives claude code from outside the llm loop. you write a spec, run `mz init`, then `mz run`, and walk away. the orchestrator is a deterministic state machine that assembles a clean context for each step and hands it to the first-party `claude` binary. state lives on disk in `.mz/`, so crashes don't matter and you can read what the tool was thinking with `cat`. single binary, mit license, no accounts, no telemetry. it was built because every other agent harness either degrades over long runs or runs on a third-party surface that anthropic can turn off. mz does neither.

### Pitch structure (for reuse across contexts)

1. **What it is** — rust cli that drives claude code from outside the loop (1 sentence)
2. **How you use it** — spec, init, run, walk away (1 sentence)
3. **How it works** — deterministic state machine, clean context per step, first-party `claude` (1 sentence)
4. **What you get** — disk state, inspectable, single binary, mit (1 sentence)
5. **Why it exists** — the negative space against harnesses that degrade or get banned (1 sentence)

Total: 30-40 seconds spoken, 90-100 words written.

### Shorter versions

**15 seconds:**
> mz drives claude code from outside the llm loop. rust, single binary, disk-first state. write a spec, run `mz run`, walk away for 90 minutes. calls the `claude` binary directly, so there's no third-party surface to break.

**5 seconds:**
> rust cli that drives claude code from outside the loop. one command, walk away.

---

## Tagline Directions

Three candidates. Each is testable next to GSD 2.0's "one command, walk away." Each feels different from the others. The final pick happens in the identity phase, but all three are shippable.

### Direction A — the manifesto compressed

**tagline: the orchestrator runs outside the loop.**

**rationale:** the manifesto line IS the tagline. no separate marketing layer. it is precise, it is a category position, it is the core message. using it as the tagline means the brand never has a gap between what it says at the top of the page and what it says in the slogan — they are the same thing.

**test vs "one command, walk away":** GSD 2.0's tagline describes the *user experience*. metaphaze's tagline describes the *architecture*. they are fundamentally different claims. a senior engineer comparing them sees "one command, walk away" and thinks "everyone says that." they see "the orchestrator runs outside the loop" and thinks "wait, what does that mean" — and then they read. that is the right reaction.

**risk:** possibly too technical for a first-read if the reader is not already primed on agent architecture. this is not actually a risk because metaphaze does not want readers who are not primed on agent architecture.

### Direction B — the role assignment

**tagline: claude builds. mz drives.**

**rationale:** the second half of the manifesto. shorter, more rhythmic, more immediately legible than direction a. works as a standalone line on a t-shirt, a tab title, or a terminal banner. pairs the two subjects (claude and mz) in their proper relationship — claude is the creative work, mz is the structure — without needing a full explanation.

**test vs "one command, walk away":** GSD 2.0's tagline is about the user ("one command, walk away"). metaphaze's tagline is about the tool's relationship to claude ("claude builds. mz drives."). positioning itself against the model, not against the user's experience, is a sharper claim. it is also untranslatable to a non-claude-native tool, which makes it defensible — a competitor cannot say "gpt builds, we drive" without admitting they are in the wrong lane.

**risk:** a reader who does not know mz might parse "mz" as a typo. this is actually fine because the tagline is always immediately followed by the full name somewhere on the same surface.

### Direction C — the refusal as headline

**tagline: no accounts. no telemetry. no node.**

**rationale:** positioning by refusal. uses the refusals list as the brand's primary signal. the three items are ordered by how viscerally they will resonate with the senior operator: accounts (tool fatigue), telemetry (privacy), node (runtime fatigue). three short declarative negatives set in monospace read as a statement of taste, not a marketing line.

**test vs "one command, walk away":** GSD 2.0's tagline describes what the user gets. direction c describes what the user does not get — and in a market where every tool promises everything, refusing specific things is the more credible signal. a senior engineer reading both will trust the one with the shorter list of promises.

**risk:** three negatives in a row can read as cranky or contrarian to a casual visitor. for the senior operator this is not a risk (they are cranky and contrarian). for everyone else, it is a filter, and the filter is doing its job.

### Recommendation

Lead with **Direction A** (`the orchestrator runs outside the loop.`) as the primary tagline because it is the core message, the manifesto, and the positioning, compressed into one sentence. Use **Direction B** (`claude builds. mz drives.`) as the subhead directly beneath it. Use **Direction C** (`no accounts. no telemetry. no node.`) as the opener of the refusals section further down the page. Three directions, three locations, one consistent voice.

---

## Persona Mapping — The Senior Operator

BRIEF.md defines exactly one persona. Map the messaging to that persona precisely and do not invent secondary audiences.

### Who they are (summary from BRIEF.md)

- Senior / staff engineer or indie hacker, 28-45, lives in tmux.
- Runs Claude Code, Codex, and one other agent in parallel panes.
- Tries new tools constantly and most disappoint within a week.
- Wants to walk away for 90 minutes and come back to working code.
- Reads the README before installing. Distrusts gradients, signup walls, "agentic AI."
- Got burned by the April 4 crackdown. Actively shopping for a durable replacement.

### Primary motivation

**Relief from tool fatigue.** Not excitement. Not discovery. Not "innovation." The Senior Operator has tried enough fragile, hyped, banned-next-week tools that their emotional job-to-be-done is "find the one that will still be here in six months and let me stop thinking about this category." Every message should serve that motivation.

### Key message for this persona

**mz is the orchestrator you install once and stop thinking about.**

This is the persona-specific version of the core message. It is not the one on the landing page — it is the one the brand would say to the Senior Operator directly in a private conversation. It names the emotional promise (stop thinking about it) and the functional promise (install once) in one line.

### Supporting points for this persona

1. **"will this still work in 6 months?"** — yes, because mz shells out to the first-party `claude` binary and does not depend on a reverse-engineered sdk.
2. **"can i run this without an account?"** — yes. no accounts, no telemetry, no signup wall, no license server. `cargo install mz` and you own it.
3. **"is it one binary?"** — yes. one rust binary. no node, no python, no container, no runtime.
4. **"does it survive if the llm gets dumber?"** — yes. the orchestrator is deterministic rust. the plan does not live in the llm's head.
5. **"will the state survive a crash?"** — yes. `.mz/` is on disk and inspectable. `mz resume` picks up from the last completed step.
6. **"does the brand look like it was made by someone like me?"** — yes. monospace everywhere, no gradients, no stock photos, no "book a demo" button. the readme is the marketing.

Each of these six questions is a question from the Senior Operator's actual head (per `discover/market-landscape.md`). Each has a one-sentence, mechanism-backed answer. The brand should be able to answer any of them without hedging.

### Tone for this persona

From the tone spectrum: formal 3 / serious 2 / authoritative-leaning / technical. Short sentences. Direct. The Senior Operator respects precision and resents performance. The brand does not need to be warm to this reader — it needs to be trustworthy. Trustworthy is the warmer half of the spectrum relative to "marketing-distant."

### Proof points to surface

| Persona concern | Proof point |
|---|---|
| will it still work in 6 months | source has no http client pointed at a claude api endpoint — only a child-process call to `claude` |
| no accounts / no signup | `cargo install mz && mz init && mz run` — three commands, no signup screen |
| one binary | `cargo install mz` compiles to a single executable; no runtime dependency |
| won't degrade | 12 phases, 97 steps — mz was built by mz running on mz |
| won't lose work | `.mz/` directory is on disk, human-readable, `mz resume` works after a crash |
| made by someone like me | readme is the marketing, monospace everywhere, refusals list is public, mit license |

### Discovery channels

Per BRIEF.md: Hacker News, X (developer side), GitHub trending, recommendations from devs whose taste the Senior Operator trusts. The brand's primary distribution surface is the README. Secondary is the GitHub repo itself (topics, description, release notes). Tertiary is a landing page that renders the README content with minor layout polish. Nothing else until the brand has traction.

### Trust signals the brand must hit

- single binary install
- mit license
- readme without hero illustrations
- code that compiles
- tests that pass
- commit history that reads like someone cares

### Anti-trust signals the brand must avoid

- gradients
- "transform your workflow"
- signup walls
- "agentic ai" anywhere
- enterprise language
- stock photos
- glowing orbs
- "book a demo" buttons
- roi calculators

---

## Message Priority (for page layouts)

When assembling any marketing surface — README, landing page, social — messages go in this priority order. Stop when you run out of space.

1. **the orchestrator runs outside the loop.** (core message / headline)
2. **claude builds. mz drives.** (manifesto second line / subhead)
3. **`cargo install mz`** (the install instruction, visible above the fold)
4. **what it is:** rust cli that drives claude code, deterministic orchestrator, disk-first state.
5. **the three supporting messages:** won't degrade, won't lose work, won't get banned.
6. **the refusals list:** no accounts, no telemetry, no dashboard, no saas, no enterprise edition, etc.
7. **how to use it:** init, run, resume, status. with real output.
8. **why it exists:** one paragraph at most. the manifesto is the "why"; this is just the spelling-out.
9. **license:** mit.

No other message belongs above "how to use it." No testimonials, no "trusted by" logos, no feature grids, no "compare plans" table. The brand has one plan: free, mit, solo-built.
