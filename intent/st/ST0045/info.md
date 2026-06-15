---
verblock: "15 Jun 2026:v0.1: matts - Initial version"
intent_version: 2.11.14
status: WIP
slug: update-whiteboard-for-per-workstream-files
created: 20260615
completed:
---

# ST0045: Update whiteboard for per-workstream files

## Objective

Restructure the `in-whiteboard` coordination protocol from flat shared files (Protocol 2.0) to per-node directories with single-writer inboxes (Protocol 3.0). Eliminate the contention, unbounded growth, and cleanse-difficulty that the shared `asks.md` / per-stream files / `lamplight.md` accreted under N concurrent writers; make the human a first-class `hv` (hypervisor) node; rewrite the `in-whiteboard` skill; and roll the change out as a new Intent release. Lamplight is the reference implementation (migrated live 2026-06-15).

## Context

Under Protocol 2.0 every coordination surface was a SHARED file with N writers: `asks.md` (all streams append and remove), the per-stream files (touched by peers during cross-stream `archive`), and `lamplight.md` (the shared platform-edit channel). The cost compounded with stream count -- edit collisions (`modified-since-read`), cleanse that required cross-stream judgment plus coordination to avoid collision, and files growing to tens of KB that were reloaded on every `pickup`, chewing context. Because cleanse was hard it was deferred, so the files ballooned.

Protocol 3.0 gives every file exactly one writer. Each participant is a **node** with its own directory `intent/whiteboard/<node>/` holding `wip.md` (its live board, single-writer), `inbox.<sender>.md` (one per other node, each written only by that sender and cleansed only by the owner), and `.history/YYYYMMDD/`. Point-to-point messaging routes to `<recipient>/inbox.<me>.md`; broadcast is an `announce` that writes one line to every peer's inbox (subsuming `lamplight.md`); the recipient owns its inbox lifecycle (read -> action -> clear), so cleanse is single-owner and collision-free; and `archive` only ever touches your own directory. The human becomes a first-class node, conventionally `hv` (the hypervisor) -- generic in Intent, not project-specific -- giving escalations a real address (`hv/inbox.<me>.md`).

See `design.md` for the full model, the skill diff, the migration playbook, the rollout steps, and the open considerations (skill blast-radius, the inbox append/cleanse race, naming, archival discipline).

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- (none -- standalone change to the `in-whiteboard` skill + protocol)

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
