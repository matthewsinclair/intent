# Session Restart Context

## Project

Intent v2.6.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0026 (Steel Thread Zero) complete. All 11 WPs done. 462 tests passing across 22 files. CHANGELOG and docs updated.

## TODO

1. Tag v2.6.0 and push to both remotes (`local` and `upstream`)
2. Create GitHub release notes

## Key Files

- `intent/st/COMPLETED/ST0026/` -- completed ST with all docs
- `intent/llm/MODULES.md` -- Intent's own module registry
- `intent/llm/DECISION_TREE.md` -- Intent's own code placement guide
- `CHANGELOG.md` -- updated with WP-09/WP-10 additions
- `intent/wip.md` -- current state tracker

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
