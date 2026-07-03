---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: "critic-author subagent"
scope: Medium
status: Done
---

# WP-03: critic-author subagent

## Objective

Build `critic-author` -- a two-tier prose reader that runs the mechanical tier by default and hands off to `/in-detrope` for full LLM diagnosis only under direct instruction (design.md D3, D5). It reports; it never modifies (critic contract).

## Deliverables

- `intent/plugins/claude/subagents/critic-author/agent.md` -- own `style`/`craft` categories + mode verbs (D6); tools Read/Grep/Bash (no Write); report format per the critic contract; mechanical tier by default incl. the mechanical trope pass; emits a `/in-detrope` handoff recommendation (the diogenes-handoff pattern) for full diagnosis under direct instruction.
- `intent/plugins/claude/subagents/critic-author/metadata.json`.
- Registration in `intent/plugins/claude/subagents/.manifest/global-agents.json`.

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-03` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP01 (`AU` validates) and WP02 (the rule library the critic loads, incl. the mechanical trope surface).
