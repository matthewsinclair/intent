---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.14.0
status: WIP
slug: author-project-type-pack
created: 20260703
completed:
---

# ST0052: Author project-type pack

## Objective

Add non-code "project-type" packs to Intent and deliver the `author` pack (books, courseware, long-form authoring with Claude) end to end -- `intent lang init author` canon, an `IN-AU-*` rule library, a `critic-author` reader, and an `/in-author-essentials` skill -- reusing the existing `languages` axis. Establish the reusable pattern that a later `content` (web-content editing) pack will follow.

## Context

hv wants Intent to support project kinds beyond programming languages: a `content` pack (editing web content) and an `author` pack (original long-form -- books, courseware, blog). Ratified direction (2026-07-03): reuse the existing `languages` axis rather than a parallel `domain` field -- one pack mechanism (rules + critic + skill + canon), so a repo can carry `languages: [elixir, author]` and both packs compose. `author` ships first: most of its rule set is already written as prose in the global `CLAUDE.md` (banned filler, `eg`-not-`e.g.`, no vanity metrics, T-shirt sizing, detrope-at-every-step), and there is a live corpus (the agentic course, blog drafts) to dogfood on.

Grounding during scoping surfaced that "reuse the `languages` axis" spans TWO layers with different openness: the canon-template + config + `/in-session` layer is open (template-driven, no allowlist -- `intent lang init author` works once a template dir exists), but the rule-library + critic + ID layer carries a closed language-code enum (`AG|EX|RS|SW|LU|SH`) that needs a small, agreed schema bump to admit `AU`. See design.md D2.

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
