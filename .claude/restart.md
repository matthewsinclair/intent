# Claude Code Session Restart

## WIP

No active steel threads. Intent v2.3.4 is fully released with documentation current.

## TODO

Possible next actions (check with user for priority):

1. **Parked steel threads** -- review ST0010 and ST0015 to decide if still relevant
   - `intent/st/NOT-STARTED/ST0010/info.md`
   - `intent/st/NOT-STARTED/ST0015/info.md`

2. **Fix subagent sync false-positive** -- `intent claude subagents sync` reports "modified locally" when the source has changed but the installed copy is just stale (not user-modified). The checksum comparison logic is in `intent/plugins/claude/bin/intent_claude_subagents` around lines 485-507. Should distinguish "source updated" from "user modified locally"

3. **New feature work** -- create a new steel thread with `intent st new "Title"`

## Key Files

| File                                              | Purpose                                |
|---------------------------------------------------|----------------------------------------|
| `CHANGELOG.md`                                    | Feature history (v1.0.0 through v2.3.4)|
| `intent/wip.md`                                   | Work in progress tracker               |
| `intent/restart.md`                               | Detailed session restart context       |
| `intent/usr/user_guide.md`                        | User-facing guide (v2.3.4)            |
| `intent/usr/reference_guide.md`                   | Command reference (v2.3.4)            |
| `bin/intent`                                      | Main CLI (GLOBAL_COMMANDS on line 41)  |
| `bin/intent_treeindex`                            | Treeindex CLI command                  |
| `intent/plugins/claude/bin/intent_claude_subagents`| Claude subagent management            |
| `intent/plugins/agents/bin/intent_agents`         | AGENTS.md management                   |
| `tests/run_tests.sh`                              | Test runner (265 tests)                |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.3.4 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Check `intent/.treeindex/<dir>/.treeindex` before exploring unfamiliar directories
