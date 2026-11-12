---
node: vc
name: Validation Claude
role: validation
session_id: 7683276e-e120-4768-84a2-a8267004e72d
heartbeat_at: 2026-11-12 14:00Z
status: active
focus: "FOLDED for hv's absence (2026-11-12 14:00Z), on hv's word: "Please tell all of your Claudes to bank, fold, and hold." Every seat holds until vc or hv says go. P0: Intent 3.3.1. Waiting on devbin 0.1.23: k342, devbin-vc's landing, hv's rewrite and cut, the brew steps and the sweep. Then dc's slot (WP-06 proof on rhadamanth, 0682's gate run on gyges beside it, vc's landing, --leg kovacs, the dry run with intentd up), vc's freeze, ic's two reads, hv's cut. Owed by hv on return: "push" for intent.laksa.io at 3.3.0 (ic's c14c60e, rehearsed; ic hold 23), and the Login Items check."
claims: []
---

# Validation Claude (vc)

## DOING

_(none)_

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- NEXT (P0, hv decisions 39 to 47), in order. (1) devbin's last proof run, k342, of its two units still banked: 0342-1, the fix for the wait-on-a-process-substitution race, and 0344-1, which runs a cut's remote legs after its local gates (k340 put 0334-2, 0341-1, 0340-1 and 0340-2 on devbin's main at 8f35cb72). Then devbin-vc's landing, hv's history rewrite and hv's cut of devbin 0.1.23, hv's brew steps, and devbin-vc's sweep into Intent's tree; cc gives the sweep a diff check for a packaging/homebrew change (hold 36). (2) dc's one Desk slot on rhadamanth, booked from devbin-vc's Desk after the sweep, every step one after the other and none beside a kovacs run (hv decision 45): (a) the WP-06 (c) to (e) proof on refs/bank/dc/0093-06/cde-draft-unproved-v3-on-fdd9fa2df rebased onto the sweep, its arms red then green and the bats suite at 24 jobs; (b) 0682's by-hand bin/devbin gate rust with TMPDIR on a 1.7G disk image, run on gyges beside (a) in a clone carried by git bundle, since gyges is not bound by hv decision 45 and gate rust needs no GNU parallel (vc, 2026-11-12; it stays on rhadamanth after (a) if gyges lacks anything it needs); (c) dc banks, and vc lands WP-06 with 0682's Resolution and close; (d) bin/devbin release gates --leg kovacs; (e) the dry run, whose pre-flight runs no gate. intentd stays up through the slot and no seat stops it (hv decision 46), so (e) stops at the pre-flight step, which still runs every check and names every refusal: it passes when the live daemon is the only refusal, and any other is a finding. Before (e), vc calls a freeze: every seat commits its board, then nothing writes to Intent's tree until hv's cut has finished, not even a pickup, and vc confirms git status is empty. (3) At that HEAD, which vc sends to ic, ic reads that reference_current_check.sh exits 0 and that DIRT_SCOPE has not moved since 9500cb77a, the pair docs/known-defects.md was driven on at 52c251f0a; ic regenerates docs/reference only if a landing moved the register or ALL_VARIANTS. The rest of releasing.md step 2 needs no more work (vc's read, by 13:09Z on vc's date -u): every user-facing issue closed since v3.3.0 has its CHANGELOG entry, while 0687 changes Debug builds alone, 0688 a parity tool, and WP-06 and 0682 the development tooling, so none needs one, and only the sweep could add one, by cc's hold 36 read; and the explorer screenshots need no run, since nothing the explorer draws has changed since v3.3.0. (4) hv's cut of v3.3.1 (hv decision 44) by intent/docs/releasing.md: hv stops intentd at step 0, then runs the dry run, whose rows after the pre-flight print for the first time there, then the cut; the macOS stage first runs in the cut, before the tag; hv starts intentd again at the end of step 4; then releasing.md step 5, the public site's db/release.json, which ic writes, commits and rehearses with the --dry-run, and pushes on hv's "push" through vc (hv decision 47; ic first brings the site from 3.2.7 to 3.3.0, now); then vc tells agentic-engineering-vc it is installable. (5) Other projects' memory folders get ST0093 WP-01's prune when the plan rolls out to the estate. (edited)

## Holds

_(none)_

## Watch-outs

- NEVER truncate the output of `git remote -v` (hv, 2026-10-23): read and quote it whole. (edited)
- Client names in Intent's committed record: only the client's own name is confidential (hv 2026-11-10; narrowed 2026-11-11: an initiative's name is not). Names already on upstream are fixed forward only. (edited)

## Decisions

- (hv, 2026-10-23) Molt never touches Intent on rhadamanth except `intent issues add|edit`, committing only what the verb wrote; no repo commits Python. (edited)
- (2026-10-25) Every Intent judge runs `intent critic` on each changed .rs and shell file as a leg; red on warning or above; no planted positives. (edited)
- (hv, 2026-10-25) Brew by default; dev or opt only while working on that project, and switch back to brew after its release. (edited)
- (hv, 2026-10-25) devbin-vc runs Intent's pushes to local and upstream, fast-forward only, on vc's "push" after build all at HEAD; cuts stay hv's. (edited)
- (2026-10-27) Before a worktree is removed, its node points refs/attic/<node>/<worktree> at its HEAD; attic refs are never deleted. (edited)
- (hv, 2026-10-27) Every localfold and EOD fold archives what is spent, so a pickup prints only live state; ST0089 is built after the cut. (edited)
- (hv, 2026-11-10, first-hand in prose in vc's session, read by 08:48Z on vc's date -u) THE TREE HOLDS ONLY WHAT IS LIVE OR TO BE DONE. hv, verbatim: "We _should_ tidy things up. If issues and ST/WPs are resolved, then lets clean them out of the tree now. We should only have stuff that is live or to be done, old stuff can be dehydrated." So a resolved artefact's realised files leave the tree, by `intent organize --apply` once its whole preview has been read. vc's reading of the boundary: a Not Started thread is to be done and stays, listed with `intent st hydrate` before the apply; ST0056 and ST0057 are Completed and stay, because tests, the reference docs and the pre-commit guards read their files, until ST0084 relocates them; and before an apply every authored file in the removal list is checked byte for byte against tracked canon. First done on 2026-11-10: 41 closed-issue views and the trees of ST0081, ST0086 and ST0087. Closing an issue or completing a thread unlists it and leaves its files, and no step of a fold reconciles the tree, so views had gathered since 2026-10-23. This replaces the practice of 2026-10-25, when an apply's closed-issue removals were put back as an accident.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
