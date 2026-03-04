---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: "Skill Rename (intent-* to in-*)"
scope: Small
status: Done
---

# WP-01: Skill Rename (intent-_ to in-_)

## Objective

Rename all existing Intent skills from the `intent-*` prefix to the shorter `in-*` prefix. Typing `/intent-elixir-essentials` is tedious; `/in-elixir-essentials` is faster, memorable, and still clearly namespaced as Intent skills.

## Deliverables

### Directory Renames

| Current Name                 | New Name                 |
| ---------------------------- | ------------------------ |
| `intent-essentials`          | `in-essentials`          |
| `intent-elixir-essentials`   | `in-elixir-essentials`   |
| `intent-ash-ecto-essentials` | `in-ash-ecto-essentials` |
| `intent-phoenix-liveview`    | `in-phoenix-liveview`    |
| `intent-elixir-testing`      | `in-elixir-testing`      |
| `intent-autopsy`             | `in-autopsy`             |

### Files to Modify

1. **Source directories**: Rename `intent/plugins/claude/skills/intent-*` to `in-*`
2. **SKILL.md internal references**: Update self-references (e.g., autopsy skill references `intent-autopsy` in its script path)
3. **in-essentials SKILL.md**: Update Rule 5 example (`intent claude skills install in-elixir-essentials`)
4. **Help file**: Update `lib/help/claude.help.md` examples
5. **CLAUDE.md**: Update skill references in project CLAUDE.md
6. **CLAUDE.md template**: Update `lib/templates/llm/_CLAUDE.md`
7. **MEMORY.md**: Update skill names in auto-memory
8. **Test files**: Update any test references to old skill names

### Migration

- Uninstall old skills from `~/.claude/skills/intent-*`
- Install new skills to `~/.claude/skills/in-*`
- Update manifest at `~/.intent/skills/installed-skills.json`
- `intent claude skills sync` should handle the transition

## Acceptance Criteria

- [ ] All 6 skills renamed and installable under new names
- [ ] `intent claude skills list` shows `in-*` names
- [ ] `intent claude skills install in-essentials` works
- [ ] No references to old `intent-essentials` (etc.) remain in source
- [ ] All existing tests pass
- [ ] Skills function correctly after rename

## Dependencies

- None (foundational -- must be done first)
- Blocks: WP-02 (new skills use `in-*` naming convention)
