# Summarize Task — {{milestone_id}}/{{slice_id}}/{{task_id}}

Review the work that was done and write a concise summary.

## Task Plan

{{task_plan}}

## Files Changed

Look at the git diff for the most recent commit to see what was actually changed.

## Output

Write to `.mz/milestones/{{milestone_id}}/slices/{{slice_id}}/tasks/{{task_id}}-SUMMARY.md`:

```markdown
---
status: complete
files_changed:
  - list/each/file.ts
files_created:
  - list/new/files.ts
tests_passed: true|false
---

## What was done
[2-3 sentences]

## Notable decisions
- [Decisions that affect future tasks]

## Gotchas
- [Surprises for future tasks]
```
