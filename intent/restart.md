# Claude Code Session Restart -- narrative state

## Current state (2026-07-02)

**v2.14.0 is SHIPPED (ST0050 + ST0051).** Tag `v2.14.0` (release commit `c7842f1`) on both remotes + GitHub release; Intent self-upgraded 2.13.1 -> 2.14.0 clean (post-tag wrap `a6f6662`, `intent doctor` green). A minor -- new command surface. **ST0050 (`intent todo`)**: a flat DOING/TODO/DONE projection of `intent/st/**` into `intent/todo.md` that cannot drift (checkboxes derived from real `status:`) -- minimal markdown + keyed-by-bucket `--json`; `done`/`notdone`/`toggle` verbs wrapping `intent st/wp` (inheriting the ST0048 close-gate); `done --flush`/`--prune` + the `## DONE:<T>` sticky watermark; `completed:` upgraded to an ISO 8601 timestamp (legacy `%Y%m%d` tolerated everywhere it is read). Six WPs, closed 23/23; dogfooded (first ISO `completed:` stamp). **ST0051 (output width)**: `intent st sync --write` hardcoded 80 -> config `dft_width` (default 120) + `get_default_width`; stdout keeps terminal width; `--width` overrides. vc independently audited ST0050: PASS -- ship-clean. matts accepted the sticky-watermark. Detail: `intent/st/COMPLETED/ST0050/` + `ST0051/`; narrative `intent/history/v2.14.0.md`; notes `docs/releases/2.14.0/`.

**v2.13.1 (prior) SHIPPED (ST0048 + ST0049).** Acceptance close-gate fail-by-default (a missing or zero-AC `acceptance.md` BLOCKS `st done`/`wp done`; `acceptance: exempt` the sole escape); comprehensive retroactive 2.13.0 + 2.13.1 release notes (`docs/releases` resumed; no 2.10-2.12 backfill). Detail: `intent/st/COMPLETED/ST0048/` + `ST0049/`.

**v2.13.0 (prior) SHIPPED (ST0047).** `intent claude start` + `intent claude ws` -- the MAAC whiteboard launcher + workstream lifecycle, first-class in Intent. Fleet swept to 2.13.0 (2026-06-25). Detail: `intent/st/COMPLETED/ST0047/`.

## v2.14.1 follow-ups (from the vc audit -- non-blocking)

1. **AC-01.8 enumeration Highlander** -- `intent todo`'s markdown + JSON emitters each re-walk `intent/st/**` and duplicate the `norm < since` predicate; AC-01.8 over-claims "no second traversal". Unify the enumeration (one pass feeds both) or reword the AC (a weakening -> matts' nod). Field-extraction Highlander is already fine.
2. **AT-name traceability** -- `acceptance.md` AT `::names` are notional, not the real bats `@test` names.
3. **`intent upgrade` false-no-op + `scripts/release` `confirm()`** -- in-session `./bin/intent upgrade` reported "already at 2.14.0" while config.json was 2.13.1; and the push `confirm()` reads raw stdin + strict `[yY]`, so a stray escape aborted the first push. Confirm `detect_project_version` can't skip the config stamp for a fleet member; read `/dev/tty` in `confirm()`.

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0050/` + `ST0051/` -- closed thread docs (this release).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.14.0.md` -- shipped-work ledger / narrative.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
