# Session Restart Context

## Project

Intent v2.3.3 -- a CLI tool for managing steel threads and project documentation. Written in bash. Located at `/Users/matts/Devel/prj/Intent/`.

## Current Work

**ST0019: Treeindex** -- design complete, implementation pending.

Read these files in order to get up to speed:

1. `intent/st/ST0019/info.md` -- objective and solution overview
2. `intent/st/ST0019/design.md` -- .treeindex format, fingerprint, algorithm
3. `intent/st/ST0019/tasks.md` -- task breakdown and dependencies
4. `intent/st/ST0019/WP/01/info.md` -- WP01: CLI command spec
5. `intent/st/ST0019/WP/02/info.md` -- WP02: subagent spec
6. `intent/st/ST0019/WP/03/info.md` -- WP03: integration spec

## Next Action

Start WP01: create `bin/intent_treeindex`. Reference `bin/intent_fileindex` for patterns (argument parsing, project detection, error handling). The plan file at `.claude/plans/nested-tinkering-reddy.md` has the full implementation plan.

## Key Patterns

- Commands live in `bin/intent_<name>` and are auto-routed by `bin/intent`
- Global commands are listed in `GLOBAL_COMMANDS` on line 41 of `bin/intent`
- Subagents live in `intent/plugins/claude/subagents/<name>/` with agent.md + metadata.json
- Tests are in `tests/` and run with `tests/run_tests.sh` (bats framework)
- Always run tests before committing
