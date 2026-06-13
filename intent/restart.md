# Claude Code Session Restart -- narrative state

## Current state (2026-06-14)

v2.11.12 is the shipped baseline. ST0044 committed so far: `fa90bb2` (WP-01/02/03/08 -- acceptance.md doc + `intent ac` / `intent at` instrumentation + `normalise_st_id` octal/ST-prefix fix); `95a3507` (WP-04 close-gate); `fc4f75a` (AC-08.1 satisfied). **In flight, uncommitted:** ST0044 WP-05 (template references + show/edit) is GREEN -- ST/WP `info.md` templates point at `acceptance.md` (no restated ACs); `st show` / `st edit` / `st show all` learn the `acceptance` type; `st edit` reworked to pure emit-path (global, no editor launch); `tests/unit/st_new_acceptance.bats` 2/2 + regressions; ready to commit. WP-06 is next. ST0043 (rethink `intent upgrade`) follows ST0044.

## ST0044 -- acceptance.md + AC/AT process (active)

WP-01/02/03/08 committed (`fa90bb2`); WP-04 close-gate committed (`95a3507`) -- `intent ac gate <stid>[/NN]` in `bin/intent_acceptance`, consulted by `st done` (whole thread) and `wp done` (NN group) before they mutate; opt-in / legacy-safe; no `--force`. AC-08.1 satisfied (`fc4f75a`). WP-05 GREEN (uncommitted): ST/WP `info.md` templates point at `acceptance.md` and restate no ACs (Highlander); `st show` / `st edit` / `st show all` learn the `acceptance` type; `st edit` reworked to pure emit-path (validates the type, prints the file's absolute path, global -- replaces the macOS `open` / `$EDITOR` launch, which is why edit had no tests). `tests/unit/st_new_acceptance.bats` 2/2; regressions green; WP-05 ATs flipped green via `intent at`. The gate dogfoods on the thread itself: `ac status ST0044` = 12/16 BLOCKED (correct -- AC-00.1 sign-off + WP-06/07 open). Next: WP-06 (skill / process integration). Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

## ST0043 -- Rethink `intent upgrade` (next)

WIP. Architecture-B in `intent/st/ST0043/info.md`; ACs drafted in `intent/st/ST0043/acceptance.md`. Targets v2.12.0.

## Where detail lives

- `.claude/restart.md` -- ST0044 build state (WP-04/05 done, WP-06 next) + ratified mechanisms.
- `intent/st/ST0044/` -- info / design / tasks / acceptance (live tracker).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.11.*.md` -- shipped-work ledger / narratives.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; user runs the full test suite externally (single-file bats fine); matts is the acceptance verifier this session.
