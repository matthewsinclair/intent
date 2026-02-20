# Claude Code Session Restart

## WIP

ST0022 implemented (special chars, slugs, --start flag). Needs tag bump and push.

## TODO

1. **Tag bump** -- `git tag -f v2.4.0 HEAD` then force-push to both remotes
2. **Regenerate .treeindex** -- directories changed by ST0022
3. **Fix subagent sync false-positive** -- reports "modified locally" when source changed but installed copy is just stale
4. **Parked steel threads** -- review ST0010 and ST0015

## Key Files

| File                                               | Purpose                                |
|----------------------------------------------------|----------------------------------------|
| `CHANGELOG.md`                                     | Feature history (v1.0.0 through v2.4.0)|
| `VERSION`                                          | Current version (2.4.0)                |
| `bin/intent_st`                                    | Steel thread commands (modified by ST0022)|
| `intent/wip.md`                                    | Work in progress tracker               |
| `intent/restart.md`                                | Session restart context                |
| `bin/intent`                                       | Main CLI (GLOBAL_COMMANDS on line 41)  |
| `intent/plugins/claude/bin/intent_claude_skills`   | Skills lifecycle management            |
| `intent/plugins/claude/bin/intent_claude_subagents`| Claude subagent management             |
| `intent/plugins/claude/bin/intent_claude_upgrade`  | Project upgrade command                |
| `tests/run_tests.sh`                               | Test runner (327 tests, 16 files)      |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.4.0 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
