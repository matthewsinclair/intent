---
node: dc
name: DevX Claude
role: devx
session_id: 2aba47df-5301-4790-a6b7-e11815afe307
heartbeat_at: 2026-11-10 13:12Z
status: active
focus: "2026-11-10, localfolded on hv's word for hv's compact ("Ok, context is high here, lets bank, localfold+compact here in intent-* and then continue on the bounce", relayed by vc); dc stays active. ST0091: WP-04's trigger unit is banked (refs/bank/dc/st0091/wp04-trigger-v1, AT-04.1 to-write, doing 152). WP-01 is L in two banks, and bank (a) is in progress at refs/bank/dc/st0091/wp01a-wip-v1 (doing 151). Nothing of dc's waits on vc. Claims: ST0090/02, ISSUE:0670, ST0091/01, ST0091/04. Worktrees: tmp/wt-dc-0091 (bank a in progress, HEAD 8770037a2) and tmp/wt-dc-0090-app (hold 37)."
claims: [ST0090/02, ISSUE:0670, ST0091/01, ST0091/04]
---

# DevX Claude (dc)

## DOING

- ST0091/01, the cut runs through devbin's core and Intent's driver retires, dc's on vc's order of 2026-11-10 09:09Z; vc judges it and claims nothing. L, built as two banks on the train after WP-04's trigger unit, judged in one run, each leaving the tests it touches green on the bank before it (vc decisions 112 to 114; WP-01's body, "Rulings on dc's design"). (edited)

  Bank (a) is IN PROGRESS. Its work so far is banked at refs/bank/dc/st0091/wp01a-wip-v1 (blob c0edf8e7b26c, patch-id acbc126c7e7c), against the trigger unit. tmp/wt-dc-0091 holds it unstaged, on HEAD 8770037a2, a scratch commit of the trigger unit. The plan, the layout and a recipe to rebuild the worktree are banked at refs/bank/dc/st0091/wp01a-progress-v2.md (blob b87e61ae7d9b), which adds vc's ruling below to v1.

  Done: releasestamp.lib takes the driver's stamp helpers and three new ones, and MODULES.md names the core, preflight and stamp.

  Next, in order:
  - pair_at_head in releasebuild.lib;
  - the scripts bin/.devbin/cut/preflight/{clean,daemon,faces,spill} and bin/.devbin/cut/stamp/{config,lock,schema,cli,agents,upgrade,views}, declared as commands in config.yaml;
  - the release config;
  - the driver's deletion;
  - the comments;
  - the tests.

  Then bank (b), the gates, with contract_check's handler held for hv.

  vc's ruling after vc's fold (not yet on vc's board; vc writes it into WP-01's body after the bounce): `preflight clean` is built in bank (a) as the fourth seam check, writing nothing, so that a hand edit inside a stamp step's scope cannot ride the release commit. The driver refused it at L538.

  The condition is that it must not refuse the core's own resume. It refuses dirty and untracked paths only while VERSION reads as HEAD's, the state before any stamp began. Once VERSION differs, a stamp is in progress and the core's own tree rule judges. dc drives the resume, and if the core does not resume as vc reads it, dc says so and the check refuses all dirt.
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
