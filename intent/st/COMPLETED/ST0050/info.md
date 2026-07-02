---
verblock: "02 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.13.1
status: Completed
slug: intent-todo-a-flat-doing-todo-done-view-of-steel
created: 20260702
completed: 2026-07-02T21:36:06Z
---

# ST0050: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages

## Objective

Add `intent todo`, a CLI command that renders a single flat, nested checklist of a project's steel threads and their work packages -- bucketed into **DOING / TODO / DONE** and projected from each thread's real `status:`. It gives a "50,000-foot" view of the remaining work in one file (`intent/todo.md`) that no existing surface provides: `steel_threads.md` is a flat table, and per-project working notes are prose snapshots -- neither shows the WIP / not-started / just-done shape across steel threads _and_ work packages at a glance.

## Context

The need surfaced in a downstream project running many concurrent steel threads across several worker sessions: there was no one place to answer "what is in flight, what is queued, and what just landed" spanning both steel threads and their work packages. `intent st list` and `intent wp list <ST>` answer per-status and per-thread questions, but not the whole-portfolio nested view.

A working **read-path prototype already exists**, authored in the downstream project's checkout of Intent at `bin/intent_todo`: `intent todo update` and `intent todo [list]` generate and print `intent/todo.md`. It dispatches automatically through `intent`'s default `intent_<command>` rule (no change to `bin/intent`), reuses `intent_helpers` (Highlander -- no re-implemented frontmatter parsing), and is `shellcheck`-clean bar the repo-standard SC1091 source-follow note.

This steel thread captures that prototype and the remaining work to make `intent todo` a first-class, tested, released Intent feature: the mutation verbs (`done` / `notdone` / `toggle`), CLI/help integration, tests, docs, and release.

The design of record, the as-built prototype, the proposed work-package breakdown, and the open decisions are in `design.md`.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- **ST0046** (`add-modules-properly-to-the-intent-cli`) -- a sibling "generate a portfolio view from the project tree" concern. Both enumerate `intent/**` and render a derived, regenerable artifact; `intent todo` reads `intent/st/**` status the way a future `intent modules sync` reads modules.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
