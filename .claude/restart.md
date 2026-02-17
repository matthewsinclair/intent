# Claude Code Session Restart

## WIP

None. v2.4.0 is released.

## TODO

1. **Parked steel threads** -- review ST0010 and ST0015
2. **Regenerate .treeindex** -- new directories added (intent-essentials skill)
3. **Fix subagent sync false-positive** -- reports "modified locally" when source changed but installed copy is just stale

## Key Files

| File                                               | Purpose                                |
|----------------------------------------------------|----------------------------------------|
| `CHANGELOG.md`                                     | Feature history (v1.0.0 through v2.4.0)|
| `VERSION`                                          | Current version (2.4.0)                |
| `intent/wip.md`                                    | Work in progress tracker               |
| `intent/restart.md`                                | Session restart context                |
| `bin/intent`                                       | Main CLI (GLOBAL_COMMANDS on line 41)  |
| `intent/plugins/claude/bin/intent_claude_skills`   | Skills lifecycle management            |
| `intent/plugins/claude/bin/intent_claude_subagents`| Claude subagent management             |
| `intent/plugins/claude/bin/intent_claude_upgrade`  | Project upgrade command                |
| `tests/run_tests.sh`                               | Test runner (302 tests, 15 files)      |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.4.0 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
