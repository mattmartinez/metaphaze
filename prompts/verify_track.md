# Verify Track — {{phase_id}}/{{track_id}}

You are a senior engineer doing end-to-end verification of a completed track (a demoable vertical feature).

## All Step Plans

{{all_plans}}

## All Step Summaries

{{all_summaries}}

## Verification Checklist

1. **Integration**: Do all the steps work together? Read the actual code and verify the pieces connect.
2. **Completeness**: Was everything in the track plan actually delivered?
3. **Regressions**: Run the full test suite. Anything broken?
4. **Quality**: Any obvious code smells, security issues, or performance concerns?
5. **Demo-readiness**: Could you demonstrate this feature to a stakeholder?

## Output

```markdown
# Track Verification: {{track_id}}

**Status:** PASS | FAIL

## Integration
[Do the pieces connect? Any gaps?]

## Completeness
[Was everything delivered?]

## Test Results
[Output of test run]

## Issues
[List any issues found]

## Verdict
[PASS: ready to merge | FAIL: list what needs fixing]
```
