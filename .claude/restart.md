# Claude Code Session Restart

## WIP

No active steel threads. v2.4.0 released with ST0020, ST0021, ST0022 all completed.

## TODO

1. **Fix subagent sync false-positive** -- `intent claude subagents sync` reports "modified locally" when the source file changed but the installed copy is just stale. Should distinguish "source updated" from "user modified locally". See Known Issues in MEMORY.md.
2. **Review parked STs** -- ST0010 and ST0015 in `intent/st/NOT-STARTED/`. Decide if still relevant or should be cancelled.
3. **Consider v2.5.0 scope** -- review autopsy findings and project needs for next release.

## Key Files

| File                                               | Purpose                                    |
| -------------------------------------------------- | ------------------------------------------ |
| `CHANGELOG.md`                                     | Feature history (v1.0.0 through v2.4.0)    |
| `VERSION`                                          | Current version (2.4.0)                    |
| `intent/wip.md`                                    | Work in progress tracker                   |
| `intent/restart.md`                                | Session restart context                    |
| `intent/autopsy/20260220.md`                       | First autopsy report                       |
| `bin/intent`                                       | Main CLI (GLOBAL_COMMANDS on line 41)      |
| `intent/plugins/claude/bin/intent_claude_skills`   | Skills lifecycle management                |
| `intent/plugins/claude/bin/intent_claude_subagents`| Subagent management (sync bug lives here)  |
| `intent/plugins/claude/bin/intent_claude_upgrade`  | Project upgrade command                    |
| `tests/run_tests.sh`                               | Test runner (17 .bats files)               |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.4.0 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- User typically pastes full implementation plans as opening messages
- A markdown linter auto-formats files on save (table alignment, spacing)
