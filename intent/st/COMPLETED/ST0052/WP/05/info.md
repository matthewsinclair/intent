---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
wp_id: WP-05
title: "Skill and dispatch wiring"
scope: Medium
status: Done
---

# WP-05: Skill and dispatch wiring

## Objective

Wire the author pack into the session + review flow so a project that declares `author` activates the pack automatically: the essentials skill loads on `/in-session`, and `/in-review` dispatches `critic-author` (and, per D7, NOT the code critics unless code languages are also declared).

## Deliverables

- `intent/plugins/claude/skills/in-author-essentials/SKILL.md` -- the authoring pipeline (outline -> draft -> detrope -> revise -> structural check; detrope-every-step).
- `/in-review` SKILL.md -- add the `author -> critic-author` mapping to the language-dispatch list + a `Task(subagent_type="critic-author", ...)` example (D7).
- `/in-session` SKILL.md -- add the `author` row to the languages fan-out so `/in-author-essentials` loads when `author` is declared (D7).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-05` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP03 (the critic must exist to dispatch to). The code-critic exclusion is already free from `/in-review`'s per-language dispatch; this WP is the author-side registration.
