---
verblock: "10 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.16.1
status: WIP
slug: add-in-intent-issues-command
created: 20260710
completed:
---

# ST0055: Add `intent issues` command

## Objective

Add `intent issues` -- a first-class, lightweight issue tracker built into the Intent CLI. Formalise the ad-hoc `intent/issues/` convention (used today in Utilz and Lamplight) into a supported command with five verbs:

    intent issues [list] [--kind open|closed|all]   # list all issues, or issues of a kind
    intent issues add TITLE                          # add a new issue, print its ID:TITLE
    intent issues show ID [--json]                   # show one issue, optionally as JSON
    intent issues close ID                           # OPEN -> CLOSED (aka done)
    intent issues open ID                            # CLOSED -> OPEN

An issue is the sub-steel-thread unit: a bug, follow-up, or observation too small for a full steel thread but worth tracking in-tree rather than leaving as prose drift.

## Context

Issues are already managed ad-hoc across the fleet under `intent/issues/{OPEN,CLOSED}/` with a hand-copied `_templ/0000-issue-title.md`, but with no tooling. That has produced predictable drift: two divergent on-disk shapes (Utilz keeps flat `NNNN-slug.md` files; Lamplight grew directory-per-issue with attachments, JSON sources, and per-issue WP/ subdirs), inconsistent status values (`RESOLVED` alongside `CLOSED`), hand-allocated ids, and no listing.

Intent already tracks large units (steel threads), their sub-units (work packages), and a flat cross-ST view (`intent todo`). Issues fill the gap **below** a steel thread and **outside** any ST -- ad-hoc trackable work. The growing "Open follow-ups / Backlog" prose in `restart.md` / `wip.md` is exactly the material that wants this home.

Greenfield in Intent's own repo (no `intent/issues/` exists yet); the fleet adopts the command on `intent upgrade`. See `design.md` for the on-disk canon, status model, and the decision to formalise a single shape from the two ad-hoc ones.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- [List any related steel threads here]

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
