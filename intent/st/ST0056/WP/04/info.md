---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-04
title: "intentsvcs facade: core command families"
scope: XL
status: Done
---

# WP-04: intentsvcs facade: core command families

## Objective

Build the outer API of intentsvcs -- the Highlander layer every skin (clap, GraphQL, MCP) calls -- for the core command families: st/wp lifecycle, ac/at operations, list/show/status views.

## Deliverables

- Facade signatures carrying a principal + project context; typed errors rendering remedies with full cause chains (the Lamplight DD-10 / `cause_chain` pattern)
- st family: new/list/show/start/done/cancel/edit-adjacent ops with transactional multi-file writes (canon + views + DB in one operation)
- wp family: new/list/start/done
- ac/at families: the four AC states (satisfy, descope/rescope, withdraw/reinstate), AT set/lint, the close-gate read from the model
- Status rollups + todo projection as queries
- Every verb lands with its dual-path conformance test stub (activated when intentd exists in WP-08)
- Event-log envelopes written by every mutation (AC-04.5/AT-04.5, renumbered in from WP-02 at its close -- the envelope test could not exist before these verbs)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-04` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-03.
