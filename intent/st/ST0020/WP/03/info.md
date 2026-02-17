---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: Create Ash/Ecto Reference Doc + Skill
scope: Medium
status: Not Started
---

# WP-03: Create Ash/Ecto Reference Doc + Skill

## Objective

Create comprehensive Ash/Ecto database access guidance — reference doc for deep consultation and skill for always-on enforcement. Ash-first, never raw Ecto.

## Deliverables

- `intent/plugins/claude/subagents/elixir/ash-ecto.md` — reference doc (~200-400 lines)
- `intent/plugins/claude/skills/ash-ecto-essentials/SKILL.md` — skill (~150 lines)

## Design Principles

**Critical: Ash-first, never raw Ecto.**

- No `mix ecto.gen.migration` — always `mix ash.codegen`
- No direct Repo calls — all through Ash domain code interfaces
- No raw Ecto.Query in web modules — use Ash.Query or code interface options
- Explicitly conform to `deps/ash/usage-rules.md` and `deps/ash_ai/usage-rules.md`

## Skill Rules (7)

1. All database access through domain code interfaces
2. `mix ash.codegen <name>` for migrations, never `mix ecto.gen.migration`
3. Set actor on query/changeset, not on action call
4. Prefer code interface options over manual Ash.Query pipelines
5. Custom change/validation modules, not anonymous functions
6. Atomic changes preferred; `require_atomic? false` only when necessary
7. `Ash.Query.filter` is a macro — always `require Ash.Query`

## Acceptance Criteria

- [ ] Reference doc covers migrations, code interfaces, queries, relationships, changes, validations, preparations, notifiers, error handling
- [ ] Skill under 150 lines with 7 rules + examples
- [ ] References `deps/ash/usage-rules.md` as authoritative source
- [ ] No overlap with elixir-essentials or phoenix-liveview skills
- [ ] No raw Ecto patterns taught (Ash way only)

## Dependencies

- WP-01 (distilled rules inform Ash-specific patterns)
