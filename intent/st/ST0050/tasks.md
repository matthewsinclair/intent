# Tasks - ST0050: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages

## Tasks

- [x] WP-01 — Read path + output: `generate` (minimal DOING/TODO/DONE markdown) + `generate_json` (keyed-by-bucket, nested work packages)
- [x] WP-02 — Mutation verbs: `done` / `notdone` / `toggle` wrapping `intent st` / `intent wp` (gate-inheriting); `parse_wp_specifier` extracted to `intent_helpers`
- [x] WP-03 — CLI integration: curated `todo` entry in `intent help`; fall-through dispatch guard
- [x] WP-04 — Tests: `tests/unit/intent_todo.bats` harness (the AC rollup)
- [x] WP-05 — Docs + release: MODULES.md row, README + usage-rules.md, CHANGELOG 2.14.0, impl.md / tasks.md
- [x] WP-06 — DONE flush / prune + ISO `completed:` timestamp + `## DONE:<T>` watermark

## Task Notes

Design decisions D1–D4 ratified by hv (2026-07-02); WP-06 verb placement + as-built sub-decisions ruled by cc and recorded in `design.md`. Acceptance contract in `acceptance.md` (23 ACs across the six WPs); the harness is `tests/unit/intent_todo.bats`.

## Dependencies

ST0050 and ST0051 (output width) are independent — `todo.md` is width-agnostic — and ship together in v2.14.0. WP-06's ISO `completed:` change touches `bin/intent_st`, so it landed after the WP-01/02 read path and mutation verbs were green.
