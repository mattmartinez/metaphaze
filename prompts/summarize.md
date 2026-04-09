# Summarize Step — {{phase_id}}/{{track_id}}/{{step_id}}

Review the work that was done and write a concise summary.

## Step Plan

{{step_plan}}

## Files Changed

Look at the git diff for the most recent commit to see what was actually changed.

## Output

Write to `.mz/phases/{{phase_id}}/tracks/{{track_id}}/steps/{{step_id}}-SUMMARY.md`:

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
- [Decisions that affect future steps]

## Gotchas
- [Surprises for future steps]
```
