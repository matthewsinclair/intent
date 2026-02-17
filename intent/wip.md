---
verblock: "17 Feb 2026:v0.4: matts - v2.4.0 upgrade complete"
intent_version: 2.4.0
---
# Work In Progress

## Current State

ST0020 complete. Intent v2.4.0 is released with all projects upgraded.

### Recently Completed

**ST0020: Modernizing Elixir Support** -- Completed 2026-02-17
- Skills system: `intent claude skills` CLI command (install/list/sync/uninstall/show)
- Three Elixir skills: elixir-essentials, ash-ecto-essentials, phoenix-liveview
- Upgrade command: `intent claude upgrade` with dry-run/apply modes
- Four new reference docs: ash-ecto.md, liveview.md, testing.md, project-structure.md
- Elixir RULES.md + ARCHITECTURE.md templates for `intent agents init --template elixir`
- usage-rules.md for Intent itself
- 37 new BATS tests (292 total across 15 files)
- Refactored agent.md from 23 overlapping rules to 12 non-overlapping
- Rolled out upgrade to 8 projects (Intent, Prolix, Laksa-web, Lamplight, Anvil, MeetZaya, Multiplyer, Utilz)

**ST0019: Treeindex** -- Completed 2026-02-04
- `intent treeindex` CLI command (612 lines, bash 3.2 compatible)
- Shadow directory at `intent/.treeindex/`, fingerprint-based staleness
- 53 bats tests

### Not Started

- **ST0010**: (parked, not started)
- **ST0015**: (parked, not started)

## What's Next

Possible directions:
1. Pick up ST0010 or ST0015 if still relevant
2. Tag v2.4.0 and push to remotes, create GitHub release
3. Update user_guide.md and reference_guide.md for v2.4.0 (new commands: skills, upgrade)
4. Regenerate .treeindex files for new/changed directories
5. Address subagent sync false-positive for "modified locally" detection

## Key References

- Steel threads: `intent/st/` (ST0020 most recent)
- Changelog: `CHANGELOG.md`
- Documentation: `intent/usr/user_guide.md`, `intent/usr/reference_guide.md`
- Test suite: `tests/run_tests.sh` (302 tests across 15 files)
- Skills source: `intent/plugins/claude/skills/`
- Upgrade command: `intent/plugins/claude/bin/intent_claude_upgrade`
