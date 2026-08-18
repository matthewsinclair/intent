# Tasks - ST0023: Remove Backlog from Intent

## Tasks

- [x] WP01: Remove Backlog from documentation (docs, TPD, blogs)
- [x] WP02: Remove Backlog from LLM templates
- [x] WP03: Remove Backlog from subagent definitions
- [x] WP04: Remove Backlog CLI commands and config from bin/
- [x] WP05: Remove Backlog config keys from JSON files
- [x] WP06: Remove/update test files
- [x] WP07: CI/CD, examples, and cleanup
- [x] WP08: Version bump, CHANGELOG, treeindex, release

## Additional Work

- [x] Consolidated duplicate `version`/`intent_version` config fields to just `intent_version`
- [x] Cleaned global agent manifest description
- [x] Cleaned stale STP/backlog entries from .claude/settings.local.json
- [x] Fixed test side-effect: tests modifying real source agent.md (sandbox approach)
- [x] Highlander Rule audit: identified 25 violations, documented in ST0025

## Dependencies

- WP01 completed before WP02-WP08
- WP04+WP05+WP06 executed as group
- WP08 last (depends on all others)
