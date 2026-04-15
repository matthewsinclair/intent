# Claude Code Session Restart

## Current State

Intent v2.8.2. No active steel threads. Clean working tree. All 16 managed Intent projects in `~/Devel/prj/` now at 2.8.2.

## Recent (2026-04-15)

v2.8.2 released in two commits under one tag:

- `1059dc7` -- **ST0033** cwd-resilient dispatch (`bin/intent` exports `INTENT_ORIG_CWD` and `cd`s to `$PROJECT_ROOT` before `exec`ing subcommands; `intent_treeindex` and `intent_fileindex` consult `INTENT_ORIG_CWD` for relative path arguments) and **ST0032** close-out (Credo checks wired into `.credo.exs` via `lib/scripts/configure_credo.exs`). Regression tests in `tests/unit/subdir_invocation.bats`.
- `84a3a5f` -- **Upgrade chain gap.** `bin/intent_upgrade` previously halted mid-chain at 2.6.0. New `migrate_v2_6_0_to_v2_8_0` (pure stamp), every starting-version case chains through 2.8.0 -> 2.8.1 -> 2.8.2, 2.6.0/2.7.0 added as starting cases, pre-v2 fallback chain extended. Tag force-moved and pushed.

461/461 BATS tests green. E2E verified: 2.6.0 project upgrades cleanly to 2.8.2.

## Fleet Upgrade

All 16 Intent projects in `~/Devel/prj/` stamped at 2.8.2 (skipped `A3/*` per user direction). 11 went via `intent upgrade` from 2.8.1; 4 older projects (Courses/Agentic Coding, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex) landed at 2.6.0 and were finished by direct `jq` stamp before the chain-gap fix shipped -- any future chain run from them will now complete cleanly.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
