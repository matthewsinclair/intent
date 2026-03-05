# Session Restart Context

## Project

Intent v2.6.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0026 (Steel Thread Zero) Phase 2 in progress. WP-06 and WP-07 complete. WP-08 next.

## TODO

### Next: WP-08 (Guardrails)

- Create `bin/intent_modules` command script
- Implement `intent modules check` (scan modules vs MODULES.md)
- Implement `intent modules check --register` (auto-register missing)
- Implement `intent modules find` (locate module by concern)
- Create Claude Code hook for `Write` to `lib/**/*.ex`
- Create `lib/templates/llm/_DEPENDENCY_GRAPH.md` template
- Implement dependency graph check (scan alias/import/use vs declared deps)
- Integrate with `intent audit quick`

### Remaining Phase 2: WP-09 through WP-10

| WP    | Title                 | Key deliverables                          |
| ----- | --------------------- | ----------------------------------------- |
| WP-09 | Retrofit Installation | `intent st zero install` (brownfield)     |
| WP-10 | Integrator Command    | `intent init --with-st0000` (greenfield)  |

### Documentation

- Comprehensive update: README, CHANGELOG, blog posts
- New blog post for agent/claude work (rather than updating existing posts)

Dependencies: WP-06+03+04 -> WP-09 -> WP-10

See `intent/st/ST0026/tasks.md` for detailed task lists.

## Key Files

- `intent/st/ST0026/info.md` -- full ST spec (474 lines, 15 deliverables)
- `intent/st/ST0026/design.md` -- design decisions + as-built deviations
- `intent/st/ST0026/impl.md` -- as-built implementation notes
- `intent/st/ST0026/done.md` -- completed Phase 1 tasks
- `intent/st/ST0026/tasks.md` -- remaining Phase 2 tasks
- `intent/llm/MODULES.md` -- Intent's own module registry
- `intent/llm/DECISION_TREE.md` -- Intent's own code placement guide

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (407 tests across 20 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
