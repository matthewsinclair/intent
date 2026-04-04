---
verblock: "04 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: "Plan Granularity Standards"
scope: Small
status: Done
---

# WP-04: Plan Granularity Standards

## Objective

Add a "Plan Quality Standards" section to `in-plan/SKILL.md` that enforces specific granularity requirements for workplans. Adapted from Superpowers' `writing-plans` skill, which demands complete, executable steps with no ambiguity.

## Deliverables

### New section in `in-plan/SKILL.md`

Insert after "### 2. Show detailed workplan" and before "### 3. Invoke relevant coding skills":

```markdown
### 3. Plan quality standards

Every plan must meet these standards before presenting to the user:

**No placeholders**: Plans must not contain "TBD", "TODO", "handle edge cases", "implement as needed", or similar deferred language. If you don't know the answer yet, that's a question for the user, not a placeholder.

**Specific file paths**: Every step must name the files it will create or modify. "Update the config" is not a step. "Add pool_size to config/runtime.exs" is.

**Small steps**: Each step should be independently verifiable. If a step modifies more than 3 files, split it. If a step takes more than a few minutes of coding, split it.

**Verification per step**: Each step includes how to verify it worked (test command, build command, or manual check). A step without verification is not a step.
```

Renumber subsequent sections (current "### 3" becomes "### 4", etc.).

## Acceptance Criteria

- [ ] `in-plan/SKILL.md` has "Plan quality standards" section
- [ ] Existing procedure sections renumbered correctly
- [ ] No em dashes in added content
- [ ] `intent claude skills sync` works

## Dependencies

- Coordinate with WP-01 (chains_to) and WP-02 (Red Flags) if modifying `in-plan` in the same session
- No hard dependency -- can be merged cleanly if done separately
