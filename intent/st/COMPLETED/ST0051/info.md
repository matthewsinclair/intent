---
verblock: "02 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.13.1
status: Completed
slug: intent-output-width-dft-width-config-for
created: 20260702
completed: 20260702
---

# ST0051: intent output width: dft_width config for generated files, terminal width for stdout

## Objective

Give Intent a single, config-driven default width for generated files, and make every command that writes a file use it while commands that emit to the terminal use the live terminal width. Concretely: add `dft_width` (default 120) to project config; introduce a Highlander `get_default_width` helper (the file-side sibling of the existing `get_terminal_width`); and route the `intent st` table renderer by destination -- `sync --write` (the `steel_threads.md` file) at `dft_width`, `list` (stdout) at terminal width, with an explicit `--width N` overriding both.

## Context

Surfaced while capturing ST0050 (`intent todo`): `intent st new` regenerated `steel_threads.md` in a narrow terminal and the slug column truncated (`add-modules-properly-t...`) in the tracked file. The file's width should not depend on the terminal that happened to run the command -- a file is read anywhere, so it wants a stable, generous default; only stdout should track the live terminal. Intent already has `get_terminal_width()` (`bin/intent_helpers:58`, Highlander) for the stdout side; the file writer wrongly reuses it (`bin/intent_st:844`, via the shared `sync`/`list` render path), which is the bug. This thread adds the file-side counterpart plus the destination-based rule, and a config knob so the default is tunable per project. The only current width-sensitive (columnar) generator is `steel_threads.md`; `intent todo`'s `todo.md` is a nested checklist and is width-agnostic. The thread also audits the remaining generators and documents the file-vs-stdout width principle so future generators comply.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- **ST0050** (`intent-todo-...`) -- co-shipping in the same release (2.14.0); capturing it surfaced this width bug. Independent of this thread: `todo.md` is a checklist, not a columnar table, so it does not consume the width helper.
- **ST0046** (`add-modules-properly-to-the-intent-cli`) -- its slug is the one currently truncated in the `steel_threads.md` index; the fix restores it at width 120.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
