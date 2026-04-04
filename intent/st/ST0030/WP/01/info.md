---
verblock: "04 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: "Skill Dependency Chains"
scope: Small
status: Done
---

# WP-01: Skill Dependency Chains

## Objective

Add `chains_to:` frontmatter field to Intent's workflow skills, creating a navigable skill graph without auto-activation. When a skill completes, Claude should suggest the next skill(s) in the chain.

## Deliverables

### Frontmatter additions

Add `chains_to:` to these 4 workflow skills:

| Skill       | `chains_to:`             | Rationale                                           |
| ----------- | ------------------------ | --------------------------------------------------- |
| `in-start`  | `["in-plan", "in-next"]` | After orientation, either plan new work or continue |
| `in-plan`   | `["in-next"]`            | After planning, pick the first work unit            |
| `in-next`   | `["in-plan"]`            | After picking work, may need to plan it             |
| `in-finish` | `["in-verify"]`          | Before finishing, verify claims (requires WP-03)    |

### Skill chain section

Add a brief "Skill Chain" section at the bottom of each modified skill:

```markdown
## Skill Chain

After completing this skill, consider:

- `/in-plan` -- if starting new work that needs planning
- `/in-next` -- if continuing existing work
```

### No infrastructure changes

The `chains_to:` field is purely semantic. Claude reads it from SKILL.md frontmatter when the skill is loaded. No changes to `intent_claude_skills`, `claude_plugin_helpers.sh`, or manifests.

## Acceptance Criteria

- [ ] 4 skills have `chains_to:` in frontmatter
- [ ] Each skill has a "Skill Chain" section at the bottom
- [ ] `intent claude skills sync` works without errors
- [ ] Existing tests pass

## Dependencies

- None (can be done independently)
- WP-03 (`in-verify`) should exist before `in-finish` chains to it
