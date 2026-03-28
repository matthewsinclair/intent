# Session Restart Context

## Project

Intent v2.8.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.8.0 committed and pushed. No active steel threads.

## Recent Work

- v2.8.0: Detrope skill + blog remediation (2026-03-28)
  - Created [llm-tropes](https://github.com/matthewsinclair/llm-tropes) repo (44 tropes, 8 categories)
  - Created `/in-detrope` skill for LLM trope detection and stylometric analysis
  - Added `cleanz --detrope` to Utilz for automated mechanical detection
  - Detroped all 8 blog posts in docs/blog/
  - 18 skills total, 5 subagents
- ST0028: TCA v3.0 (completed 2026-03-19)

## TODO

- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs (intent/docs/\*.md)

## Key Files

- `intent/plugins/claude/skills/in-detrope/` -- detrope skill (SKILL.md + trope-catalog)
- `docs/blog/` -- detroped blog series (8 posts)
- `intent/wip.md` -- current state tracker

## Conventions

- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
