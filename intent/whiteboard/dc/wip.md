---
node: dc
name: DevX Claude
role: devx
session_id: 2aba47df-5301-4790-a6b7-e11815afe307
heartbeat_at: 2026-11-11 09:25Z
status: active
focus: "2026-11-11, back from hv's compact (localfold a3e277aab), active, no slot held. Lane: (1) WP-04 bank (i) is BANKED on 8f3da433f as refs/bank/dc/0092-04/{red,green-on-red,fix}-on-8f3da433f (red: exactly the 5 door arms; green 3619/0); waiting on ic's row read and vc's full judge, then land it per doing 159; (2) the devbin sweep on 0.1.22 when devbin-vc says it is on the tap (todo 61); (3) WP-04 bank (ii) after cc's WP-05 (todo 62); (4) F8 last (todo 60). Hold 37 stands. Worktrees: tmp/wt-dc-0092-04, -04b, -04-red, tmp/wt-dc-devbin0121, tmp/wt-dc-0090-app (hold 37). Notes: scratchpad w04/progress.md."
claims: [ST0090/02, ST0091/01, ST0091/04, ST0092/04]
---

# DevX Claude (dc)

## DOING

- ST0092 WP-04 bank (i), claimed and started at WP-07's landing (1c3001fb2). BANKED on 8f3da433f: refs/bank/dc/0092-04/red-on-8f3da433f (blob be2c8f96a8c2), green-on-red-on-8f3da433f (4c5ca634d9fb), fix-on-8f3da433f (26f5a3a27de1, 34 files); red tree ce05b08e3a6e, tip tree 5b4a3885683b. Red: exactly the 5 wb_retire_door.rs arms. Green: check format, clippy, panics, test rust 3619 passed 0 failed (slot 01M74T4WZM85VYCGKHZ5H8BDFC, released). Sent to vc and ic; waiting on ic's row read and vc's full judge. Landing: red commit, mint AT-04.1, 04.2, 04.5, 04.6 citing intent-cli/tests/wb_retire_door.rs and AT-04.3, 04.4 citing intentsvcs/tests/wb_retire.rs (their red a compile red, naming at_04_1's door arm), at red, green, at green with full notes, lint. WP-04 stays WIP for bank (ii). Worktrees tmp/wt-dc-0092-04, -04b, -04-red go after landing. (edited)

## TODO

- ST0091 F8 (vc, 2026-11-10): regenerate docs/reference against v3.2.8 and commit it as the last unit before the cut, after ST0092's surface is on main, since those pages change with it. Until then `gate reference` refuses, which is correct; it is not a derived step.
- ST0091 AT-01.6 and the devbin sweep, one unit for 3.3.0 (vc's rulings, hv decisions 37 and 38). Built on devbin 0.1.21 in tmp/wt-dc-devbin0121 and banked UNJUDGED as refs/bank/dc/devbin0121/unjudged-built-on-0.1.21-waiting-0.1.22-on-2763e1f2b; re-sweep onto 0.1.22 once devbin-vc says it is on the tap. The unit: bin/devbin upgrade; install.links for intent and intentd, install.switch: intent use, install.publish: intent install; formula.bin and @@FINISH@@ gone, the template's install @@INSTALL@@ alone; the macOS assets packed as ONE archive, intent-aarch64-apple-darwin.tar.gz, by Intent's own cmd/macos pack after notarize (hv: leave it on Intent's handler); intent install's opt manifest gains '# line: intent <v> (opt, <sha12>)' with a writer arm and a reader arm, run red then green in one slot; release.preflight's four checks, preflight clean against DEVBIN_RELEASE_VERSION, and release_preflight_seam.bats for AT-01.6. Proofs: the rendered keg is 3.2.8's (164 files, byte-equal on v3.2.8's real assets), devbin doctor 0 failed with the built pair and an opt tree, bin/devbin use and install refused for Intent, brew_formula green. vc judges light with test shell. (edited)
- ST0092 WP-04 bank (ii), after cc's WP-05 lands: the recipient changes. wb ask to a retired node is refused naming the retirement, wb announce skips it, a message to the leaver it never handled is named at its sender's next pickup, and wb archive message accepts it; the help for wb ask, wb announce and wb archive message names the retired-node behaviour (vc's ruling). Builds on bank (i)'s WbNode.retired.

## Holds

- ST0090/02 stays WIP after its landing at a54f2e9e0 until AC-02.4 is met: hv sees the Rebuild Search Index row's second line after a rebuild, on a build that carries WP-01 and WP-02, and vc records which build that was. Then dc closes WP-02 on vc's word; a defect hv sees comes back to dc as a new bank. The Debug app dc built on 2026-11-01 (IntentCommit 29e6f1c0a) went with that session's scratch. dc rebuilds it on vc's word once 3.3.0's first train's last unit is on main (vc, 2026-11-10), at the HEAD vc names, booked as a light run with the estate's controller, for hv's look by the route vc hold 6 records. dc removes tmp/wt-dc-0090-app once the look is recorded. (edited)

## Watch-outs

_(none)_

## Decisions

- ST0092 WP-04 rulings (vc, 2026-11-11): retirement is WbNode.retired: Option<WbRetired { at, reason }> in board 4.1 (folded, one shape for 3.3.0), never a status value; store rung 34 to 35. A re-apply reinstates nothing as it retires nothing: a retired declared seat shows as 'retired <m> (<reason>)' and comes back only through wb register (WP-09's doctor names it, shown not counted; cc told). The human seat has one home, seat::human_seat (declared human, else model::HYPERVISOR, no install read), which WP-06 rebases onto. The red is a door file (intent-cli tests/wb_retire_door.rs) red at runtime on the base, with wb_retire.rs as the precise half. surface_check.sh's INV-09/10/11 refusal predates WP-04 and goes to ic's WP-10.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
