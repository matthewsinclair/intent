# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- fully elaborated, 7 WPs, ready for WP-01. 22 skills, 5 subagents, 462 tests.

## TODO

ST0031/WP-01: Define ACI format and extraction protocol.

- Create ACI template in `docs/course/templates/aci-template.md`
- Hand-craft 3-5 ACIs from known lessons
- Document 6 extraction lenses with specific commands
- Decide on autopsy script reuse for correction mining

Then WP-02 (pilot: Intent + Lamplight + MeetZaya) and WP-03 (landscape research) run in parallel.

## Key Files

| File                                        | Purpose                                   |
| ------------------------------------------- | ----------------------------------------- |
| `intent/st/ST0031/`                         | Steel thread (info, design, tasks, 7 WPs) |
| `docs/course/`                              | Course content directory (empty)          |
| `~/.claude/plans/temporal-wishing-panda.md` | Full plan with all design details         |
| `intent/wip.md`                             | Work in progress tracker                  |
| `intent/restart.md`                         | Session restart context                   |
| `intent/done.md`                            | Completed work log                        |

## Project Conventions

- ALWAYS use `intent` CLI for ST/WP operations (never manual file moves)
- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- macOS bash 3.x: no `declare -A`, no `${VAR^}` -- use explicit alternatives
