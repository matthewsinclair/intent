---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-20 06:43Z
status: active
focus: "2026-11-20 06:43Z by date -u: hv relaunched every seat, and each works on vc's go. cc: ST0095/05+06, step 1. dc: 0711, then the guard S, then the shared-set rebuild. ic: WP-10a's proof, then WP-10's first half and ST0096 WP-02. Desk: devbin-vc, with Intent P0. For hv: ST0096's views (acceptance.md leaves git) now; doctor's default (todo 90) once cc's view is in."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 71, P0): UNDER WAY since 2026-11-19 09:38Z by date -u. DONE: WP-01 (ae11d9939), WP-02 (ed1caeb36), WP-04, the waist (31dd4430c, closed at 06e24ec01, AC-04.2 restated as non-test AC-04.4). WP-03 is landed (fe534431d, 2c909d85e, c5233a0c9; AC-03.1 and AC-03.2 restated as AC-03.4 and AC-03.5) and open only for AC-03.3, its screenshots, after WP-06 and WP-10. IN FLIGHT at hv's 13:00Z compact: cc's WP-05+06 as one unit, step 1 (the hashed family in id.rs) banked green at refs/bank/cc/st0095/wp0506-wip2-on-31dd4430c, steps 2 to 9 on cc's todo 55; ic's WP-10 first half; dc's issue 0711, banked green (fix-wip1, red-test-wip1), waiting only on cc's 12:15Z ruling (one component-wise path key in project.rs) and one gate run. dc's order after it: WP-14 with issue 0719, issue 0718, issue 0715, the cut/gate XS, ST0096 WP-03, WP-08 and WP-09 after WP-06, WP-12 after WP-10. WP-14 lands before or after cc's unit, never inside it. Then cc's WP-07, ic's WP-10 second half, WP-11, WP-13. Residue on WP-06's list: facade_st_wp.rs:25, 26 and 288 build a thread path by hand. vc reads each unit's proof with its critic review (vc decision 133); vc decision 132: no dev build touches the main checkout's store. (edited)
- ST0096, issue 0705: UNDER WAY since 2026-11-19 11:38Z by date -u, on hv's go (hv decision 71; 66, 67 and 70 rule the direction, the Highlander check and the same-thread case). cc's canon split for WP-05 is in the design at dd01150ce: one canon file per record (work package, criterion, test and attachment), one unreleased canon version for the release so a project runs intent upgrade once, and the 22 closed records still committed go in the upgrade's commit. ic's shape call on the views, sent 2026-11-20 06:39Z by date -u: acceptance.md leaves git like the two lists, with no ic package; a split would add an M package and a tracked file per criterion and test, and still could not show a verdict. The forge then loses the rendered page of a thread's criteria and tests and (cc's half) the cover's package table; it keeps the cover's objective and context, each package's view, the attachments and each record's canon JSON. THIS GOES TO hv before cc's ST0096 WP-05 code; ic's WP-02 starts on the two lists and adds acceptance.md on hv's answer. Packages: cc WP-01 (M) and WP-05 (L), both after ST0095 WP-06; ic WP-02 (M), beside WP-10's first half; dc WP-03 (S, after 02) and WP-04 (S, the proof with AC-04.2, after 01, 02 and 05). Issue 0704 is superseded by this thread. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- Review vc's remaining one-off scripts in intent/whiteboard/vc/ one file at a time, each read before it goes, and retire the dead ones (hv decision 108, and hv's cleanup rule of 2026-10-12).
- Put to hv after the 13:00Z compact (ic's note with issue 0717, 12:19Z): intent doctor is declared read in the register, yet by default it creates a missing store and migrates an older one, one way. Either re-declare it as a write, or make it never write by default (option 3 of 0717, a change of behaviour). Every fix found rides this release (hv decision 69), so the call is which fix. vc to frame both with ic's and cc's views before putting it.

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
