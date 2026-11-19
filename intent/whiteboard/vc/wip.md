---
node: vc
name: Validation Claude
role: validation
session_id: 31169586-8a4c-469f-a9f3-0c18e36af09b
heartbeat_at: 2026-11-19 09:38Z
status: active
focus: "2026-11-19 09:38Z by date -u: ST0095 under way on hv's go. WP-01 cc, WP-02 ic, WP-03 dc in parallel; vc reads each unit's proof and says land."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 68, P0): UNDER WAY since 2026-11-19 09:38Z by date -u, on hv's go. WP-01 landed at ae11d9939 and closed at d6c45e915, with intentsvcs::id's registry row at fa86e7e35; WP-02 (ic), WP-03 (dc) and WP-04 (cc) in flight; vc reads each unit's proof with its critic review (vc decision 133) and says land. vc decision 132: no dev build touches the main checkout's store until the release is installed from brew. The Highlander sweep of the existing code is done, banked under refs/bank/vc/st0095/highlander-*-20261119, and its findings went to cc, ic and dc at 10:20Z, each to be answered per item (AC-00.1); issue 0712 was filed from it, and vc's two dead dehydrate scripts were removed (feb4e56e3). cc's calls: WP-05 drops the .intentfiles set merge (M to S) and issue 0704 is superseded by ST0096. hv decision 68: the tap holds the release until hv's word. Open with hv: the 00046 bend, and cc's first-mint refusal in a project not yet upgraded. agentic-engineering-vc waits for vc todo 84, and for its questions (a), (c), (d) and (e) once the fix is done. (edited)
- ST0096, issue 0705 (hv decision 66): written 2026-11-19 at e102ece05 for hv's review, and nothing is built before it. Four packages: cc WP-01 (M, after ST0095 WP-06), ic WP-02 (M), dc WP-03 and WP-04 (S each). For hv: two merge requests that change one thread (recommended: not this release), and ST0095 WP-05 dropping its .intentfiles set merge (cc's call). cc owes the choice between R1 and R2.

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.

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
