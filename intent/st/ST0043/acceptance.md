---
verblock: "13 Jun 2026:v0.1: matts - Initial version"
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
> STATUS: DRAFT -- ACs are derived from info.md (the ST0042 review evidence base) and are to be ratified when ST0043's build session opens. Created now to seed the dogfood; ST0043 is a separate later session ("44 first").

## Acceptance Criteria

### ST-level

- AC-00.1 `intent upgrade` is a convergent orchestrator: detect state + numeric semver sanity, then verified backup, then walk an ordered ledger of structural steps (each with a state-based `needs` probe, a `run`, and a postcondition verify), then a single delegation to `intent claude upgrade --apply` for all canon/skills/subagents convergence, then stamp the target once, last. The confirmed defects below are all killed and the affected test suite is rewritten green.

### Interruption safety + stamp

- AC-U.1 (F-UPG-3) After any interruption, a re-run performs exactly the remaining work; applicability is decided by probing on-disk state (or a step-scoped sentinel), never the top-level stamp. No step ever writes a version newer than its own step. The stamp is written once, last.

### Version-range robustness

- AC-U.2 (F-UPG-1/2) Semver sanity (refuse downgrade, error on missing VERSION) runs before any mutation; unknown / future versions do not hard-fail mid-mutation; no enumerated `needs_*` globs that break at a future version (eg 2.20.0).

### Backup

- AC-U.3 Backup is verified -- `error()` on copy failure; no unconditional "Backup created successfully".

### Highlander (single writer, single stamper)

- AC-U.4 Canon files have a single writer and the stamp has a single technology. `intent_upgrade` is the sole stamper; `intent_claude_upgrade` is the sole canon engine (its `VERSION_BUMP` stripped).
- AC-U.5 Migration code moves out of `bin/intent_helpers` (sourced by all commands, consumed only by upgrade) into upgrade-only scope; `detect_project_version` stays shared.

### Portability defects (T11 / T12)

- AC-U.6 The BSD-only `sed -i ''` is gone (Linux upgrades work) and the CLAUDE.md version-sed is anchored (historical dates are not rewritten).

### Tests

- AC-U.7 `tests/unit/intent_upgrade_dispatcher.bats` and the affected migration bats are rewritten to the new architecture and pass.

## Acceptance Tests

### Interruption safety

- AT-U.1 `tests/unit/intent_upgrade_orchestrator.bats::interrupted upgrade re-run completes only the remaining work via state probe` -- covers AC-U.1 -- status: to-write (red-first)
- AT-U.1b `tests/unit/intent_upgrade_orchestrator.bats::no step writes a version newer than its own step` -- covers AC-U.1 -- status: to-write (red-first)

### Version-range robustness

- AT-U.2 `tests/unit/intent_upgrade_orchestrator.bats::future or unknown version does not hard-fail before mutation` -- covers AC-U.2 -- status: to-write (red-first)
- AT-U.2b `tests/unit/intent_upgrade_orchestrator.bats::semver sanity refuses downgrade and missing VERSION before any mutation` -- covers AC-U.2 -- status: to-write (red-first)

### Backup

- AT-U.3 `tests/unit/intent_upgrade_orchestrator.bats::backup failure surfaces via error() and aborts` -- covers AC-U.3 -- status: to-write (red-first)

### Highlander

- AT-U.4 `tests/unit/intent_upgrade_orchestrator.bats::only intent_upgrade writes the stamp; canon has one writer` -- covers AC-U.4 -- status: to-write (red-first)
- AC-U.5 -- partly structural -- status: to-write -- `tests/unit/intent_upgrade_orchestrator.bats::migration code is not sourced into non-upgrade commands` -- covers AC-U.5

### Portability

- AT-U.6 `tests/unit/intent_claude_upgrade.bats::sed edits are portable and version-sed is anchored` -- covers AC-U.6 -- status: to-write (red-first)

### Tests rewrite

- AT-U.7 `tests/unit/intent_upgrade_dispatcher.bats` (rewritten) green under the new architecture -- covers AC-U.7 -- status: to-write (red-first)
- Coverage: every AC above has at least one AT. Open question carried from info.md: the fleet-floor decision may prune the pre-v2 `stp_to_intent` step and its ACs/ATs -- ratify at the ST0043 build session.
