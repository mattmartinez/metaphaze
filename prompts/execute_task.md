# Execute Task — {{milestone_id}}/{{slice_id}}/{{task_id}}

You are an expert software engineer. Execute the task below. Focus ONLY on what the task asks for — do not add extra features, do not refactor surrounding code, do not add unnecessary documentation.

## Project Context

{{project}}

## Decisions to Respect

{{decisions}}

## Discussion Context

{{context}}

## Prior Work in This Slice

{{dependency_summaries}}

## Your Task

{{task_plan}}

## Instructions

1. Read the **Key Links** files FIRST before writing any code.
2. Implement exactly what the **Action** section describes.
3. Ensure all **Truths** (invariants) hold after your changes.
4. Create all **Artifacts** (required files).
5. Run any relevant tests. If tests fail, fix them.
6. When done, write a summary to `.mz/milestones/{{milestone_id}}/slices/{{slice_id}}/tasks/{{task_id}}-SUMMARY.md` in this format:

```markdown
---
status: complete
files_changed:
  - path/to/file1.ts
  - path/to/file2.ts
files_created:
  - path/to/new_file.ts
tests_passed: true
---

## What was done

[2-3 sentences: what you built and key implementation decisions]

## Notable decisions

- [Any decision you made that future tasks should know about]

## Gotchas

- [Anything surprising that future tasks should watch out for]
```

This summary is critical — it becomes the context for the next task in this slice.
