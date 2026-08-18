# Design - ST0025: Fix Highlander Violations

## Approach

Systematic consolidation in dependency order: shared helpers first, then consumers. Each work package is independently testable. The existing 318-test suite validates that consolidation preserves behavior.

**Principle:** Extract shared logic to `bin/intent_helpers` (already sourced by all scripts), then delete the duplicates from each consumer. No new files -- just consolidation into the existing shared module.

## Audit Findings

### Category 1: Shared Helper Functions (Extract to intent_helpers)

#### V01: `error()` defined in every script [HIGH]

Identical `error() { echo "Error: $1" >&2; exit 1; }` in 11 files. Already exported by `intent_config`.

| File                   | Line |
| ---------------------- | ---- |
| `bin/intent`           | 24   |
| `bin/intent_config`    | 198  |
| `bin/intent_st`        | 9    |
| `bin/intent_init`      | 23   |
| `bin/intent_upgrade`   | 9    |
| `bin/intent_info`      | 9    |
| `bin/intent_llm`       | 9    |
| `bin/intent_treeindex` | 67   |
| `bin/intent_fileindex` | 10   |
| `bin/intent_main`      | 20   |
| `bin/intent_help`      | 21   |

**Fix:** Delete from all scripts that source `intent_config`. Keep only the canonical definition in `intent_config`.

#### V02: INTENT_HOME detection duplicated [HIGH]

Same boilerplate in 12 bin/ + 4 plugin files:

```bash
if [ -z "$INTENT_HOME" ]; then
  INTENT_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi
```

Plugin variant:

```bash
PLUGIN_BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INTENT_ROOT="$(cd "$PLUGIN_BIN/../../../.." && pwd)"
INTENT_BIN="$INTENT_ROOT/bin"
```

**Fix:** `bin/intent` (the dispatcher) is the single entry point. It sets and exports `INTENT_HOME`. Subcommand scripts trust the export. Keep the guard only as a one-liner fallback for standalone invocation during development.

#### V03: `calculate_checksum()` triplicated [MEDIUM]

| File                          | Function                     | Notes                            |
| ----------------------------- | ---------------------------- | -------------------------------- |
| `intent_claude_subagents:267` | `calculate_checksum()`       | sha256sum + shasum fallback      |
| `intent_claude_skills:47`     | `calculate_skill_checksum()` | Identical logic, different name  |
| `intent_claude_upgrade:210`   | Inline                       | Only shasum variant, no fallback |

**Fix:** Add `calculate_checksum()` to `intent_helpers`. Delete from both plugin scripts and upgrade.

#### V04: Terminal-width detection triplicated [LOW-MEDIUM]

| File                      | Lines   |
| ------------------------- | ------- |
| `intent_claude_subagents` | 170-176 |
| `intent_claude_skills`    | 189-195 |
| `intent_st`               | 803-807 |

**Fix:** Add `get_terminal_width()` to `intent_helpers`.

#### V05: `require_jq()` guard repeated 6+ times [LOW-MEDIUM]

Inconsistent jq-not-installed error messages across `intent_config`, `intent_claude_subagents` (4x), `intent_claude_skills` (2x). Some mention RedHat/CentOS, some don't.

**Fix:** Add `require_jq()` to `intent_helpers` with complete platform-specific instructions.

#### V06: `require_claude()` guard repeated 6 times [HIGH]

Claude Code detection (`[ ! -d "$HOME/.claude" ]`) with inconsistent messages in `intent_claude_subagents` (4x) and `intent_claude_skills` (2x).

**Fix:** Add `require_claude()` to `intent_helpers`.

#### V07: `require_project_root()` pattern repeated 3 times [MEDIUM]

Identical 8-line block in `intent_agents` (lines 47-58, 389-399, 429-439).

**Fix:** Add `require_project_root()` to `intent_helpers`.

#### V08: `find_project_root()` re-implemented in fileindex [MEDIUM]

`intent_fileindex:35` has a simplified copy missing legacy STP structure detection.

**Fix:** Source `intent_config` from `intent_fileindex` and use the canonical function.

### Category 2: Template & Config Consolidation

#### V09: CLAUDE.md generated in 3 independent places [CRITICAL]

| Location                       | Function             | Notes                           |
| ------------------------------ | -------------------- | ------------------------------- |
| `lib/templates/llm/_CLAUDE.md` | Template file        | Most complete                   |
| `bin/intent_init:190-218`      | Inline heredoc       | Shorter, missing agent sections |
| `bin/intent_helpers:1018-1071` | `create_claude_md()` | Upgrade path, different content |

**Fix:** Both `intent_init` and `create_claude_md()` should copy and sed-substitute the template file, not generate inline.

#### V10: Config JSON template in 4+ places [MEDIUM]

| Location                                      | Version           | Notes                                      |
| --------------------------------------------- | ----------------- | ------------------------------------------ |
| `bin/intent_init:79-87`                       | Hardcoded `2.1.0` | Should call `get_intent_version`           |
| `bin/intent_helpers:create_default_v2_config` | Dynamic           | Correct                                    |
| `bin/intent_config:172-179`                   | Hardcoded `2.1.0` | Should call `get_intent_version`           |
| `bin/intent_bootstrap:123-129`                | Hardcoded `2.1.0` | Should call `create_default_global_config` |
| `bin/intent_helpers:migrate_v0_to_v2`         | Dynamic           | Correct                                    |

**Fix:** `intent_init` should call `create_default_v2_config()`. `intent_bootstrap` should source `intent_config` and call `create_default_global_config()`. All should use `get_intent_version`.

#### V11: Version fallback "2.2.1" stale in 14 locations [MEDIUM]

Pattern `get_intent_version 2>/dev/null || echo "2.2.1"` in 9 files, 14 locations. Some use `2.2.1`, one uses `2.3.4`, one uses `2.1.0`.

**Fix:** Define `INTENT_VERSION_FALLBACK` in `intent_helpers` (or make `get_intent_version` always succeed).

#### V12: Manifest JSON skeleton in 5 places [LOW-MEDIUM]

`{"version": "1.0.0", "installed": []}` written inline in `intent_claude_subagents` (3x), `intent_claude_skills` (1x), `intent_helpers` (1x).

**Fix:** `ensure_manifest()` function in `intent_helpers`.

#### V13: Basic AGENTS.md double-defined [LOW]

`_generate_basic_agents_md()` in `intent_helpers:667-701` and `_create_basic_agents_md()` in `intent_init:145-169`.

**Fix:** `intent_init` calls the helpers version.

#### V14: `steel_threads.md` inline vs template [LOW]

`intent_st:314-328` generates a stripped-down version instead of copying from `lib/templates/prj/st/_steel_threads.md`.

**Fix:** Copy from template.

#### V15: LLM template content overlap [LOW]

`_CLAUDE.md` and `_llm_preamble.md` duplicate agent descriptions and command lists.

**Fix:** Deduplicate or accept as intentional (different audiences).

### Category 3: Plugin Script Deduplication

#### V16: Manifest path resolution repeated 7 times [CRITICAL]

In `intent_claude_subagents`, the same project-vs-global manifest path block appears 7 times inline.

**Fix:** Extract `get_agent_manifest_path()` (matching the cleaner `get_skill_manifest_path()` pattern).

#### V17: Operation summary block repeated 6 times [HIGH]

Identical counter-based summary pattern in install/sync/uninstall for both subagents and skills.

**Fix:** Extract `print_operation_summary()` to `intent_helpers`.

#### V18: Force/overwrite confirmation repeated 4 times [MEDIUM]

**Fix:** Extract `confirm_overwrite()` to `intent_helpers`.

#### V19: Flag parsing repeated 6 times [LOW]

**Fix:** Lower priority; standardize on `while`/`case` pattern.

### Category 4: Correctness Issues

#### V20: `intent_claude_upgrade` bypasses install lifecycle [MEDIUM]

Does `cp` directly instead of calling install functions. Manifest never updated. Also only copies `SKILL.md` not `cp -r` for skills with subdirectories.

**Fix:** Upgrade should call the install functions or at minimum update manifests.

#### V21: Early migrate functions bypass `update_config_version()` [LOW-MEDIUM]

`migrate_v2_0_to_v2_1()` and `migrate_v2_1_to_v2_2()` inline jq instead of calling the extracted function.

**Fix:** Call `update_config_version()`.

### Category 5: Legacy & Minor

#### V22: `intent_main` near-duplicates `intent` [LOW]

**Fix:** Reduce to a thin shim that execs `bin/intent`.

#### V23: `info()` quiet helper duplicated [LOW]

In `intent_bootstrap` and `intent_doctor`.

**Fix:** Add to `intent_helpers` if `QUIET` is standardized.

#### V24: Config reading in `intent_st` bypasses `load_intent_config` [LOW]

Uses `grep -oE` instead of `jq` via `intent_config`.

**Fix:** Source `intent_config` and use the standard path.

#### V25: Sorted-pairs pattern x5 in `intent_fileindex` [LOW]

Internal to one file.

**Fix:** Extract local `write_sorted_index()` function.

## Design Decisions

1. **`intent_helpers` as the consolidation target** -- Already sourced by all scripts. No new files needed for WP01.
2. **Shared plugin helper library with callbacks** -- For WP07, created `intent/plugins/claude/lib/claude_plugin_helpers.sh`. Each plugin script defines callbacks for its specific source/target/copy semantics, then delegates install/sync/uninstall to shared functions. This preserves script-specific behavior while eliminating ~80% duplicated code.
3. **Backwards compatibility** -- Functions keep existing signatures. Consumers just delete their local copies.
4. **Test-first validation** -- Run full test suite after each WP to catch regressions.
5. **Config variables + callbacks pattern** -- Plugin scripts set `PLUGIN_TYPE`, `PLUGIN_TYPE_CAP`, `PLUGIN_CMD`, `PLUGIN_TYPE_PLURAL` and define 8 callbacks before sourcing the shared library.

## Alternatives Considered

1. **New `intent_common.sh` file** -- Rejected for WP01. `intent_helpers` already serves this role.
2. **Merge subagents + skills into one script** -- Rejected. They're different enough in semantics (init, status, show differ significantly) to justify separate scripts; just share the mechanical parts via callbacks.
3. **`${PLUGIN_TYPE^}` for capitalization** -- Rejected. macOS ships bash 3.x which doesn't support this syntax. Used explicit `PLUGIN_TYPE_CAP` variable instead.
