---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-06
title: "CLI parity long tail"
scope: XL
status: WIP
---

# WP-06: CLI parity long tail

## Objective

Port the remaining command surface behind the conformance ladder, command by command, until the v2 suite cannot tell the difference except where the ratified register says so.

## Deliverables

- issues, todo, doctor, config, lang, init/bootstrap, organize, learn, llm, modules, ext, plugin, treeindex/fileindex, st_zero, info, help
- Each command lands facade + CLI + conformance results together; deviations recorded in the register at land time, never discovered later
- doctor rebuilt as model/DB integrity queries + file checks (including the WP-03 skew check and unparsed-state reporting)
- The retire/deviate register complete across the surface

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-06` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-05. Interleaves freely with WP-07 after WP-04.
