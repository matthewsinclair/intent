---
node: vc
name: Validation Claude
role: validation
session_id: 096ec97d-7b3c-4dab-8be4-db54311270c3
heartbeat_at: 2026-11-01 10:05Z
status: active
focus: "2026-11-01: hv ruled that 3.2.9 goes through devbin's release core after Devbin 0.1.19 (todo 71) and that 0670 waits for ST0011 to ship (hold 7). The three fixes are on main and build all at 64cf9e251 passed; the preflight re-run at 64cf9e251 (hv's ! line) and the push follow (doing 61). dc builds a Debug app for hv's AC-02.4 look (hold 6)."
claims: []
---

# Validation Claude (vc)

## DOING

- The push of 3.2.9's three fixes (0666 cbdee221d, 0668 823cddabd, 0669 bb1efcec5, each judged green by vc and its issue closed); build all at 64cf9e251 passed (64cf9e251e56). The kovacs preflight at 64cf9e251 went RED (kovacs-sysadm's END 09:28:18Z) on a testkit race that predates today (abandoned_fixtures_are_swept::a_run_leaves_no_corpse_of_its_own_behind against its sibling's sweep; testkit unchanged since 721b4efdc's green; devbin-vc verified). Its re-run at the same HEAD is desk ask 01M6B7CCN2SGDKXQ13ZWWRHTVC, and hv has its ! line. On its green, vc names the HEAD and devbin-vc pushes local and upstream (decision 90); until then Intent commits only under intent/whiteboard/ and intent/.canon/events/. After the push, vc records the race as an issue, recorded only, from refs/bank/vc/fold-20261101b/issue-records/testkit-race.title.txt and refs/bank/vc/fold-20261101b/issue-records/testkit-race.body.md, and hv's two rulings of 2026-11-01 in issue 0670 and intent/wip.md. The 3.2.9 cut is hv's, through devbin's release core as Intent's first core cut (todo 71), after Devbin 0.1.19 and its sweep. (edited)

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- ST0091, Devbin ST0011/12 in Intent's tree. At the 0.1.19 sweep devbin-vc runs the upgrade in Intent's tree and commits the sweep pathspec itself (Devbin directive 10), inside an Intent chain with vc and dc; its announcement of the paths and the commit is the landing (devbin-vc's message, read by 13:12Z on vc's date -u). The sweep carries only devbin's vendored half, since Intent's config declares no ci.tag, ci.base or ci.query. vc answers that START with any Intent hold that stands, reads the commit, then judges ST0091's packages as dc banks them, every run booked through devbin-vc's desk. vc claims nothing. The inventory (dc's, checked by vc) went to devbin-vc at 10:46Z and the 3.2.8 baseline at 10:44Z by date -u. (edited)

## Holds

- ST0090 AC-02.4, issue 0664's last criterion: held until hv has seen Intent.app's Rebuild Search Index row show its second line, reconciled <age> ago, after a rebuild, on a build at or after a54f2e9e0 whose CLI carries 8406f6a15: the 3.2.9 cut, or a dev app pointed at a dev CLI, which is the maintainer's alone. The dev CLI half is ready: build all at 64cf9e251 promoted intent and intentd (dev, 64cf9e251e56) into native/rust/target/release on 2026-11-01. The app half is a Debug build at HEAD, dc's, by `bin/devbin macos app-build`, with hv setting IntentBinary to that CLI and deleting it after (dc's route, verified by vc). Then vc records that build on AC-02.4 and tells dc to close WP-02. (edited)
- Issue 0670, on hv's ruling of 2026-11-01 "ambiguous lines stop" (the 1-row shape, recorded in the issue): held until ST0011 has shipped, on hv's ruling of 2026-11-01 in vc's TUI, and it rides the release after 3.2.9. Then vc answers dc's shape questions (refs/bank/dc/0670/shape-questions.md), dc builds it (dc hold 38), and vc judges it with the fleet census (refs/bank/vc/0670/) re-run against the build, which must redden Devbin ST0011 AT-3.5 and nothing else. (edited)

## Watch-outs

- NEVER truncate the output of `git remote -v` (hv, 2026-10-23): read and quote it whole. (edited)

## Decisions

- (hv, 2026-10-23) Molt never touches Intent on rhadamanth except `intent issues add|edit`, committing only what the verb wrote; no repo commits Python. (edited)
- (hv confirmed, 2026-10-25) Intent adopts devbin 0227's shared formula renderer after 3.2.7, once it takes the assets shape. (edited)
- (2026-10-25) Every Intent judge runs `intent critic` on each changed .rs and shell file as a leg; red on warning or above; no planted positives. (edited)
- (hv, 2026-10-25) Brew by default; dev or opt only while working on that project, and switch back to brew after its release. (edited)
- (hv, 2026-10-25) devbin-vc runs Intent's pushes to local and upstream, fast-forward only, on vc's "push" after build all at HEAD; cuts stay hv's. (edited)
- (2026-10-27) Before a worktree is removed, its node points refs/attic/<node>/<worktree> at its HEAD; attic refs are never deleted. (edited)
- (hv, 2026-10-27) Every localfold and EOD fold archives what is spent, so a pickup prints only live state; ST0089 is built after the cut. (edited)
- (hv, 2026-11-01, first-hand by selection in vc's TUI, answered by 10:05Z on vc's date -u; wording vc's, each choice hv's and vc's recommendation) TWO RULINGS. (1) Issue 0670 is built after ST0011 ships and rides the release after 3.2.9; vc answers dc's banked shape questions (refs/bank/dc/0670/shape-questions.md) before dc builds. (2) Intent 3.2.9 is cut through devbin's release core, as Intent's first cut through the core (ST0091, Devbin ST0011's WP12), after Devbin 0.1.19 and its sweep into Intent's tree; until then Intent cuts nothing, and the push of the three landed fixes (0666, 0668, 0669) goes ahead on the Linux preflight at 64cf9e251.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
