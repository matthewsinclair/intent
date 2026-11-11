---
node: ic
name: Interface Claude
role: interface
session_id: 677e6c35-f820-4993-8ec4-ad639debeb1b
heartbeat_at: 2026-11-11 10:47Z
status: active
focus: "2026-11-11, localfolded for hv's second compact (via vc: "Ok, let's localfold+compact everyone here in intent-* again. And then continue on the bounce."). No hold, status active, no desk slot held. On the bounce: /in-session, commit the heartbeat-only pickup in its own chain, then doing 145 (ST0092 WP-06: banked green on 867cad6d3; waits on dc's WP-04 for its rebase and the retired-seat arm), row reads per todo 63, WP-10 per todos 67, 68 and 69. Claim ST0092/06, worktree tmp/wt-ic-0092-06, no stash."
claims: [ST0092/06]
---

# Interface Claude (ic)

## DOING

- ST0092 WP-06, ic's whole (vc 2026-11-11; cc keeps WP-05 and WP-09), size L, claimed and started at 5fa96c53f. HOLD NO DESK SLOT ACROSS A COMPACT; ask afresh for each run and start within one turn of the grant. WORKTREE tmp/wt-ic-0092-06, own target, REBASED onto 867cad6d3 (WP-05 landed): WIP commits red 6b6dc423d, green draft 5f70d2ef2, v2 7f50bb54d, v3 9f88669de (tip). BANKED on 867cad6d3: refs/bank/ic/st0092/wp06-red-on-867cad6d3 (blob 476f21c91d91, tree 60056cefa644) and refs/bank/ic/st0092/wp06-green-on-red-on-867cad6d3 (blob 2bb5141eac1a, tree e00f1642b050); the refs on 4dc6a34a7 are superseded and deleted. MEASURED: red on the old code (slot 01M74YSMJPM22MKC5MSQMM5HTR, 09:26Z): six arms fail each for want of the feature, the unprofiled-launch pin passes; green v2 whole suites, clippy, format clean (01M74ZGQNSGHH062PM34VNFHM9, 09:38 to 09:47Z); v3 arms, time and one-clock guards, clippy, format clean (01M750320NGBPDCA8H6JSR99V7, 09:48Z); the rebased tip 9f88669de GREEN WHOLE (01M751VPAA9VF0FGRQH0CEZ8HQ, 10:24:57 to 10:33:29Z, released): WP-06 arms pass, intent-cli and intentsvcs suites 0 failed, clippy and format clean. SHAPE: brief.rs on roster_readme::Roster::of; Facade::seat_brief is the one function for pickup, --json, MCP, wb brief and claude_cwi; seat::human_seat (DROP mine at the WP-04 rebase, dc's signature) and seat::never_started; store wb_last_pickups; Pickup brief, last_pickup and taken_at; StatusRow live_items, live_messages, last_pickup, never_started, WbStatus taken_at; estate::taken_at() is the one clock reading; render.rs pickup_age(&Pickup) and row_age(&WbStatus, &StatusRow) take the answer and read no clock (vc's ruling); the launcher prints the door's launch context, cites no tracker and never calls intent; wb brief joins MCP_SERVING_MATCH in the roster guard. VC'S RULINGS: (1) unprofiled launch byte-equal to today's; (2) the human seat is seat::human_seat; (3) the library returns the last pickup stamp, read before the write, and the reading's taken_at; (4) MCP fields live_items and live_messages; (5) WbNodeNotRegistered names `intent wb init --apply` on a declared board; (6) the age functions take the record. LEFT: rebase onto dc's WP-04 when it lands (dc re-banks after conflicts with WP-05); the retired-seat arm against WbNode.retired (wb brief and the launcher refuse at exit 1 naming wb register; status omits retired unless --all), red then green; AT rows AT-06.1, .2, .3, .5 minted at landing; bank red, green-on-red and fix for vc's full judge. (edited)

## TODO

- ST0092 row reads, as the table's owner, before vc judges, findings one per message to the builder copied to vc; nothing written to ST0092's canon. DONE: WP-02, WP-03, WP-07, WP-08b, WP-05 (landed 867cad6d3; its table delta read clean), dc's WP-04 bank (i) (three wording findings, all met in its wording delta, blob 1dcfba2335f4), cc's WP-09 (one finding: AC-09.1's retired-seat sentence; vc ruled the bank lands as is, WP-09 stays WIP, and the retired-seat arm follows WP-04 as its own bank). OWED: dc's WP-04 rows again after its rebase onto WP-05 (it conflicts in facade.rs, seat.rs, views.rs), and WP-09's retired-seat bank when cc banks it. One-shape rule: FacadeError::build_cannot_answer is the one exit-2 predicate `fail` reads; any later variant joins it, never a second mapping. (edited)
- PARKED UNTIL AFTER THE 3.3.0 CUT, on vc's word of 2026-11-10: the dispatch table's `doctor` row still has target state `pending-hv` on "Should doctor exit non-zero when it finds something?", which the shipped exit contract (INV-04, doctor exits 1 on a counted finding) answered long ago. ic, as the table's owner, brings the row's correction to vc after the cut.
- ST0092 WP-10 prep, ruled by vc on 2026-11-10; nothing written until WP-10 starts (after cc's packages land). dc's inventory: refs/bank/ic/st0092/wp10-dc-inventory-on-7c16cffef.md. (1) `estate control`'s when_to_use takes, after 'recorded verbatim.': 'A session bound to a whiteboard node takes it only as its board's mediator (vc on a board that declares no profile) or with `--hv`; a session bound to no node, in a project or not, takes it as before.' (2) `wb pickup`'s desk line ('a session that wears the desk is also told so, with the controller's duties and the asks waiting') is still owed; dc's WP-08b put the desk in the facade, so it goes in at WP-10, or with WP-06's pickup row if that lands first. (3) `wb status`'s desk sentence landed with WP-08b (hats null meaning), and WP-06's draft extends that row; at WP-10 extend that one sentence to say who wears the desk, never add a second. (edited)
- ST0092 WP-10, owed in 3.3.0 (vc 2026-11-11, from dc's find): intent/st/ST0056/parity/tools/surface_check.sh refuses on main's own table ("invariant(s) this check has no probe for and does not skip: INV-09 INV-10 INV-11", and INV-12 arrives with WP-05) and nothing runs it, so it has been red unseen since INV-09. Each of INV-09 to INV-12 gets a probe in that check or a declared skip naming the Rust arms that hold it; then either a gate runs the check, or ic shows the Rust probes supersede it and it goes with its MODULES.md row. Tell vc which way.
- ST0092 WP-10, from cc's issue 0680 note (2026-11-11, vc copied): (a) the estate page, the TUI and Intent.app draw cc's new status key `running_idle` (oldest start first; each {id, by, name, lanes, started_at}; a running request whose asker's session is `idle` exactly), read from cc's landed reader, once it lands. (b) shell.html:764 decides ctlState ORPHANED by itself; it is to read `s.controller_orphaned`, and a controller with no session row reads as live in the CLI's words ("its session could not be read here, so it is taken as live"). (c) cc's rule for the per-hold twin at 752 (behind 1007, 1080 and 1125): only a session row in state `gone` is evidence its holder is gone, so orphaned(q) becomes `who && who.state === "gone"`, and a hold with no row reads as unknown, drawn held with no ORPHANED and no take step; the reader carries no per-request verdict and gets none. (edited)

## Holds

_(none)_

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
