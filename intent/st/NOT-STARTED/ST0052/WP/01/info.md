---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: "AU language-code schema bump"
scope: Small
status: Not Started
---

# WP-01: AU language-code schema bump

## Objective

Admit `author` (code `AU`) as a first-class rule-library language by adding it to the ID/validation layer, so `IN-AU-*` rule IDs validate and the author rule pack + critic can exist. This is the foundation WP02 and WP03 build on. Scope is the ID/validation layer only; the headless runner (`bin/intent_critic`) is deliberately NOT touched (deferred with the headless prose gate, design.md D4).

## Deliverables

- `rules/_schema/rule-schema.md` -- `language` enum includes `author`.
- `rules/_schema/id-scheme.md` -- language-codes table gains `AU`; the validator-regex doc (~:155) gains `AU`.
- `rules/_schema/index-generator.md` -- the duplicate validator-regex doc (~:128) gains `AU`.
- `intent/plugins/claude/bin/intent_claude_rules` -- validator regex (the check + the error string, ~:161-162) gains `AU`.
- Guard test under `tests/unit/` asserting `IN-AU-STYLE-001` (and similar) now validates AND a malformed id still fails, so the enum widening cannot silently regress.
- `bin/intent_critic` NOT modified (out of scope, D4).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-01` heading (single source of truth). Do not restate ACs here.

## Dependencies

- None (first WP). Blocks WP02 and WP03 -- their `IN-AU-*` IDs and `critic-author` validation depend on the widened enum/regex.
