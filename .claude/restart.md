# Claude Code Session Restart

## WIP

No active steel threads. v2.5.0 released with ST0023, ST0024, ST0025 all completed. 339 tests passing across 17 BATS test files.

## TODO

1. **Review parked STs** -- ST0010 and ST0015 in `intent/st/NOT-STARTED/`. Decide if still relevant or should be cancelled.
2. **ST0025 deferred violations** -- Template consolidation (CLAUDE.md generated in 3 places, config JSON in 4+ places), correctness fixes (upgrade bypasses install lifecycle), legacy cleanup. See `intent/st/COMPLETED/ST0025/design.md` for full audit.
3. **Consider v2.6.0 scope** -- Review project needs and decide on next features.

## Key Files

| File                                                | Purpose                                               |
| --------------------------------------------------- | ----------------------------------------------------- |
| `CHANGELOG.md`                                      | Feature history (v1.0.0 through v2.5.0)               |
| `VERSION`                                           | Current version (2.5.0)                               |
| `intent/wip.md`                                     | Work in progress tracker                              |
| `intent/restart.md`                                 | Session restart context                               |
| `bin/intent`                                        | Main CLI (GLOBAL_COMMANDS on line 41)                 |
| `bin/intent_helpers`                                | Shared helpers (error, checksum, require_jq, etc.)    |
| `bin/intent_wp`                                     | Work package management                               |
| `intent/plugins/claude/lib/claude_plugin_helpers.sh` | Shared plugin callback library                        |
| `intent/plugins/claude/bin/intent_claude_skills`    | Skills lifecycle (299 lines, uses callback pattern)   |
| `intent/plugins/claude/bin/intent_claude_subagents` | Subagent management (613 lines, uses callback pattern)|
| `intent/plugins/claude/bin/intent_claude_upgrade`   | Project upgrade command                               |
| `intent/st/COMPLETED/ST0025/design.md`              | Full Highlander audit with 25 violations              |
| `tests/run_tests.sh`                                | Test runner (17 .bats files, 339 tests)               |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.5.0 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- User typically pastes full implementation plans as opening messages
- A markdown linter auto-formats files on save (table alignment, spacing)
