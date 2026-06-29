---
verblock: "29 Jun 2026:v0.2: matts - Objective + context"
intent_version: 2.13.0
status: WIP
slug: acceptance-close-gate-fails-empty-or-missing
created: 20260629
completed:
---

# ST0048: Acceptance close-gate fails empty or missing contract

## Objective

Make the acceptance close-gate refuse to mark a WP/ST done when it has no verifiable contract -- zero acceptance criteria, or no `acceptance.md` at all -- instead of vacuously passing. Provide one explicit, visible exemption (`acceptance: exempt`) as the sole escape hatch. Default = enforced.

## Context

The acceptance model (`intent ac` / `intent at`, consumed by `intent st done` / `intent wp done`) computes "done" as "every in-scope AC is satisfied." With zero ACs that predicate is vacuously true, so a unit closes with nothing to verify it against -- a real, repeated hole reported from a downstream Intent project. This is No-Silent-Errors applied to the acceptance layer: an absent contract is a failure that must surface, not a quiet pass.

The change reverses the deliberate opt-in-by-presence behaviour (matts, 2026-06-16; test `AT-04.4`). hv ratified it on 2026-06-29 as a shipped-as-broken fix, shipping in patch **2.13.1** with a migration-led release note. Companion documentation (the comprehensive 2.13.0 MAAC release note) is tracked separately and ships in the same 2.13.1 cut. Full design, decisions, and the open WP-granularity fork are in `design.md`; the ratified boundary is in `acceptance.md`.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- ST0044 -- introduced `acceptance.md`, the AC/AT process, and the opt-in close-gate this thread hardens.
- ST0049 (pending) -- comprehensive 2.13.0 MAAC release note; ships in the same 2.13.1 cut.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
