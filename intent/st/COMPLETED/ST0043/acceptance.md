---
verblock: "15 Jun 2026:v0.3: matts - Ratified + numeric AC/AT ids (parser-conformant)"
st_id: ST0043
title: "Rethink 'intent upgrade' -- acceptance contract"
---

# ST0043 Rethink 'intent upgrade' -- Acceptance

> Canonical acceptance contract for ST0043. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written).
>
> STATUS: RATIFIED 2026-06-15 (matts, open-gate). Fleet-floor ruling: support floor v2.9.x -- the whole fleet is matts's own projects (all v2.10.x+), dragged forward to 2.12.x. Everything that migrates a below-v2.9.0 project is deleted (fail-forward; git preserves). Day-one ledger: [relocate_config, languages_field]; the pre-v2 `stp_to_intent` step and the v2.4->v2.5 `prune_backlog` (Backlog.md removal) are both pruned as below-floor. (AC/AT ids are numeric per the `intent ac/at` parser; the prior AC-U.\* draft ids were silently unparseable.)

## Acceptance Criteria

### ST-level

- AC-00.1 `intent upgrade` is a convergent orchestrator: detect state + numeric semver sanity, then verified backup, then walk an ordered ledger of structural steps (each with a state-based `needs` probe, a `run`, and a postcondition verify), then a single delegation to `intent claude upgrade --apply` for all canon/skills/subagents convergence, then stamp the target once, last. The confirmed defects below are all killed.

### Interruption safety + stamp

- AC-01.1 (F-UPG-3) After any interruption, a re-run performs exactly the remaining work; applicability is decided by probing on-disk state (or a step-scoped sentinel), never the top-level stamp. No step ever writes a version newer than its own step. The stamp is written once, last.

### Version-range robustness

- AC-01.2 (F-UPG-1/2) Semver sanity (refuse downgrade, error on missing VERSION) runs before any mutation; unknown / future versions do not hard-fail mid-mutation; no enumerated `needs_*` globs that break at a future version (eg 2.20.0).

### Backup

- AC-01.3 Backup is verified -- `error()` on copy failure; no unconditional "Backup created successfully". (Already correct in v2.11.14 inline backup; the orchestrator must preserve it.)

### Highlander (single writer, single stamper)

- AC-01.4 Canon files have a single writer and the stamp has a single technology. `intent_upgrade` is the sole stamper (jq); `intent claude upgrade` is the sole canon engine (its `VERSION_BUMP` action + the two `sed` stamps stripped).
- AC-01.5 Migration code moves out of `bin/intent_helpers` (sourced by all commands, consumed only by upgrade) into upgrade-only scope (`bin/intent_migrations`); `detect_project_version` stays shared.

### Portability defects (T11 / T12)

- AC-01.6 The BSD-only `sed -i ''` is gone (Linux upgrades work) and the CLAUDE.md version handling is anchored (historical dates are not rewritten). Canon engine is `intent/plugins/claude/bin/intent_claude_upgrade`.

### Tests

- AC-01.7 (non-test) `tests/unit/intent_upgrade_dispatcher.bats` and the affected migration bats are rewritten to the new architecture; below-v2.9.0 migration tests (incl. `ext_migration.bats`) deleted; the full suite is green. -- evidence: full suite green (matts) 2026-06-15 -- satisfied: yes

## Acceptance Tests

### ST-level

- AT-00.1 `tests/unit/intent_upgrade_orchestrator.bats::convergent upgrade from a v2.9.x project lands at target (relocate then languages then canon then stamp-once)` -- covers AC-00.1 -- status: green

### Interruption safety + stamp

- AT-01.1 `tests/unit/intent_upgrade_orchestrator.bats::interrupted upgrade re-run completes only the remaining work via state probe` -- covers AC-01.1 -- status: green
- AT-01.2 `tests/unit/intent_upgrade_orchestrator.bats::no ledger step writes the version; only the orchestrator stamps, last` -- covers AC-01.1 -- status: green

### Version-range robustness

- AT-01.3 `tests/unit/intent_upgrade_orchestrator.bats::future or unknown version does not hard-fail before mutation` -- covers AC-01.2 -- status: green
- AT-01.4 `tests/unit/intent_upgrade_orchestrator.bats::semver sanity refuses downgrade and missing VERSION before any mutation` -- covers AC-01.2 -- status: green

### Backup

- AT-01.5 `tests/unit/intent_upgrade_orchestrator.bats::backup failure surfaces via error() and aborts before mutation` -- covers AC-01.3 -- status: green

### Highlander

- AT-01.6 `tests/unit/intent_upgrade_orchestrator.bats::only intent_upgrade writes the stamp and the canon engine carries no version-bump` -- covers AC-01.4 -- status: green
- AT-01.7 `tests/unit/intent_upgrade_orchestrator.bats::migration code is not sourced into non-upgrade commands` -- covers AC-01.5 -- status: green

### Portability

- AT-01.8 `tests/unit/intent_claude_upgrade.bats::sed edits are portable and the version handling is anchored` -- covers AC-01.6 -- status: green
- Coverage: every test-backed AC (AC-00.1, AC-01.1..AC-01.6) has >=1 covering AT; AC-01.7 is non-test (the rewritten suite is the evidence, satisfied on matts's green full-suite run). Fleet-floor ratified 2026-06-15: ledger = [relocate_config, languages_field]; below-v2.9.0 migration code + ext_migration.bats deleted (fail-forward; git preserves).
