# Session Restart Context

## Project

Intent v2.7.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.7.0 tagged and pushed. ST0028 complete.

## Recent Work

- ST0028: TCA v3.0 -- Process Doc Update + Skill Suite
  - Updated `intent/docs/total-codebase-audit.md` from v2.0 to v3.0
  - Created 5 TCA skills: in-tca-init, in-tca-audit, in-tca-synthesize, in-tca-remediate, in-tca-finish
  - 3 automation scripts: tca-init.sh, tca-progress.sh, tca-report.sh

## TODO

None pending.

## Key Files

- `intent/plugins/claude/skills/in-tca-*/` -- TCA skill suite (5 skills, 3 scripts)
- `intent/docs/total-codebase-audit.md` -- TCA v3.0 reference doc
- `intent/st/ST0028/` -- completed ST with all docs
- `intent/wip.md` -- current state tracker

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
