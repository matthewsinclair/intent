---
node: vc
name: Validation Claude
role: validation
session_id: 31169586-8a4c-469f-a9f3-0c18e36af09b
heartbeat_at: 2026-11-19 11:33Z
status: active
focus: "2026-11-19 10:47Z by date -u: localfold before hv's compact of every seat, not released. Waiting for cc, ic and dc to report folded; then hv compacts all, and vc lands the next proof (WP-03, WP-04) on the bounce."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 70, P0): UNDER WAY since 2026-11-19 09:38Z by date -u, on hv's go. Landed: WP-01 (ae11d9939), the id waist's registry row (fa86e7e35), WP-02 (ed1caeb36, closed at 0922779b1), and WP-03's two units (fe534431d, which closes issue 0716; 2c909d85e; c5233a0c9). AC-03.1 and AC-03.2 were withdrawn for non-test AC-03.4 and AC-03.5, both satisfied, and WP-03 stays open for AC-03.3, the screenshots, after WP-06 and WP-10. In flight: WP-04 (cc, bank wp04-v3, its proof run to be on main at e81e1596e or later, since issue 0706 touched four of its files). Off cc's chain since 11:33Z: issue 0711 offered to dc, and issue 0717 to ic with cc's meaning (its third option, a store that migrates only on an explicit opt-in, comes to hv before code). vc reads each unit's proof with its critic review (vc decision 133). vc decision 132: no dev build touches the main checkout's store. (edited)
- ST0096, issue 0705 (hv decision 66): written 2026-11-19 at e102ece05, and nothing is built before hv's go. hv decision 70 (by 11:21Z): the same-thread case is closed in this release, which takes a canon file per work package, criterion and test and thread views none of them share (cc's meaning, ic's views, L to XL on cc's chain); on the design as a whole hv asked for more explanation, which vc gave in prose right after recording decision 70. Packages: cc WP-01 (M, after ST0095 WP-06), ic WP-02 (M, held), dc WP-03 and WP-04 (S each), and cc's same-thread package, designed after ST0095 WP-04 lands and written into the design before code. cc chose R1 (AC-01.3 satisfied); ST0095's WP-05 dropped its .intentfiles set merge, and issue 0704 is superseded by this thread. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- Issue 0715 (st attach --from says ok and changes nothing while intentd watches the tree) needs an owner: put it to cc once WP-04 has landed, since the cause sits between the attach verb and intentd's ingest.
- Review vc's remaining one-off scripts in intent/whiteboard/vc/ one file at a time, each read before it goes, and retire the dead ones (hv decision 108, and hv's cleanup rule of 2026-10-12).
- When ic rules on two shape points for hashed claims (asked 2026-11-19 10:39Z: does a board's rendered claims line print the stored or the shown form; does a hashed issue's claim keep the ISSUE: prefix), send its answer to agentic-engineering-vc, whose course unit 207 cites it. AE was told to read claims with intent wb status --json instead.
- Issue 0711 (migrate's case-only path clash, S) can move off cc's chain (cc, 2026-11-19): offer it to dc once WP-03 has landed, or to whichever seat runs out of work first after WP-04 lands; otherwise it stays cc's after WP-09.
- hv's word on ic's wording for ST0095 WP-06 (ic's message of 11:30Z, cc-checked): the first mint's refusal in a project not yet upgraded, and doctor's canon-format-behind advisory. Put to hv in vc's session; record the ruling on hv's board, tell ic and cc, then clear ic's message.

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
