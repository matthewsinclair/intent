---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: Create elixir-essentials Skill
scope: Small
status: Not Started
---

# WP-02: Create elixir-essentials Skill

## Objective

Create an installable skill that enforces core Elixir coding patterns during code generation, fixing the problem of Claude defaulting to imperative nested conditionals.

## Deliverables

- `intent/plugins/claude/skills/elixir-essentials/SKILL.md` (~100-150 lines)

## Installs To

`.claude/skills/elixir-essentials/SKILL.md` in target projects

## Mandatory Rules (8)

1. **Multi-clause pattern matching over conditionals** — NEVER nested `if/case/cond` on struct/map fields
2. **`@impl true` on all behaviour callbacks** — mount, handle_event, handle_info, init, etc.
3. **Tagged tuples for fallible functions** — `{:ok, result}` or `{:error, reason}`, never bare nil
4. **`with` for railway-oriented composition** — chain 2+ fallible operations
5. **Pipe operator for transformations** — 2+ transforms use pipes, data is first argument
6. **Naming conventions** — `?` for booleans, `!` for raising, `_` for unused
7. **Assertive data access** — `struct.field` for required, `map[:key]` for optional
8. **No debug artifacts** — no `IO.inspect/2`, no `dbg()`, no `IO.puts` debugging

Each rule includes a concrete before/after code example.

## Acceptance Criteria

- [ ] File exists at correct path
- [ ] Under 150 lines
- [ ] 8 mandatory rules, each with before/after example
- [ ] No overlap with ash-ecto-essentials or phoenix-liveview skills
- [ ] Pure language patterns only (no framework specifics)

## Dependencies

- WP-01 (distilled rules are the basis)
