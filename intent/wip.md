---
verblock: "28 Mar 2026:v0.22: matts - Detrope skill + blog remediation"
intent_version: 2.8.0
---

# Work In Progress

## Current State

v2.8.0 with detrope skill and blog series remediation. Committed and pushed to both remotes.

## This Session

- Created [llm-tropes](https://github.com/matthewsinclair/llm-tropes) repo (44 tropes, 8 categories)
- Created `/in-detrope` skill (SKILL.md + vendored trope-catalog.md)
- Added `cleanz --detrope` to Utilz for automated trope detection
- Detroped all 8 blog posts in docs/blog/
- Version bumped to v2.8.0, all docs updated

## Active Steel Threads

None.

## TODO

- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs (intent/docs/\*.md)

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`

## Key References

- llm-tropes repo: `https://github.com/matthewsinclair/llm-tropes`
- Detrope skill: `intent/plugins/claude/skills/in-detrope/`
- Blog posts: `docs/blog/`
- Test suite: `tests/run_tests.sh` (22 .bats files, 462 tests)
