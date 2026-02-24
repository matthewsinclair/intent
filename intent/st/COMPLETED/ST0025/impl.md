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

| Function               | Replaces                                                    |
| ---------------------- | ----------------------------------------------------------- |
| `error()`              | Identical definition in 10 scripts                          |
| `calculate_checksum()` | `calculate_checksum()` in subagents, `calculate_skill_checksum()` in skills, inline shasum in upgrade |
| `get_terminal_width()` | 6-line terminal width detection blocks in subagents, skills, intent_st |
| `require_jq()`         | 8 inline jq guard blocks with inconsistent error messages   |
| `require_claude()`     | 6 inline Claude detection blocks                            |
| `require_project_root()` | 3 identical 8-line blocks in intent_agents                |
| (sourcing fix)         | Duplicate `find_project_root()` in fileindex                |

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

## Challenges & Solutions

1. **Dispatcher exec boundary:** `bin/intent` uses `exec` to launch subcommands, so functions from the parent process are NOT inherited. Scripts that didn't previously source anything (`intent_st`, `intent_info`, `intent_llm`, `intent_fileindex`) needed explicit sourcing added with the fallback pattern: `: "${INTENT_HOME:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"`

2. **require_jq return semantics:** Had to use `return 1` not `exit 1` so callers could decide the severity. In `intent_config` (top-level parse context) it's `exit 1`; in plugin functions it's `return 1`.
