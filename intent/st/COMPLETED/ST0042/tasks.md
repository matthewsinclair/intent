# Tasks - ST0042: Fable 5 Review of Intent codebase

## Tasks

- [x] Write review methodology (design.md) and coverage map
- [x] Mechanical sweeps: placeholder drift, hardcoded versions, shellcheck, size outliers
- [x] Dimension reviews 1-8 per coverage map
- [x] Dedup + I-verify load-bearing findings
- [x] Findings into design.md; MFIC leak notes into impl.md
- [x] Draft proposed WPs with t-shirt sizes (intent wp new)
- [x] Review gate: user adjudication (2026-06-11: audit retired, WP-06 scoped, patch cadence)
- [x] Execute approved WPs (all nine, single arc; see impl.md execution notes)

## Task Notes

Slate WP8 (upgrade rethink) became ST0043 rather than a WP here; slate WP9/WP10 map to WP/08 and WP/09 (see design.md directory mapping).

## Dependencies

WP execution order was dependency-driven: test isolation (09a) before anything that runs the suite; config eval (01) before the config-parsing consolidation (05b); canon docs (08) before the prune (06) so doc edits did not race.
