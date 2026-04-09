# Metaphaze

A spec-driven context engine for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Decomposes projects into milestones, slices, and tasks, then executes them autonomously with a fresh context window per task.

## The Problem

AI coding agents degrade over long sessions. Context fills up, earlier instructions get forgotten, and output quality drops. Orchestrator tools built on third-party SDKs face subscription restrictions as providers crack down on agent harnesses.

## How Metaphaze Solves It

Metaphaze is a Rust CLI that sits outside the LLM. It drives a state machine on disk and calls `claude -p` (Claude Code's headless mode) for each unit of work. Every task gets a clean context window with only the information it needs pre-inlined.

- The **orchestrator never degrades** — it's deterministic Rust code, not an LLM
- Each **task gets a fresh context** — no accumulated garbage from prior work
- All **state lives on disk** in `.mz/` — survives crashes, session restarts, and context resets
- Runs on a **first-party surface** — calls the `claude` binary directly, no third-party SDK

## Install

```bash
cargo install --git https://github.com/mattmartinez/metaphaze
```

Requires [Rust](https://rustup.rs/) and [Claude Code](https://docs.anthropic.com/en/docs/claude-code) installed.

## Usage

```bash
# Start a new project
mz init

# Capture decisions and resolve ambiguity
mz discuss

# Decompose into milestones, slices, and tasks
mz plan

# Execute one task at a time
mz next

# Or let it run autonomously
mz auto

# Check progress
mz status

# Change direction mid-flight
mz steer "drop PayPal, Stripe only"
```

## How It Works

### The Hierarchy

Work decomposes into three levels:

- **Milestone** — a shippable version (4-10 slices)
- **Slice** — a demoable vertical feature (1-7 tasks)
- **Task** — a single context-window unit of work

The iron rule: if a task doesn't fit in one context window, split it.

### The Loop

```
mz auto
  ├── read STATE from .mz/
  ├── find next pending task
  ├── read task PLAN + dependency SUMMARIES
  ├── build prompt with all context pre-inlined
  ├── claude -p (fresh context, execute task)
  ├── verify must-haves
  ├── commit to slice branch
  ├── advance state
  └── loop
```

### State on Disk

```
.mz/
  state.yaml              # current position in the plan
  PROJECT.md              # project description, tech stack
  DECISIONS.md            # append-only decision register
  milestones/
    M001/
      ROADMAP.md          # milestone plan with success criteria
      CONTEXT.md          # decisions from discussion phase
      slices/
        S01-login-flow/
          PLAN.md         # slice plan
          tasks/
            T01-PLAN.md     # task spec with must-haves
            T01-SUMMARY.md  # what was done (context for T02)
            T01-VERIFY.md   # verification results
```

### Git Strategy

Each slice gets its own branch (`mz/M001/S01`). Each task is an atomic commit. When a slice passes verification, it squash-merges to main.

### Must-Haves

Every task plan declares:
- **Truths** — invariants that must hold (e.g., "all existing tests still pass")
- **Artifacts** — files that must exist when done
- **Key Links** — files the agent must read before starting

The verifier checks these after each task and after each slice completes.

### Model Routing

Planning and verification use Opus (deep reasoning). Task execution uses Sonnet (fast, capable). Discussion uses Opus (needs to ask good questions).

## Phases

| Phase | Command | What Happens |
|-------|---------|-------------|
| Init | `mz init` | Interactive project setup |
| Discuss | `mz discuss` | Probing questions to lock down decisions |
| Plan | `mz plan` | Decompose milestone into slices and tasks |
| Execute | `mz next` / `mz auto` | Build each task with fresh context |
| Verify | (automatic) | Check must-haves after each task and slice |
| Steer | `mz steer "..."` | Record decision, re-plan remaining work |

## Design Principles

**The orchestrator is not an LLM.** The state machine, loop control, and context assembly are all deterministic Rust. Claude only does the creative work — planning, coding, verifying.

**Summaries are the memory system.** Each completed task writes a SUMMARY.md that becomes input for the next task. No magic memory layer — just structured files on disk that any session can read.

**Decisions are append-only.** DECISIONS.md is a register of settled questions. Once a decision is recorded, it's injected into every future prompt. No re-litigating.

**Fresh context is non-negotiable.** Every task starts with a clean context window. The orchestrator pre-inlines exactly what the agent needs — no "please read this file" hoping.

## Status

Early development. The core loop works. Planned additions:

- [ ] Parallel slice execution via git worktrees
- [ ] `--max-budget-usd` cost ceiling
- [ ] Stuck detection (loop detection in state transitions)
- [ ] Crash recovery with auto-restart
- [ ] `mz log` for execution history

## License

MIT
