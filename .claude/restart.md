# Claude Code Session Restart

## WIP

ST0026 (Steel Thread Zero) is active. Fully elaborated with 11 work packages, 15 deliverables, 80+ tasks. No code written yet -- planning phase complete, ready for implementation.

## TODO

1. **Start WP-01**: Rename 6 skills from `intent-*` to `in-*` prefix. See `intent/st/ST0026/WP/01/info.md`.
2. **Then WP-02**: Create 5 new workflow skills (/in-start, /in-plan, /in-standards, /in-next, /in-finish). See `intent/st/ST0026/WP/02/info.md`.
3. **Parallel WP-03**: LLM templates (CLAUDE.md, MODULES.md, DECISION_TREE.md). See `intent/st/ST0026/WP/03/info.md`.
4. **Parallel WP-05**: Archetype templates (9 Elixir module templates). See `intent/st/ST0026/WP/05/info.md`.
5. **Then WP-04**: Memory injection (`intent claude prime`). See `intent/st/ST0026/WP/04/info.md`.
6. **Anytime WP-11**: Port TN004 tech note from laksa-web. See `intent/st/ST0026/WP/11/info.md`.

## Key Files

| File                                                 | Purpose                                    |
| ---------------------------------------------------- | ------------------------------------------ |
| `intent/st/ST0026/info.md`                           | Full ST spec (472 lines, 15 deliverables)  |
| `intent/st/ST0026/design.md`                         | Design (7 decisions, dependency graph)     |
| `intent/st/ST0026/tasks.md`                          | Task breakdown (80+ tasks by WP)           |
| `intent/st/ST0026/impl.md`                           | Implementation tracking                    |
| `intent/st/ST0026/WP/*/info.md`                      | 11 work package specs                      |
| `intent/wip.md`                                      | Work in progress tracker                   |
| `intent/restart.md`                                  | Session restart context                    |
| `bin/intent`                                         | Main CLI (GLOBAL_COMMANDS on line 41)      |
| `bin/intent_helpers`                                 | Shared helpers                             |
| `intent/plugins/claude/skills/`                      | Skill source dirs (6 skills, intent-* prefix) |
| `intent/plugins/claude/bin/intent_claude_skills`     | Skills lifecycle (299 lines)               |
| `intent/plugins/claude/lib/claude_plugin_helpers.sh` | Shared plugin callback library             |
| `tests/run_tests.sh`                                 | Test runner (17 .bats files, 339 tests)    |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Never use em dashes in skill files -- multi-byte truncation in list display
- Markdown linter auto-formats files on save (table alignment, spacing)
