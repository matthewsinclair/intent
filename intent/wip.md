---
verblock: "14 Jun 2026:v0.84: matts - ST0044 WP-06 done (AC-06.1 signed off); uncommitted on top of 0b1b3b5, ready to commit"
intent_version: 2.11.12
---

# Work In Progress

## Current State

**2026-06-14 -- ST0044 WP-06 done (uncommitted on top of commit `0b1b3b5`).** ST0044 (acceptance.md as a default steel-thread doc + the AC/AT process that makes "done" externally verified) is the active build, dogfooded on itself with matts as verifier. Committed: `fa90bb2` (WP-01/02/03/08 + `normalise_st_id` fix); `95a3507` (WP-04 close-gate); `fc4f75a` (AC-08.1); `0b1b3b5` (WP-05 templates + show/edit, `st edit` -> pure emit-path). **WP-06** (skill / process integration) is done: the five-step + open/close gates are documented in `intent/docs/working-with-llms.md` D11 (the canon home), with light pointers threaded into `/in-plan` (open-gate), `/in-verify` (red-first + witness RED) and `/in-finish` (close-gate) -- each referencing D11, none restating it (Highlander). Thread-through chosen over a new `/in-acceptance` skill. AC-06.1 (non-test) satisfied with matts sign-off; installed skills synced (`intent claude skills sync`). `ac status ST0044` = 13/16 BLOCKED -- only AC-00.1 (ST-level sign-off) and WP-07 (AC-07.1 test + AC-07.2 dogfood) remain. Uncommitted, ready to commit. Resume detail: `.claude/restart.md`. Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.12 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0044 WP-07 + self-close** -- WP-07 dogfood: write AT-07.1 red-first -> green (both open STs ST0043 + ST0044 carry an `acceptance.md`); AC-07.2 (non-test) records ST0044's own five-step run (this whole arc). Then ST0044 self-close: satisfy AC-00.1 with matts's ST-level sign-off, which finally lets `intent st done ST0044` through the close-gate -- the gate it built closes it. WP-04/05/06 are done. The standing backlog:

1. **ST0043 — Rethink `intent upgrade`** (WIP, after ST0044; targets **v2.12.0 minor**). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md`.
2. **Flush done work into the ledger** -- move completed-arc detail out of `wip.md` / `restart.md` / `.claude/restart.md` into `intent/done.md` (terse ledger) + `intent/history/*.md` (verbose narrative) so the WIP docs stay lean. Do it as the ST0044 arc closes -- wip.md / restart.md currently still carry full ST0042 + ST0044 build detail.
3. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
6. **ST0040 deferred items** (revisit on field evidence).
7. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: ST0044 WP-06 done (uncommitted) -- five-step + open/close gates documented in `working-with-llms.md` D11; thread-through pointers in `in-plan` / `in-verify` / `in-finish`; AC-06.1 satisfied (matts); installed skills synced.
- **2026-06-14**: `0b1b3b5` (main) -- ST0044 WP-05 (templates reference `acceptance.md`; `st show` / `st edit` learn the `acceptance` type; `st edit` -> pure emit-path).
- **2026-06-14**: `fc4f75a` (main) -- ST0044 AC-08.1 satisfied (WP-08 MODULES.md evidence).
- **2026-06-14**: `95a3507` (main) -- ST0044 WP-04 acceptance close-gate (`intent ac gate` + `st done` / `wp done` wiring + template re-indent).
- **2026-06-14**: `fa90bb2` (main) -- ST0044 WP-01/02/03/08 (acceptance.md + `intent ac`/`at`) + `normalise_st_id` octal/ST-prefix fix.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
