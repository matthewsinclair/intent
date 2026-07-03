---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: "intent lang init author canon"
scope: Small
status: Done
---

# WP-04: intent lang init author canon

## Objective

Make `intent lang init author` succeed -- supply the canon template so a project can declare itself an author project (adds `author` to config `languages` + installs the concretised author RULES canon).

## Deliverables

- `intent/plugins/agents/templates/author/RULES.md` -- the concretised author-rules canon (copied to `intent/llm/RULES-author.md` on init).
- `intent/plugins/agents/templates/author/ARCHITECTURE.md` -- book/course information architecture (parts / chapters / modules / learning objectives).
- Verified: `intent lang init author` installs `RULES-author.md`, appends the Language Packs entry to `intent/llm/RULES.md`, and adds `author` to `intent/.config/config.json` `languages`.
- Optional (hv-flagged wart): fix `intent lang init <missing>` writing the RULES.md marker block before it validates the template exists (validate first, then mutate).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-04` heading (single source of truth). Do not restate ACs here.

## Dependencies

- No hard dependency (can run alongside WP02/WP03); pairs with WP05 for the end-to-end declare-then-activate UX.
