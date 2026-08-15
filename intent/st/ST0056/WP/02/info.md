---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-02
title: "Workspace and reified model: intentsvcs types, schema faces, store"
scope: L
status: Done
---

# WP-02: Workspace and reified model: intentsvcs types, schema faces, store

## Objective

Lay the cargo workspace and reify the model: the intentsvcs type layer as the single authored master, generating its three committed faces (JSON Schema via schemars, SQL DDL, GraphQL SDL), with the SQLite store bootstrapped and rebuildable.

## Deliverables

- Cargo workspace (crates per the WP-01 binary decision); CI with fmt/clippy/test on macOS + Linux
- intentsvcs entity types: project, steel_thread, work_package, acceptance_criterion (four states), acceptance_test, issue, doc_section, file_index, event_log
- The three schema faces generated + committed + CI drift-checked; `intent schema` prints them
- Store bootstrap: rusqlite (bundled, WAL, FTS5); a schema bump is a MIGRATION (D01 reversed, D36 -- "delete-and-rebuild on schema bump / no DB migrations ever" is void: the DB is durable truth and migrations are normal)
- Cloud seams: `project_id` UUID, principal on every facade signature (default `local`), append-only event log
- Serialise/deserialise laws under proptest; `serde_ignored`-style inbound refusal
- The rusqlite dep-graph guard (Highlander: one Cargo.toml)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-02` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-01 ratified.
