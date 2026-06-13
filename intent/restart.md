# Claude Code Session Restart -- narrative state

## Current state (2026-06-14)

v2.11.12 is the shipped baseline. Commit `fa90bb2` (main) landed ST0044 WP-01/02/03/08 (acceptance.md doc + `intent ac` / `intent at` instrumentation) plus the `normalise_st_id` Highlander fix (octal-safe + ST-prefixed-short). **In flight, uncommitted:** ST0044 WP-04 (the close-gate) is GREEN -- `intent ac gate` + `st done` / `wp done` wiring + template re-indent; `tests/unit/acceptance_close_gate.bats` 4/4 + regressions; ready to commit. WP-05 is next. ST0043 (rethink `intent upgrade`) follows ST0044.

## ST0044 -- acceptance.md + AC/AT process (active)

WP-01/02/03/08 committed (fa90bb2). WP-04 close-gate GREEN (uncommitted): `intent ac gate <stid>[/NN]` in `bin/intent_acceptance`, consulted by `st done` (scope = whole thread) and `wp done` (scope = the NN group) before they mutate; opt-in / legacy-safe (no acceptance.md or no live ACs -> done unchanged); no `--force`. The template re-indents its example AC/AT lines so freshly stamped STs are not self-gated. `tests/unit/acceptance_close_gate.bats` 4/4; regressions green; WP-04 ATs flipped green via `intent at`. The gate dogfoods on the thread itself: `ac gate ST0044` = 9/15 BLOCKED (correct -- WP-05/06/07 + sign-off open). Next: WP-05. Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

## ST0043 -- Rethink `intent upgrade` (next)

WIP. Architecture-B in `intent/st/ST0043/info.md`; ACs drafted in `intent/st/ST0043/acceptance.md`. Targets v2.12.0.

## Where detail lives

- `.claude/restart.md` -- WP-04 green-build spec + ratified gate mechanism.
- `intent/st/ST0044/` -- info / design / tasks / acceptance (live tracker).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.11.*.md` -- shipped-work ledger / narratives.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; user runs the full test suite externally (single-file bats fine); matts is the acceptance verifier this session.
