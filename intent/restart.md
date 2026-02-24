# Session Restart Context

## Project

Intent v2.5.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.5.0 released. Two completed steel threads this session:

- ST0023: Remove Backlog from Intent (all 8 WPs done, 70+ files changed)
- ST0025: Fix Highlander Violations (audit done, plan documented, implementation pending)

Key changes in v2.5.0:

- Deleted CLI commands: bl, task, status, migrate, backlog
- Removed Node.js from CI pipeline
- Cleaned all docs, templates, subagents, examples
- Consolidated version/intent_version config fields
- Fixed test side-effect (sandbox approach in agent_commands.bats)
- Test suite: 14 files, 318 tests, all passing

## Active Steel Threads

- ST0024: Add work packages as first-class citizens (not started)
- ST0025: Fix Highlander Violations (25 violations documented, 6 WPs planned)

## What's Next

1. ST0025 implementation: Extract shared helpers, consolidate templates, dedup plugins
2. ST0024: Work packages as first-class citizens
3. Review ST0010 and ST0015

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
- Tests: `tests/unit/` (14 .bats files), run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
- NO Claude attribution in commit messages
- Never use em dashes in skill files (multi-byte truncation bugs)
