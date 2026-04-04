---
verblock: "04 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: "Rationalization Tables"
scope: Small
status: Done
---

# WP-02: Rationalization Tables

## Objective

Add adversarial "Red Flags" tables to 4 workflow skills. These tables anticipate how the model will try to skip or shortcut the skill's requirements, providing self-correction prompts that improve enforcement durability. This is the most novel pattern from Superpowers.

## Deliverables

### Red Flags tables for 4 skills

#### `in-essentials/SKILL.md`

```markdown
### Red Flags

| Rationalization                           | Reality                                                |
| ----------------------------------------- | ------------------------------------------------------ |
| "I'll use the CLI later"                  | Manual creation causes drift. Use the CLI now.         |
| "This is too small for a steel thread"    | Every piece of work gets tracked. No exceptions.       |
| "I know where things are, skip treeindex" | You lose context on compaction. Check treeindex first. |
| "I'll update docs at the end"             | Sessions get interrupted. Update docs as you go.       |
```

#### `in-plan/SKILL.md`

```markdown
### Red Flags

| Rationalization                       | Reality                                               |
| ------------------------------------- | ----------------------------------------------------- |
| "This is simple, no plan needed"      | Simple tasks grow. The plan takes 2 minutes.          |
| "I'll figure it out as I go"          | Ad-hoc coding produces ad-hoc results.                |
| "The user wants speed, skip planning" | Plans prevent rework. Rework is slower than planning. |
| "I already know the codebase"         | Check MODULES.md anyway. Memory drifts.               |
```

#### `in-finish/SKILL.md`

```markdown
### Red Flags

| Rationalization                          | Reality                                                      |
| ---------------------------------------- | ------------------------------------------------------------ |
| "I'll update restart.md next session"    | Next session won't have this context. Write it now.          |
| "The code speaks for itself"             | Code changes without docs are invisible to the next session. |
| "Just one more quick fix before wrap-up" | Finish means finish. No new code.                            |
```

#### `in-standards/SKILL.md`

```markdown
### Red Flags

| Rationalization                           | Reality                                           |
| ----------------------------------------- | ------------------------------------------------- |
| "This helper is only used once, it's OK"  | Check MODULES.md. Someone else may have built it. |
| "The coordinator needs this logic inline" | If it's more than parse/call/format, extract it.  |
| "Pattern matching is overkill here"       | It's never overkill. It's the Elixir way.         |
```

### Placement

Add the "Red Flags" section as the LAST section in each skill, after all procedural content. This ensures the model reads the rules first, then encounters the self-correction prompts.

Do NOT add rationalization tables to domain-specific skills (`in-elixir-essentials`, `in-ash-ecto-essentials`, `in-phoenix-liveview`, `in-elixir-testing`). Those are reference material, not workflow enforcement.

## Acceptance Criteria

- [ ] 4 skills have "Red Flags" tables
- [ ] No em dashes in any added content
- [ ] All markdown tables are column-aligned
- [ ] Tables placed as last section in each skill
- [ ] `intent claude skills sync` works without errors

## Dependencies

- None (can be done independently of other WPs)
