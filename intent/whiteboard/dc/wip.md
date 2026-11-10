---
node: dc
name: DevX Claude
role: devx
session_id: 2aba47df-5301-4790-a6b7-e11815afe307
heartbeat_at: 2026-11-10 12:22Z
status: active
focus: "2026-11-10, after hv's compact, on vc's direction. vc ruled on the ST0091 WP-01 inventory (decision 112, 7718a6248): WP-01 is M and its writes are released, except the intentd check's wiring, which waits for devbin's pre-flight seam (with hv). Next: WP-04's trigger unit in tmp/wt-dc-0091 as the train's first bank, then WP-01's design with the split of where each check lives. Claims: ST0090/02, ISSUE:0670, ST0091/01, ST0091/04. Worktrees: tmp/wt-dc-0091 and tmp/wt-dc-0090-app (hold 37)."
claims: [ST0090/02, ISSUE:0670, ST0091/01, ST0091/04]
---

# DevX Claude (dc)

## DOING

- ST0091/01, the cut runs through devbin's core and Intent's driver retires, dc's on vc's order of 2026-11-10 09:09Z; vc judges it and claims nothing. Rescoped to L on vc's ruling of dc's design (refs/bank/dc/st0091/wp01-design-v1.md, blob 87f6f82b59f7), and split into two banks on the train after WP-04's trigger unit. The two are judged in one run, and each leaves the tests it touches green on the bank before it. Bank (a) holds: (edited)
  - `stamp`, seven derived steps, each printing exactly the paths it wrote;
  - `preflight`, the daemon, faces and spill checks, each writing nothing, wired once devbin 0327 is swept in;
  - the release config;
  - the driver's deletion;
  - pair_at_tag, moved into releasebuild.lib as pair_at_head, with prepush_push_range.bats re-pointed and tagging annotated.
  Bank (b) holds `gate` (pair and provenance on macOS, then doctor, reference and rust) and release.gates. contract_check's gate is held for hv. The trailer, devbin 0328, is one config line after the sweep.
- ST0091/04, the workflows fire on the candidate and the docs describe the cut as built, dc's; vc judges it and claims nothing. Started and claimed on vc's C4 ruling (decision 112). Its trigger part is banked as the train's first unit at refs/bank/dc/st0091/wp04-trigger-v1 (blob ee3409fcdced, patch-id 2c4e9bc3ab35, base 991e92946), held staged in tmp/wt-dc-0091. tests.yml and linux-release.yml fire on candidate/** and not on tags; tests.yml calls rust.yml for a candidate or a dispatch, so its verdict is inside the one run the cut reads, and test-summary fails a candidate on which the macOS or rust leg was skipped. AT-04.1 cites tests/unit/release_candidate_trigger.bats and stays to-write until the train lands, since `intent at red` refuses a row whose cited file is not on main; each red driven against its three tests is the base's own line restored. vc judges the bank with the train in one run on its tip, and vc puts one dispatch of tests.yml on main, after the push, to hv as a precondition of the cut. The docs part (AC-04.2) follows the rest of the train, and AC-04.3 is hv's first cut through the core. (edited)

## TODO

- ST0091/02, kovacs runs as the cut's remote leg and prepush keeps only its push gates (S): after WP-01, claimed in the chain that starts it.
- ST0091/03, the tap and the macOS assets publish through the built-ins, with prepare as a shadow (S), and issue 0667's fix on AC-03.3: after WP-02. devbin-vc's answer, relayed in vc's CHAIN END of 7718a6248: the cask's sha256 is filled from the sums across formula and cask, which stands for issue 0634's check. devbin never built a brew smoke and none is pending, so Intent keeps its own and `cmd/brew` does not go whole. vc rewords WP-03's body and AC-03.1 to that before dc reaches WP-03. (edited)
- Issue 0665, a completed thread's green row that stays conforming after its cited test is deleted: after 0670 and in the same code, once hv has ruled its shape and vc says so.
- Issue 0649, the lints the critic defers to clippy that no gate runs: measure first, with one booked run that counts each named lint's findings per crate, then propose to vc which lints are enabled where and which leave critic_tool_codes.

## Holds

- ST0090/02 stays WIP after its landing at a54f2e9e0 until AC-02.4 is met: hv sees the Rebuild Search Index row's second line after a rebuild, on a build that carries WP-01 and WP-02, and vc records which build that was. Then dc closes WP-02 on vc's word; a defect hv sees comes back to dc as a new bank. The Debug app dc built on 2026-11-01 (IntentCommit 29e6f1c0a) went with that session's scratch. dc rebuilds it on vc's word once 3.3.0's first train's last unit is on main (vc, 2026-11-10), at the HEAD vc names, booked as a light run with the estate's controller, for hv's look by the route vc hold 6 records. dc removes tmp/wt-dc-0090-app once the look is recorded. (edited)
- ISSUE:0670 stays claimed and unbuilt until ST0091's four packages are on main (vc's order of 2026-11-10 09:09Z). It rides 3.3.0 (hv directive 1, vc decision 109), and it no longer waits for Devbin's ST0011, which closes only at Intent's first cut through the core, 3.3.0 itself (vc's reading, open to hv's reversal). vc's answers to dc's two shape questions are in the issue's record under The shape of the build: rivals as dc read them, a file:line row read at that one line, and whole-token matching beside it. vc judges the build with the fleet census at refs/bank/vc/0670/ re-run against it, which must redden Devbin ST0011 AT-3.5 and nothing else (vc todo 77). (edited)

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
