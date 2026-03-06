# Claude Code Session Restart

## WIP

v2.6.0 complete. ST0026 done (moved to COMPLETED). All docs updated.

## TODO

1. Tag v2.6.0 and push to both remotes
2. Create GitHub release notes

## Key Files

| File                                | Purpose                                  |
| ----------------------------------- | ---------------------------------------- |
| `intent/st/COMPLETED/ST0026/`       | Completed ST with all docs               |
| `intent/wip.md`                     | Work in progress tracker                 |
| `intent/restart.md`                 | Session restart context                  |
| `CHANGELOG.md`                      | Updated with all WP-09/WP-10 additions   |
| `intent/llm/MODULES.md`            | Intent's own module registry             |
| `bin/intent_st_zero`               | ST Zero retrofit command (new in WP-09)  |
| `bin/intent_audit`                 | Audit command (umbrella-aware)           |
| `tests/run_tests.sh`              | Test runner (22 .bats files, 462 tests)  |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- Markdown linter auto-formats files on save (table alignment, spacing)
- macOS bash 3.x: no `declare -A`, no `${VAR^}` -- use explicit alternatives
