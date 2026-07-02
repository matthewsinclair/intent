# cc archive -- 20260702

Archived from the live cc board when the v2.14.0 homerun completed (ST0050 + ST0051 closed, release handed to matts).

## v2.14.0 homerun -- DONE

**ST0050 (`intent todo`) -- CLOSED 23/23**, `intent/st/COMPLETED/ST0050/`. A flat DOING/TODO/DONE projection of `intent/st/**` into `intent/todo.md` that cannot drift (checkboxes derived from real `status:`). Six work packages:

- **WP-01 read path + output** -- `generate` (minimal `todo.md`: bare `## DOING/TODO/DONE` + data, no title/legend/provenance) + `generate_json` (keyed-by-bucket, nested `work_packages`), sharing one enumeration (Highlander).
- **WP-02 mutation verbs** -- `done`/`notdone`/`toggle` wrap `intent st`/`wp` and inherit the ST0048 close-gate (D2 proven by test); `notdone` reopens to WIP (D1). `parse_wp_specifier` extracted to `bin/intent_helpers` (Highlander; `1/1` == `ST1/1` == `ST0001/01`).
- **WP-03/04** -- curated `todo` entry in `intent help` over the existing `intent_<command>` fall-through dispatch; the `intent_todo.bats` harness (18 tests) as the AC rollup.
- **WP-05** -- README (`See what's in flight`), usage-rules.md (`### Todo view`), CHANGELOG `## [2.14.0]`, `docs/releases/2.14.0/RELEASE_NOTES.md`, `intent/history/v2.14.0.md`, impl.md/tasks.md.
- **WP-06 DONE lifecycle** -- `## DONE:<T>` flush watermark; `done --flush` (clear the view) / `done --prune` (emit items to stdout for archiving, then flush); `completed:` upgraded to an ISO 8601 UTC timestamp for exact flush, legacy `%Y%m%d` tolerated on every read (`normalize_completed`; `steel_threads.md` date-part truncation).

Commits: `fa43630` (WP-01/02), `b68a37f` (WP-06), `6493215` (WP-03/04), `a10f279` (WP-05), `c145171` (close). Dogfooded -- ST0050 got the first ISO `completed:` stamp (`2026-07-02T21:36:06Z`). Full suite green (matts, external).

**ST0051 (output width) -- CLOSED**, `50e248f`. `intent st sync --write` hardcoded width 80 -> `get_default_width` (config `dft_width`, default 120); stdout keeps terminal width; `--width` overrides; dead list-branch pruned. `output_width.bats` 5/5.

## Rulings (as-built)

- hv RATIFIED D1-D4 (2026-07-02). cc RULED WP-06: verb placement `done --flush/--prune` (flags on `done`, disambiguated from `done <spec>` by argument); membership `completed >= <T>` inclusive; `<T>` sticky (authoritative in the heading, `update` preserves, only flush advances; first-gen default = start-of-today UTC -> "swept daily" became "swept on flush"); UTC timestamps; `--prune` items to stdout / note to stderr; `flush_watermark` the shared advance. Recorded in `ST0050/design.md`; the sticky-watermark model flagged to matts for acceptance-verify.
- ST0050 + ST0051 independent; both ship in v2.14.0 (a minor -- new command surface).

## vc collaboration

vc (Validation Claude) delivered an advisory D1-D4 review (hv-ratified) with two build-notes -- (1) `st start` reopen does not clear `completed:`; (2) `completed:` was `%Y%m%d` -- both carried into WP-06 (the sticky-watermark `>=` model supersedes the same-day-reopen concern; `completed:` upgraded to ISO). vc held the as-built audit for the close trigger, which cc fired at close (`vc/inbox.cc.md`, 21:32).
