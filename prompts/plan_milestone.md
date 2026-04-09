# Plan Milestone — {{milestone_id}}

Decompose this milestone into slices and tasks.

## Project

{{project}}

## Decisions

{{decisions}}

## Context (from discussion phase)

{{context}}

## Rules

1. **Slices** are demoable vertical features. Each slice should deliver something a user could see or interact with. 4-10 slices per milestone.

2. **Tasks** are single-context-window units of work. Each task must be completable by an AI agent in one fresh session (~50 tool calls). If a task is too big, split it. 1-7 tasks per slice.

3. **Dependencies**: Tasks within a slice are sequential (T01 before T02). Slices may depend on other slices — declare this explicitly.

4. **Must-haves per task**: Every task plan must declare:
   - **Truths**: Invariants that must hold (e.g., "all existing tests still pass")
   - **Artifacts**: Files that must exist when done (e.g., "src/auth/oauth.ts")
   - **Key Links**: Files the agent MUST read before starting (e.g., "src/db/schema.ts")

## Output Format

Output the plan in this exact format:

```markdown
# Milestone {{milestone_id}} — [Title]

[One paragraph describing what this milestone delivers]

## Dependencies Between Slices
- S02 depends on S01 (needs auth before payments)
- S03 depends on S01

## S01 — [Slice Title]

[One paragraph: what this slice delivers and why it matters]

### T01 — [Task Title]

**Must-haves:**
- Truths: [invariants]
- Artifacts: [files that must exist]
- Key Links: [files to read first]

**Action:**
[Clear description of what to build. Be specific about file paths, function signatures, data structures. The executing agent has never seen this codebase — give it everything it needs.]

### T02 — [Task Title]
...

## S02 — [Slice Title]
...
```

Be precise. Each task plan is the ONLY context the executing agent will receive (plus dependency summaries). If something is important, say it explicitly.
