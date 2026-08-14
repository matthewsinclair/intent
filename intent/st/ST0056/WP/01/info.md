---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-01
title: "Design canon: architecture, data model, migration and parity specs"
scope: L
status: Done
---

# WP-01: Design canon: architecture, data model, migration and parity specs

## Objective

Turn the ratified rubber-duck session into the reviewable design canon: everything hv signs off before a line of Rust exists (document-before-code). design.md carries the architecture, truth model, decision log D01-D17 and alternatives; this WP completes the remaining specs and closes the open questions.

## Deliverables

- design.md at review quality (landed 2026-08-14; maintained through the check-in)
- The data-model spec: entity tables, `thread.json` / `issues/<n>.json` shapes, first JSON Schema draft
- The migration spec: two-hop flow, refuse-lossy residue format, canary + fleet-corpus plan
- The parity contract: v2 command-surface inventory (every command/flag/output/exit code) + the keep/retire/deviate register format
- The full-ladder acceptance contract: WP-02..12 AC/AT groups in acceptance.md
- Open questions closed as decision-log additions: one binary vs two, launchd label, 3.0.0 subscription extent, `.cache` layout

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-01` heading (single source of truth). Do not restate ACs here.

## Dependencies

- None. Gate for all subsequent WPs. The command-surface inventory and BATS classification can be delegated (IC-friendly, design-neutral, v2-side).
