---
verblock: "03 Jul 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: "Author rule library seed"
scope: Medium
status: Done
---

# WP-02: Author rule library seed

## Objective

Author the starter `IN-AU-*` rule library -- the mechanical `style` tier first, then the judgment `craft` tier -- and stand up the net-new mechanical trope surface (design.md D5). Regenerate the rule index. No trope catalogue is re-authored; only the mechanical detection layer is added, sourced Highlander from `llm-tropes`.

## Deliverables

- `rules/author/style/<slug>/RULE.md` (mechanical/default tier): banned filler + `eg`-not-`e.g.` + house style; no vanity metrics + T-shirt sizing; chapter/module front-matter + learning-objectives presence; heading hygiene; the mechanical trope pass.
- `rules/author/craft/<slug>/RULE.md` (judgment/on-instruction tier): voice/register consistency; cross-chapter continuity; citation/attribution -- Detection = critic-as-reader; full-trope diagnosis references `/in-detrope` (no fork).
- Mechanical trope surface (D5): resolve its form (⚙ rule Detection heuristics vs a vendored `data/trope-indicators` file), sourced from `llm-tropes`.
- Textual Bad/Good examples in each `RULE.md` (prose has no runtime -- CI-LIMITATIONS textual-only).
- Regenerated `rules/author/index.json` (+ top-level index).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-02` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP01 (the `AU` code must validate before `IN-AU-*` IDs can exist). Blocks WP03 (the critic loads this library).
