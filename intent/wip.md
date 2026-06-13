---
verblock: "14 Jun 2026:v0.82: matts - ST0044 WP-04 close-gate GREEN (4/4); uncommitted on top of fa90bb2, ready to commit"
intent_version: 2.11.12
---

# Work In Progress

## Current State

**2026-06-14 -- ST0044 WP-04 close-gate GREEN (uncommitted on top of commit `fa90bb2`).** ST0044 (acceptance.md as a default steel-thread doc + the AC/AT process that makes "done" externally verified) is the active build, dogfooded on itself with matts as verifier. Committed in `fa90bb2` (main): WP-01/02 (acceptance.md stamped via the doc-set glob), WP-03 (the `intent ac` / `intent at` CLI -- list/status/satisfy, red/green/na, done/notdone, green-only-from-red, model-A grammar), WP-08 (MODULES reg), plus a Highlander fix to `normalise_st_id` (octal-safe `10#` + ST-prefixed-short; `0044` was silently resolving to ST0036). **WP-04** (the close-gate) is GREEN: `intent ac gate <stid>[/NN]` added to `bin/intent_acceptance`, both `done` handlers consult it before mutating, and the template's example AC/AT lines are re-indented so freshly stamped STs carry no live col-0 ACs. Opt-in / legacy-safe: no acceptance.md or no live ACs leaves `done` exactly as before. `tests/unit/acceptance_close_gate.bats` 4/4; regressions green. WP-04 ATs flipped green via the `intent at` CLI (dogfood); the gate dogfoods on the thread itself -- `ac gate ST0044` reports 9/15 BLOCKED (WP-05/06/07 + the ST-level sign-off still open), which is correct: ST0044 honestly says it is not done. Uncommitted, ready to commit. Resume detail: `.claude/restart.md`. Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.12 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0044 WP-05** -- template references + show/edit: `info.md` and `WP/info.md` reference `acceptance.md` and restate no ACs (Highlander); extend `st show` / `st edit` to know the `acceptance` file type (they currently hardcode `info|design|impl|tasks`). WP-04 close-gate is GREEN. Then WP-06 (skill/process), WP-07 (dogfood), ST0044 self-close (satisfy AC-00.1 with matts sign-off -> the gate lets `st done ST0044` through). The standing backlog:

1. **ST0043 — Rethink `intent upgrade`** (WIP, after ST0044; targets **v2.12.0 minor**). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md`.
2. **Flush done work into the ledger** -- move completed-arc detail out of `wip.md` / `restart.md` / `.claude/restart.md` into `intent/done.md` (terse ledger) + `intent/history/*.md` (verbose narrative) so the WIP docs stay lean. Do it as the ST0044 arc closes -- wip.md / restart.md currently still carry full ST0042 + ST0044 build detail.
3. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
6. **ST0040 deferred items** (revisit on field evidence).
7. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: ST0044 WP-04 close-gate GREEN (uncommitted) -- `intent ac gate` verb + `st done` / `wp done` wiring + template re-indent so fresh STs aren't gated; 4/4 + regressions; ATs flipped green via `intent at` (dogfood).
- **2026-06-14**: `fa90bb2` (main) -- ST0044 WP-01/02/03/08 (acceptance.md + `intent ac`/`at`) + `normalise_st_id` octal/ST-prefix fix.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
