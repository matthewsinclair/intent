---
verblock: "25 Jun 2026:v0.2: matts - Scope the lifecycle WP"
wp_id: WP-03
title: "Workstream lifecycle: ws archive + ws hygiene"
scope: Small
status: Done
---

# WP-03: Workstream lifecycle: ws archive + ws hygiene

## Objective

Complete the deterministic lifecycle: retire a workstream while keeping its history, and run a programmatic (non-Claude) structural lint + tidy over a whiteboard. The mechanical complement to the Claude-driven `/in-whiteboard archive` (which keeps the semantic "what is DONE" judgement).

## Deliverables

- `ws archive <wsid>`: move a node out of active discovery (eg `.archived/<wsid>/`), `.history/` intact, gone from `ws list`.
- `ws hygiene [<ws>]`: validate structure (parseable `wip.md` frontmatter + `.history/.gitkeep` per node; header + sentinel-or-entries per inbox); exit non-zero with a report on violation.
- Mechanical tidy only: normalise sentinels / format; warn on oversized boards (the 30k-token autopsy lesson) + stale heartbeats; never touch DOING content.
- AT-03.1..03.4 (archive round-trip; corrupt-fixture flag; DOING untouched; clean-board pass).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-03` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-01 (archive + hygiene operate on `ws new` output / the Protocol 3.0 shape).
