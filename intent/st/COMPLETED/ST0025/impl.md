# Implementation - ST0025: Fix Highlander Violations

## Implementation Log

### Pre-work (during ST0023)

**Test side-effect fix:** Tests in `agent_commands.bats` were modifying the real source file at `$INTENT_HOME/intent/plugins/claude/subagents/intent/agent.md` and using `git checkout` in teardown to "undo" changes -- which reverted ALL uncommitted edits.

Fixed by:

1. Added `create_source_sandbox()` helper that creates a temp INTENT_HOME with copied subagent sources and symlinked bin/ scripts
2. Three tests that simulate source changes now use the sandbox exclusively
3. Removed `git checkout` from teardown; added INTENT_HOME save/restore
4. All 318 tests pass; real source files are never modified

### WP01: Extract shared helpers (24 Feb 2026)

**7 shared functions added to `bin/intent_helpers`** (top of file, before migration functions), all with `export -f`:

| Function                 | Replaces                                                                                              |
| ------------------------ | ----------------------------------------------------------------------------------------------------- |
| `error()`                | Identical definition in 10 scripts                                                                    |
| `calculate_checksum()`   | `calculate_checksum()` in subagents, `calculate_skill_checksum()` in skills, inline shasum in upgrade |
| `get_terminal_width()`   | 6-line terminal width detection blocks in subagents, skills, intent_st                                |
| `require_jq()`           | 8 inline jq guard blocks with inconsistent error messages                                             |
| `require_claude()`       | 6 inline Claude detection blocks                                                                      |
| `require_project_root()` | 3 identical 8-line blocks in intent_agents                                                            |
| (sourcing fix)           | Duplicate `find_project_root()` in fileindex                                                          |

**16 files changed:**

- `bin/intent_helpers` -- added 7 shared functions
- `bin/intent` -- deleted local `error()`
- `bin/intent_config` -- deleted `error()` + export, replaced 2 jq guards with `require_jq`
- `bin/intent_init` -- deleted local `error()`
- `bin/intent_help` -- deleted local `error()`
- `bin/intent_main` -- deleted local `error()`
- `bin/intent_upgrade` -- deleted local `error()`, replaced inline shasum with `calculate_checksum()`
- `bin/intent_treeindex` -- deleted local `error()`
- `bin/intent_st` -- added intent_helpers sourcing, deleted `error()`, replaced terminal width block
- `bin/intent_info` -- added intent_helpers sourcing, deleted `error()`
- `bin/intent_llm` -- added intent_helpers sourcing, deleted `error()`
- `bin/intent_fileindex` -- added intent_helpers+config sourcing, deleted `error()` and `find_project_root()`
- `intent/plugins/claude/bin/intent_claude_subagents` -- deleted `calculate_checksum()`, replaced terminal width, 4 jq guards, 4 claude guards
- `intent/plugins/claude/bin/intent_claude_skills` -- deleted `calculate_skill_checksum()`, replaced terminal width, 2 jq guards, 2 claude guards
- `intent/plugins/claude/bin/intent_claude_upgrade` -- replaced 2 inline shasum blocks
- `intent/plugins/agents/bin/intent_agents` -- replaced 3 require_project_root patterns

**Key design decisions:**

- Scripts that `exec` from the dispatcher need their own `source "$INTENT_HOME/bin/intent_helpers"` since exported functions aren't inherited across exec
- `require_jq()` returns 1 (not exit) -- callers choose: `require_jq || exit 1` in config, `require_jq || return 1` in plugin functions
- V02 (INTENT_HOME dedup) deferred -- the guard pattern is a harmless safety net for standalone invocation during development

**Result:** ~150 lines of duplicated code removed. All 318 tests passing. Squashed into v2.5.0 release commit.

### WP07: Plugin script refactoring via shared callback library (24 Feb 2026)

**Created `intent/plugins/claude/lib/claude_plugin_helpers.sh`** (~400 lines). Shared library using callback pattern -- each plugin script defines 8 callbacks and 4 config variables, then sources the shared library to get install/sync/uninstall/manifest operations.

**Config variables set by each plugin script:**

| Variable             | Skills value | Subagents value |
| -------------------- | ------------ | --------------- |
| `PLUGIN_TYPE`        | `"skill"`    | `"agent"`       |
| `PLUGIN_TYPE_CAP`    | `"Skill"`    | `"Agent"`       |
| `PLUGIN_TYPE_PLURAL` | `"skills"`   | `"agents"`      |
| `PLUGIN_CMD`         | `"skills"`   | `"subagents"`   |

**8 callbacks defined by each plugin script:**

| Callback                     | Purpose                              |
| ---------------------------- | ------------------------------------ |
| `plugin_get_manifest_path`   | Returns manifest file path           |
| `plugin_get_source_file`     | Returns source file path by name     |
| `plugin_is_installed`        | Returns 0 if installed               |
| `plugin_copy_to_target`      | Copies source to target location     |
| `plugin_remove_target`       | Removes installed item               |
| `plugin_checksum_target`     | Returns checksum of installed file   |
| `plugin_get_available_names` | Returns list of available items      |
| `plugin_manifest_extra`      | Returns extra jq fields for manifest |

**Shared functions provided:**

| Function                      | Replaces                                                                  |
| ----------------------------- | ------------------------------------------------------------------------- |
| `plugin_ensure_manifest`      | `ensure_skill_manifest()`, inline init in `update_installed_manifest()`   |
| `plugin_update_manifest`      | `update_skill_manifest()`, `update_installed_manifest()`                  |
| `plugin_remove_from_manifest` | `remove_skill_from_manifest()`, `remove_from_manifest()`                  |
| `plugin_install`              | `intent_claude_skills_install()`, `intent_claude_subagents_install()`     |
| `plugin_sync`                 | `intent_claude_skills_sync()`, `intent_claude_subagents_sync()`           |
| `plugin_uninstall`            | `intent_claude_skills_uninstall()`, `intent_claude_subagents_uninstall()` |

**Files changed:**

- `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- NEW, ~400 lines shared library
- `intent/plugins/claude/bin/intent_claude_skills` -- 654 -> 299 lines, callbacks + list/show/help
- `intent/plugins/claude/bin/intent_claude_subagents` -- 1015 -> 613 lines, callbacks + init/list/show/status/help
- `bin/intent_helpers` -- added `get_config_field()` shared function
- `bin/intent_st` -- replaced inline grep config extraction with `get_config_field()`
- `bin/intent_wp` -- replaced inline grep config extraction with `get_config_field()`
- `tests/unit/agent_commands.bats` -- updated `create_source_sandbox()` to include lib/ directory

**Result:** ~350 lines of duplicated code removed. All 339 tests passing.

## Challenges & Solutions

1. **Dispatcher exec boundary:** `bin/intent` uses `exec` to launch subcommands, so functions from the parent process are NOT inherited. Scripts that didn't previously source anything (`intent_st`, `intent_info`, `intent_llm`, `intent_fileindex`) needed explicit sourcing added with the fallback pattern: `: "${INTENT_HOME:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"`

2. **require_jq return semantics:** Had to use `return 1` not `exit 1` so callers could decide the severity. In `intent_config` (top-level parse context) it's `exit 1`; in plugin functions it's `return 1`.

3. **macOS bash 3.x compatibility:** Cannot use `${PLUGIN_TYPE^}` for capitalization (bash 4+ only). Used explicit `PLUGIN_TYPE_CAP` variable instead.

4. **Usage string vs display label:** `PLUGIN_TYPE_PLURAL` ("agents") differs from the CLI command ("subagents"). Added separate `PLUGIN_CMD` variable for usage strings like `Usage: intent claude subagents install`.

5. **Test sandbox needed lib/ directory:** `create_source_sandbox()` in agent_commands.bats needed updating to symlink the new `intent/plugins/claude/lib/` directory so tests could find the shared library.

6. **Unmanaged agent detection:** Shared `plugin_uninstall()` needed the "unmanaged agent" check (agent exists on disk but not in manifest). Applied to shared library so both plugins benefit.
