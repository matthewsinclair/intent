# Claude Code Session Restart

## WIP

**ST0019: Treeindex** -- build `intent treeindex` CLI command and companion Claude Code subagent.

Design is complete. Implementation has not started.

## TODO

1. Read `intent/st/ST0019/info.md` for overview
2. Read `intent/st/ST0019/design.md` for .treeindex format and algorithm
3. Read `intent/st/ST0019/WP/01/info.md` for WP01 spec (CLI command -- start here)
4. Implement WP01: create `bin/intent_treeindex` (use `bin/intent_fileindex` as a reference for bash patterns)
5. Add `treeindex` to `GLOBAL_COMMANDS` in `bin/intent` line 41
6. Implement WP02: create subagent at `intent/plugins/claude/subagents/treeindex/` (agent.md + metadata.json)
7. Implement WP03: register in global-agents.json, update CLAUDE.md
8. Run `tests/run_tests.sh` before committing

## Key Files

| File                                    | Purpose                          |
|-----------------------------------------|----------------------------------|
| `intent/st/ST0019/info.md`             | Steel thread overview            |
| `intent/st/ST0019/design.md`           | Technical design                 |
| `intent/st/ST0019/impl.md`             | Implementation notes             |
| `intent/st/ST0019/tasks.md`            | Task checklist                   |
| `intent/st/ST0019/WP/01/info.md`       | WP01: CLI command spec           |
| `intent/st/ST0019/WP/02/info.md`       | WP02: subagent spec              |
| `intent/st/ST0019/WP/03/info.md`       | WP03: integration spec           |
| `.claude/plans/nested-tinkering-reddy.md` | Full implementation plan      |
| `bin/intent_fileindex`                  | Reference for bash CLI patterns  |
| `bin/intent` (line 41)                  | GLOBAL_COMMANDS registration     |
