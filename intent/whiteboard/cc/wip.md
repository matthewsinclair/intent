---
node: cc
name: Control Claude
role: control
session_id: 7e123f3a-d490-465a-a475-bff3f68353b6
heartbeat_at: 2026-10-13 08:51Z
status: active
focus: "LOCALFOLDED 2026-10-13 for hv's demo: holding (hold 29) until vc relays the demo is over; resume is doing 78 (the WP-03 objective write, then the kovacs proof on vc's GO after dc's lane), then todo 40"
claims: [ST0081]
---

# Control Claude (cc)

## DOING

- ST0081 (Intent on Windows through WSL) IS cc's, on vc's order of 2026-10-12 from hv's rulings: 3.2.4 is ONE release, cut only once the work runs on main's HEAD. HELD BY THE DEMO HOLD until vc relays that hv's demo of 2026-10-13 is over. LANDED 2026-10-12: the 3.2.4 CHANGELOG section at 21f72c2b9; WP-01 at 86e4b0f7d (bin/int linux doctor|build|package|smoke, the dispatch-only linux-release workflow, SUPPORT_PATHS in cmd/shared/support.lib); WP-05 at 1c21a04ff (start and stop with --at-login refuse off macOS before writing anything); AT-05.1 recorded at 188a42fe1, still to-write (watch-out 58); the clippy 0.1.99 fix at ee47b0e37. MEASURED AT BOOT, 2026-10-13 07:00Z to 07:03Z by cc's clock: main 3ea997c93; the installed pair 701c4d98b lacks only ee47b0e37 and 1c21a04ff under DIRT_SCOPE, which vc's `bin/devbin build all` right before hv's push covers; `intent ac gate ST0081` BLOCKED 0/7; no worktree and no unlanded bank (refs/bank/cc/0081/wp01 and wp05 are spent and kept, watch-out 57). vc's RULINGS OF 2026-10-13 under hv's pen, live channel, carried here: (a) cc RUNS the kovacs proof and vc JUDGES it: vc books the slot in the box queue, takes the census at both ends and judges from cc's logs. (b) THE GO COMES AFTER dc's LANE: SUPPORT_PATHS (support.lib:76) ships intent/plugins/claude, and 0591, 0592 and 0593 are skills under it, so they, and M2 if hv scopes it in, land first and the proof runs ONCE, on the tree that gets pushed. (c) kovacs proves bin/int linux on aarch64 and AT-05.1; it does NOT satisfy AC-01.1 or AC-01.2, which name the workflow_dispatch run (x86_64, the kept artifact, smoke in the floor container) that follows hv's push. NEXT, in order, once the demo hold lifts: (0) the `intent set` on WP-03's objective (todo 40), before WP-03 starts. (1) THE KOVACS PROOF on vc's GO, once hv says the VM is ready; vc carries the VM-readiness and Rosetta questions to hv, and an answer hv gives in cc's session is relayed to vc verbatim and not acted on. A git bundle of main, never a push; clone into the VM's own filesystem; `bin/int linux doctor`, `build --jobs 1`, `package` and `smoke`, each logged with its rc, and `file` on both binaries. For AT-05.1, apply the test half of 1c21a04ff ALONE (the test file and its suite.rs line) to 8c51de484 and run its two arms (expect red, and read that the red is the refusal assertion rather than a build or environment failure), then run them at the HEAD that gets pushed (expect green), then `intent at green ST0081 AT-05.1` citing both runs, on vc's word. NAME judging checkouts after a number, never a tool, because issue 0594 reds armed_tool_preconditions.bats in a path that names one. (2) WP-03 (todo 40). WP-02 waits on hv's D55; WP-04 is hv's to arrange. NO PUSH, NO RELEASE. (edited)

## TODO

- ST0081 WP-03, after the kovacs run, on vc's yes of 2026-10-12 to interface (1), which vc relayed to devbin-vc. The install step's ONE home is a bare `## Linux` section in docs/install.md, so the anchor stays #linux; the WSL specifics go in a new top-level docs/windows-wsl.md, which links to install.md#linux and never restates it. vc's three asks: (a) link docs/windows-wsl.md from docs/index.md; (b) every command on both pages comes from the VM's run of the package; (c) install.md's "Verifying an install" and "What the install has to contain" read true for the Linux package, by a line or by a pointer. The WSL page names jq and shellcheck beside Intent, because interface (2) is intent, shellcheck, git and jq. vc's RULINGS OF 2026-10-13 under hv's pen, live channel, carried here: THE VM's RUN GOVERNS THE WRITING, so WP-03 does not wait for the push. Once the demo hold lifts, and before WP-03 starts, `intent set intent:///threads/ST0081/wp/03 objective --from <file>` rewords the objective to say three things: the pages are written from the VM's run of the package; every command on them is re-run against the dispatch run's x86_64 artifact before the tag (where that runs is open: kovacs could run the static musl pair under Rosetta, unmeasured, and vc carries that to hv); and AC-03.1's run, from a release download on a fresh WSL distro, closes it after the tag. THE TWO CHANGELOG LINES GO IN WP-03's BANK: one for the Linux package and the WSL page, one for WP-05's --at-login refusal; vc has both on its pre-cut checklist. CHANGELOG.md is a shared aggregator, and dc's lines for 0591-0594 may land in it, so diff it against HEAD before the git add. cc's plan, to put to vc with the bank: installing WSL itself cannot be run on kovacs, so the WSL page links Microsoft's docs for that step rather than giving a command. (edited)

## Holds

- **ALL OF cc's WORK, ST0081 INCLUDED, IS HELD UNTIL vc RELAYS THAT hv's DEMO OF 2026-10-13 IS OVER** (hv's word, given in vc's session before vc's clock read of 08:47Z, relayed by vc): NOTHING IS TOUCHED BEFORE THE DEMO -- no code, no build, no VM work, no push, and no record write, the `intent set` on WP-03's objective included. When vc relays that the demo is over, archive this hold and resume doing 78 in its order.

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.
- UNTIL THE KOVACS RUN, THE PRE-COMMIT GATE'S STALE-AT ADVISORY NAMES ST0081's AT-05.1 ON EVERY COMMIT ("the file EXISTS while the row says to-write"). It is expected and advisory: vc ran stale_at_check.sh on 2026-10-12, which flags AT-05.1 and exits 0. The row stays to-write on vc's ruling, because its two arms are ignored on macOS and have run nowhere, and `red` would claim a failure nobody has seen. It clears when kovacs runs the arms red at 8c51de484 and green at the landed HEAD, and `intent at green ST0081 AT-05.1` records both.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
