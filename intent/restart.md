# Claude Code Session Restart -- narrative state

## Current state (2026-06-14)

v2.11.12 is the shipped baseline. ST0044 committed so far: `fa90bb2` (WP-01/02/03/08 + `normalise_st_id` fix); `95a3507` (WP-04 close-gate); `fc4f75a` (AC-08.1); `0b1b3b5` (WP-05 templates + show/edit, `st edit` -> pure emit-path). **In flight, uncommitted:** ST0044 WP-06 (skill / process integration) done -- the five-step + open/close gates documented in `working-with-llms.md` D11 (canon home), with light pointers in `/in-plan` / `/in-verify` / `/in-finish` (each referencing D11). AC-06.1 satisfied (matts sign-off); installed skills synced; ready to commit. `ac status ST0044` = 13/16 BLOCKED (AC-00.1 sign-off + WP-07 remain). WP-07 + self-close are next. ST0043 (rethink `intent upgrade`) follows ST0044.

## ST0044 -- acceptance.md + AC/AT process (active)

Committed: `fa90bb2` (WP-01/02/03/08); `95a3507` (WP-04 close-gate -- `intent ac gate` consulted by `st done` / `wp done`, opt-in / legacy-safe, no `--force`); `fc4f75a` (AC-08.1); `0b1b3b5` (WP-05 -- templates reference `acceptance.md`, `st show` / `st edit` learn the `acceptance` type, `st edit` reworked to pure emit-path replacing the macOS `open` / `$EDITOR` launch). WP-06 done (uncommitted): the five-step + open/close gates documented in `intent/docs/working-with-llms.md` D11 (canon home); light pointers in `/in-plan` (open-gate), `/in-verify` (red-first + witness RED), `/in-finish` (close-gate), each referencing D11 (Highlander -- thread-through chosen over a new `/in-acceptance` skill). AC-06.1 (non-test) satisfied with matts sign-off; installed skills synced. `ac status ST0044` = 13/16 BLOCKED (AC-00.1 sign-off + WP-07's AC-07.1/07.2 remain). Next: WP-07 dogfood + ST0044 self-close (satisfy AC-00.1 -> the close-gate lets `st done ST0044` through). Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

## ST0043 -- Rethink `intent upgrade` (next)

WIP. Architecture-B in `intent/st/ST0043/info.md`; ACs drafted in `intent/st/ST0043/acceptance.md`. Targets v2.12.0.

## Where detail lives

- `.claude/restart.md` -- ST0044 build state (WP-04/05 done, WP-06 next) + ratified mechanisms.
- `intent/st/ST0044/` -- info / design / tasks / acceptance (live tracker).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.11.*.md` -- shipped-work ledger / narratives.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; user runs the full test suite externally (single-file bats fine); matts is the acceptance verifier this session.
