# Session Restart Context

## Project

Intent v2.5.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.5.0 released. All steel threads completed or parked. 339 tests passing.

Recent completions:

- ST0023: Remove Backlog from Intent (70+ files, simplified CI)
- ST0024: Add work packages (`intent wp` command, 29 tests)
- ST0025: Fix Highlander Violations (shared helpers + plugin callback library)

## Architecture

- Commands live in `bin/intent_<name>` and are auto-routed by `bin/intent`
- Global commands: `GLOBAL_COMMANDS` on line 41 of `bin/intent`
- Plugin commands: `intent claude skills` -> `intent/plugins/claude/bin/intent_claude_skills`
- Plugin commands: `intent claude subagents` -> `intent/plugins/claude/bin/intent_claude_subagents`
- Plugin commands: `intent claude upgrade` -> `intent/plugins/claude/bin/intent_claude_upgrade`
- Plugin commands: `intent agents` -> `intent/plugins/agents/bin/intent_agents`
- Shared plugin library: `intent/plugins/claude/lib/claude_plugin_helpers.sh` (callback pattern)
- Shared helpers: `bin/intent_helpers` (error, checksum, terminal width, require_jq, require_claude, etc.)
- Skill definitions: `intent/plugins/claude/skills/<name>/SKILL.md`
- Subagent definitions: `intent/plugins/claude/subagents/<name>/`
- Templates: `lib/templates/` (ST, WP, LLM)
- Tests: `tests/unit/` (17 .bats files), run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)

## Deferred Work (ST0025)

Highlander violations still open (lower priority):

- WP02: Template/config consolidation (V09-V15) -- CLAUDE.md generated in 3 places, config JSON in 4+ places
- WP04: Correctness issues (V20-V21) -- upgrade bypasses install lifecycle, early migrations bypass update_config_version
- WP05: Legacy/minor cleanup (V22-V25) -- intent_main near-duplicates intent, info() helper duplicated

## Conventions

- NO Claude attribution in commit messages -- ever
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- User typically pastes full implementation plans as opening messages
- A markdown linter auto-formats files on save
