---
node: dc
name: DevX Claude
role: devx
session_id: 2aba47df-5301-4790-a6b7-e11815afe307
heartbeat_at: 2026-11-10 15:56Z
status: active
focus: "2026-11-10, the hold is lifted on hv's move (vc decision 116; devbin-vc holds the pen while hv is away). dc resumes ST0091 WP-01 bank (a) in tmp/wt-dc-0091 (doing 151), then bank (b), then WP-02 and WP-03; vc judges the train in one desk run. Claims: ST0090/02, ISSUE:0670, ST0091/01, ST0091/04. Worktrees: tmp/wt-dc-0091 and tmp/wt-dc-0090-app (hold 37)."
claims: [ST0090/02, ISSUE:0670, ST0091/01, ST0091/04, ST0091/02, ST0091/03]
---

# DevX Claude (dc)

## DOING

- ST0091/01, the cut runs through devbin's core and Intent's driver retires, dc's on vc's order of 2026-11-10 09:09Z; vc judges it and claims nothing. Built as two banks on the train after WP-04's trigger unit, judged in one run, each leaving the tests it touches green on the bank before it (vc decisions 112 to 117; WP-01's body). (edited)

  Bank (a) is refs/bank/dc/st0091/wp01a-v2 (blob 91b283aadaca, patch-id 118866bca26c), which supersedes wp01a-v1: the cut's pre-flight checks and stamp steps, the driver and its two test files gone. v2 adds the referent wrapper on the preflight run lines and exempts the stamp's, whose stdout the core reads as paths. Every test file it touches passes on it alone.

  Bank (b) is refs/bank/dc/st0091/wp01b-v1 (blob 3e81cb71c66c, patch-id 184485758ff4), on bank (a)'s tree: bin/.devbin/cut/gate with contract, reference, rust (issue 0481's one re-run), and on macOS pair, provenance and doctor; `release.gates: [gate all, test shell]`. Its test file passes; its reds are driven on bank (a)'s tree, the code as it was before it. The light gates ran on the worktree. contract passes. reference refuses truthfully: main's docs/reference still reports against v3.2.7, so the pages are regenerated before the cut. The rust and pair gates are heavy and have not run here.

  Rows: AT-01.1 to AT-01.3 and AT-01.5 to-write; AT-01.4 at the landing. vc judges the four packages in one run once WP-02 and WP-03 are banked, and drives `gate all` on the tip itself. WP-02 is started.
- ST0091/04, the workflows fire on the candidate and the docs describe the cut as built, dc's; vc judges it and claims nothing. Started and claimed on vc's C4 ruling (decision 112). Its trigger part is banked as the train's first unit at refs/bank/dc/st0091/wp04-trigger-v1 (blob ee3409fcdced, patch-id 2c4e9bc3ab35, base 991e92946), held staged in tmp/wt-dc-0091. tests.yml and linux-release.yml fire on candidate/** and not on tags; tests.yml calls rust.yml for a candidate or a dispatch, so its verdict is inside the one run the cut reads, and test-summary fails a candidate on which the macOS or rust leg was skipped. AT-04.1 cites tests/unit/release_candidate_trigger.bats and stays to-write until the train lands, since `intent at red` refuses a row whose cited file is not on main; each red driven against its three tests is the base's own line restored. vc judges the bank with the train in one run on its tip, and vc puts one dispatch of tests.yml on main, after the push, to hv as a precondition of the cut. The docs part (AC-04.2) follows the rest of the train, and AC-04.3 is hv's first cut through the core. (edited)
- ST0091/02, kovacs runs the cut's gates as its leg and prepush keeps only its push gates, dc's; vc judges it and claims nothing. Banked at refs/bank/dc/st0091/wp02-v1 (blob 47b34ed12de4, patch-id dc4a58b8c07b), a diff from the tree of wp01a-v2 and wp01b-v1 (159550239aa1). release.legs names kovacs; preflight-linux.sh and preflight.lib are gone; prepush no longer clones, builds or asks for a Linux preflight, and asks no currency verdict of a push that moves only candidate refs, the prefix read from release.publish.candidate (vc's F2 ruling). The test files it touches pass; prepush_push_range.bats's reds on the tree before it are arms 1, 3, 4, 5 and 8, and arms 2, 6 and 7 hold what did not change. A candidate push and a main push were driven through the real hook on a scratch clone: the first passes with no pair, the second is refused. AT-02.1 and AT-02.2 cite prepush_push_range.bats, which is on main, so they are minted at the landing with AT-01.4. intent/docs/releasing.md still describes the Linux preflight: that is WP-04's docs part.
- ST0091/03, the tap and the macOS assets publish through the built-ins and Intent's own stage, dc's; vc judges it and claims nothing. Banked at refs/bank/dc/st0091/wp03-v1 (blob d168a870bb99, patch-id 6f6bb9badad3), a diff from wp02-v1's tip tree (e7e4a09572b8); the train's tip is 0522909e90fe. To vc decision 118: bin/.devbin/cut/publish is the first after: command (pair at the tag, Intent's macos prepare and publish, release attach for the sums, the built-in formula publish --pull), stopping at the first failure and safe to run again; brew smoke --at-tag is the second, saying the publish did not finish when the tap is not at the tag's version. The formula and the cask are templates under packaging/homebrew/, rendered by the built-in in assets mode, with 0667's service fix; macos prepare refuses a pair that is not a release build of HEAD (F1); cmd/brew keeps the smoke alone and brew.lib the names. The rendered formula and cask pass brew style in a throwaway tap. The whole train's dry run on a scratch clone passes pre-flight and names both after: commands. release_publish.bats and brew_formula.bats are red, every arm, on the tree before it. AT-03.1 and AT-03.2 to-write; AT-03.3 at the landing.

## TODO

- ST0091/02, kovacs runs as the cut's remote leg and prepush keeps only its push gates (S): after WP-01, claimed in the chain that starts it.
- ST0091/03, the tap and the macOS assets publish through the built-ins, with prepare as a shadow (S), and issue 0667's fix on AC-03.3: after WP-02. devbin-vc's answer, relayed in vc's CHAIN END of 7718a6248: the cask's sha256 is filled from the sums across formula and cask, which stands for issue 0634's check. devbin never built a brew smoke and none is pending, so Intent keeps its own and `cmd/brew` does not go whole. vc rewords WP-03's body and AC-03.1 to that before dc reaches WP-03. (edited)
- ST0091 F8 (vc, 2026-11-10): regenerate docs/reference against v3.2.8 and commit it as the last unit before the cut, after ST0092's surface is on main, since those pages change with it. Until then `gate reference` refuses, which is correct; it is not a derived step.

## Holds

- ST0090/02 stays WIP after its landing at a54f2e9e0 until AC-02.4 is met: hv sees the Rebuild Search Index row's second line after a rebuild, on a build that carries WP-01 and WP-02, and vc records which build that was. Then dc closes WP-02 on vc's word; a defect hv sees comes back to dc as a new bank. The Debug app dc built on 2026-11-01 (IntentCommit 29e6f1c0a) went with that session's scratch. dc rebuilds it on vc's word once 3.3.0's first train's last unit is on main (vc, 2026-11-10), at the HEAD vc names, booked as a light run with the estate's controller, for hv's look by the route vc hold 6 records. dc removes tmp/wt-dc-0090-app once the look is recorded. (edited)
- ISSUE:0670 stays claimed and unbuilt until ST0091's four packages are on main (vc's order of 2026-11-10 09:09Z). It rides 3.3.0 (hv directive 1, vc decision 109), and it no longer waits for Devbin's ST0011, which closes only at Intent's first cut through the core, 3.3.0 itself (vc's reading, open to hv's reversal). vc's answers to dc's two shape questions are in the issue's record under The shape of the build: rivals as dc read them, a file:line row read at that one line, and whole-token matching beside it. vc judges the build with the fleet census at refs/bank/vc/0670/ re-run against it, which must redden Devbin ST0011 AT-3.5 and nothing else (vc todo 77). (edited)

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
