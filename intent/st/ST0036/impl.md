# Implementation - ST0036: Directory relocation `.intent/` -> `intent/.config/` (v2.10.0 breaking)

## Status (2026-04-26)

7 of 9 WPs Done (WP01-WP07). WP08 half-done in working tree (manual `mv` proves the layout-flip is clean; canon-apply phase pending). WP09 awaits WP08.

## Sequence as built

| WP   | Commit                   | Notes                                                                                                                                                 |
| ---- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| WP01 | `4dcccce`                | `migrate_v2_9_0_to_v2_10_0` + `intent_relocate_dotintent` -- atomic mv with cross-FS fallback; sentinel + recovery anchor coordinated with WP07.      |
| WP02 | `5369afd` then `33a99d0` | Path probes flipped; `detect_project_version` got a documented narrow exception so `intent upgrade` still recognises v2.0-v2.9.x layouts to upgrade.  |
| WP03 | `777c5b0`                | Literal sweep across `bin/`, `intent/plugins/`, `intent/docs/`, `intent/usr/`. `~/.intent/ext/` references preserved.                                 |
| WP04 | `5f8b61e` (+ `f04db11`)  | Single material flip: `lib/templates/hooks/pre-commit.sh` (4 hits). `_usage-rules.md` Project Structure flip turned out to be N/A (no such section).  |
| WP05 | `b62ea58`                | 11 BATS files flipped; new `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` (6 scenarios); doctor sentinel scenario in `global_commands.bats`.             |
| WP06 | `32df058`                | New `lib/templates/_treeindexignore` template (Highlander cleanup of `bin/intent_treeindex` heredoc); canon installer ships it.                       |
| WP07 | `1debc03`                | `intent/docs/migration-v2.10.0.md` (~250L). Anchor `recovery-from-interrupted-migration` matches WP01 + intent_doctor diagnostic strings exactly.     |
| WP08 | (working tree)           | Manual `mv .intent intent/.config` performed as diagnostic on 2026-04-26. Pending: revert + invoke `intent upgrade` so canon-apply Phase 3 also runs. |
| WP09 | (pending)                | Coordination doc updates into ST0035/WP15-17.                                                                                                         |

## Dogfood-driven fixes (out-of-band but in-ST)

These landed mid-ST, prompted by fleet-rollout dogfood and the WP02 regression:

- `33a99d0` -- `detect_project_version` narrow exception (recognise v2.0-v2.9.x).
- `81c2b30` -- `intent_upgrade` suppresses legacy STP messaging on v2.x->v2.x hops (`LEGACY_STP_SOURCE` boolean gate).
- `f04db11` -- `lib/templates/llm/_CLAUDE.md` + Intent's own `CLAUDE.md` flip stale `.intent/` to `intent/.config/`.
- `ef2dd0e` + `255436c` -- per-project session-id state file (multi-session safe) and robust `/in-session` skill resolution chain (per-project -> legacy shared -> always-unknown).

## Diagnostics that proved the design

1. **WP05 manual `mv` validation** (2026-04-26 session). Before doing the per-test BATS flips, manually renamed Intent's own `.intent/` -> `intent/.config/` and re-ran the BATS suite. **Result: +1 passing, 0 newly broken**. The +1 was test 374 (`intent critic dispatches to bin/intent_critic`) which had been failing because Intent's CLI rejected Intent's own repo as not-a-project. The 26 remaining failures were all per-test fixture/assertion flips -- 0 hard-coded `.intent/` literals depended on Intent's old layout. WP02 path probes were proven correct end-to-end.

2. **WP05 mktemp bug surfaced**. `agents_sync_idempotent` test had been silently broken on macOS for some time: `mktemp /tmp/foo-XXXXXX.md` creates the LITERAL file `foo-XXXXXX.md` (BSD mktemp does not substitute X's when followed by a suffix). First test run "passed" (created the literal file); second run collided with "File exists". Fixed in WP05 by dropping the `.md` suffix from the mktemp template.

3. **WP06 canon installer end-to-end**. Verified on a synthetic v2.10.0 project: dry-run shows `INSTALL_TREEINDEXIGNORE` action queued; `--apply` installs byte-identical to template; second dry-run shows `PRESENT` and zero actions (idempotent). The Highlander refactor of `ensure_treeindexignore` was also exercised in isolation: function is idempotent (mtime unchanged on second call).

## Open items for tomorrow

1. **WP08 second half**: revert the working-tree manual `mv` and invoke `intent upgrade` properly so the canon-apply Phase 3 lands too. Verify with `tests/run_tests.sh` (expect 774 green) and `intent doctor` (expect clean). Commit as one coherent rename + canon-apply diff.
2. **WP09**: coordination notes into `ST0035/WP15`, `WP16`, `WP17` info.md files plus `ST0036/impl.md` finalisation.
3. **Then ST0035 resumes** at WP14 (Intent self-dogfood for the canon LLM config), which now runs on a v2.10.0-relocated Intent.

## Key code paths (for next-session navigation)

- `bin/intent_helpers:1115-1198` -- `intent_relocate_dotintent` (atomic mv with refusals + cross-FS fallback).
- `bin/intent_helpers:1214+` -- `migrate_v2_9_0_to_v2_10_0` (relocate -> stamp -> canon-apply).
- `bin/intent_helpers:368-374` -- `detect_project_version` narrow exception (recognise legacy `.intent/`).
- `bin/intent_doctor:315-330` -- sentinel detection (Check 4c).
- `intent/plugins/claude/bin/intent_claude_upgrade` -- canon installer; new `INSTALL_TREEINDEXIGNORE` action.
- `bin/intent_treeindex:421-433` -- `ensure_treeindexignore` reads from `lib/templates/_treeindexignore`.
- `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` -- 6-scenario migration test suite.
- `tests/lib/test_helper.bash:42-65` -- `create_test_project` emits `intent/.config/config.json` with stamp `2.10.0`.
