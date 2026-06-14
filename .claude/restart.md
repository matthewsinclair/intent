# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate. (Languages: shell only; no whiteboard in this project.)
2. **Verify the tree.** v2.11.12 is the shipped baseline. ST0044 commits on `main`: `fa90bb2` (WP-01/02/03/08 + `normalise_st_id` fix), `95a3507` (WP-04 close-gate), `fc4f75a` (AC-08.1), `0b1b3b5` (WP-05 templates + show/edit). On top sits **uncommitted** WP-06 + the three resume docs. Dirty tree is expected.
3. **Read `intent/st/ST0044/acceptance.md`** -- the live AC/AT tracker -- plus `design.md` / `tasks.md`.

## Committed (main)

- `fa90bb2` -- WP-01/02/03/08. `acceptance.md` is a default doc-set file. `intent ac` / `intent at` in `bin/intent_acceptance` (`ac list/status/satisfy/gate`, `at list/red/green/na` + `done`/`notdone`; `green` only from `red`; model-A grammar). `normalise_st_id` octal-safe + ST-prefixed-short.
- `95a3507` -- WP-04 close-gate. `intent ac gate <stid>[/NN]` is the single authority; `st done` (whole thread) / `wp done` (NN group) consult it before mutating and refuse on BLOCKED. Opt-in / legacy-safe.
- `fc4f75a` -- AC-08.1 satisfied (WP-08 MODULES.md registration evidence).
- `0b1b3b5` -- WP-05. ST + WP `info.md` templates point at `acceptance.md` and restate no ACs (Highlander). `acceptance` added to `st show` (specific + `all`) and `st edit`. `st edit <id> <type>` reworked to **pure emit-path**: validate the type, print the file's absolute path, no editor (global; replaces the macOS `open` launch).

## WP-06 -- skill / process integration -- done (uncommitted)

Thread-through chosen over a new `/in-acceptance` skill. One canon home + light pointers:

- **`intent/docs/working-with-llms.md` D11 "Acceptance: AC/AT and the five-step"** (after D10; ToC updated to D1-D11). Covers the two axes (AC coverage / AT proof, green only from red), the five-step, both gates (open-gate = ACs before code; close-gate = computed `intent ac gate`, opt-in/legacy-safe), the CLI authority, the lifecycle mapping, and a Why.
- **Pointers, each referencing D11, none restating it:** `/in-plan` step 1 (open-gate -- ratify ACs before code), `/in-verify` new rule 6 (red-first + witness RED + computed verdict), `/in-finish` step 2 (close-gate -- `st done` refuses while BLOCKED).

AC-06.1 (non-test) satisfied with matts sign-off; installed skills synced (`intent claude skills sync` -- 3 updated). `ac status ST0044` = 13/16 BLOCKED. Ready to commit.

## Next: WP-07 dogfood + ST0044 self-close

1. **WP-07** -- write **AT-07.1** red-first -> green: `tests/unit/st_new_acceptance.bats::open STs ST0043 and ST0044 each have an acceptance.md` (covers AC-07.1, test-backed). **AC-07.2** (non-test) records ST0044's own five-step run -- satisfy via `intent ac satisfy ST0044 AC-07.2 --evidence "..."` (matts). Flip AT-07.1 green via `intent at`; tasks.md WP-07 `[x]`.
2. **Self-close** -- with every other AC satisfied, the only thing left is **AC-00.1** (ST-level sign-off, non-test). matts satisfies it (`intent ac satisfy ST0044 AC-00.1 --evidence "..."`), which takes `ac status ST0044` to 16/16 PASS -- then `intent st done ST0044` passes its own close-gate (the gate it built closes it). The dogfood payoff.
3. Then **ST0043** (rethink `intent upgrade`, v2.12.0; ACs drafted in `intent/st/ST0043/acceptance.md`). Also pending: flush done-arc detail from wip/restart into `done.md` + `history/` as ST0044 closes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` (repo convention) only when matts asks. User runs the full suite externally -- single-file bats runs are fine. matts is the acceptance verifier this session (ratifies ACs, witnesses RED, signs off non-test ACs). An AT's cited `path::name` must match the real `@test` name. The `intent at` status-set wipes any trailing note (status token must stay first), so record the red-witness lap on the WP Coverage line, not the AT line.
