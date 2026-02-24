# Tasks - ST0025: Fix Highlander Violations

## Tasks

- [x] WP01: Extract shared helper functions to intent_helpers (V01-V08)
- [ ] WP02: Consolidate template and config generation (V09-V15) [DEFERRED]
- [ ] WP03: Deduplicate plugin scripts (V16-V19) [DEFERRED]
- [ ] WP04: Fix correctness issues (V20-V21) [DEFERRED]
- [ ] WP05: Legacy and minor cleanup (V22-V25) [DEFERRED]
- [ ] WP06: Version bump, CHANGELOG, test validation, release [DEFERRED]

## Work Package Details

### WP01: Extract shared helpers [COMPLETE]

Extract to `bin/intent_helpers`, delete from consumers:

- [x] `error()` -- deleted from 10 scripts, added to intent_helpers with export -f (V01)
- [ ] INTENT_HOME -- deferred, guard pattern is harmless safety net (V02)
- [x] `calculate_checksum()` -- extracted from subagents/skills/upgrade (V03)
- [x] `get_terminal_width()` -- extracted from subagents/skills/st (V04)
- [x] `require_jq()` -- extracted from config/subagents/skills (V05)
- [x] `require_claude()` -- extracted from subagents/skills (V06)
- [x] `require_project_root()` -- extracted from intent_agents 3x (V07)
- [x] `find_project_root()` -- removed duplicate from fileindex (V08)
- [x] Run full test suite -- 318 tests passing

### WP02-WP06: Deferred

Remaining violations (V09-V25) are lower priority. WP01 resolved the highest-impact duplications. The remaining work can be addressed in a future steel thread when needed.

## Dependencies

- WP01 complete -- shared helpers now available for future WP02-WP05 work
