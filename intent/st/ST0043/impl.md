# Implementation - ST0043: Rethink 'intent upgrade'

## As-built

Convergent orchestrator (Architecture B) shipped per `design.md`. Built red-first against `tests/unit/intent_upgrade_orchestrator.bats` (AT-00.1, AT-01.1..01.8); all green.

- `bin/intent_upgrade` -- rewritten from a 524-line version-case ladder into a ~150-line orchestrator: detect state -> semver sanity (refuse downgrade, error on missing/unrecognised version, fail-forward floor of v2.9.0) -> verified backup -> walk the `LEDGER` of state-probed steps -> single `intent claude upgrade --apply` -> stamp the target once, last (sole jq stamper). A flat `LEDGER="relocate_config languages_field"` dispatched by `step_<id>_needs/_run/_verify` naming convention (bash 3.2, no `declare -A`).
- `bin/intent_migrations` -- NEW, upgrade-only scope (sourced only by `intent_upgrade`). Holds the two ledger-step trios + `intent_relocate_dotintent` (moved verbatim from helpers). No step writes `intent_version`.
- `bin/intent_helpers` -- pruned 2026 -> 369 lines. Every `migrate_*`/`needs_*`/`intent_relocate_dotintent`/pre-v2 converter/ceremony function deleted (fail-forward; git preserves). KEPT shared: `detect_project_version` (+ `detect_stp_version` alias).
- `intent/plugins/claude/bin/intent_claude_upgrade` -- sole canon engine: `VERSION_BUMP` action + its two `sed -i ''` stamps removed; `canon_substitute_placeholders` rewritten to a portable in-place edit (no BSD `sed -i ''`). Linux upgrades work; the orchestrator is now the only stamper.

## Deviations from the design

- **`create_project_backup` deleted + its test removed.** Design flagged it "dead/unused"; the caller-grep found one live caller -- a dedicated test in `core_functionality.bats` (`.backup_` prefix). It is a second backup impl (Highlander); both function and test deleted.
- **AT-01.3 fixture uses `2.10.5`, not a future version.** The repo `VERSION` is still `2.11.14` (the bump to 2.12.0 is a release step), so a `2.11.99` fixture tripped the new downgrade guard. `2.10.5` is below target and unknown to the old ladder -- the right "unknown-but-handled" case.
- **The skills/subagents sync tail is kept in the orchestrator** (in-whiteboard install + `claude skills/subagents sync`, failure-tolerant). AC-01.4 is satisfied by canon _files_ having a single writer (`intent claude upgrade`) and the stamp a single writer (`intent_upgrade`); the mirror syncs are not canon-file writers.
- **Canon-engine path** is `intent/plugins/claude/bin/intent_claude_upgrade`, not `bin/intent_claude_upgrade` as info.md / the ACs say.

## Test rewrite (AC-01.7)

- NEW: `tests/unit/intent_upgrade_orchestrator.bats` (AT-00.1, AT-01.1..01.7); AT-01.8 appended to `tests/unit/intent_claude_upgrade.bats`.
- NEW (retarget): `tests/unit/intent_migrations_relocate.bats` + `tests/unit/intent_migrations_languages.bats` -- the relocate edge-cases + back-fill variants, pointed at the new `intent_migrations` functions; the deleted `needs_v2_*` predicate tests dropped, stamp assertions dropped (the stamp is the orchestrator's).
- DELETED: `tests/unit/ext_migration.bats` (tested the deleted v2.8.2->v2.9.0 migration + case-ladder static gates); `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` + `tests/unit/migrate_v2_10_x_to_v2_11_0.bats` (superseded by the `intent_migrations_*` files); the `create_project_backup` test in `core_functionality.bats`.
- REWRITTEN: `tests/unit/intent_upgrade_dispatcher.bats` -- dropped the glob-case-ladder test; the behavioural tests (lands-at-target, in-whiteboard install, subagent sync, backup abort) kept against the orchestrator.

## Related close-gate fixes (same release, see design.md sec 8)

`bin/intent_acceptance` hardened against the vacuous-green mode that is a real defect: F1 (malformed / non-numeric AC/AT lines silently dropped -> now warn + block). F6 (missing acceptance.md) was investigated and deliberately left as opt-in-by-presence (a thread with no contract has not opted into the AC regime, so the gate stays OPEN / exit-0; matts ruling 2026-06-16) -- the bug report's "missing must block" was superseded. Guard tests in `acceptance_close_gate.bats` (F1 blocks loudly; the no-contract thread stays open).

## Status

7/8 ACs satisfied (`intent ac status ST0043`). AC-01.7 (non-test) awaits matts's full-suite green.
