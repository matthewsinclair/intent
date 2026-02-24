# Tasks - ST0025: Fix Highlander Violations

## Done

- [x] WP01: Extract shared helper functions to intent_helpers (V01-V08)
- [x] WP07: Plugin script refactoring via shared callback library (V16-V18)

## Deferred

- [ ] WP02: Consolidate template and config generation (V09-V15)
- [ ] WP04: Fix correctness issues (V20-V21)
- [ ] WP05: Legacy and minor cleanup (V22-V25)

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

### WP07: Plugin script refactoring [COMPLETE]

Shared callback library for install/sync/uninstall operations:

- [x] Create `intent/plugins/claude/lib/claude_plugin_helpers.sh` (~400 lines)
- [x] Refactor `intent_claude_skills` (654 -> 299 lines) with 8 callbacks
- [x] Refactor `intent_claude_subagents` (1015 -> 613 lines) with 8 callbacks
- [x] Add `get_config_field()` to `bin/intent_helpers`
- [x] Update `intent_st` and `intent_wp` to use `get_config_field()`
- [x] Update test sandbox to include lib/ directory
- [x] Run full test suite -- 339 tests passing

### WP02-WP05: Deferred

Remaining violations are lower priority. WP01 and WP07 resolved the highest-impact duplications (~500 lines removed total). The remaining work can be addressed in future steel threads when needed.
