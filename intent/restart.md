# Session Restart Context

## Project

Intent v2.8.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.8.0 committed and pushed. No active steel threads. 19 skills, 5 subagents.

## Recent Work

- v2.8.0: /in-handoff skill + skills list display fixes (2026-03-31)
  - Created `/in-handoff` skill for permanent session handoff documents (ST0029)
  - Handoff docs stored at `intent/.handoff/YYYYMMDD-NNN-<slug>.md`
  - Fixed skills list: dynamic name column, terminal-width-aware, compact format
  - Fixed `get_terminal_width()` fallback chain
- v2.8.0: Detrope skill + blog remediation (2026-03-28)
- ST0028: TCA v3.0 (completed 2026-03-19)

## TODO

- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs (intent/docs/\*.md)

## Key Files

- `intent/plugins/claude/skills/in-handoff/` -- handoff skill (SKILL.md + handoff-prep.sh)
- `intent/.handoff/` -- handoff document archive
- `intent/wip.md` -- current state tracker

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
