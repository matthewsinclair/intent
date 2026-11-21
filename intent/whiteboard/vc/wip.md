---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-21 12:55Z
status: active
focus: "2026-11-21 12:50Z by date -u: localfolded for hv's 12:50Z compact, not released. 3.5.0 is at its last unit: dc's ELIDE fix, then cc's 0746 as the final T's gate, then step 3, the leg compare, hv's dry-run look, the cut and push. Todo 97 has it all."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 74; P0, 'or P(-1)', hv decision 72): UNDER WAY since 2026-11-19. DONE: WP-01, WP-02, WP-04, WP-07 (landed 62045266e, devbin-cc on cc's brief; done at ac0fa4011), WP-14. LANDED on 2026-11-20 on builders' proofs, read by vc and not rerun: WP-05+06 (ed302b7e4); WP-14 (7ed879ab3); WP-12's guard half (1d3604937); the remint doors (45028cbd3, d663e3ea6); 0718, 0715, 0729 (cc2358b01); WP-07 (62045266e). THE SHARED SET is FROZEN at e50978ce0 (decision 132). OPEN: WP-03 for AC-03.3; WP-06 for AC-06.6 (the drill). IN FLIGHT (2026-11-20 17:41Z): ic's WP-10 Part 2 (SDL 25, 0732, 0734, NotAddressable's MIN_PREFIX_HEX floor; proved on v7, rebasing onto 9a87b035c, gyges gate then land); dc's WP-08+09 (gate since 17:32Z); devbin-cc's 0735 (ingest refuses a non-RFC 3339 ts, cc's brief). NEXT: ic's WP-10 T1 to T9, WP-11, WP-13; dc's WP-12 routing half. Rules: a bank that renames or removes a cited test re-cites its AT row in the land; a bank touching surface/dispatch-table.json runs gen_dispatch_table.sh before banking (the gyges gate runs no repo-local guard). (edited)
- ST0096, issue 0705 (hv decisions 66, 70, 71, 74 and 75): UNDER WAY since 2026-11-19. DONE: WP-01 (landed 9a87b035c, cc; done e65e170cc) and WP-02 (landed c7e1cee74, ic; done 48608cbcb). LANDED: WP-03's shell half (1c45f817c, devbin-dc; WP-03 started f49fecad1; open on it: vc todo 94). 0733 closed (29d5ffe94, 4b47fe6c8). 0707 stays open for cc's upgrade half. hv decision 75 (2026-11-20 17:49Z): organize --default and --force keep setting realised from status; no Machine 6 edge. IN FLIGHT: dc's 0722 plus ST0095 WP-08+09 as one stack (both-v2 abdf74504, wp0809-on-0722 41c6c6373), one gate on the tip. NEXT: cc's WP-05 (L, plan to vc before code; the critical path); then dc's WP-04. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- devbin-dc, lent by the Desk (hv decision 72), on dc's briefs. DONE: 0723+0724 (9e533703e), 0725 to 0728 (dddd6345b), 0731 (fb3a501d1), ST0095 WP-12's guard half (1d3604937) and ST0096 WP-03's shell half (1c45f817c). NOW: issue 0733 on dc's brief. dc reads each proof; vc lands it. (edited)
- ST0096 WP-03's frozen-pair shape (dc's suggestion, 2026-11-20): once the gate's pair carries WP-02, drive the bare fresh clone with no view on disk through the commit gate again, with Desk run 1's control (01M7WW8B); landed at 1c45f817c, refused then only by e50978ce0's doctor. Also open on WP-03: the brew smoke on a real keg. Issue 0733 (dc, devbin-dc unit) is the shared canon reading for stale_at and absent_at.
- ST0095's close is waiting on hv's word: AC-00.1, an independent whole-tree Highlander sweep of every id concern in every language, with the start-of-thread findings (refs/bank/vc/st0095/highlander-*) resolved or answered; and AC-00.2, each unit's critic-per-diff record read by vc. Every AT row conforms, and these two block the thread gate.

## Holds

- THE RESTORE AFTER THE TAP (hv decision 77), released when `bin/devbin cut tap` has run for 3.5.0. Then commit bin/.devbin/config.yaml's release.after back to `cut publish` plus `brew smoke --at-tag`, with release_publish.bats's release.after arm changed with it. The comment above release.after names both lines.

## Watch-outs

- Client names in Intent's committed record: only the client's own name is confidential (hv 2026-11-10; narrowed 2026-11-11: an initiative's name is not). Names already on upstream are fixed forward only. (edited)

## Decisions

- (hv, 2026-10-23) Molt never touches Intent on rhadamanth except `intent issues add|edit`, committing only what the verb wrote; no repo commits Python. (edited)
- (hv, 2026-10-25) Brew by default; dev or opt only while working on that project, and switch back to brew after its release. (edited)
- (hv, 2026-10-25) devbin-vc runs Intent's pushes to local and upstream, fast-forward only, on vc's "push" after build all at HEAD; cuts stay hv's. (edited)
- (2026-10-27) Before a worktree is removed, its node points refs/attic/<node>/<worktree> at its HEAD; attic refs are never deleted. (edited)
- (hv, 2026-10-27) Every localfold and EOD fold archives what is spent, so a pickup prints only live state. Its second clause, that ST0089 is built after the cut, is superseded by hv decision 59 (2026-11-17): ST0089 stays parked until hv rules on its policy. (edited)
- (hv, 2026-11-10) The tree holds only what is live or to be done: a resolved artefact's realised files leave by `intent organize --apply` after its preview is read; Not Started threads, ST0056 and ST0057 stay. Full record: `intent wb show vc --all`, decision 108's earlier text in git. (edited)
- (vc, 2026-11-19 09:44Z by date -u, a sequencing ruling in vc's remit, on dc's measurement of 09:41Z) ST0095: UNTIL THE RELEASE IS INSTALLED FROM BREW, NO DEV BUILD RUNS AGAINST THE MAIN CHECKOUT'S STORE, AND MAIN'S CANON STAYS IN TODAY'S FORMAT. Every seat's wb, st, wp and issues verbs in the main checkout stay on brew 3.4.0. ST0095's builds, suites and proofs run in worktrees, each with its own store, on gyges, or, for the drill, in a clone. From the moment WP-06 lands, nobody rebuilds the shared set in native/rust/target/release/, so nothing that can reach main's store carries schema 36; dc's WP-03 guard covers the commit gate. The build spread is dc's, as proposed: cc on rhadamanth's cargo lane, booked at the Desk; ic's suites on gyges through dc's leg repository, booking nothing; dc in rhadamanth's bats lane, its builds behind cc's; each seat's gate runs with its own DEVBIN_GATE_CACHE; kovacs unused until the rehearsal (hv decision 45).
- (vc, 2026-11-19 09:53Z by date -u, under hv decision 67) HOW ST0095's AND ISSUE 0705's HIGHLANDER CHECK RUNS. (1) EXISTING CODE: an independent sweep of every place the tree mints, reads, formats, orders, validates or scans an id, or reads or writes .intentfiles or the two aggregate views, in every language, run now by three read-only reviewers vc launched (intentsvcs; the other crates and every test; everything that is not Rust), and given none of the seats' censuses. vc compares it with the censuses and sends each finding to the seat whose package owns it, which adds it to that package's scope. The reports are banked under refs/bank/vc/st0095/highlander-*. (2) EVERY CHANGE: each unit's proof carries a critic-<lang> review of its diff for every language the diff touches, with each Highlander finding fixed or answered, and vc reads it against the diff before saying land. From WP-04 on, a unit that handles an id does so through intentsvcs::id and says so. (3) AT THE CLOSE: the same sweep runs again over the whole tree and must find one home for every concern. (1) and (3) are ST0095's AC-00.1 and (2) is its AC-00.2; 0705's thread carries the same.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
