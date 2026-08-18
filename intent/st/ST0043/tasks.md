# Tasks - ST0043: Rethink 'intent upgrade'

## Tasks

- [x] Ratify ACs open-gate (matts); fleet-floor ruling (v2.9.x, full prune); numeric AC/AT ids
- [x] Map the upgrade subsystem (read-only audit) -> design.md
- [x] Write red-first ATs (intent_upgrade_orchestrator.bats + AT-01.8) -- matts witnessed RED
- [x] Build `bin/intent_migrations` (relocate_config + languages_field steps + intent_relocate_dotintent)
- [x] Rewrite `bin/intent_upgrade` as the convergent orchestrator (detect -> sanity -> backup -> ledger -> delegate -> stamp-once)
- [x] Strip `VERSION_BUMP` + make `sed` portable in the canon engine
- [x] Prune migration code from `bin/intent_helpers` (2026 -> 369 lines; keep detect_project_version)
- [x] Test rewrite: new intent*migrations*\*.bats; delete ext_migration + old migrate bats; trim dispatcher + core_functionality
- [x] All 9 ATs green; upgrade/migration/acceptance suites green
- [ ] AC-01.7: full suite green (matts runs the external suite) -> satisfy AC-01.7
- [ ] v2.12.0 release (shared with ST0045): version bump, CHANGELOG, history narrative

## Task Notes

Close-gate fixes (F1 malformed-line silent-drop, F6 vacuous-green-on-missing-contract) landed in `bin/intent_acceptance` in the same arc -- release-scoped tooling hardening, guard-tested.

## Dependencies

- AC-01.7 sign-off depends on matts's full-suite run.
- Release is bundled with ST0045 into v2.12.0.
