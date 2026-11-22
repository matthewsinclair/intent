---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-21 17:12Z
status: active
focus: "2026-11-21 16:28Z by date -u: localfolded for hv's global fold, not released. 3.5.0 shipped (940c9535a); gyges is on 3.5.0. Next on hv's return: hv's post-3.5.0 scope (dc todos 87, 88; cc standby) and ST0095's close (todo 99)."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids: SHIPPED in v3.5.0 (940c9535a, 2026-11-21). Package records still open: WP-05 shows WIP and WP-13 Not Started though 4a4930e4e landed its docs; their owners close them. The thread's close waits on todo 99 (AC-00.1, AC-00.2), hv's word. (edited)
- ST0096, issue 0705: WP-01, WP-02, WP-05 done; WP-03 and WP-04 WIP. Its WP-03 frozen-pair drive (the bare fresh clone through the commit gate with Desk run 1's control, 01M7WW8B) is to run on brew 3.5.0 when hv sets post-3.5.0 scope. 0733 and 0707 closed. (edited)
- 3.5.1 RELEASED 2026-11-22 11:58Z: tag v3.5.1 at 1cc654e5a, main on local and upstream at 1cc654e5a. Every leg green: candidate run 45018364402, local, macOS and kovacs. brew smoke --at-tag is green on keg 3.5.1. rhadamanth is on brew 3.5.1, doctor is clean, intentd is 3.5.1, and Intent.app is 3.5.1 at 1cc654e5a. Open: (a) devvm's upgrade to 3.5.1 is with AE's vc, arranged by devbin-vc. (b) dc's post-tag explorer-shots check and its rehearsal-worktree removal. (c) step 5's site version (intent.laksa.io still reads 3.3.2) waits on hv's word. (edited)

## TODO

_(none)_

## Holds

_(none)_

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
