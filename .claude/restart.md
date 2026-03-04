# Claude Code Session Restart

## WIP

ST0026 (Steel Thread Zero) Phase 1 complete (WP-01 through WP-05 + WP-11). Version bump to v2.6.0 pending. Phase 2 (WP-06 through WP-10) not started.

## TODO

1. **Version bump**: Bump to v2.6.0, update CHANGELOG.md, tag and push to both remotes.
2. **WP-06**: Automated enforcement -- 6 Credo check templates + `intent audit quick` command. See `intent/st/ST0026/WP/06/info.md`.
3. **WP-07**: Health checks + learnings -- `intent audit health` + `intent learn`. See `intent/st/ST0026/WP/07/info.md`.
4. **WP-08**: Guardrails -- `intent modules check` + dependency graph enforcement. See `intent/st/ST0026/WP/08/info.md`.
5. **WP-09**: Retrofit installation -- `intent st zero install` for brownfield projects. See `intent/st/ST0026/WP/09/info.md`.
6. **WP-10**: Integrator command -- `intent init --with-st0000` for greenfield. See `intent/st/ST0026/WP/10/info.md`.

## Key Files

| File                                                 | Purpose                                       |
| ---------------------------------------------------- | --------------------------------------------- |
| `intent/st/ST0026/info.md`                           | Full ST spec (474 lines, 15 deliverables)     |
| `intent/st/ST0026/design.md`                         | Design decisions + as-built deviations        |
| `intent/st/ST0026/impl.md`                           | As-built implementation notes (Phase 1)       |
| `intent/st/ST0026/done.md`                           | Completed Phase 1 tasks                       |
| `intent/st/ST0026/tasks.md`                          | Remaining Phase 2 tasks                       |
| `intent/wip.md`                                      | Work in progress tracker                      |
| `intent/restart.md`                                  | Session restart context                       |
| `intent/llm/MODULES.md`                              | Intent's own module registry (26 modules)     |
| `intent/llm/DECISION_TREE.md`                        | Intent's own code placement guide             |
| `bin/intent`                                         | Main CLI entry point                          |
| `bin/intent_helpers`                                 | Shared helpers                                |
| `intent/plugins/claude/skills/`                      | 11 skills (6 renamed in-* + 5 new workflow)   |
| `intent/plugins/claude/bin/intent_claude_prime`      | Memory injection command (new in Phase 1)     |
| `intent/plugins/claude/lib/claude_plugin_helpers.sh` | Shared plugin callback library                |
| `tests/run_tests.sh`                                 | Test runner (18 .bats files, 365 tests)       |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- Markdown linter auto-formats files on save (table alignment, spacing)
