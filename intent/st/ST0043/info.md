---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
intent_version: 2.11.11
status: WIP
slug: rethink-intent-upgrade
created: 20260611
completed:
---

# ST0043: Rethink 'intent upgrade'

## Objective

Replace the version-stepping migration chain in `intent upgrade` with a convergent declared-end-state architecture ("make it so"), reserving an ordered structural-step ledger for genuine one-way transforms. Make interruption recovery safe by construction, stop the stamp from being load-bearing, and resolve the canon-ownership split between `intent_upgrade` and `intent_claude_upgrade`.

## Context

Spun out of the ST0042 Fable 5 review (theme T8) because the rethink is design-level and dwarfs the other review WPs (~1800 lines across two installers, L+). The review findings are the input; this thread owns the redesign and execution.

### Why the current model is wrong (evidence from the review)

- Of 18 v2.x migration steps, ~11 are pure version stamps; only the v2.10 relocation is a genuine structural project mutation. The chain is ~80% ceremony whose only job is moving a number the next step reads.
- The code already mistrusts its own stamps: three recent predicates were retrofitted to probe observable state "regardless of stamp" (`bin/intent_upgrade:100-110`, `bin/intent_helpers:1033-1037,1070-1073`). When every recent predicate probes state, state is the source of truth and the stamp is a stale cache.
- `intent_claude_upgrade` already is a convergent diagnose/plan/apply installer (Phase 1 probes each artefact against `lib/templates/`, Phase 3 applies, dry-run by default). The convergent model is the half that works.

### Confirmed defects this thread must kill

- F-UPG-3 (HIGH, confirmed): mid-chain steps stamp the live target version, not their step version (`bin/intent_helpers:1442-1450` et al). An interrupted chain claims success; a re-run dispatches on `"2.11."*`, runs only the languages migration (which no-ops because the relocated config does not exist yet), and reports success -- a permanent silent half-migration.
- F-UPG-1/2 (HIGH/MEDIUM): the dispatcher hard-fails on unknown/future versions (eg 2.12.0) after backup + `create_v2_directory_structure` mutation has already begun; `needs_*` globs enumerate `2.12.*..2.19.*` literally and break at 2.20.0.
- Backup is unverified: `cp -r ... 2>/dev/null || true` then unconditional "Backup created successfully" (`bin/intent_upgrade:138-144`).
- Canon files have three writers (`create_claude_md`, `canon_refresh_with_user_section`, ad-hoc sed) and the stamp has two technologies (jq vs sed) -- Highlander violation at the centre of the subsystem.
- T11/T12 fold in here: the BSD-only `sed -i ''` that breaks Linux upgrades (`intent_claude_upgrade:231,1051,1056`) and the unanchored CLAUDE.md version-sed that rewrites historical dates (`intent_claude_upgrade:1056`) both die as side effects of the redesign.

### Recommended target (Architecture B, from the review)

`intent upgrade` becomes a ~150-line orchestrator:

1. Detect current state + numeric semver sanity (refuse downgrade, error on missing VERSION) -- all before any mutation.
2. Verified backup (`error()` on copy failure).
3. Walk an ordered ledger of structural steps, each with a state-based `needs` probe, a `run`, and a postcondition verify. Bash 3.2-friendly: flat array of step ids with naming-convention dispatch (`step_<id>_needs` / `step_<id>_run`), no `declare -A`. Day-one ledger: `stp_to_intent` (collapsed pre-v2 trio), `prune_backlog`, `relocate_config` (existing `intent_relocate_dotintent`, kept verbatim -- it already meets the bar), `languages_field`.
4. Single delegation to `intent claude upgrade --apply` for all canon/skills/subagents convergence; failure surfaced via `error()`, not a warning.
5. Stamp the target once, after all applicable steps' postconditions verify.

Invariant: after any interruption, a re-run performs exactly the remaining work, applicability decided by probing on-disk state (or a step-scoped sentinel) -- never the top-level stamp; the stamp is written once, last. Corollary: no migration may ever write a version newer than its own step.

Ownership split to resolve: `intent_upgrade` = orchestrator + sole stamper; `intent_claude_upgrade` = sole canon engine (strip its `VERSION_BUMP`). Migration code moves out of `bin/intent_helpers` (sourced by all ~25 commands, consumed only by upgrade) into upgrade-only scope (`bin/intent_migrations` or inline). `detect_project_version` stays shared.

### Migration path (delete / keep)

Delete (fail-forward, no stubs): the 15-arm case ladder + duplicate pre-v2 chain (`intent_upgrade:160-420`), the 17-clause `needs_*` gate (`:113-117`), all pure-stamp migrate functions + enumerated `needs_*` predicates, `show_migration_summary`, `count_migration_files`, the unused second backup `create_project_backup`, `create_claude_md`, the `VERSION_BUMP` action + CLAUDE.md version-sed, the tail-call `skills/subagents sync || true` block (`intent_upgrade:422-441`, subsumed by the single delegation).

Keep (relocated to upgrade-only scope): `intent_relocate_dotintent` verbatim, the pre-v2 structural migrations + converters collapsed into one `stp_to_intent` step, `migrate_v2_10_x_to_v2_11_0`'s state-probed back-fill as `languages_field`, `intent_claude_upgrade` Phase 1-3 machinery minus stamping.

### Open questions for design phase

- Fleet floor: lowest installed version in the fleet (memory says none below v2.10.x) determines whether the pre-v2 `stp_to_intent` step is live code or itself prunable.
- Canary discipline: two-project canary before fleet pickup (the ST0035 pattern).

### Risk

Dispatcher rewrite invalidates `tests/unit/intent_upgrade_dispatcher.bats` and parts of the migration bats -- the test rewrite is in scope. State probes are strictly more robust than stamp dispatch, and the backup finally gets verified, so ancient-stamp members are safer under the new code, not riskier.

## Related Steel Threads

- ST0042 -- Fable 5 review; theme T8 is this thread's source and full evidence base
- ST0035/ST0036 -- produced the v2.10 relocation (the one genuine structural step) and the canon installer being consolidated here
- ST0037 -- the `languages` field back-fill that becomes the `languages_field` ledger step

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
