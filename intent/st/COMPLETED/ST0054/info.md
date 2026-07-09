---
verblock: "09 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.16.0
status: Completed
slug: update-intent-for-latest-usage-rules-md-format
created: 20260709
completed: 2026-07-09T09:11:14Z
---

# ST0054: Update Intent for latest usage-rules.md format

## Objective

Bring Intent's `usage-rules.md` handling and its `usage_rules` (Hex library) interop guidance up to date with the library's v1.x model. The `usage_rules` package was rebuilt at v1.0 (2026-02-07): configuration moved into `mix.exs` (the `:usage_rules` project key), the CLI-argument form was removed, and the tool now generates Claude Code agent **skills** into `.claude/skills/`. Intent's interop doc, the `/in-standards` skill, and the `_usage-rules.md` template still describe the pre-v1.0 (v0.1.x) model. Align all three with v1.x, and define Intent's official policy for the `.claude/skills` overlap the library now creates with Intent's own curated skills.

## Context

Surfaced during a Laksa deps-hygiene sweep (2026-07-09) that bumped `usage_rules` 0.1.26 -> 1.2.6 (the current Hex release). Two distinct artifacts share the filename `usage-rules.md`, and Intent's docs conflate them:

- **Intent's `usage-rules.md`** -- a hand-authored project DO / NEVER contract, templated by `lib/templates/llm/_usage-rules.md`, linked from the generated `AGENTS.md`, never itself generated. Hand-authored by design.
- **The library's `usage-rules.md`** -- per-dependency rule files (`deps/<pkg>/usage-rules.md`, plus topical `deps/<pkg>/usage-rules/<topic>.md` folders since v1.x) that the library aggregates into a target file or repackages as agent skills.

The v1.0 rebuild changed the library in three ways Intent has not caught up with:

1. **Config-driven, not CLI-driven.** `mix usage_rules.sync` now rejects task arguments; all setup is the `:usage_rules` key in `mix.exs` `project/0` (`file:`, `usage_rules:`, `skills:`). There is no zero-config mode -- Intent's root `usage-rules.md` is NOT auto-included unless configured.
2. **Agent-skills generation -- a live collision.** The library writes `SKILL.md` files (default `.claude/skills/`) for eg `ash-framework`, `phoenix-framework`. Intent already ships curated skills in the same directory and domains (`in-ash-ecto-essentials`, `in-phoenix-liveview`, `in-elixir-essentials`). Two tools now want to own dep-rule delivery in one directory.
3. **Topical sub-rule folders.** Deps ship a `usage-rules/` directory of topic files alongside the single `usage-rules.md`; Intent's `/in-standards` points only at the single file.

Both tools are current (Intent 2.16.0, usage_rules 1.2.6); the gap is Intent's _model_ of the library, not either tool's version. The intended posture is that Intent projects stay Intent-native (curated skills plus on-demand deps reads); this ST makes Intent's guidance say so accurately, then ships as 2.16.1 so Laksa can re-integrate against it.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- Surfaced by the Laksa deps-hygiene sweep (2026-07-09). Laksa stays Intent-native (does not wire `usage_rules.sync`) and re-integrates via `intent claude upgrade --apply` once this ships as Intent 2.16.1.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
