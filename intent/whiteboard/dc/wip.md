---
node: dc
name: DevX Claude
role: devx
session_id: b5900b85-5513-4ada-92b4-02956562a5e3
heartbeat_at: 2026-11-19 12:16Z
status: active
focus: "2026-11-19 12:16Z by date -u: localfolded before hv's 13:00Z lid, not released. Issue 0711 is in DOING 170: red and green done (refs/bank/dc/0711/fix-wip1); cc's 12:15Z ruling puts one component-wise one-file key in project.rs, asked by the batch guard and the bucket merge, then one gate run and the proof. Next after it: WP-14 (TODO 73), 0715 (74), the cut/gate XS (72), ST0096 WP-03, WP-08 and WP-09 (75), WP-12. Issues 0718 and 0719 are filed for vc to sequence. Handoff: refs/bank/dc/st0095/handoff-20261119j."
claims: [ST0095/03, ISSUE:0711]
---

# DevX Claude (dc)

## DOING

- ST0095 WP-03 (M), on vc's go of 09:38Z. Its two units landed by vc at 11:33Z: the gate fix at fe534431d (closes issue 0716), the store guard at 2c909d85e, and the capture function with the brew smoke at c5233a0c9. AC-03.4 and AC-03.5 replace AC-03.1 and AC-03.2 as non-test criteria and are satisfied. The landed worktrees are removed. WP-03 stays open for AC-03.3, the explorer screenshots with explorer_shots.rs:60, which are dc's after WP-06 (vc, 11:33Z). (edited)
- Issue 0711 (claimed; S, intentsvcs). The fix to cc's 11:37Z ruling is banked at refs/bank/dc/0711/fix-wip1 (0c2b4b582, base 24929441f): every path in migrate.rs's batch is keyed by its Unicode lowercase, giving Blocked::Collision { path, other }. RED: the test alone (refs/bank/dc/0711/red-test-wip1) failed alone, exit 101, 11:58Z to 12:03Z. GREEN: bin/devbin gate rust on fix-wip1, exit 0, 12:04Z to 12:10Z. critic-rust then found the fold's second home. cc's ruling, by 12:15Z: ONE pub(crate) key in project.rs, beside canon_blob_rel and attachment_name, built from Path::components() with . dropped and each component Unicode-lowercased; its doc names both limits (NFC/NFD, and lowercase not being full case folding: ſ/s, ς/σ); no crate. The batch guard and legacy.rs's bucket merge (about :741) both ask it. A case variant with a matching sha counts as already held; one whose sha differs is withheld with a finding naming the canon spelling, never silently. Tests for both, plus a/./b.png beside a/b.png, plus tests pinning each remedy arm and the order without sorting. Update MODULES.md's migrate.rs row. Then one gate run, bank fix-wip2, and the proof to vc. Follow-ups filed: issues 0718 (the model invariant: attachment door and projection) and 0719 (classify's view names), both to use the same key, sequenced by vc.

## TODO

- ST0095 WP-12 (M), after WP-03 and ic's read door (WP-10): the gating scripts ask the waist, a commit-gate guard refuses a new id pattern, the drill runs in a clone, and the fleet note. With it goes a separate proposal to retire census items 8, 9 and 11, the by-hand ST0056 instruments.
- ST0096 (issue 0705's thread): hv said go (hv decision 71, a95709410; vc, 11:41Z). dc's two packages, in vc's order after issues 0714 and 0711. WP-03 (S) comes after ic's ST0096 WP-02 (the two lists leave git): the commit gate and the pull-request check work with steel_threads.md and todo.md out of git, and read their paths from the one declared set. WP-04 (S) comes after cc's WP-01 and WP-05 and ic's WP-02: the proof through a forge stand-in, five merge requests from one main in two clones and two worktrees, merged in turn by a bare repository's git merge-tree --write-tree, with no conflict and doctor clean after each pull. Its AC-04.2 is hv decision 70's same-thread case, which cc's WP-05 closes: two merge requests from one main change one thread, one adding a work package and the other a criterion and a test, and each merges with no conflict and doctor clean after each pull. ST0095 WP-12 comes after these, once WP-10 lands. (edited)
- hv decision 68, for vc before the rehearsal, not urgent: (a) how the cut stops before the formula is published, and how that step runs later on hv's word; (b) how agentic-engineering's seats, and our own estates for the fleet sweep, install the release build without the tap (vc todo 84).
- ST0095 WP-12 (from vc, 10:31Z, and ic): add to the fleet note (AC-12.3) that issue 0712 is fixed in cc's WP-04, so a package whose criteria use a one-digit group (eg AC-1.1) now counts them and wp done can block where it did not (Report A: 182 such rows in Devbin, Lamplight and Prolix). Route the five gated joins through ic's read door, `intent id <token>...` (one line per token: id token= kind= id= shown= canon= path=, or ambiguous, unknown, not-an-id; --json too), which lands in WP-10. The retirement proposal goes to vc file by file.
- XS (vc, 11:52Z, under hv decisions 67 and 69, in this release): bin/.devbin/cut/gate's reading of a guard's exit status joins precommit's tool_verdict through one reporting primitive both gates share; today cut/gate has its own copy with the same meaning (gave no verdict (rc=N)) reported through its own refuse(). File it as an issue when starting. After issue 0711, before ST0096 WP-03.
- ST0095 WP-14 (vc, 11:59Z, from cc; next after 0711; claim when starting): kind tags and a path's owner, each in one home. AC-14.1: one enum for the id families, with one wire spelling per surface (issue 0708), extending id.rs's IdKind. AC-14.2: one owner-of-a-path function for plan.rs, sync.rs, project.rs's classify and search, with no st_prefix key. One exchange of meaning with cc for each half. It lands before or after cc's WP-05+06 unit, never inside it; land it first if possible, since cc adds Form's hashed variant to id.rs in that unit.
- Issue 0715 (vc, 11:59Z, from cc; after WP-14; claim when starting). cc's meaning, in substance: st attach --from over an existing attachment replaces it in the store, in the canon and, when the thread is realised, in the realised file, all in the one verb, and prints ok only when all three stood. The realised file is the authoring surface, so it is written FIRST: a watching daemon that ingests it in between takes in the new bytes, and the store write that follows agrees. If the realised file already holds bytes the store does not (an unsynced hand edit), the verb refuses before writing anything, naming intent sync --to-store <ID> and editing the file directly. If the store write fails after the file was written, the file's old bytes go back, so a refusal never leaves the file ahead of the store. The code is facade.rs's put_attachment and place_attachment; the CLI wording is ic's.
- ST0095 WP-08 (the order) and WP-09 (criteria follow their package; doctor verifies), each S (vc, 11:59Z, from cc): once WP-06 lands, each with one exchange of meaning with cc; claim each when starting. vc's order: 0711, WP-14, 0715, the cut/gate XS, ST0096 WP-03 once ic's WP-02 lands, WP-08 and WP-09 after WP-06, then WP-12 after WP-10.

## Holds

_(none)_

## Watch-outs

- For WP-03's brew smoke (vc, 10:03Z, cc's meaning, going to hv): in a project not yet upgraded, the new build works on its ordinals and refuses only the first mint, with intent upgrade as its one remedy.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
