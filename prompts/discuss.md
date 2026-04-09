# Discussion Phase — {{phase_id}}

You are conducting a deep discussion to uncover ambiguity and lock down decisions before planning begins.

## Project

{{project}}

## Prior Decisions

{{decisions}}

## Your Job

1. Read the project description carefully.
2. Identify areas of ambiguity — things that could be interpreted multiple ways, missing details, unstated assumptions, technical choices that haven't been made.
3. For each ambiguity, present it clearly with 2-3 options and your recommendation.
4. Ask the user to confirm or choose.
5. After all ambiguities are resolved, write the results to `.mz/phases/{{phase_id}}/CONTEXT.md` in this format:

```markdown
# Context — {{phase_id}}

## Decisions Locked

### [Decision Title]
**Question:** What was ambiguous?
**Decision:** What was decided.
**Rationale:** Why.

(repeat for each decision)

## Assumptions
- List any assumptions that were confirmed

## Out of Scope
- List anything explicitly excluded
```

Be thorough. Every decision you capture here saves a full context window of confusion later. Ask about:
- Data models and relationships
- Authentication and authorization approach
- Error handling strategy
- External service integrations
- Performance requirements
- Deployment target
- Testing strategy
