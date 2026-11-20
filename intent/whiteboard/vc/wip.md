---
node: vc
name: Validation Claude
role: validation
session_id: 5af3bd13-884c-4502-be68-30788850cf56
heartbeat_at: 2026-11-20 15:08Z
status: active
focus: "2026-11-20 15:08Z by date -u: LOCALFOLDED for hv's compact cycle, not released. Landed today: through WP-05+06 at ed302b7e4 (DOING 62). The shared set is frozen at e50978ce0. Next after the compact: land proofs as they arrive, from devbin-cc (WP-07), ic (WP-02 with 0730, and the remint doors), dc (WP-14 rebased), devbin-dc (WP-12's guard half) and cc (ST0096 WP-01)."
claims: []
---

# Validation Claude (vc)

## DOING

- ST0095, git-style ids (hv decisions 61 to 74; P0, 'or P(-1)', hv decision 72): UNDER WAY since 2026-11-19. DONE: WP-01, WP-02, WP-04. LANDED on 2026-11-20, each on its builder's proof, read by vc and not rerun: WP-10a (7e0eb368c); 0711 (045086a17); 0720 (26a752653); the guard S of WP-03 (ea22af4c7); 0723+0724 (9e533703e); WP-10 PART 1 (c437d1d83; intent id; 0713 and 0721 closed); 0725-0728 (dddd6345b); 0731 (fb3a501d1); WP-05+06 (ed302b7e4, cc's unit: the hashed family, the one mint, a merge that knows both families, store schema 36, SDL 24). THE SHARED SET was rebuilt once more at e50978ce0, with intent id and schema 35, and is FROZEN from WP-06's land (decision 132). OPEN: WP-03 for AC-03.3 (screenshots); WP-06 for AC-06.3 (ic's remint doors replace st/issues renumber), AC-06.1's issue half (needs ST0096 WP-01) and AC-06.6 (the drill). NEXT, all on ed302b7e4, after hv's compact cycle: devbin-cc builds WP-07 on cc's brief (signature agreed with ic); ic rebases ST0096 WP-02 with 0730 and adds the remint doors (AC-10.5), two banks proved by one gyges run; dc rebases WP-14+0719 (proved at 789906b18, re-gate), then 0718+0715+0722+0729 on WP-14's land, then WP-08 and WP-09; devbin-dc builds WP-12's guard half (base fb3a501d1); cc takes ST0096 WP-01 then WP-05. Then ic's WP-10 Part 2 (after WP-07), WP-11 and WP-13, and dc's WP-12 other halves. Proofs land in the order they arrive, and a second lander rebases if its files moved. vc reads each proof with its critic review (vc decision 133). (edited)
- ST0096, issue 0705: UNDER WAY since 2026-11-19 11:38Z by date -u, on hv's go (hv decision 71; 66, 67 and 70 rule the direction, the Highlander check and the same-thread case). cc's canon split for WP-05 is in the design at dd01150ce: one canon file per record (work package, criterion, test and attachment), one unreleased canon version for the release so a project runs intent upgrade once, and the 22 closed records still committed go in the upgrade's commit. hv decision 74 (2026-11-20): acceptance.md and the cover's package table stop being committed; the design carries it (8f6beab03), and AC-02.1, AC-02.2 and AC-03.1 now name the aggregate views. WP-05's criteria AC-05.1 to AC-05.3 are recorded in cc's meaning (45253b5b6). ic's WP-02 design point is answered in cc's meaning and is in the design with AC-02.3 (e77a68439). WP-02 is PROVED (ic, 2026-11-20 09:16Z: refs/bank/ic/st0096/wp02-wip4-on-eb3de27a5, blob f336d0316831, patch-id b9e7622278e1, 22 paths; one gyges gate, no re-run; AC-02.1 to AC-02.3; issue 0707's doctor half; a latent sync bug fixed, a package renumber's thread now counted as unmerged canon), read and verified by vc. It LANDS AFTER cc's ST0095 WP-05+06 unit, on cc's call: it shares 8 files with cc's bank and plan.rs with cc's step 5, which now carries the latent-bug fix as a plan::Conflicts field. ic then rebases over cc's unit and takes cc's field in place of packages_minted_twice, keeping its test. It also folds in issue 0730 (R11's remainder: gitstate::is_tracked made the one-path reading of tracked_matching's question), runs one gyges gate for both and sends the proof. vc closes 0707 (its doctor half; cc's upgrade fold rides WP-01 or WP-05) and 0730 at the land, as the proof says. Packages: cc WP-01 (M) and WP-05 (L), both after ST0095 WP-06; WP-03 (S, after 02) goes to devbin-dc on dc's brief, dc reading its proof (vc todo 91); dc WP-04 (S, the proof with AC-04.2, after 01, 02 and 05). Issue 0704 is superseded by this thread. (edited)

## TODO

- When a build of the release that carries ST0095 exists (eg its candidate), tell agentic-engineering-vc, through the Desk, how AE's seats install it without the tap (hv decision 68). dc proposes the route, together with how the cut holds its formula step until hv's word.
- devbin-dc, lent by the Desk (hv decision 72), on dc's briefs. DONE: 0723+0724 (9e533703e), 0725 to 0728 (dddd6345b) and 0731 (fb3a501d1), each closed in its land. NOW: ST0095 WP-12's guard half (AC-12.1), on dc's brief (blob 99bbfa108), based on fb3a501d1, with 0731's two cosmetic leftovers folded in (pipefail's context label, residue's doubled NO VERDICT line). NEXT: ST0096 WP-03, once ic's ST0096 WP-02 lands, after cc's ST0095 WP-05+06 unit. dc reads each proof; vc lands it. (edited)

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
