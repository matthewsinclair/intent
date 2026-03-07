# Session Restart Context

## Project

Intent v2.6.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.6.0 tagged and pushed. No active steel threads.

## Recent Work

- Bugfix: Credo checks target moved from `lib/mix/checks/` to `credo_checks/` (outside `lib/`)
- Prevents prod compile failures since Credo is a dev/test-only dependency
- Added `elixirc_paths` hint on first directory creation

## TODO

None pending.

## Key Files

- `intent/plugins/claude/skills/in-cost-analysis/` -- skill source (SKILL.md, scripts/, data/)
- `intent/analysis/` -- cost analysis reports
- `intent/st/COMPLETED/ST0027/` -- completed ST with all docs
- `intent/wip.md` -- current state tracker

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
