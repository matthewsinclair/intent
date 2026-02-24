---
verblock: "24 Feb 2026:v0.9: matts - ST0023 done, ST0025 documented"
intent_version: 2.5.0
---

# Work In Progress

## Current State

v2.5.0 released. ST0023 (Remove Backlog from Intent) completed:

- Removed all Backlog.md integration: CLI commands, config, docs, templates, subagents, tests, CI
- Deleted 5 bin scripts, 3 test files, all backlog directories
- Simplified CI pipeline (no more Node.js dependency)
- Consolidated duplicate version/intent_version config fields
- Fixed test side-effect: agent_commands.bats no longer modifies real source files (sandbox approach)
- Highlander Rule audit: 25 violations identified, documented in ST0025
- 318 tests passing across 14 test files

Previous releases:

- ST0020 (Elixir support), ST0021 (Autopsy), ST0022 (st new hardening) -- all completed in v2.4.0

## Active Steel Threads

- ST0024: Add work packages as first-class citizens (WIP, not started)
- ST0025: Fix Highlander Violations (WIP, documented, not started)

## What's Next

1. ST0025: Fix Highlander Rule violations (25 identified, 6 work packages planned)
2. ST0024: Work packages as first-class citizens within steel threads
3. Review ST0010 and ST0015 to decide if still relevant

## Key References

- Steel threads: `intent/st/` (ST0024, ST0025 active)
- Parked: `intent/st/NOT-STARTED/` (ST0010, ST0015)
- Changelog: `CHANGELOG.md`
- Documentation: `intent/usr/user_guide.md`, `intent/usr/reference_guide.md`
- Test suite: `tests/run_tests.sh` (14 .bats files, 318 tests)
- Skills source: `intent/plugins/claude/skills/`
