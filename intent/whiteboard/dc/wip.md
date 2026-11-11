---
node: dc
name: DevX Claude
role: devx
session_id: 2aba47df-5301-4790-a6b7-e11815afe307
heartbeat_at: 2026-11-11 10:27Z
status: active
focus: "2026-11-11 10:27Z, localfolded for hv's second compact (hv via vc: 'localfold+compact everyone here in intent-* again. And then continue on the bounce'); active, no slot held. On the bounce: /in-session, commit the heartbeat-only pickup in its own chain, then: (1) WP-04 bank (i) rebased on 867cad6d3 (doing 159): re-ask a slot with --place-of 01M751XM0NFWE737SY9JNTDY7G (devbin-vc denied it for the compact and keeps its place), run scratchpad w04/run-c.sh, rebank as measured, send vc; (2) WP-04 bank (ii) drafted on (i)'s tip in tmp/wt-dc-0092-04d (todo 62): build, red then green in a slot, bank, send vc and ic; (3) the devbin sweep when 0.1.22 is on the tap (todo 61); (4) F8 docs/reference last (todo 60). Hold 37 stands. Worktrees: -04c, -04c-red, -04d live; -04, -04b, -04-red superseded (remove at a fold by the checks); tmp/wt-dc-devbin0121; tmp/wt-dc-0090-app (hold 37). Notes: scratchpad w04/progress.md."
claims: [ST0090/02, ST0091/01, ST0091/04, ST0092/04]
---

# DevX Claude (dc)

## DOING

- ST0092 WP-04 bank (i). REBASED onto 867cad6d3 (after WP-05 landed) in tmp/wt-dc-0092-04c with ic's three wording fixes folded in (all read clean by ic, upheld by vc); red tree tmp/wt-dc-0092-04c-red. Refs, UNMEASURED: refs/bank/dc/0092-04/red-unmeasured-on-867cad6d3 (blob 3a7b799ca29c), green-on-red-unmeasured-on-867cad6d3 (50dfcbecb67d), fix-unmeasured-on-867cad6d3 (aff52f1a8391); red tree 2d2162073e23, tip tree 4cdc6d6fbba4 (stacking verified). Two conflicts, both taking WP-05's side: (1) facade.rs land_board_write tail = drop_closed_pair_views()?; record_landed(..)?; prune_retired_pair_views(&rendered); (2) dispatch-table.json wb init when_to_use = WP-05's text whole with WP-04's two sentences after 're-applies the board's recorded declaration.'. seat.rs, views.rs, WbInit merged cleanly; told vc. Slot 01M751XM0NFWE737SY9JNTDY7G never ran (asked devbin-vc to deny it for the compact). NEXT: re-ask a slot, run /w04/run-c.sh (red in -04c-red, green plus schema stamp check in -04c), rebank as measured, send vc for the full judge. Landing: red, mint AT-04.1, 04.2, 04.5, 04.6 citing intent-cli/tests/wb_retire_door.rs and AT-04.3, 04.4 citing intentsvcs/tests/wb_retire.rs (compile red, naming at_04_1's door arm), at red, green, at green full notes, lint. The 8f3da433f refs and tmp/wt-dc-0092-04, -04b, -04-red are superseded and go at the next fold. (edited)

## TODO

- ST0091 F8 (vc, 2026-11-10): regenerate docs/reference against v3.2.8 and commit it as the last unit before the cut, after ST0092's surface is on main, since those pages change with it. Until then `gate reference` refuses, which is correct; it is not a derived step.
- ST0091 AT-01.6 and the devbin sweep, one unit for 3.3.0 (vc's rulings, hv decisions 37 and 38). Built on devbin 0.1.21 in tmp/wt-dc-devbin0121 and banked UNJUDGED as refs/bank/dc/devbin0121/unjudged-built-on-0.1.21-waiting-0.1.22-on-2763e1f2b; re-sweep onto 0.1.22 once devbin-vc says it is on the tap (as of 2026-11-11 10:27Z its macOS CI job is being re-run). The unit: bin/devbin upgrade; install.links for intent and intentd, install.switch: intent use, install.publish: intent install; formula.bin and @@FINISH@@ gone, the template's install @@INSTALL@@ alone; the macOS assets packed as ONE archive, intent-aarch64-apple-darwin.tar.gz, by Intent's own cmd/macos pack after notarize (hv: leave it on Intent's handler); intent install's opt manifest gains '# line: intent <v> (opt, <sha12>)' with a writer arm and a reader arm, run red then green in one slot; release.preflight's four checks, preflight clean against DEVBIN_RELEASE_VERSION, and release_preflight_seam.bats for AT-01.6. Proofs: the rendered keg is 3.2.8's (164 files, byte-equal on v3.2.8's real assets), devbin doctor 0 failed with the built pair and an opt tree, bin/devbin use and install refused for Intent, brew_formula green. vc judges light with test shell. (edited)
- ST0092 WP-04 bank (ii), stacked on (i)'s rebased tip 4cdc6d6fbba4 in tmp/wt-dc-0092-04d, DRAFTED, UNBUILT: refs/bank/dc/0092-04/ii-draft-unbuilt-on-4cdc6d6fbba4 (blob d8a5cae30357, tree 46ff68d230d6, 11 files). FacadeError::WbRecipientRetired (wb ask refuses, exit 1, remedy names wb register); wb announce skips retired; Pickup.stranded: Vec<WbStranded{recipient, recorded_at, reason}> (skipped when empty) with a CLI 'stranded:' line carrying the archive command; store archive_messages' one-message form (id given) takes a message to a retired recipient unhandled, the --handled sweep does not; views::live_to is the one reading of a live message (holds_live, wb_retire's awaiting, pickup). Help for wb ask, announce, archive, pickup; CHANGELOG; error_remedies and mandatory_fields lists. Tests: wb_retire.rs at_04_7 and at_04_8; door arm at_04_7_and_8 (runtime red on (i)'s tip: the ask succeeds). New ATs AT-04.7 (AC-04.1) and AT-04.8 (AC-04.3). NEXT after (i)'s green: a slot for (ii)'s red (door file only on (i)'s tip) and green, bank as its own bank, vc judges both in one full run, one landing each. (edited)

## Holds

- ST0090/02 stays WIP after its landing at a54f2e9e0 until AC-02.4 is met: hv sees the Rebuild Search Index row's second line after a rebuild, on a build that carries WP-01 and WP-02, and vc records which build that was. Then dc closes WP-02 on vc's word; a defect hv sees comes back to dc as a new bank. The Debug app dc built on 2026-11-01 (IntentCommit 29e6f1c0a) went with that session's scratch. dc rebuilds it on vc's word once 3.3.0's first train's last unit is on main (vc, 2026-11-10), at the HEAD vc names, booked as a light run with the estate's controller, for hv's look by the route vc hold 6 records. dc removes tmp/wt-dc-0090-app once the look is recorded. (edited)

## Watch-outs

_(none)_

## Decisions

- ST0092 WP-04 rulings (vc, 2026-11-11): retirement is WbNode.retired: Option<WbRetired { at, reason }> in board 4.1 (folded, one shape for 3.3.0), never a status value; store rung 34 to 35. A re-apply reinstates nothing as it retires nothing: a retired declared seat shows as 'retired <m> (<reason>)' and comes back only through wb register (WP-09's doctor names it, shown not counted; cc told). The human seat has one home, seat::human_seat (declared human, else model::HYPERVISOR, no install read), which WP-06 rebases onto. The red is a door file (intent-cli tests/wb_retire_door.rs) red at runtime on the base, with wb_retire.rs as the precise half. surface_check.sh's INV-09/10/11 refusal predates WP-04 and goes to ic's WP-10.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
