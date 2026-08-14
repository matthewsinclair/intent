---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-12
title: "Cutover and v3.0.0 release"
scope: L
status: Not Started
---

# WP-12: Cutover and v3.0.0 release

## Objective

Cut v3.0.0: shell pruned fail-forward, docs converged, release docs written before the tag, and the fleet pointed at the migration.

## Deliverables

- `bin/` (shell) pruned at the cut; BATS final disposition (conformance survivors ported or retired with the shell)
- Docs sweep: working-with-llms.md, the canon narrative, README, CHANGELOG
- Release docs BEFORE the cut (the v2.19.0 practice): `intent/history/v3.0.0.md` + `docs/releases/3.0.0/RELEASE_NOTES.md`
- The v3.0.0 tag + GitHub release + brew formula live
- Fleet migration sequencing recorded (canary order, per-project residue expectations)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-12` heading (single source of truth). Do not restate ACs here.

## Dependencies

- All prior WPs.
