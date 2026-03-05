# Claude Code Session Restart

## WIP

ST0026 (Steel Thread Zero) Phase 2 in progress. WP-06 done (audit command + 6 Credo check templates). WP-07 next.

## TODO

1. **WP-07**: Health checks + learnings -- `intent audit health` + `intent learn`. See `intent/st/ST0026/WP/07/info.md`.
2. **WP-08**: Guardrails -- `intent modules check` + dependency graph enforcement. See `intent/st/ST0026/WP/08/info.md`.
3. **WP-09**: Retrofit installation -- `intent st zero install` for brownfield projects. See `intent/st/ST0026/WP/09/info.md`.
4. **WP-10**: Integrator command -- `intent init --with-st0000` for greenfield. See `intent/st/ST0026/WP/10/info.md`.
5. **Release**: Update CHANGELOG.md, tag v2.6.0 and push to both remotes when Phase 2 complete.

## Key Files

| File                                                 | Purpose                                       |
| ---------------------------------------------------- | --------------------------------------------- |
| `intent/st/ST0026/info.md`                           | Full ST spec (474 lines, 15 deliverables)     |
| `intent/st/ST0026/design.md`                         | Design decisions + as-built deviations        |
| `intent/st/ST0026/impl.md`                           | As-built implementation notes (Phase 1+2)     |
| `intent/st/ST0026/done.md`                           | Completed Phase 1 tasks                       |
| `intent/st/ST0026/tasks.md`                          | Phase 2 tasks (WP-06 done, WP-07..10 remain)  |
| `intent/wip.md`                                      | Work in progress tracker                      |
| `intent/restart.md`                                  | Session restart context                       |
| `intent/llm/MODULES.md`                              | Intent's own module registry                  |
| `bin/intent`                                         | Main CLI entry point                          |
| `bin/intent_helpers`                                 | Shared helpers                                |
| `bin/intent_audit`                                   | Audit command (new in WP-06)                  |
| `lib/templates/credo_checks/elixir/`                 | 6 Credo check templates (new in WP-06)        |
| `intent/plugins/claude/skills/`                      | 11 skills (6 renamed in-* + 5 workflow)       |
| `intent/plugins/claude/bin/intent_claude_prime`      | Memory injection command                      |
| `intent/plugins/claude/lib/claude_plugin_helpers.sh` | Shared plugin callback library                |
| `tests/run_tests.sh`                                 | Test runner (19 .bats files, 382 tests)       |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- Markdown linter auto-formats files on save (table alignment, spacing)
- macOS bash 3.x: no `declare -A`, no `${VAR^}` -- use explicit alternatives
