---
verblock: "15 Apr 2026:v0.2: matts - Scope and close-out"
intent_version: 2.8.2
status: Completed
slug: make-the-intent-cli-resilient-to-knowing-where
created: 20260415
completed: 20260415
---

# ST0033: Make the intent CLI resilient to cwd when locating the project root

## Objective

`intent <cmd>` must work from any directory inside an Intent project, not only from the project root. Outside any project, it must fail cleanly with a "not in an Intent project" error and must not create `.intent/` or `intent/` at the invoker's cwd.

## Context

`bin/intent_config:37` already has `find_project_root()` that walks up from cwd looking for `.intent/config.json` (and legacy `stp/` markers). `bin/intent` calls `load_intent_config` and errors correctly when no root is found. The defect is downstream: after dispatch, subcommand scripts (`intent_st`, `intent_wp`, `intent_init`, …) read relative paths off the unchanged cwd. When the invoker is in a subdirectory, these reads miss and commands either misbehave or create artefacts in the wrong place.

Fix locus: a single point in the dispatcher. After `load_intent_config` sets `PROJECT_ROOT`, export `INTENT_ORIG_CWD=$(pwd)` and `cd "$PROJECT_ROOT"` before `exec`'ing the subcommand. This is Highlander-compliant — no subcommand relative-path audit needed. Two subcommands that accept user-supplied path arguments (`intent_treeindex`, `intent_fileindex`) consult `INTENT_ORIG_CWD` when their argument is relative and does not resolve against the project root.

## Related Steel Threads

- ST0032: Fix Credo checks (shipped in the same release)
- ST0026: Added `intent audit`, Credo integration, plugin discovery

## Scope

### In scope

- `bin/intent` dispatcher: export `INTENT_ORIG_CWD`, `cd "$PROJECT_ROOT"` before exec.
- `bin/intent_treeindex`: resolve relative `DIR` against `INTENT_ORIG_CWD` when needed.
- `bin/intent_fileindex`: resolve relative `STARTDIR` against `INTENT_ORIG_CWD` when needed.
- `tests/unit/subdir_invocation.bats`: regression tests covering read/write paths from subdirectories and a clean-failure test from outside any project.

### Out of scope

- Rewriting per-subcommand relative path usage (the dispatcher `cd` makes that moot).
- Changing the "not in an Intent project" error text.
- Touching the legacy `stp/` fallback branches in `find_project_root`.
