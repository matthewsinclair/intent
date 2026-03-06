# Session Restart Context

## Project

Intent v2.6.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0027 complete. New `in-cost-analysis` skill added (12th skill, 12 total). v2.6.0 tagged and pushed.

## Recent Work (ST0027)

- `/in-cost-analysis` skill: estimates development cost of reproducing a codebase
- Includes agentic leverage ratio (human-equivalent hours / agentic hours)
- `intent/analysis/` convention established for dated cost analysis reports
- Generated analyses for Intent (10.5x leverage), Lamplight (83.2x), laksa-web (97.1x), Conflab (88.0x)

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
