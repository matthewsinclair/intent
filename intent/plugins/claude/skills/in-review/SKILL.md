---
description: "Two-stage code review: spec compliance then code quality"
chains_to: ["in-verify"]
---

# Two-Stage Code Review

Review code in two distinct passes. Do not combine them -- each pass has a different focus.

## When to invoke

- After completing a work package or significant task
- Before marking a steel thread as done
- When the user asks for a code review

## Procedure

### Stage 1: Spec compliance

Does the implementation match the plan?

Checklist:

- [ ] All deliverables from the WP/ST `info.md` are implemented
- [ ] No extra features added beyond what was planned
- [ ] File paths match what was specified in the plan
- [ ] Edge cases identified in the design are handled
- [ ] No "TODO" or "FIXME" left unresolved from the plan

**For Elixir projects**: Delegate to the `diogenes` agent for test spec validation. Provide the design doc and the implementation, ask diogenes to verify alignment.

### Stage 2: Code quality

Is the implementation well-written?

Checklist:

- [ ] Highlander Rule: no duplicated code paths (check MODULES.md)
- [ ] Thin Coordinators: business logic in services, not controllers/LiveViews
- [ ] PFIC: pattern matching, tagged tuples, pipe chains where appropriate
- [ ] No silent error swallowing
- [ ] No unnecessary abstractions or over-engineering
- [ ] Tests cover the happy path and at least one error path

**For Elixir projects**: Delegate to the `elixir` agent for comprehensive code review. Provide the files changed and ask for Usage Rules compliance review.

### After both stages

- Fix critical issues before proceeding
- Log non-critical issues as TODOs in `tasks.md`
- Invoke `/in-verify` to confirm fixes

## Red Flags

| Rationalization                          | Reality                                                |
| ---------------------------------------- | ------------------------------------------------------ |
| "The code works, review is unnecessary"  | Working code can still be wrong. Review catches drift. |
| "I wrote it, I know it's correct"        | Author blindness is real. Review with fresh eyes.      |
| "Review will slow us down"               | Rework from missed issues is slower than review.       |
| "Stage 1 is enough, skip quality review" | Correct-but-messy code becomes tomorrow's bug.         |
