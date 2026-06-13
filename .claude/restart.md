# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate. (Languages: shell only; no whiteboard in this project.)
2. **Verify the tree.** v2.11.12 is the shipped baseline. Commit `fa90bb2` (on `main`) landed ST0044 WP-01/02/03/08 + the `normalise_st_id` fix. On top sits **uncommitted** WP-04 (the close-gate, GREEN -- ready to commit) + the three resume docs. Dirty tree is expected.
3. **Read `intent/st/ST0044/acceptance.md`** -- the live AC/AT tracker -- plus `design.md` / `tasks.md`.

## Committed (fa90bb2, main)

ST0044 WP-01/02/03/08 + the normaliser fix:

- `acceptance.md` is a default steel-thread doc (template stamped via the doc-set `*.md` glob). `intent ac` / `intent at` instrumentation in `bin/intent_acceptance`: `ac list/status/satisfy`, `at list/red/green/na`, `done`/`notdone` aliases; `green` only from `red`; model-A grammar (non-test ACs carry inline `-- evidence: <ref> -- satisfied: yes|no`; test-backed ACs computed from a green covering AT). Dispatch in `bin/intent` (ac/at with `set -- "$COMMAND" "$@"` noun re-inject).
- Highlander fix: `intent at` now routes the st-id through `normalise_st_id` (helpers:286), which was made octal-safe (`10#` -- `0044` was silently resolving to ST0036 under /bin/bash) and given an ST-prefixed-short branch (`ST44` -> `ST0044`). `tests/unit/helpers.bats` guards it under /bin/bash.
- Tests committed: `intent_acceptance_cli.bats` (7), `helpers.bats` (4); regressions green (st_commands 53, wp_commands 29).

## WP-04 close-gate -- GREEN (uncommitted)

`intent st done` / `intent wp done` now refuse to close while the in-scope acceptance contract is BLOCKED. Built and verified this session:

1. **`intent ac gate <stid>[/NN]`** in `bin/intent_acceptance` -- the single authority. Exit 0 when there is no thread dir, no acceptance.md, or zero in-scope ACs (opt-in / legacy-safe), or when every in-scope AC is satisfied; else print `gate: <scope> BLOCKED -- N/M satisfied; unsatisfied: AC-...` and exit 1. Reuses `split_target` / `in_wp_filter` / `ac_is_satisfied`.
2. **Wiring**: `st done` (`bin/intent_st`, after the info.md existence check) calls `ac gate "$ST_ID"`; `wp done` (`bin/intent_wp`, after the WP_FILE check) calls `ac gate "$ST_ID/$WP_NUM"`. On non-zero they `error` (refuse). `INTENT_HOME` is exported by `bin/intent`, so the sibling call resolves.
3. **Template fix**: `lib/templates/prj/st/ST####/acceptance.md` re-indents its example AC/AT lines (4-space) under bracketed guidance, so freshly stamped STs carry no live col-0 `- AC-`/`- AT-` lines and stay un-gated.

`tests/unit/acceptance_close_gate.bats` 4/4; regressions green (st_commands 53, wp_commands 29, intent_acceptance_cli 7, helpers 4). WP-04 ATs flipped green via the `intent at` CLI (dogfood); tasks.md WP-04 `[x]`. The gate dogfoods on the thread itself: `ac gate ST0044` = 9/15 BLOCKED (AC-00.1, AC-05.1, AC-06.1, AC-07.1, AC-07.2, AC-08.1 open -- correct, ST0044 is not done). Ready to commit (ask matts first).

Note: AC-08.1 (WP-08 MODULES.md reg, non-test) is committed-done but still reads `satisfied: no` -- a real dogfood gap. Satisfy it truthfully via `intent ac satisfy ST0044 AC-08.1 --evidence "<ref>"` when convenient (left to matts as the non-test owner).

## Next: WP-05

Template references + show/edit. `info.md` and `WP/info.md` templates reference `acceptance.md` and restate no ACs (Highlander); extend `st show` / `st edit` to know the `acceptance` file type (they currently hardcode `info|design|impl|tasks`). Then WP-06 (map the five-step onto the skill set + docs), WP-07 (dogfood, ongoing), then ST0044 self-close (satisfy AC-00.1 with matts sign-off -> the gate lets `intent st done ST0044` through). Then **ST0043** (rethink `intent upgrade`, v2.12.0; ACs drafted in `intent/st/ST0043/acceptance.md`).

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics (no test counts in release notes / CHANGELOG / wip). Fail-forward. Commit to `main` (repo convention) only when matts asks. User runs the full suite externally -- single-file bats runs are fine. matts is the acceptance verifier this session. An AT's cited `path::name` must match the real `@test` name -- reconcile on drift. The `intent at` status-set wipes any trailing note (status token must stay first), so record the red-witness lap on the WP Coverage line, not the AT line.
