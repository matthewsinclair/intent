# Claude Code Session Restart

## WIP

v2.7.0 -- TCA v3.0 + 5 TCA skills. 17 skills, 5 subagents. No active work.

## TODO

- Consider peer language skills (in-rust-essentials, in-swift-essentials)

## Key Files

| File                                     | Purpose                                 |
| ---------------------------------------- | --------------------------------------- |
| `intent/plugins/claude/skills/in-tca-*/` | TCA skill suite (5 skills, 3 scripts)   |
| `intent/docs/total-codebase-audit.md`    | TCA v3.0 reference doc                  |
| `intent/st/ST0028/`                      | Completed ST with all docs              |
| `intent/wip.md`                          | Work in progress tracker                |
| `intent/restart.md`                      | Session restart context                 |
| `tests/run_tests.sh`                     | Test runner (22 .bats files, 462 tests) |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- Markdown linter auto-formats files on save (table alignment, spacing)
- macOS bash 3.x: no `declare -A`, no `${VAR^}` -- use explicit alternatives
