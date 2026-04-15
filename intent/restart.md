# Claude Code Session Restart

## Current State

Intent v2.8.2. No active steel threads. Clean working tree.

## Recent (2026-04-15)

v2.8.2 released, containing:

- **ST0033** -- cwd-resilient dispatch. `bin/intent` exports `INTENT_ORIG_CWD=$(pwd)` and `cd "$PROJECT_ROOT"` before `exec`ing project subcommands, so `intent <cmd>` now works from any directory inside a project. `intent_treeindex` and `intent_fileindex` consult `INTENT_ORIG_CWD` when resolving relative path arguments. Outside any project, commands fail cleanly ("not in an Intent project") and no longer create stray `.intent/` or `intent/` at cwd. Regression tests in `tests/unit/subdir_invocation.bats`.
- **ST0032** -- Credo check wiring. `intent st zero` (D5a) and `intent audit` now patch `.credo.exs` via `lib/scripts/configure_credo.exs`. 2 broken templates deleted, 4 fixed, `bracket_access_on_struct` added.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
