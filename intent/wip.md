---
verblock: "20 Feb 2026:v0.6: matts - ST0022 implemented"
intent_version: 2.4.0
---

# Work In Progress

## Current State

ST0022 (Harden `st new`) implemented: special character escaping, slug generation, and `--start` flag. All WPs complete, 327 tests passing across 16 files.

## What's Next

1. Commit as-built docs and push
2. Bump tag to include ST0022 changes
3. Regenerate .treeindex files for changed directories
4. Address subagent sync false-positive for "modified locally" detection
5. Review ST0010 and ST0015 if still relevant

## Key References

- Steel threads: `intent/st/` (ST0022 most recent active)
- Changelog: `CHANGELOG.md`
- Documentation: `intent/usr/user_guide.md`, `intent/usr/reference_guide.md`
- Test suite: `tests/run_tests.sh` (327 tests across 16 files)
- Skills source: `intent/plugins/claude/skills/`
- Upgrade command: `intent/plugins/claude/bin/intent_claude_upgrade`
