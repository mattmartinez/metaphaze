# Brand Brief

## Brand
- **Name:** metaphaze
- **Date:** 2026-04-10

## Company
- **Company name:** metaphaze
- **Industry:** Developer tools / AI coding infrastructure
- **Founded:** 2026
- **Size:** Solo / OSS
- **Stage:** mvp
- **Existing brand?** no

## Brand Mode
- **Mode:** new
- **Reason:** Brand-new project. Needs an identity that matches what the tool actually is — a brutalist, single-binary CLI that orchestrates Claude Code, built by and for senior developers who are tired of fragile, hyped agent harnesses.

### Existing Brand State (evolve only)
N/A — new brand

### Evolution Scope (evolve only)
N/A — new brand

## Business
- **Problem:** AI coding agents degrade over long autonomous sessions. Orchestrator tools built on third-party SDKs face subscription restrictions when providers crack down. Developers who want hands-off, multi-hour AI coding workflows are stuck choosing between fragile harnesses and one-shot interactive sessions.
- **Solution:** A Rust CLI (`mz`) that drives Claude Code from outside the LLM loop. The state machine, orchestration, and context assembly are deterministic Rust — Claude only does the creative work. Each task gets a clean context window with exactly the inputs it needs pre-inlined. State lives on disk in `.mz/` and survives crashes, session resets, and reruns. Calls the first-party `claude` binary directly so it can never be banned.
- **Business model:** Open source (MIT). No SaaS, no accounts, no telemetry. Single `cargo install` and you own it.
- **Defensibility:** First-party surface (immune to provider crackdowns), single-binary simplicity, deterministic orchestrator that doesn't degrade with context length. The methodology (phase → track → step decomposition with disk-based state) is the moat — not the code.

## Personas

### Primary: The Senior Operator
- **Role:** Senior software engineer / staff engineer / indie hacker who builds with AI agents
- **Age range:** 28-45
- **Day-in-the-life:** Lives in the terminal. Has Claude Code, Codex, and probably one other agent running in tmux panes. Tries new tools constantly but most disappoint within a week. Wants to step away from the keyboard and have something real built when they come back.
- **Frustration:** Agent harnesses that look great in demos but fall apart on long runs. Tools that get banned by providers a month after launch. SaaS dashboards for things that should be a CLI. Marketing copy that says "agentic" forty times and explains nothing.
- **Aspiration:** Type one command, walk away for 90 minutes, come back to working code that respects their decisions. Own the entire stack — no accounts, no cloud dependencies, no surprises when a provider changes their mind.
- **Discovery:** Hacker News, X (developer side), GitHub trending, recommendations from devs whose taste they trust. Reads the README before installing.
- **Trust signals:** Single binary. MIT license. Good README without hero illustrations. Code that actually compiles and tests that actually pass. Distrusts: gradients, "transform your workflow," signup walls, anything that says "agentic AI."

## Brand Essence

### Promise
- **Core promise:** Whenever someone interacts with metaphaze, they should feel like they're using a tool built by someone who shares their frustrations.
- **Functional promise:** Hands-off autonomous coding that doesn't degrade over long runs and can't be banned.
- **Emotional promise:** Relief. The "finally, someone gets it" feeling of finding a tool that respects your time and intelligence.

### Point of View
- **Category disagreement:** Agent harnesses don't need to be SaaS. They don't need accounts, dashboards, or telemetry. The orchestrator should be deterministic code that drives the LLM, not an LLM trying to drive itself.
- **Underestimated truth:** The methodology is more valuable than the code. Phase → track → step decomposition with disk-based state survives any model upgrade, any provider change, and any context-window improvement. Tools built on this foundation outlive their dependencies.
- **Manifesto line:** The orchestrator runs outside the loop. Claude builds. mz drives.

### Personality
- **Personality:** Brutalist, honest, technical
- **Personality reference:** suckless meets Linear meets Charmbracelet — stark and direct at the core, intentional in the details, terminal-native in execution
- **Not us:** Corporate, hyped
- **Never be:** Enterprise dev tool. No stock photos of teams pointing at screens. No "ROI calculator." No "agentic AI" anywhere. No glowing orbs.
- **Tone:** Direct. Lowercase when it fits. Short sentences. Code examples over prose. The README is the marketing.

## Competitive Landscape
- **Direct competitors:** GSD 1.0 (Claude Code skill framework), GSD 2.0 (Pi SDK based), oh-my-claudecode, claude-flow / Ruflo, Pi Coding Agent
- **What sets you apart?** First-party surface (calls the `claude` binary directly, can't be banned). Single Rust binary, no Node runtime. Deterministic orchestrator outside the LLM loop. Disk-first state that survives anything.
- **Brands admired:** suckless.org, htmx, Linear, Tailwind, Vercel, Charmbracelet, Bubble Tea

## Inspiration
- **Styles liked:** Brutalist core (suckless, htmx) + restrained polish (Linear, Tailwind, Vercel) + terminal craft (Charmbracelet, Bubble Tea)
- **Styles to avoid:** Enterprise dev tool aesthetics — stock team photos, blue gradients, ROI calculators, case studies, signup walls
- **Existing assets:** None yet

## Constraints
- **Timeline:** Open
- **Budget:** $0 (solo / OSS)
- **Must-haves:**
  - Single binary install — brand should reflect `cargo install` simplicity
  - Open source forever (implied by MIT license)
- **Non-negotiables:**
  - No SaaS layer
  - No accounts
  - No telemetry
  - Brand has to look right in monospace

## Goals
- **Business goal:** Build a tool worth maintaining for years. Earn trust in the developer community as the durable, no-bullshit alternative to agent SaaS.
- **Brand goal:** Become the brand that senior devs trust to outlive provider crackdowns and stay simple as it scales.
- **Success metrics:** GitHub stars from devs whose taste matters, organic mentions in HN/X dev threads, contributors who fork and don't bail.

## Deliverables
- [ ] Discovery & research
- [ ] Brand strategy & voice
- [ ] Visual identity
- [ ] Design system

## Notes
The product is already built (12 phases shipped, 97 steps complete) — the brand needs to match what exists, not the other way around. The TUI is the primary surface. Every brand decision should make sense in monospace, in 256 colors, in a terminal that may not even have emoji support.
