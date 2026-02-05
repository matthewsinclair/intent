# Session Restart Context

## Project

Intent v2.3.4 -- a CLI tool for managing steel threads and project documentation. Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

No active work. All steel threads complete or parked. Documentation is current at v2.3.4. GitHub release v2.3.4 is live.

## Parked Steel Threads

- **ST0010**: `intent/st/NOT-STARTED/ST0010/info.md`
- **ST0015**: `intent/st/NOT-STARTED/ST0015/info.md`

## Possible Next Work

1. Review and pick up ST0010 or ST0015
2. Fix subagent sync false-positive: `intent claude subagents sync` incorrectly reports "modified locally" when the source has changed but the installed file hasn't (version mismatch, not user modification). Logic is in `intent/plugins/claude/bin/intent_claude_subagents` around lines 485-507
3. New feature work

## Key Patterns

- Commands live in `bin/intent_<name>` and are auto-routed by `bin/intent`
- Global commands are listed in `GLOBAL_COMMANDS` on line 41 of `bin/intent`
- Plugin commands: `intent agents` -> `intent/plugins/agents/bin/intent_agents`
- Plugin commands: `intent claude subagents` -> `intent/plugins/claude/bin/intent_claude_subagents`
- Subagent definitions: `intent/plugins/claude/subagents/<name>/`
- Tests are in `tests/unit/` (14 .bats files, 265 tests) and run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
- Always run tests before committing
- NO Claude attribution in commit messages
