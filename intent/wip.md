---
verblock: "24 Feb 2026:v0.10: matts - v2.5.0 released, all STs done"
intent_version: 2.5.0
---

# Work In Progress

## Current State

v2.5.0 released. Four steel threads completed this release cycle:

- ST0023: Remove Backlog from Intent (all 8 WPs, 70+ files changed)
- ST0024: Add work packages as first-class citizens (bin/intent_wp, 29 tests)
- ST0025: Fix Highlander Violations (WP01 shared helpers + WP07 plugin refactoring)
- ST0020/ST0021/ST0022: Completed in v2.4.0

Key changes in v2.5.0:

- `intent wp` command: new, done, start, list, show, help
- Shared plugin callback library (`claude_plugin_helpers.sh`)
- Plugin scripts reduced: skills 654->299 lines, subagents 1015->613 lines
- 8 shared helpers extracted to `bin/intent_helpers`
- `get_config_field()` replaces inline grep patterns
- 339 tests passing across 17 test files

## Active Steel Threads

None. All steel threads completed or parked.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`

## What's Next

1. Review ST0010 and ST0015 to decide if still relevant or should be cancelled
2. ST0025 deferred work: WP02-WP05 (template consolidation, correctness fixes, legacy cleanup)
3. Consider new features for v2.6.0

## Key References

- Steel threads: `intent/st/COMPLETED/` (ST0020-ST0025)
- Parked: `intent/st/NOT-STARTED/` (ST0010, ST0015)
- Changelog: `CHANGELOG.md`
- Release notes: `docs/releases/2.5.0/RELEASE_NOTES.md`
- Documentation: `intent/usr/user_guide.md`, `intent/usr/reference_guide.md`
- Test suite: `tests/run_tests.sh` (17 .bats files, 339 tests)
- Skills source: `intent/plugins/claude/skills/`
- Plugin shared lib: `intent/plugins/claude/lib/claude_plugin_helpers.sh`
