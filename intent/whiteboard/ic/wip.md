---
node: ic
name: Interface Claude
role: interface
session_id: 677e6c35-f820-4993-8ec4-ad639debeb1b
heartbeat_at: 2026-11-11 12:19Z
status: active
focus: "2026-11-11, localfolded for hv's third compact (via vc, hv first-hand at 12:15Z: "lets bank, localfold+compact everyone here and then continue on the bounce. I am about to go AFK"). No hold, status active. On the bounce: /in-session, commit the heartbeat-only pickup in its own chain, then doing 145 (ST0092 WP-06 rebased on 4a4498e6d, banked UNMEASURED with the retired-seat edge; its run waits on desk slot 01M757013KSVD5MEAWKCZS5PTB, which grants on its own: start scratchpad wp06-drive5.sh within one turn of the grant), todo 63 (cc's WP-09 follow-up rows), WP-10 per todos 67, 68 (ruled) and 69. Claim ST0092/06, worktree tmp/wt-ic-0092-06 detached at be1032dcb, no stash."
claims: [ST0092/06]
---

# Interface Claude (ic)

## DOING

- ST0092 WP-06, ic's whole (vc 2026-11-11; cc keeps WP-05 and WP-09), size L, claimed and started at 5fa96c53f. HOLD NO DESK SLOT ACROSS A COMPACT; ask afresh for each run and start within one turn of the grant. WORKTREE tmp/wt-ic-0092-06, own target, REBASED onto 4a4498e6d (dc's WP-04 bank (i) landed at 8965685b9, vc's AC-06.1 rewording at 4a4498e6d). Commits, red then green: d3a9bd812 red (WP-06 arms), aef6e9b32 red 2 (retired-seat arms), c6ac46a23 green (v1-v3 squashed), be1032dcb green 2 (retired-seat edge); the worktree is detached at be1032dcb. BANKED UNMEASURED on 4a4498e6d: refs/bank/ic/st0092/wp06-red-unmeasured-on-4a4498e6d (blob c9b70ec9b3a8, tree 8f9317f7b85a) and wp06-green-on-red-unmeasured-on-4a4498e6d (blob 410bcd6d89a6, tree 22bea139c2f8), both trees verified exact; the refs on 867cad6d3 are deleted. THE RETIRED-SEAT EDGE, on vc's rulings of 2026-11-11: `wb brief` (with or without --launch) and `claude start` of a retired seat refuse at exit 1 with FacadeError::WbSeatRetired naming the retirement, its reason and `intent wb register <m> --name "<name>" --role <role>`; Facade::seat_brief drops every seat whose node's `retired` is set from the roster it built (Roster::of untouched); the wb brief row and the CHANGELOG say so. Arms: seat_brief.rs a_retired_seat_is_refused_its_brief_and_left_out_of_the_others, claude_cwi_door.rs start_of_a_retired_seat_passes_the_refusal_through_and_launches_nothing. NOT YET RUN: desk slot 01M757013KSVD5MEAWKCZS5PTB (cargo lane, granted on its own after cc's 0680 run 2, vc's WP-04 (ii) judge and cc's WP-09 follow-up) runs scratchpad wp06-drive5.sh <log>: red arms at aef6e9b32, then green at be1032dcb with seat_brief, claude_cwi_door, intent-cli and intentsvcs whole, clippy, format. Then: rebank as measured, mint AT-06.1, .2, .3 and .5 at landing, bank for vc's full judge. dc's bank (ii) adds Pickup.stranded beside my Pickup.taken_at: a trivial conflict when (ii) lands first. (edited)

## TODO

- ST0092 row reads, as the table's owner, before vc judges, findings one per message to the builder copied to vc; nothing written to ST0092's canon. DONE: WP-02, WP-03, WP-07, WP-08b, WP-05, WP-09 (landed at 4c0d27cda), dc's WP-04 bank (i) twice (on 867cad6d3, three wording findings met; on 720c861d7, clean, landed 8965685b9), dc's WP-04 bank (ii) clean (read unbuilt; its measured bank refs/bank/dc/0092-04/ii-fix-measured-on-8965685b9 carries a byte-identical table delta, one fmt note sent). OWED: cc's WP-09 retired-seat follow-up bank when cc banks it. One-shape rule: FacadeError::build_cannot_answer is the one exit-2 predicate `fail` reads; any later variant joins it, never a second mapping. (edited)
- PARKED UNTIL AFTER THE 3.3.0 CUT, on vc's word of 2026-11-10: the dispatch table's `doctor` row still has target state `pending-hv` on "Should doctor exit non-zero when it finds something?", which the shipped exit contract (INV-04, doctor exits 1 on a counted finding) answered long ago. ic, as the table's owner, brings the row's correction to vc after the cut.
- ST0092 WP-10 prep, ruled by vc on 2026-11-10; nothing written until WP-10 starts (after cc's packages land). dc's inventory: refs/bank/ic/st0092/wp10-dc-inventory-on-7c16cffef.md. (1) `estate control`'s when_to_use takes, after 'recorded verbatim.': 'A session bound to a whiteboard node takes it only as its board's mediator (vc on a board that declares no profile) or with `--hv`; a session bound to no node, in a project or not, takes it as before.' (2) `wb pickup`'s desk line ('a session that wears the desk is also told so, with the controller's duties and the asks waiting') is still owed; dc's WP-08b put the desk in the facade, so it goes in at WP-10, or with WP-06's pickup row if that lands first. (3) `wb status`'s desk sentence landed with WP-08b (hats null meaning), and WP-06's draft extends that row; at WP-10 extend that one sentence to say who wears the desk, never add a second. (edited)
- ST0092 WP-10, owed in 3.3.0, RULED by vc 2026-11-11, received before ic read 10:51Z on date -u, on ic's reading (dc's find): intent/st/ST0056/parity/tools/surface_check.sh refuses on main's own table because its hand-written INV map has no entry for INV-09 to INV-12. The check STAYS MANUAL (runner_roster_check.sh rosters it manual on purpose: a --help sweep over every path), and is neither gated nor removed. WP-10 writes, nothing before it starts: (1) four declared skips in INV_SKIPPED with inv_skip_reason entries, each reason the table's held_by sentence written by hand (never derived, so the map still refuses the next new INV): INV-09 twin_spellings_agree.rs and edit_and_browse_reach_one_model.rs; INV-10 st_edit_opens_or_prints.rs, naming the new arm below once it exists; INV-11 dispatch::table()'s value_facades/value_args load checks and mcp.rs's expansion test beside a_subcommand_slot_is_not_published; INV-12 Facade::require_writer_as with intentsvcs tests/wb_hv.rs and intent-cli tests/hv_seat_door.rs. (2) One new arm beside the_two_flags_together_are_refused_rather_than_ranked: `edit --browser --path` and `edit --browser --editor` each refused with nothing on stdout (render.rs browsed() already refuses both; no test held it). It passes first run and its AT note says so, as AT-05.5 and AT-05.7 do. Then run the check green on a current binary in a WP-10 slot. (edited)
- ST0092 WP-10, from cc's issue 0680 note (2026-11-11, vc copied): (a) the estate page, the TUI and Intent.app draw cc's new status key `running_idle` (oldest start first; each {id, by, name, lanes, started_at}; a running request whose asker's session is `idle` exactly), read from cc's landed reader, once it lands. (b) shell.html:764 decides ctlState ORPHANED by itself; it is to read `s.controller_orphaned`, and a controller with no session row reads as live in the CLI's words ("its session could not be read here, so it is taken as live"). (c) cc's rule for the per-hold twin at 752 (behind 1007, 1080 and 1125): only a session row in state `gone` is evidence its holder is gone, so orphaned(q) becomes `who && who.state === "gone"`, and a hold with no row reads as unknown, drawn held with no ORPHANED and no take step; the reader carries no per-request verdict and gets none. (edited)

## Holds

_(none)_

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
