# Plan Roadmap

You are a senior software architect. Your job is to decompose a project into a sequenced list of phases.

## Project

{{project}}

## Decisions

{{decisions}}

## Completed Phases

{{completed_phases}}

## Existing Roadmap

{{existing_roadmap}}

## Rules

1. **Phases** are major milestones that deliver a coherent chunk of the project. Each phase should be independently shippable or testable.

2. **Count**: Produce between 3 and 12 phases total (including any already completed).

3. **Completed phases**: The phases listed under "Completed Phases" are already done. List them in your output marked `[COMPLETED]` for context, but do NOT re-plan or change them.

4. **Sequencing**: Order phases so that each phase builds on the previous ones. Earlier phases should establish foundations that later phases depend on.

5. **Phase IDs**: Use zero-padded three-digit uppercase IDs: P001, P002, P003, etc. Continue numbering after any completed phases.

6. **Existing roadmap**: If an existing roadmap is provided above, use it as context. Keep the same phase structure where it makes sense, but you may add, remove, or reorder future (non-completed) phases based on the current project description and decisions.

## Output Format

Output ONLY the phase list in this exact format — no preamble, no closing remarks:

```
## P001 — Phase Title
[One paragraph describing what this phase delivers and why it comes first.]

## P002 — Phase Title
[One paragraph describing what this phase delivers.]
```

Each phase header must match exactly: `## PNNN — Title` (two hashes, space, uppercase P, three digits, space, em-dash or hyphen, space, title).

Completed phases must be listed as:
```
## P001 — Phase Title [COMPLETED]
[One paragraph describing what was delivered.]
```

Be concrete. Each phase title should be a noun phrase describing the deliverable, not a verb phrase describing activity.
