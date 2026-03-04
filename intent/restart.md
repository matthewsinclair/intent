# Session Restart Context

## Project

Intent v2.5.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0026 (Steel Thread Zero) Phase 1 complete. Version bump to v2.6.0 pending.

## TODO

### Immediate: Version Bump

- Bump version to v2.6.0 in relevant files
- Update CHANGELOG.md with Phase 1 deliverables
- Tag and push to both remotes (`local` + `upstream`)

### Phase 2: ST0026 WP-06 through WP-10

| WP    | Title                    | Key deliverables                               |
| ----- | ------------------------ | ---------------------------------------------- |
| WP-06 | Automated Enforcement    | 6 Credo check templates + `intent audit quick` |
| WP-07 | Health Check & Learnings | `intent audit health` + `intent learn`         |
| WP-08 | Guardrails               | `intent modules check` + dependency graph      |
| WP-09 | Retrofit Installation    | `intent st zero install` (brownfield)          |
| WP-10 | Integrator Command       | `intent init --with-st0000` (greenfield)       |

Dependencies: WP-06 -> WP-07, WP-06 -> WP-08, WP-06+03+04 -> WP-09 -> WP-10

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
- Run `tests/run_tests.sh` before committing (365 tests across 18 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
