---
verblock: "29 Jun 2026:v0.2: matts - Objective + context (acceptance-exempt: authorial)"
intent_version: 2.13.0
status: WIP
slug: comprehensive-2-13-0-release-note-how-maac-works
created: 20260629
completed:
---

# ST0049: Comprehensive 2.13.0 release note: how MAAC works

## Objective

Author the comprehensive, retroactive v2.13.0 release note -- the canonical user-facing "how the multi-agent agentic coding (MAAC) workstream works" document. v2.13.0 introduced the MAAC command surface but shipped without a release note, and the `docs/releases/` series had lapsed after 2.9.0. This note covers the system end to end: the Protocol 3.0 whiteboard, the `/in-whiteboard` skill, and the `intent claude ws` / `intent claude start` commands.

## Context

`docs/releases/` lapsed after 2.9.0 (2.10.0 through 2.13.0 never got notes). hv ruled: do not backfill 2.10-2.12, but author a comprehensive note for 2.13.0 because it introduced the MAAC workstream system, which is complex and needs explicit how-it-works docs. This ships in the 2.13.1 cut alongside the ST0048 gate fix.

Pure authorial task: this thread carries `acceptance: exempt` (the first real consumer of ST0048's exemption marker) -- hv reviews the note's accuracy as part of the release, rather than via a formal AC contract. Source: `intent/history/v2.13.0.md` (ST0047) + `v2.12.0.md` (ST0045 Protocol 3.0) + the `/in-whiteboard` skill + `intent/whiteboard/README.md`.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- ST0047 -- shipped the `intent claude ws` / `start` command surface this note documents.
- ST0045 -- shipped Whiteboard Protocol 3.0 (v2.12.0), the model this note explains.
- ST0048 -- the acceptance-gate fix; its `acceptance: exempt` marker is dogfooded by this thread; both ship in 2.13.1.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
