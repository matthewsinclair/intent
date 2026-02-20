# Session Restart Context

## Project

Intent v2.4.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0022 (Harden `st new`) implemented with three features:

- `escape_sed_replacement()` -- safe handling of `/`, `&`, `\` in steel thread titles
- `slugify()` -- auto-generates `slug:` frontmatter field (max 50 chars), replaces Title column in listings
- `-s|--start` flag -- create and immediately start a steel thread in one command

327 tests passing across 16 files. Docs updated (reference guide, user guide, CHANGELOG).

## What's Next

1. Bump tag and push to include ST0022 changes
2. Regenerate .treeindex files for changed directories
3. Fix subagent sync false-positive ("modified locally" when source changed)
4. Review ST0010 and ST0015 to decide if still relevant

## Key Patterns

- Commands live in `bin/intent_<name>` and are auto-routed by `bin/intent`
- Global commands: `GLOBAL_COMMANDS` on line 41 of `bin/intent`
- Plugin commands: `intent claude skills` -> `intent/plugins/claude/bin/intent_claude_skills`
- Plugin commands: `intent claude subagents` -> `intent/plugins/claude/bin/intent_claude_subagents`
- Plugin commands: `intent claude upgrade` -> `intent/plugins/claude/bin/intent_claude_upgrade`
- Plugin commands: `intent agents` -> `intent/plugins/agents/bin/intent_agents`
- Skill definitions: `intent/plugins/claude/skills/<name>/SKILL.md`
- Subagent definitions: `intent/plugins/claude/subagents/<name>/`
- Templates: `intent/plugins/agents/templates/<name>/` (default, elixir)
- Tests: `tests/unit/` (16 .bats files, 327 tests), run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
- NO Claude attribution in commit messages
- Never use em dashes in skill files (multi-byte truncation bugs)
