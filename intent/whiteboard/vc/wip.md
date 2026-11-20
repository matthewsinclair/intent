---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-20 16:31Z
status: active
focus: "2026-11-20 16:22Z by date -u: LOCALFOLDED for hv's second compact, not released. Landed since the first: 7ed879ab3 (WP-14), 1d3604937 (WP-12 guard), c7e1cee74, 45028cbd3 and d663e3ea6 (ic: ST0096 WP-02 and remint), and cc2358b01 (dc: 0718, 0715 and 0729). Next: land WP-07 v3+ (devbin-cc, after cc's meaning read), cc's ST0096 WP-01, dc's 0722 and devbin-dc's ST0096 WP-03. For hv: organize --default (DOING 63)."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 74; P0, 'or P(-1)', hv decision 72): UNDER WAY since 2026-11-19. DONE: WP-01, WP-02, WP-04, WP-14 (4c72d9510). LANDED on 2026-11-20, each on its builder's proof, read by vc and not rerun, through WP-05+06 (ed302b7e4); then WP-14 with 0719 and 0708 closed (7ed879ab3); WP-12's guard half, AC-12.1 (1d3604937, devbin-dc); the remint doors, AC-10.5 and AC-06.3 (45028cbd3 and d663e3ea6, ic); 0718, 0715 and 0729 with WP-14's R1 (cc2358b01, dc; vc's W4 ruling: an attach never destroys an author's unsynced file). THE SHARED SET is FROZEN at e50978ce0 (decision 132). OPEN: WP-03 for AC-03.3; WP-06 for AC-06.1's issue half (ST0096 WP-01) and AC-06.6 (the drill). IN FLIGHT: devbin-cc's WP-07 v5. v4 (5cda68866) has ic's read: F1, F2 and R1 met, cc's meaning read done. vc ruled at 16:33Z by date -u that F3 (render::address_of's issue arm still on normalise_issue_id) and two mcp.rs doc nits fold into v5, with critic-rust on the v2..v5 interdiff and one gate; ic reads only F3's hunk. dc's 0722 (wip1 a66e2604c), design to cc on silence. NEXT: ic's WP-10 Part 2 with issue 0732 (after WP-07), WP-11 and WP-13; dc's WP-08, WP-09 and WP-12's routing half. Rule: a bank that renames or removes a cited test re-cites its AT row in the land (absent_at_check; the gyges gate runs no repo-local guard). (edited)
- ST0096, issue 0705 (hv decisions 66, 70, 71 and 74): UNDER WAY since 2026-11-19. LANDED: WP-02 at c7e1cee74 (ic; the aggregate views leave git; 0707's doctor half; 0730 closed); WP-03's shell half of AC-03.1 at 1c45f817c (devbin-dc on dc's brief; the gate reads AT rows from canon; brew smoke keeps init's .gitignore; open on it: vc todo 94). 0707 stays open for cc's upgrade half; 0733 (dc, devbin-dc) is stale_at and absent_at's one canon reading. IN FLIGHT: cc's WP-01, realisation onto the record (bank refs/bank/cc/st0096/wp01-wip7-on-d663e3ea6; gate at 298d27f24 running; it carries the seven ST0095 AT markers and Machines 6 and 7 in ST0056's data-model.md). NEXT: cc's WP-05; dc's WP-04. FOR hv (vc's condition 3 on Machine 6): organize --default and --force set every record's realised from status, which is today's behaviour, kept and not written as an edge; vc recommends accepting it. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- devbin-dc, lent by the Desk (hv decision 72), on dc's briefs. DONE: 0723+0724 (9e533703e), 0725 to 0728 (dddd6345b), 0731 (fb3a501d1), ST0095 WP-12's guard half (1d3604937) and ST0096 WP-03's shell half (1c45f817c). NOW: issue 0733 on dc's brief. dc reads each proof; vc lands it. (edited)
- ST0096 WP-03's frozen-pair shape (dc's suggestion, 2026-11-20): once the gate's pair carries WP-02, drive the bare fresh clone with no view on disk through the commit gate again, with Desk run 1's control (01M7WW8B); landed at 1c45f817c, refused then only by e50978ce0's doctor. Also open on WP-03: the brew smoke on a real keg. Issue 0733 (dc, devbin-dc unit) is the shared canon reading for stale_at and absent_at.

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
