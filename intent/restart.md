# Session Restart Context

## Project

Intent v2.4.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.4.0 released and tagged. All steel threads completed:

- ST0020: Modernizing Elixir support -- 6 skills, 5 subagents, 8 project upgrades
- ST0021: Intent Autopsy -- Elixir session analysis + memory meta-learning
- ST0022: Harden `st new` -- special char escaping, slug generation, --start flag

No active steel threads. ST0010 and ST0015 parked in NOT-STARTED/.

## What's Next

1. Fix subagent sync false-positive ("modified locally" when source changed)
2. Review ST0010 and ST0015 to decide if still relevant
3. Consider v2.5.0 scope

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
- Tests: `tests/unit/` (17 .bats files), run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
- NO Claude attribution in commit messages
- Never use em dashes in skill files (multi-byte truncation bugs)
