---
node: vc
name: Validation Claude
role: validation
session_id: 124edee5-f105-45bd-8483-ee7dc6dd6fd9
heartbeat_at: 2026-11-10 07:30Z
status: active
focus: "2026-11-06, folded for hv's compact before hv travels: nothing moves, and (devbin-vc's relay of hv's order) no desk asks, until hv's word after the travel. Issue 0672 is triaged high and its fix banked for hv's three decisions (todo 75). Holds checked at the fold: 6 stands (AC-02.4 unsatisfied), 7 stands (Devbin ST0011 is WIP), and 8's devbin half is met (0.1.19 and 0.1.20 swept, todo 71) while the desk's grant waits on hv. Watchout 33: a third remote and no release.remotes before ST0091's cut. No other Intent node is running."
claims: []
---

# Validation Claude (vc)

## DOING

_(none)_

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- ST0091, Devbin ST0011/12 in Intent's tree. The sweep half is done: devbin-vc swept devbin 0.1.19 into Intent's tree (066bd45bb, then dfef527a3 moving its source record to Homebrew's install) and then 0.1.20 (18a75d29c), each committed by devbin-vc by path, and vc read all three on 2026-11-06: each touches only bin/.devbin/. Left: dc builds ST0091's four packages (each S, Not Started) and vc judges each as dc banks it, every run booked through devbin-vc's desk; vc claims nothing. Before the first cut through the core, hv says which remotes it pushes to and release.remotes is declared (watchout 33). The inventory (dc's, checked by vc) went to devbin-vc at 10:46Z and the 3.2.8 baseline at 10:44Z by date -u on 2026-11-01. (edited)
- Judge and land the obvious-bug fixes hv put in 3.2.9 (decision 106), once devbin-vc's desk grants Intent again: it grants only devbin's ST0011 runs until devbin 0.1.19 is cut and swept into the fleet, on hv's word by 11:17Z ("no other work until devbin ships"), and it denied cc's 0671 run 01M6BBKENABGC65S1XAYRCWHD3, dc's sweep-order run 01M6BB981X1E5KDPVY3E7CBWK6 and vc's race judge 01M6BD84EXF70MX346YDTSD3GW, each asked again then. 0671 (cc, doing 103): refs/bank/cc/0671/fix-v1 (patch-id cb974be96777), vc's static read clean; the app passes CLAUDE_CONFIG_DIR from the login shell and the at-login plist carries it without intentd's boot-time heal dropping it. The testkit race (dc, hold 39): refs/bank/dc/testkit-race/fix-v1 (patch-id abcffb4ac0ea); dc's storm run passed (refs/bank/dc/testkit-race/run-v1-stamps.txt, 2 red in 40 at the base, 0 with the fix); vc's light judge re-runs it in tmp/wt-vc-judge-race. The sweep-order fix (dc, hold 40): refs/bank/dc/testkit-sweep-order/fix-v1 (patch-id 3e958d267455) with its mutant red-v1 and run script refs/bank/dc/testkit-sweep-order/run-v1.zsh, stacked on the race fix, judged on the whole workspace suite. Each lands on vc's word naming HEAD; vc claims nothing. (edited)
- Issue 0672 (high), banked for 2026-11-07 on hv's word on 2026-11-06 ("we need to bank this and come back to it tomorrow"), with three decisions owed by hv. (1) The template-only fix (S) or the full shape (M), both sized in the issue. (2) dc builds and vc judges, or vc builds with a critic and a fresh reviewer as the judge leg. (3) The push now, which carries every commit upstream lacks, the three held 3.2.9 fixes among them (hold 8), or with the 3.2.9 train. Either fix reaches a project only at the next cut and that project's next agents sync, because brew's intent renders AGENTS.md from its keg, and 3.2.9 goes through ST0091 (Not Started, four S packages). Every run needs a slot from devbin-vc, the estate's controller. The instruments that re-measure the fleet against a build are refs/bank/vc/0672/survey-swift.zsh, survey-shell-tests.zsh and survey-imports.zsh.

## Holds

- ST0090 AC-02.4, issue 0664's last criterion: held until hv has seen Intent.app's Rebuild Search Index row show its second line, reconciled <age> ago, after a rebuild, on a build at or after a54f2e9e0 whose CLI carries 8406f6a15: the 3.2.9 cut, or a dev app pointed at a dev CLI, which is the maintainer's alone. The dev CLI half is ready: build all at 64cf9e251 promoted intent and intentd (dev, 64cf9e251e56) into native/rust/target/release on 2026-11-01. The app half is a Debug build at HEAD, dc's, by `bin/devbin macos app-build`, with hv setting IntentBinary to that CLI and deleting it after (dc's route, verified by vc). Then vc records that build on AC-02.4 and tells dc to close WP-02. (edited)
- Issue 0670, on hv's ruling of 2026-11-01 "ambiguous lines stop" (the 1-row shape, recorded in the issue): held until ST0011 has shipped, on hv's ruling of 2026-11-01 in vc's TUI, and it rides the release after 3.2.9. Then vc answers dc's shape questions (refs/bank/dc/0670/shape-questions.md), dc builds it (dc hold 38), and vc judges it with the fleet census (refs/bank/vc/0670/) re-run against the build, which must redden Devbin ST0011 AT-3.5 and nothing else. (edited)
- The push of 3.2.9's three fixes (0666 cbdee221d, 0668 823cddabd, 0669 bb1efcec5, each judged green by vc and its issue closed; build all at 64cf9e251 passed): held until devbin 0.1.19 is cut and swept into the fleet and devbin-vc's desk grants again. That is hv's word in devbin-vc's window, by 11:17Z on devbin-vc's clock, "no other work until devbin ships", under which the preflight re-run 01M6B7CCN2SGDKXQ13ZWWRHTVC was denied. Devbin's sweep into Intent's tree (todo 71) moves HEAD outside BOARD_PATHS, so the next Linux preflight is aimed at a later HEAD that carries the sweep and the judged fixes, with hv's ! line; devbin-vc then pushes local and upstream on vc's word naming it (decision 90). When the desk opens, vc also records the testkit race issue (refs/bank/vc/fold-20261101b/issue-records/testkit-race.title.txt, refs/bank/vc/fold-20261101b/issue-records/testkit-race.body.md), the sweep-order issue (refs/bank/dc/testkit-sweep-order/issue.title.txt, refs/bank/dc/testkit-sweep-order/issue.body-v2.md), 0671's addition (refs/bank/vc/fold-20261101b/issue-records/0671.add.md), and hv's rulings of 2026-11-01 in issue 0670 and intent/wip.md. The 3.2.9 cut is hv's, through devbin's release core (decision 105).

## Watch-outs

- NEVER truncate the output of `git remote -v` (hv, 2026-10-23): read and quote it whole. (edited)
- Measured 2026-11-06: `git remote -v` lists a third remote beside local and upstream, and its main is an ancestor of Intent's main, so it has taken Intent's main before. bin/.devbin/config.yaml declares repo but no release.remotes, and its comment still says this checkout has two remotes; devbin's release core pushes to every listed remote when none is declared (cc's source read, 2026-10-23). So the first cut through the core (ST0091) would push to the third remote too. Which remotes a cut pushes to is hv's to say before that cut; decision 90's push names local and upstream.

## Decisions

- (hv, 2026-10-23) Molt never touches Intent on rhadamanth except `intent issues add|edit`, committing only what the verb wrote; no repo commits Python. (edited)
- (hv confirmed, 2026-10-25) Intent adopts devbin 0227's shared formula renderer after 3.2.7, once it takes the assets shape. (edited)
- (2026-10-25) Every Intent judge runs `intent critic` on each changed .rs and shell file as a leg; red on warning or above; no planted positives. (edited)
- (hv, 2026-10-25) Brew by default; dev or opt only while working on that project, and switch back to brew after its release. (edited)
- (hv, 2026-10-25) devbin-vc runs Intent's pushes to local and upstream, fast-forward only, on vc's "push" after build all at HEAD; cuts stay hv's. (edited)
- (2026-10-27) Before a worktree is removed, its node points refs/attic/<node>/<worktree> at its HEAD; attic refs are never deleted. (edited)
- (hv, 2026-10-27) Every localfold and EOD fold archives what is spent, so a pickup prints only live state; ST0089 is built after the cut. (edited)
- (hv, 2026-11-01, first-hand by selection in vc's TUI, answered by 10:05Z on vc's date -u; wording vc's, each choice hv's and vc's recommendation) TWO RULINGS. (1) Issue 0670 is built after ST0011 ships and rides the release after 3.2.9; vc answers dc's banked shape questions (refs/bank/dc/0670/shape-questions.md) before dc builds. (2) Intent 3.2.9 is cut through devbin's release core, as Intent's first cut through the core (ST0091, Devbin ST0011's WP12), after Devbin 0.1.19 and its sweep into Intent's tree; until then Intent cuts nothing, and the push of the three landed fixes (0666, 0668, 0669) goes ahead on the Linux preflight at 64cf9e251.
- (hv, 2026-11-01, first-hand in vc's session, read by 10:21Z on vc's date -u) OBVIOUS BUG FIXES ARE NOT "NEW CODE" AND RIDE THIS RELEASE. hv, verbatim, on 0671's lasting fix: "This can be done now." and "That's not 'net new code'. That's fixing an obvious bug. Fixing obvious bugs is what this release needs." So 3.2.9 carries 0671's fix (cc) and the testkit race fix (dc), each landing after the push of the three fixes on vc's word naming HEAD. 0670 stays as decision 105 ruled.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
