---
id: "0001"
title: pipe in a title corrupts markdown tables
date: 2026-07-10
reporter: matts
status: CLOSED
severity: medium
---

# 0001: pipe in a title corrupts markdown tables

## Tags

render-table, markdown, st, wp, issues, title-sanitisation

## Summary

A `|` character in a steel-thread, work-package, or issue title corrupted every markdown table the title landed in (`steel_threads.md`, `intent wp list`, `intent todo`). The shared `render_table` helper splits rows on `IFS='|'`, so a pipe inside a title cell was read as a column separator and shifted every downstream column.

## Reproduction

    $ intent wp new ST0055 "Create & list ... --kind open|closed|all"
    # -> the `open|closed|all` pipes split the wp-list row into extra columns;
    #    `intent wp list ST0055` renders a broken, misaligned table.

Surfaced live while creating ST0055's own WP-02.

## Root Cause

`render_table` (`bin/intent_helpers`) is a pipe-delimited row renderer (`IFS='|' read -r -a cells`). No caller sanitised free-text titles before they became `|`-joined cells, so any literal `|` in a title was ambiguous with the delimiter.

## Impact

Any table displaying a piped title (steel-threads index, wp list, todo) rendered with the wrong column count. Cosmetic, but the index files are committed, so the corruption persisted in-tree.

## Proposed Fix

Sanitise titles at the input boundary rather than at every render site: replace `|` with `/` when a title is first stored.

## Related

- ST0055 -- Add `intent issues` command (this fix shipped as its companion chore)

## Resolutions

FIXED (ST0055, 2026-07-10). Added `sanitize_title` to `bin/intent_helpers` (replaces `|` with `/`) and called it at the input boundary of every command that accepts a new title: `intent st new`, `intent wp new`, `intent issues add`. Guard test `tests/unit/title_pipe_sanitize_guard.bats` pins the helper plus the st/wp paths. Read-only views (eg `intent todo`) inherit already-sanitised titles. Filed here as the first dogfood record of the new `intent issues` command.
