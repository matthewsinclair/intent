---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-10
title: "Migration and fleet ingest harness"
scope: XL
status: WIP
---

# WP-10: Migration and fleet ingest harness

## Objective

Migrate v2 projects to v3 automatically and provably: the frozen legacy md parser as migrator, refuse-lossy residue discipline, and the fleet corpus as the acceptance fixture -- Intent's own tree first as canary.

## Deliverables

- Floor detection: v2.19.0+ migrates; older projects are pointed at v2's own `intent upgrade` first (two-hop; the v2 ledger is never reimplemented)
- The migrator: strict parse of the v2 estate -> refuse-lossy with residue named -> emit structured canon + regenerate views -> stamp 3.0.0 + `project_id` -> build DB -> converge canon -- one visible commit
- Rollback documented: `git revert` + the v2 formula (cheap because the migration is ONE named commit over a v2 estate git holds whole -- not because the DB is disposable, which is void under D01's reversal and D36)
- Hooks continuity verified: settings.json and `.claude/scripts` byte-untouched (0016)
- The fleet ingest harness: Intent (canary), then Lamplight/Utilz/Baize at named post-sweep revisions -- every artefact ingests or appears in the residue by name; the Lamplight AT baseline (1639 rows at `15dbccc92`) is the first fixture
- The legacy parser frozen once the fleet is over

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-10` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-06 + WP-07 (migrates the whole surface). The corpus snapshot depends on cc's v2.19.0 consumer sweeps completing first.
