---
verblock: "24 Feb 2026:v0.2: matts - WP01 complete, ST closed"
intent_version: 2.5.0
status: Completed
slug: fix-highlander-violations
created: 20260224
completed: 20260224
---

# ST0025: Fix Highlander Violations

## Objective

Eliminate duplicated code paths across the Intent CLI codebase by consolidating shared logic into single, canonical implementations. The Highlander Rule: there can be only one code path for each thing.

## Context

A comprehensive audit (performed during ST0023) identified 25 Highlander Rule violations across bin/ scripts, plugin scripts, templates, and configuration generation. These range from critical (same function defined in 11 files) to low (internal duplication within a single script). The violations create maintenance burden, allow drift between copies, and have already caused bugs (stale version strings, inconsistent jq error messages, test side-effects from modifying real source files).

The test side-effect issue (tests modifying real `intent/plugins/claude/subagents/intent/agent.md` and using `git checkout` in teardown) was fixed in ST0023 via a sandbox approach. The remaining 25 violations need systematic resolution.

## Related Steel Threads

- ST0023: Remove Backlog from Intent (audit performed during this work)

## Scope

**Completed (WP01):** 8 violations resolved (V01-V08) -- shared helper function extraction.

**Deferred (WP02-WP06):** 17 remaining violations across template consolidation, plugin dedup, correctness fixes, and legacy cleanup. These are lower priority and can be addressed in future steel threads.

### WP01 -- Completed

7 shared functions consolidated into `bin/intent_helpers`:

- `error()` -- deleted from 10 scripts (V01)
- `calculate_checksum()` -- extracted from subagents/skills/upgrade (V03)
- `get_terminal_width()` -- extracted from subagents/skills/st (V04)
- `require_jq()` -- extracted from config/subagents/skills (V05)
- `require_claude()` -- extracted from subagents/skills (V06)
- `require_project_root()` -- extracted from intent_agents 3x (V07)
- `find_project_root()` -- duplicate removed from fileindex (V08)

V02 (INTENT_HOME dedup) deferred -- low risk, guard pattern is a harmless safety net.

16 files changed, ~150 lines of duplicated code removed. All 318 tests passing.
