---
verblock: "24 Feb 2026:v0.3: matts - Completed"
intent_version: 2.5.0
status: Completed
slug: remove-backlog-from-intent
created: 20260224
completed: 20260224
---

# ST0023: Remove Backlog from Intent

## Objective

Remove all Backlog.md integration from Intent. This includes CLI commands (`bl`, `task`, `status`, `migrate`), documentation, templates, subagent definitions, tests, CI/CD pipeline dependencies, and example project artifacts.

## Context

Backlog.md integration was a third-party npm tool (`backlog.md`) that Intent wrapped with several commands. The integration is no longer used and touches ~70+ files across the codebase. Removing it simplifies the codebase, eliminates a Node.js CI dependency, and drops 3 test files (~780 lines of backlog-specific tests).

Version bump: v2.4.0 -> v2.5.0.

## Work Packages

| WP   | Scope                                    | Status |
| ---- | ---------------------------------------- | ------ |
| WP01 | Documentation: Remove Backlog references | Done   |
| WP02 | LLM Templates: Remove from generated     | Done   |
| WP03 | Subagents: Remove from agent definitions | Done   |
| WP04 | CLI Core: Remove commands and config     | Done   |
| WP05 | Configuration: Remove config keys        | Done   |
| WP06 | Tests: Remove/update test files          | Done   |
| WP07 | CI/CD, Examples, and Cleanup             | Done   |
| WP08 | Version Bump, CHANGELOG, Release         | Done   |

## Related Steel Threads

- ST0020: Elixir support (skills system)
- ST0021: intent-autopsy skill
- ST0022: st new hardening
- ST0025: Fix Highlander Violations (spawned from audit during this work)
