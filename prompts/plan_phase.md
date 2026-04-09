# Plan Phase — {{phase_id}}

Decompose this phase into tracks and steps.

## Project

{{project}}

## Decisions

{{decisions}}

## Context (from discussion phase)

{{context}}

## Rules

1. **Tracks** are demoable vertical features. Each track should deliver something a user could see or interact with. 4-10 tracks per phase.

2. **Steps** are single-context-window units of work. Each step must be completable by an AI agent in one fresh session (~50 tool calls). If a step is too big, split it. 1-7 steps per track.

3. **Dependencies**: Steps within a track are sequential (ST01 before ST02). Tracks may depend on other tracks — declare this explicitly.

4. **Must-haves per step**: Every step plan must declare:
   - **Truths**: Invariants that must hold (e.g., "all existing tests still pass")
   - **Artifacts**: Files that must exist when done (e.g., "src/auth/oauth.ts")
   - **Key Links**: Files the agent MUST read before starting (e.g., "src/db/schema.ts")

## Output Format

Output the plan in this exact format:

```markdown
# Phase {{phase_id}} — [Title]

[One paragraph describing what this phase delivers]

## Dependencies Between Tracks
- TR02 depends on TR01 (needs auth before payments)
- TR03 depends on TR01

## TR01 — [Track Title]

[One paragraph: what this track delivers and why it matters]

### ST01 — [Step Title]

**Must-haves:**
- Truths: [invariants]
- Artifacts: [files that must exist]
- Key Links: [files to read first]

**Action:**
[Clear description of what to build. Be specific about file paths, function signatures, data structures. The executing agent has never seen this codebase — give it everything it needs.]

### ST02 — [Step Title]
...

## TR02 — [Track Title]
...
```

Be precise. Each step plan is the ONLY context the executing agent will receive (plus dependency summaries). If something is important, say it explicitly.
