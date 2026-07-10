---
verblock: "15 Jun 2026:v0.1: matts - Initial version"
intent_version: 2.12.0
status: Not Started
slug: add-modules-properly-to-the-intent-cli
created: 20260615
completed:
---

# ST0046: Add modules (properly) to the intent cli

## Objective

Make `intent modules` a complete registry mechanism, not just a guardrail.
Today it can CHECK (and warn at file-creation via the advisory write hook) but
it cannot (a) reliably detect a pre-existing backlog of unregistered modules,
nor (b) mechanically register them. Add a working full-tree unregistered-module
detector and a generator (`sync` / `--write`) so a project's
`intent/llm/MODULES.md` can be brought to -- and kept at -- completeness by the
tool rather than by hand.

## Context

The Highlander Rule requires every module to be registered in
`intent/llm/MODULES.md`. `intent modules` enforces this at creation time (the
advisory write hook) and detects stale rows (registry entries whose files are
gone). Two gaps surfaced on Lamplight (a large Elixir umbrella):

1. **`check` misses unregistered files.** `intent modules check` reports
   `ok: registry matches filesystem` even when real modules are absent from the
   registry. Proven: `apps/lamplight/lib/lamplight/ingestor/parser/action.ex`
   exists on disk and `intent modules find Parser.Action` returns no matches,
   yet `check` is green. An entire backlog (Lamplight's renamed gen4 authoring
   substrate -- `Ingestor.ContentIngestor*`, `Authoring.*`, `Content.IRData`,
   the `Content.*` publish family) is invisible to the guardrail.

2. **No generator.** `--register` only prints guidance; there is no
   `sync` / `--write` that adds the missing rows. The only path to completeness
   is hand-authoring `MODULES.md` -- error-prone, and exactly what the
   Highlander tooling should remove.

So a project that accrued modules before adopting the hook, or did a bulk
rename/refactor (eg an umbrella reshape), has no mechanical path to reconcile
its registry. This ST closes both gaps.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- None directly. Motivating case: the Lamplight gen4 authoring-substrate rename
  left ~40 modules unregistered with no CLI path to reconcile the registry.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
