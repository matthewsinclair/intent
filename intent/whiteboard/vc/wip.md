---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-20 07:50Z
status: active
focus: "2026-11-20 07:50Z by date -u: localfolded for hv's estate-wide compact (fold cycle relayed by devbin-vc at 07:47Z), not released. Landed today: WP-10a (7e0eb368c), 0711 (045086a17), 0720 (26a752653). Waiting on: cc's fold after its last test run's END; ic's and dc's folds (WP-02's and the guard S's proofs may come first). Then one line to devbin-vc. After the compact: vc todo 91 (0723+0724 to devbin-dc), and land each proof as it comes."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 74; P0, 'or P(-1)', hv decision 72): UNDER WAY since 2026-11-19 09:38Z by date -u, the thread marked started on 2026-11-20 (60b9adb0e). DONE: WP-01 (ae11d9939), WP-02 (ed1caeb36), WP-04, the waist (31dd4430c). WP-03 is landed and open only for AC-03.3, its screenshots, after WP-06 and WP-10. LANDED 2026-11-20: WP-10a (7e0eb368c; 0710 closed, 0713 open for events --subject), issue 0711 (045086a17, closed), issue 0720 (26a752653, devbin-dc's on dc's brief; dc closes it). The shared set was rebuilt once at 1fd4d88b6 (dc, 07:38Z) and is frozen from WP-06's land. AT hv's FOLD CYCLE (2026-11-20 07:47Z): cc's WP-05+06 unit (L), steps 2 and 3 written, its last test run granted at 07:48Z, steps 4 to 9 on cc's todo 55, one gate run at the end. ic's WP-10 first half as two units: PART 1 (the doors, the events union closing 0713, intent id <token> for dc's WP-12, issue 0721's doctor row) and PART 2 after cc's unit (GraphQL String! ids at SDL face 25, T1 to T9, the 53 hand-built canon test files scoped with cc); ST0096 WP-02 beside it, its proof due. dc: the guard S, proved and in critic review, then WP-14 with 0719, then 0718, 0715 and 0722 as one unit, WP-08 and WP-09 after WP-06, WP-12 after PART 1. devbin-dc after the compacts: issues 0723 and 0724 (vc todo 91), then ST0096 WP-03. WP-14 lands before or after cc's unit, never inside it. Then cc's WP-07, ic's WP-10 second half, WP-11, WP-13. Residue on WP-06's list: facade_st_wp.rs:25, 26 and 288 build a thread path by hand. vc reads each unit's proof with its critic review (vc decision 133); vc decision 132: no dev build touches the main checkout's store. (edited)
- ST0096, issue 0705: UNDER WAY since 2026-11-19 11:38Z by date -u, on hv's go (hv decision 71; 66, 67 and 70 rule the direction, the Highlander check and the same-thread case). cc's canon split for WP-05 is in the design at dd01150ce: one canon file per record (work package, criterion, test and attachment), one unreleased canon version for the release so a project runs intent upgrade once, and the 22 closed records still committed go in the upgrade's commit. hv decision 74 (2026-11-20): acceptance.md and the cover's package table stop being committed; the design carries it (8f6beab03), and AC-02.1, AC-02.2 and AC-03.1 now name the aggregate views. WP-05's criteria AC-05.1 to AC-05.3 are recorded in cc's meaning (45253b5b6). Packages: cc WP-01 (M) and WP-05 (L), both after ST0095 WP-06; ic WP-02 (M), started 2026-11-20 in tmp/wt-st0096-wp02, its proof due; WP-03 (S, after 02) goes to devbin-dc on dc's brief, dc reading its proof (vc todo 91); dc WP-04 (S, the proof with AC-04.2, after 01, 02 and 05). Issue 0704 is superseded by this thread. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- Review vc's remaining one-off scripts in intent/whiteboard/vc/ one file at a time, each read before it goes, and retire the dead ones (hv decision 108, and hv's cleanup rule of 2026-10-12).
- After hv's compacts (fold cycle of 2026-11-20 07:47Z), when the Desk reopens: pass dc's brief for issues 0723 (cut/gate:234's SIGPIPE under pipefail, and the sigpipe guard reaching bin/.devbin's shebang scripts) and 0724 (precommit's six two-way guards read through guard_verdict), one XS unit, to devbin-dc through devbin-vc. The brief is at refs/bank/vc/0723-0724/brief-20261120 (git cat-file -p). dc reads the proof, vc lands it. Then ST0096 WP-03 goes to devbin-dc on dc's brief, once ic's ST0096 WP-02 lands.

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
