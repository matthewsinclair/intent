# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate. (Languages: shell only; no whiteboard in this project.)
2. **Verify the tree.** v2.11.12 is the shipped baseline. ST0044 commits on `main`: `fa90bb2` (WP-01/02/03/08 + `normalise_st_id` fix), `95a3507` (WP-04 close-gate), `fc4f75a` (AC-08.1 satisfied). On top sits **uncommitted** WP-05 + the three resume docs. Dirty tree is expected.
3. **Read `intent/st/ST0044/acceptance.md`** -- the live AC/AT tracker -- plus `design.md` / `tasks.md`.

## Committed (main)

- `fa90bb2` -- WP-01/02/03/08 + the normaliser fix. `acceptance.md` is a default doc-set file (template stamped via the `*.md` glob). `intent ac` / `intent at` in `bin/intent_acceptance`: `ac list/status/satisfy/gate`, `at list/red/green/na`, `done`/`notdone` aliases; `green` only from `red`; model-A grammar (non-test ACs carry inline `-- evidence: <ref> -- satisfied: yes|no`; test-backed ACs computed from a green covering AT). Dispatch in `bin/intent` (ac/at noun re-inject). `normalise_st_id` (helpers:286) made octal-safe (`10#`) + ST-prefixed-short branch.
- `95a3507` -- WP-04 close-gate. `intent ac gate <stid>[/NN]` is the single authority; `st done` (whole thread) and `wp done` (NN group) consult it before mutating and refuse on a BLOCKED contract. Opt-in / legacy-safe (no thread dir / no acceptance.md / no in-scope ACs -> exit 0). Template re-indents its example AC/AT lines so freshly stamped STs are not self-gated.
- `fc4f75a` -- AC-08.1 satisfied (WP-08 MODULES.md registration evidence) via `intent ac satisfy`.

## WP-05 -- template references + show/edit -- GREEN (uncommitted)

1. **Templates** -- the ST `info.md` (`lib/templates/prj/st/ST####/info.md`, new `## Acceptance` section) and the WP `info.md` (`lib/templates/prj/st/WP/info.md`, the `## Acceptance Criteria` / `- [ ]` block replaced) now point at `acceptance.md` and restate no ACs (Highlander -- one AC home).
2. **show/edit** (`bin/intent_st`) -- `acceptance` added to the `show` specific-file case, the `show all` loop, and the `edit` case. `st edit <id> <type>` reworked to **pure emit-path**: validate the type, print the file's absolute path (composable, editor-agnostic), no touch, no `open`/`$EDITOR`. This is global (all file types), per matts -- the old macOS `open` launch is why `st edit` had no tests.

`tests/unit/st_new_acceptance.bats` 2/2 (AT-05.1 templates, AT-05.2 show/edit); regressions green (st_commands 53, wp_commands 29, acceptance_close_gate 4, intent_acceptance_cli 7, helpers 4). WP-05 ATs flipped green via `intent at` (dogfood); tasks.md WP-05 `[x]`. Dogfood on the thread itself: `ac status ST0044` = 12/16 BLOCKED (open: AC-00.1 sign-off, AC-06.1, AC-07.1, AC-07.2 -- correct). Ready to commit (ask matts first).

Contract note: AC-05.2 was ratified mid-build (matts) and then refined from "opens/creates" to **pure emit-path**; STATUS note + AC-05.2 wording in `acceptance.md` record it.

## Next: WP-06

Skill / process integration. Map the five-step (verifier ratifies ACs -> builder writes red-first ATs -> verifier witnesses RED -> builder builds to green) onto the skill set + docs, and describe the open-gate and close-gate where a builder meets them. AC-06.1 is non-test (evidence = doc + skill refs); candidate skills/docs: `/in-plan`, `/in-verify`, `/in-review`, `/in-finish`, `intent/docs/working-with-llms.md`. Then WP-07 (dogfood, ongoing) and ST0044 self-close (satisfy AC-00.1 with matts sign-off -> the gate lets `intent st done ST0044` through). Then **ST0043** (rethink `intent upgrade`, v2.12.0; ACs drafted in `intent/st/ST0043/acceptance.md`).

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics (no test counts in release notes / CHANGELOG / wip). Fail-forward. Commit to `main` (repo convention) only when matts asks. User runs the full suite externally -- single-file bats runs are fine. matts is the acceptance verifier this session (ratifies ACs, witnesses RED). An AT's cited `path::name` must match the real `@test` name -- reconcile on drift. The `intent at` status-set wipes any trailing note (status token must stay first), so record the red-witness lap on the WP Coverage line, not the AT line.
