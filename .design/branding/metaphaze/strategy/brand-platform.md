# Brand Platform
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

- **Purpose (Why):** We believe the orchestrator should run outside the loop. The LLM builds, the code drives.
- **Vision:** A tool worth maintaining for a decade, still one binary, still running on whatever `claude` ships next.
- **Mission:** Give senior engineers a deterministic, single-binary orchestrator that survives long runs, crashes, and provider crackdowns.
- **Promise:** The architecture is the argument. If you understand how it works, you already trust it.
- **Values:** Five behavioral commitments, not aspirations.

---

## Purpose — The Why

**We believe the orchestrator should run outside the loop.**

The whole category got this backwards. Every agent framework built since 2023 assumes the LLM should drive itself — that if you give the model enough tools, enough context, and enough prompts, it will figure out the plan. It does not. It degrades. It loses context. It forgets constraints. It cannot be trusted with a 90-minute hands-off run.

The fix is not a better prompt. The fix is a deterministic orchestrator — written in Rust, living on disk, immune to context rot — that drives the LLM as a subroutine. Claude is extraordinary at the creative work. Claude is terrible at holding a 12-phase plan in its head. The right answer is to give Claude one step at a time, on a clean context, with exactly the inputs that step needs, and let the Rust state machine hold the shape.

metaphaze exists because someone had to write that state machine. We wrote it.

### The underestimated truth

**The methodology is worth more than the code.** Phase → track → step decomposition with disk-based state survives model upgrades, provider changes, and context-window improvements. Tools built on this foundation outlive their dependencies. The Rust code can be rewritten, but the shape of the thing — deterministic orchestrator outside the loop, pre-inlined context per step, state on disk — is the actual moat. If every line of metaphaze's source disappeared tomorrow, the methodology would still be right, and someone would rebuild it from the README.

---

## Vision

**A tool worth maintaining for a decade, still one binary, still running on whatever `claude` ships next.**

Ten years from now, Claude Code will not be the same product. The model will be different, the CLI flags will have changed, the LLM architecture may be unrecognizable. metaphaze's vision is to still be there, still a single `cargo install`, still driving whatever the first-party binary has become. The outside-the-loop orchestrator does not have to change when the inside-the-loop model does. That is the whole point.

### What the vision rules out

- No pivot to SaaS, ever.
- No rewrite in another language to chase a trend.
- No "metaphaze Enterprise" for teams of 50+.
- No multi-provider abstraction layer. If `claude` stops being the best binary, metaphaze follows the best first-party binary — it does not try to drive all of them.
- No accounts, no telemetry, no dashboard. Not in v1, not in v10.

---

## Mission

**Give senior engineers a deterministic, single-binary orchestrator that survives long runs, crashes, and provider crackdowns.**

The mission is narrow on purpose. It names the audience (senior engineers), the mechanism (deterministic, single-binary), the three specific failure modes it defeats (long runs, crashes, crackdowns), and the category (orchestrator). Every word is load-bearing.

### The mission, broken down

- **"Give"** — not sell, not license, not deploy. MIT. Free. Yours.
- **"Senior engineers"** — the Senior Operator. Not juniors, not PMs, not "technical founders." Narrower is stronger.
- **"Deterministic"** — the orchestrator runs outside the LLM's non-determinism. The same inputs produce the same plan.
- **"Single-binary"** — `cargo install mz`, full stop. No runtime, no container, no config wizard.
- **"Survives long runs"** — context rot does not exist here because each step gets a clean context window.
- **"Survives crashes"** — state lives in `.mz/` on disk. Kill the process, restart, the plan picks up.
- **"Survives provider crackdowns"** — shells out to the first-party `claude` binary. There is no third-party surface to ban.

---

## Manifesto

*Preserved exactly as locked.*

> **The orchestrator runs outside the loop.**
> **Claude builds. mz drives.**

This line is the brand. It is the headline of the landing page, the opening of the README, and the one thing that has to show up in every external expression of metaphaze without exception. Do not rewrite it. Do not soften it. Do not expand it.

### Why the line works

- **"The orchestrator runs outside the loop"** is a technical statement that doubles as a philosophical one. It names the architectural choice (outside the LLM context loop) and the category position (we are not the LLM, we drive it) in nine words.
- **"Claude builds. mz drives."** is the role assignment. Claude is the creative work, mz is the structure. The rhythm of the two three-word sentences is deliberate — it reads the same on a terminal, a README, or a poster.
- **Lowercase on `mz` is intentional.** The product name is always lowercase. The sentence does not capitalize `mz` at the start of the clause because metaphaze does not capitalize it, ever. Lowercase is voice, not style.

---

## Values — Behavioral Commitments, Not Aspirations

Five values. Each is a thing metaphaze actually does, not a thing metaphaze aspires to do. Each is testable against shipped code.

### 1. We ship binaries, not promises.

**What it means in practice:**
- The install instruction is `cargo install mz` and it works. If it does not work, the value is violated.
- No "coming soon" badges. Features exist or they do not.
- Release notes are changelogs, not marketing copy.
- The README does not describe the future. It describes what runs today.

**How to test it:** `cargo install mz && mz --version` should produce a real version number on a real binary on any platform metaphaze claims to support. If a platform is named in the README, it works.

### 2. State lives on disk where you can read it.

**What it means in practice:**
- Every unit of state — plans, steps, context, outputs — is a file in `.mz/`.
- Every file is human-readable without a custom tool. `cat`, `jq`, `less`, `grep` are enough.
- The filesystem is the API. `rm -rf .mz/` is a full reset and always works.
- Debugging means reading files, not attaching to a live process.

**How to test it:** A senior engineer can kill `mz` mid-run, inspect `.mz/` with standard Unix tools, understand what the orchestrator was doing, and either fix it or restart it.

### 3. The first-party binary is the only surface.

**What it means in practice:**
- metaphaze shells out to the `claude` binary exactly as Anthropic ships it. No reverse-engineered APIs. No scraped endpoints. No auth hacks.
- If Anthropic changes `claude`, metaphaze updates the shell-out. If they deprecate a flag, metaphaze adapts. No attempt to outsmart the vendor.
- Provider crackdowns do not affect metaphaze because metaphaze is not on the third-party surface. There is nothing to crack down on.

**How to test it:** Read `Cargo.toml` and the source. If there is any HTTP client pointed at a Claude API endpoint, the value is violated. The only thing metaphaze should call is a child process named `claude`.

### 4. We refuse things in public.

**What it means in practice:**
- The landing page has a "Things metaphaze will never have" list. No accounts, no telemetry, no dashboard, no SaaS tier, no enterprise edition, no ROI calculator, no signup wall.
- When a GitHub issue asks for one of these, the answer is "no, and here is why," not "maybe later."
- The refusals are as visible as the features. Positioning by refusal is a first-class brand tool, not an afterthought.

**How to test it:** The refusals list exists, is visible on every surface that counts (README, website if any, landing page), and matches reality. No silent additions.

### 5. The README is the marketing.

**What it means in practice:**
- There is no separate marketing site that says different things from the README. If a website exists, it renders the README content with minor layout polish, nothing more.
- The README leads with the mechanism, not the pitch. State diagram before tagline.
- Every sentence in the README has to survive being read by someone who has been burned by three agent tools this year.
- No hero illustrations, no "trusted by" logos, no case studies, no feature grids. Just the mechanism, the install, the usage, the refusals, the license.

**How to test it:** A senior engineer reads the README and runs `cargo install mz` in under two minutes without visiting any other page. If they had to go looking for something, the value is violated.

---

## Promise

### Core promise
**The architecture is the argument.**

When someone interacts with metaphaze — the README, the TUI, the `.mz/` directory, the `--help` output — they should feel like they are using a tool built by someone who shares their frustrations. The feeling is specific: "finally, someone gets it." That is the emotional target. Everything else is in service of it.

### Functional promise
**Hands-off autonomous coding runs that do not degrade over long sessions and cannot be banned by a provider.** This is a testable claim. A run that works at minute 5 should still work at minute 95. A run that worked yesterday should still work the day after Anthropic ships a new Claude release.

### Emotional promise
**Relief.** The Senior Operator has tried enough tools that promise hands-off and deliver fragile that the emotional job-to-be-done is not "excitement" or "delight" — it is relief. The relief of finding a tool that respects their time, their intelligence, and their autonomy. The relief of not having to babysit the orchestrator.

---

## Platform Summary Diagram

```
┌─────────────────────────────────────────────────────────────┐
│  WHY                                                         │
│  The orchestrator should run outside the loop.               │
│  Deterministic code drives the LLM, not the other way.       │
├─────────────────────────────────────────────────────────────┤
│  HOW                                                         │
│  Rust state machine · disk-first .mz/ · first-party claude   │
├─────────────────────────────────────────────────────────────┤
│  WHAT                                                        │
│  A single-binary CLI for hands-off multi-hour coding runs.   │
├─────────────────────────────────────────────────────────────┤
│  VALUES                                                      │
│  1. We ship binaries, not promises.                          │
│  2. State lives on disk where you can read it.               │
│  3. The first-party binary is the only surface.              │
│  4. We refuse things in public.                              │
│  5. The README is the marketing.                             │
├─────────────────────────────────────────────────────────────┤
│  MANIFESTO                                                   │
│  The orchestrator runs outside the loop.                     │
│  Claude builds. mz drives.                                   │
└─────────────────────────────────────────────────────────────┘
```
