---
verblock: "14 Jun 2026:v0.83: matts - ST0044 WP-05 GREEN (2/2); uncommitted on top of 95a3507 + fc4f75a, ready to commit"
intent_version: 2.11.12
---

# Work In Progress

## Current State

**2026-06-14 -- ST0044 WP-05 GREEN (uncommitted on top of commits `95a3507` + `fc4f75a`).** ST0044 (acceptance.md as a default steel-thread doc + the AC/AT process that makes "done" externally verified) is the active build, dogfooded on itself with matts as verifier. Committed: `fa90bb2` (WP-01/02/03/08 -- acceptance.md doc-set + the `intent ac` / `intent at` CLI + MODULES reg + the `normalise_st_id` octal/ST-prefix fix); `95a3507` (WP-04 close-gate -- `intent ac gate` consulted by `st done` / `wp done`, opt-in / legacy-safe); `fc4f75a` (AC-08.1 satisfied). **WP-05** (template references + show/edit) is GREEN: the ST and WP `info.md` templates point at `acceptance.md` and restate no ACs (Highlander); `st show` / `st edit` / `st show all` learn the `acceptance` type; and `st edit` is reworked to pure emit-path -- it prints the file's absolute path (global, no editor launch) per matts, which is why it had never been testable. `tests/unit/st_new_acceptance.bats` 2/2; regressions green (st_commands 53, wp_commands 29). WP-05 ATs flipped green via the `intent at` CLI (dogfood); `ac status ST0044` = 12/16 BLOCKED (AC-00.1 sign-off + WP-06/07 still open -- correct, the thread honestly says it is not done). Uncommitted, ready to commit. Resume detail: `.claude/restart.md`. Live AC/AT tracker: `intent/st/ST0044/acceptance.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.12 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0044 WP-06** -- skill / process integration: map the five-step (verifier ratifies ACs -> builder writes red-first ATs -> verifier witnesses RED -> builder builds to green) onto the skill set + docs, and describe the open-gate and close-gate where a builder meets them. AC-06.1 is non-test (evidence = doc + skill refs). Then WP-07 (dogfood, ongoing) and ST0044 self-close (satisfy AC-00.1 with matts sign-off -> the gate lets `st done ST0044` through). WP-04 + WP-05 are GREEN. The standing backlog:

1. **ST0043 — Rethink `intent upgrade`** (WIP, after ST0044; targets **v2.12.0 minor**). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md`.
2. **Flush done work into the ledger** -- move completed-arc detail out of `wip.md` / `restart.md` / `.claude/restart.md` into `intent/done.md` (terse ledger) + `intent/history/*.md` (verbose narrative) so the WIP docs stay lean. Do it as the ST0044 arc closes -- wip.md / restart.md currently still carry full ST0042 + ST0044 build detail.
3. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
6. **ST0040 deferred items** (revisit on field evidence).
7. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: ST0044 WP-05 GREEN (uncommitted) -- ST/WP `info.md` templates reference `acceptance.md` (no restated ACs); `st show` / `st edit` learn the `acceptance` type; `st edit` reworked to pure emit-path (global, no editor). 2/2 + regressions.
- **2026-06-14**: `fc4f75a` (main) -- ST0044 AC-08.1 satisfied (WP-08 MODULES.md evidence).
- **2026-06-14**: `95a3507` (main) -- ST0044 WP-04 acceptance close-gate (`intent ac gate` + `st done` / `wp done` wiring + template re-indent).
- **2026-06-14**: `fa90bb2` (main) -- ST0044 WP-01/02/03/08 (acceptance.md + `intent ac`/`at`) + `normalise_st_id` octal/ST-prefix fix.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
