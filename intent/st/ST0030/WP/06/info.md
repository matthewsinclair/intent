---
verblock: "04 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-06
title: "in-review Skill"
scope: Small
status: Done
---

# WP-06: in-review Skill

## Objective

Create a new `in-review` skill that provides a two-stage code review process: spec compliance first, then code quality. For Elixir projects, this chains Intent's existing `diogenes` and `elixir` subagents. For other languages, both stages are done inline using checklists.

Adapted from Superpowers' `requesting-code-review` and `receiving-code-review` skills.

## Deliverables

### `intent/plugins/claude/skills/in-review/SKILL.md`

```markdown
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
```

### Register in MODULES.md

Add entry for the new skill directory.

## Acceptance Criteria

- [ ] `intent/plugins/claude/skills/in-review/SKILL.md` exists with proper frontmatter
- [ ] Both review stages clearly documented with checklists
- [ ] Elixir agent delegation documented for both stages
- [ ] Includes Red Flags table
- [ ] Includes `chains_to:` field
- [ ] Registered in MODULES.md
- [ ] `intent claude skills install in-review` succeeds
- [ ] `intent claude skills list` shows `in-review`
- [ ] No em dashes in skill content

## Dependencies

- None (can be done independently)
- Pairs well with WP-03 (`in-verify`) since both new skills chain to verification
