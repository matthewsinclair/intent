---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-03
title: "Ingest, views and sync engine"
scope: L
status: Not Started
---

# WP-03: Ingest, views and sync engine

## Objective

Build the bidirectional flow around the authored-once principle: strict ingest (validate-or-refuse-by-name) from committed canon to DB, deterministic view generation from DB to markdown, and change detection that keeps them honest.

## Deliverables

- Strict ingest: structured JSON parse + JSON Schema validation; invalid input refused with the finding named; prose ingested verbatim (FTS-indexed)
- View renderer: info.md, acceptance.md, steel_threads.md, todo.md -- deterministic and idempotent (same data, same bytes)
- Change detection: git-index-style stat scan (mtime/size, SHA-256 rehash on change) scoped to `intent/**` + named root files
- The skew check (doctor primitive): regenerate views, require empty diff -- a hand-edited view is caught, never outvoted silently
- Unparsed-state handling: a file that stops parsing (eg merge-conflict markers) is named by doctor and refused by commands that need it
- `intent ingest --from-md` scaffolding (shared with the WP-10 migrator)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-03` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-02.
