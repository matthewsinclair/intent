# ST0043 Design -- intent upgrade as a convergent orchestrator (Architecture B)

Architecture from info.md (the ST0042 review, theme T8). This doc adds the concrete build plan distilled from the v2.11.14 subsystem map (read-only audit, 2026-06-15). Fleet-floor ratified v2.9.x (matts, full prune): everything that migrates a below-v2.9.0 project is deleted (fail-forward; git preserves).

## 1. Target architecture

`bin/intent_upgrade` becomes a ~150-line orchestrator:

1. Detect state: `VERSION=detect_project_version`; `TARGET=get_intent_version` (the repo `VERSION` file).
2. Semver sanity BEFORE any mutation: `error()` on missing / unparseable VERSION; refuse downgrade (current > target); a future / unknown current version does NOT hard-fail (no enumerated version globs, no `*) error "Unknown version"`).
3. Verified backup: `cp ... || error()` (already correct in v2.11.14 inline backup -- preserve it; no unconditional success echo on a failed copy).
4. Walk an ordered ledger of structural steps. Flat array (bash 3.2): `LEDGER="relocate_config languages_field"`. Naming-convention dispatch per step id: `step_<id>_needs` (state probe) -> `step_<id>_run` -> `step_<id>_verify` (postcondition). No `declare -A`.
5. Single delegation to `intent claude upgrade --apply` for ALL canon / skills / subagents convergence; failure -> `error()` (not a warning, not `|| true`).
6. Stamp the target ONCE, last (jq), after every applicable step's postcondition verifies.

Invariants:

- AC-01.1: after interruption, a re-run does exactly the remaining work; applicability is decided by `step_<id>_needs` probing on-disk state, never the top-level stamp. No step writes the version (only the orchestrator stamps, last) -> trivially "no step writes a version newer than its own step".
- AC-01.4: `intent_upgrade` is the SOLE stamper (jq, one technology). `intent claude upgrade` is the SOLE canon engine -- its `VERSION_BUMP` action + both `sed` stamps are stripped.

## 2. New file layout

| File                                              | Role                                                                      | Change                                                                                                                                                |
| ------------------------------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bin/intent_upgrade`                              | orchestrator + sole stamper                                               | full rewrite (was a 524-line case-ladder script; only `usage()` was a function)                                                                       |
| `bin/intent_migrations`                           | ledger steps + relocated migration code, sourced ONLY by `intent_upgrade` | NEW (satisfies AC-01.5)                                                                                                                               |
| `bin/intent_helpers`                              | shared lib                                                                | KEEP `detect_project_version` (+ `detect_stp_version` alias) shared; REMOVE every `migrate_*` / `needs_*` / `intent_relocate_dotintent` / ceremony fn |
| `intent/plugins/claude/bin/intent_claude_upgrade` | canon engine only                                                         | strip `VERSION_BUMP` (action + sed at :1051 config + :1056 CLAUDE.md); fix BSD `sed -i ''` at :231 for Linux; no unanchored historical-date rewrite   |

Path note: the canon engine is `intent/plugins/claude/bin/intent_claude_upgrade`, NOT `bin/intent_claude_upgrade` (info.md + the ACs say `bin/`).

## 3. Ledger -> existing code (probes already exist inside the deleted predicates; extract, do not reinvent)

| step              | needs (probe source)                                                                            | run (source)                                                                       | verify                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `relocate_config` | `.intent/` present && `intent/.config/` absent (from `needs_v2_10_0_upgrade` helpers:1186-1188) | `intent_relocate_dotintent` (helpers:1326-1407, moved verbatim)                    | `intent/.config/config.json` exists && `.intent/` gone |
| `languages_field` | config exists && `!has("languages")` (from `needs_v2_11_0_upgrade` helpers:1148-1152)           | back-fill body of `migrate_v2_10_x_to_v2_11_0` (helpers:1468-1551) MINUS its stamp | config `has("languages")`                              |

`prune_backlog` DROPPED: it is the v2.4->v2.5 `Backlog.md` removal (`migrate_v2_4_0_to_v2_5_0` helpers:1868-1915), vacuous at the v2.9.x floor. No generic `prune_backlog` fn exists.

## 4. Delete / Keep / Move (from the subsystem map)

- KEEP-SHARED (stay in `intent_helpers`): `detect_project_version` (517-568) + `detect_stp_version` alias (643-645).
- KEEP-RELOCATE (into `bin/intent_migrations`): `intent_relocate_dotintent` (1326-1407, verbatim) -> `step_relocate_config_run`; the languages back-fill body of `migrate_v2_10_x_to_v2_11_0` (1468-1551, minus stamp) -> `step_languages_field_run`; the two probes extracted from `needs_v2_10_0_upgrade`:1186-1188 and `needs_v2_11_0_upgrade`:1148-1152.
- DELETE (fail-forward; git preserves):
  - `intent_upgrade`: the version case ladder + pre-v2 tail case (163-423); the 17-predicate needs-gate (113); the tail-call skills/subagents sync block (425-444, subsumed by the single delegation); the `create_claude_md` call (461).
  - `intent_helpers`: every `migrate_v*_to_*` from `migrate_v0_to_v2` (690) through `migrate_v2_8_2_to_v2_9_0` (1251) AND `migrate_v2_9_0_to_v2_10_0` (1425-1463, decomposed: relocate -> step, canon-apply -> the single delegation, stamp -> the final stamp); every `needs_v2_*` predicate; the pre-v2 converters (`convert_yaml_frontmatter` 316, `update_version_in_frontmatter` 371, `convert_yaml_config_to_json` 386, `flatten_directory_structure` 498, `migrate_remaining_content` 1918, `create_default_v2_config` 1986, `_generate_basic_agents_md` 1573, `create_v2_directory_structure` 457, `update_config_version` 1716); `show_migration_summary` (611), `count_migration_files` (626), `create_project_backup` (571), `needs_migration` (598) -- all dead/unused; `create_claude_md` (2004, after confirming no `intent_init` caller -- grep showed none); `generate_ext_readme` (1203) once the v2.8.2 migration is gone.
  - `intent_claude_upgrade`: the `VERSION_BUMP` action + sed stamps (1047-1059).

ext bootstrap: deleting `migrate_v2_8_2_to_v2_9_0` drops the only upgrade-time `~/.intent/ext/` bootstrap (helpers:1265-1280). VERIFIED SAFE: `intent ext` creates the dir on demand (`intent_ext:746`), discovery tolerates absence (`ext_root_dir`/`ext_enumerate_names`), `intent init` never bootstrapped it, the worker-bee seed is dead (pruned ST0034), and the fleet is all v2.9.0+ (already bootstrapped).

## 5. Stamp consolidation (AC-01.4)

Two writers today: jq in `intent_helpers` (each `migrate_*` writes `.intent_version`, `del(.version)`) + sed `VERSION_BUMP` in the canon engine (config.json :1051, CLAUDE.md :1056). New: the orchestrator is the sole jq stamper, written once last to `intent/.config/config.json`; the canon engine's `VERSION_BUMP` is removed.

## 6. Test rewrite plan (AC-01.7)

- NEW: `tests/unit/intent_upgrade_orchestrator.bats` -- the AT-00.1 / AT-01.\* set (sec 7).
- DELETE: `tests/unit/ext_migration.bats` (tests `migrate_v2_8_2_to_v2_9_0` [deleted] + case-ladder / needs static gates, 24 tests); the `needs_v2_*_upgrade` predicate tests in `migrate_v2_9_0_to_v2_10_0.bats` (6-9) and `migrate_v2_10_x_to_v2_11_0.bats` (7-9).
- TRIM-KEEP: `migrate_v2_9_0_to_v2_10_0.bats` tests 1-5 (relocate behaviour -> retarget at `step_relocate_config_run` / `intent_relocate_dotintent` in its new home); `migrate_v2_10_x_to_v2_11_0.bats` tests 1-6 (languages back-fill -> retarget at `step_languages_field_run`).
- REWRITE: `intent_upgrade_dispatcher.bats` -- drop the case-ladder / Unknown-version asserts (1, 3); keep behavioural (2, 4, 5, 6, 7) retargeted at the orchestrator.

## 7. AT -> entry-point map (red-first targets; numeric ids per the `intent ac/at` parser)

- AT-00.1 `intent_upgrade_orchestrator.bats` -- e2e: a project stamped 2.9.0 in the `.intent/` layout, no languages -> run `intent upgrade` -> assert relocated (`intent/.config/`, `.intent/` gone), languages present, stamp == TARGET, backup made.
- AT-01.1 -- fixture already-relocated + languages-absent + stamp old -> run -> only `languages_field` runs, stamp == TARGET, no error (relocate probe skips).
- AT-01.2 -- grep guard: no `step_*_run` in `bin/intent_migrations` writes `intent_version`; only `bin/intent_upgrade` stamps.
- AT-01.3 -- current version unknown/future-but-in-range (eg "2.11.99") -> success, stamp == TARGET, no "Unknown version"; + grep guard: no enumerated version globs in `intent_upgrade`.
- AT-01.4 -- downgrade (current > TARGET) errors before backup (no `.backup/`); missing/unparseable VERSION errors before mutation.
- AT-01.5 -- backup failure -> `error()` + abort + stamp untouched (port `intent_upgrade_dispatcher.bats:188`).
- AT-01.6 -- grep guard: `intent_claude_upgrade` has no `VERSION_BUMP` / intent_version sed; only `intent_upgrade` stamps.
- AT-01.7 -- grep guard: `intent_helpers` defines no `migrate_*` / `needs_*` / `intent_relocate_dotintent`; they live in `bin/intent_migrations`; `detect_project_version` still in helpers.
- AT-01.8 `intent_claude_upgrade.bats` -- no BSD `sed -i ''` in the canon engine; version handling anchored (historical dates not rewritten).
- AC-01.7 (non-test) -- the rewritten/trimmed suite is green on matts's full-suite run.

## 8. Related close-gate fixes landing in the same release (No Silent Errors)

Two `bin/intent_acceptance` close-gate fixes, both release-scoped tooling hardening (narrate in `intent/history/v2.12.0.md`), both guard-tested in `acceptance_close_gate.bats`:

- F1 (silent drop): AC/AT lines that fail the strict numeric grammar (eg the letter-group `AC-U.1` ids) were silently dropped, making the gate vacuous. Fix: a malformed-line detector warns loudly on every read path and blocks the gate.
- F6 (missing-contract gate behaviour): re-confirmed as opt-in-by-presence, NOT a block (matts, 2026-06-16). A thread with no `acceptance.md` has not opted into the AC regime, so `ac gate` stays OPEN (exit 0) and the thread closes as before -- old STs are never forced to author a contract. Only a PRESENT acceptance.md is enforced: unsatisfied ACs or malformed lines (F1) block. The bug report's "missing must block" was superseded by this ruling.
