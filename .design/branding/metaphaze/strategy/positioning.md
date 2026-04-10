# Positioning
> Phase: strategy | Brand: metaphaze | Generated: 2026-04-10

## TL;DR

metaphaze is the single-binary Rust orchestrator that drives Claude Code from outside the LLM loop. It sits in the empty lower-right quadrant of the 2x2 — **brutalist and first-party** — and holds that corner on technical merits: deterministic state machine, disk-first `.mz/`, calls the `claude` binary directly. Launch copy does not reference the April 4 crackdown. The architecture is the argument.

---

## Positioning Statement

**For senior software engineers who want multi-hour autonomous coding runs without fragile harnesses, accounts, or provider risk, metaphaze is the single-binary Rust orchestrator that drives Claude Code from outside the LLM loop — because the state machine, context assembly, and recovery logic are deterministic Rust on disk, and the only thing it shells out to is the first-party `claude` binary.**

### Parsed

| Component | Content |
|---|---|
| Target audience | Senior software engineers running multi-hour agent workflows in the terminal |
| Need | Hands-off orchestration that survives context bloat, crashes, and provider policy changes |
| Brand | metaphaze |
| Category | Single-binary Rust orchestrator for Claude Code |
| Key benefit | Driven from outside the LLM loop, on durable first-party rails |
| Reason to believe | Deterministic Rust state machine, disk-first state in `.mz/`, calls the `claude` binary directly, MIT license, no accounts, no telemetry |

### Why each part is load-bearing

- **"Senior software engineers"** — not "developers using Claude Code." The Senior Operator in BRIEF.md is a narrower, angrier market. Narrower is stronger. Do not broaden.
- **"Multi-hour autonomous"** — the specific job that interactive tools (Cursor, Windsurf, Zed) do not do and that fragile harnesses fail at after 90 minutes.
- **"Single-binary Rust"** — not an adjective. A category frame. Puts metaphaze on the shelf with `rg`, `fzf`, `bat`, not on the shelf with claude-flow and Pi.
- **"Drives Claude Code from outside the LLM loop"** — the manifesto line compressed into the positioning. Everything flows from this.
- **"Deterministic Rust on disk"** and **"first-party `claude` binary"** — two concrete, falsifiable reasons to believe. Either both are true or the positioning collapses. Both are true.

---

## 2x2 Positioning Map

Axes inherited from `discover/competitive-audit.md`. These are the two axes that actually matter right now.

- **X-axis:** third-party (fragile) ← → first-party (durable)
- **Y-axis:** polished SaaS ↑ / brutalist ↓

```
                    POLISHED SaaS
                          ↑
                          |
                          |
    claude-flow/Ruflo ●   |
                          |
                          |   ● oh-my-claudecode
                          |
                          |
 THIRD-PARTY  ────────────┼──────────────  FIRST-PARTY
  (fragile)               |                  (durable)
                          |
     Pi Coding Agent ●    |
                          |
            GSD 2.0 ●     |
                          |     ● GSD 1.0
                          |
                          |           ★ metaphaze
                          |
                          ↓
                      BRUTALIST
```

### Quadrant read

- **Upper-left — polished SaaS × third-party.** claude-flow/Ruflo. "Distributed swarm intelligence." 314 MCP tools. Gradient banners. npm-only. This is the worst quadrant for the Senior Operator and the one metaphaze must visibly not occupy. No illustrations, no gradients, no feature-count bragging.
- **Upper-right — polished SaaS × first-party.** oh-my-claudecode. "A weapon, not a tool." Runs inside Claude Code so the plumbing is durable, but the brand is try-hard marketing. Quadrant is open at the top but not where metaphaze wants to sit.
- **Lower-left — brutalist × third-party.** GSD 2.0, Pi Coding Agent. Honest voices, but both are built on SDK surfaces that can be restricted at the provider's discretion. Structurally fragile even though the brand reads well.
- **Lower-right — brutalist × first-party.** GSD 1.0 is loitering here but is a skill framework, not a real orchestrator. **The corner is empty. metaphaze takes it.**

### White space rationale

Three things have to be simultaneously true for the corner to be defensible:

1. **The orchestrator must call the first-party `claude` binary directly.** Every third-party harness is one provider announcement away from degradation. metaphaze's choice to shell out to `claude` as-is is a hard technical moat.
2. **The orchestrator must be a single deterministic binary.** Not a plugin marketplace (oh-my-claudecode), not an npm package (claude-flow, Pi), not a skill framework you have to drive yourself (GSD 1.0). `cargo install mz` and the thing exists on disk. Nothing else to wire up.
3. **The brand must read as intentional brutalism, not neglect.** GSD 1.0 is brutalist by absence of a designer. metaphaze is brutalist by choice — monospace everywhere, `#0a0a0a`/`#ededed`/`#5fb878`, TUI as the hero asset. suckless-adjacent philosophy with Berkeley Graphics-grade typographic discipline.

No current competitor executes all three. The corner holds for 6-12 months before the aesthetic gets copied. Commit hard, ship fast.

---

## Positioning Against Competitors

metaphaze positions itself by contrast, not by attack. Never name a competitor in copy. Let the architecture draw the line.

| Dimension | metaphaze | claude-flow / Ruflo | GSD 2.0 | oh-my-claudecode | Pi Coding Agent |
|---|---|---|---|---|---|
| Runtime | Single Rust binary | Node + npm | Node + Pi SDK | Claude Code plugin | Node + npm |
| Surface | First-party `claude` binary | Reverse-engineered SDK | Pi SDK | Inside Claude Code | Multi-provider abstraction |
| State | `.mz/` on disk, inspectable | In-memory + cache | Milestone files | Plugin memory | Session state |
| Orchestrator | Deterministic Rust | LLM-driven, swarm | Pi executor | Multi-agent plugin | Tool-loop agent |
| Accounts | None | None required | None | Claude Code | None |
| Install | `cargo install mz` | `npm install -g` | `npm install -g` | Plugin marketplace | `npm install -g` |
| Brand posture | Brutalist craft | Enterprise pastiche | GitHub default | Marketing veneer | Domain-joke minimal |

### What metaphaze says without saying it

- **Against claude-flow:** no emoji headers, no gradient banners, no "314 MCP tools" flex. The absence is the rebuttal.
- **Against GSD 2.0:** the install instruction is `cargo install mz`, not `npm install -g @something/something`. The runtime is the rebuttal.
- **Against oh-my-claudecode:** the landing page is the README, not a marketing site. The distribution model is the rebuttal.
- **Against Pi:** metaphaze does one thing — drive Claude Code — instead of abstracting over every provider. The specificity is the rebuttal.

---

## Category Frame

metaphaze is not an "AI coding agent." It is not an "agentic framework." It is not a "copilot."

**Category as written:** *single-binary orchestrator for Claude Code*.

This is a narrower category than any competitor uses. That is the point. Narrow categories are easier to own. "Orchestrator for Claude Code" is a phrase the Senior Operator will type into search, read on a README, and understand in under two seconds. "Agentic framework" is a phrase they roll their eyes at.

**Banned words in all category language:** agentic, framework, platform, swarm, autonomous agent system, AI-native, intelligent agent, copilot, assistant.
**Preferred words:** orchestrator, driver, runner, binary, state machine, loop, pipeline.

---

## The Refusals

Positioning by refusal. A public list of what metaphaze will never be. This list lives on the landing page as the brand's second-most-visible asset after the hero headline. Ordered by how much each refusal hurts a competitor.

- no accounts
- no cloud
- no dashboard
- no telemetry
- no signup wall
- no SaaS tier
- no enterprise edition
- no ROI calculator
- no third-party SDK
- no node runtime
- no mascot

Each line is a position taken. Together, they describe the shape of the brand without needing adjectives.

---

## Positioning Tests

Before shipping any copy, run it against these three tests. If it fails any one, rewrite.

1. **The two-second test.** A senior dev sees the metaphaze landing page in a tab next to a `claude-flow` landing page. Within two seconds they can tell metaphaze was made by someone who actually writes code. If the copy needs them to read past the fold, it failed.
2. **The "finally" test.** The Senior Operator reads the hero headline and feels one specific thing — "finally, someone gets it." Not "interesting." Not "I should try this." Finally. If the copy prompts any other reaction, it failed.
3. **The monospace test.** The copy renders correctly in pure monospace with no sans-serif fallback. Every character has to be typeable. No special Unicode, no em-dash abuse that breaks `<pre>` blocks, no smart quotes. If it doesn't survive `cat README.md`, it failed.
