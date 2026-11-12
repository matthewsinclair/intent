---
node: vc
name: Validation Claude
role: validation
session_id: 7683276e-e120-4768-84a2-a8267004e72d
heartbeat_at: 2026-11-12 13:00Z
status: active
focus: "BACK from hv's third compact (2026-11-12 12:59Z). P0: Intent 3.3.1 (hv decision 44). cc, dc and ic hold with nothing unbanked. Waiting on devbin 0.1.23: its last proof run k342 (0342-1 and 0344-1), devbin-vc's landing, hv's rewrite and cut, the brew steps and devbin-vc's sweep. Then dc's one Desk slot on rhadamanth (WP-06 proof, 0682's run, vc's landing, --leg kovacs, the dry run; hv decision 45), with intentd stopped, vc's freeze and ic's two reads before the dry run, and hv's cut. Owed by hv: who stops intentd before the dry run, dc or hv, and the Login Items check."
claims: []
---

# Validation Claude (vc)

## DOING

_(none)_

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- NEXT (P0, hv decisions 39 to 45), in order. (1) devbin's last proof run, k342, of its two units still banked: 0342-1, the fix for the wait-on-a-process-substitution race, and 0344-1, which runs a cut's remote legs after its local gates (k340 put 0334-2, 0341-1, 0340-1 and 0340-2 on devbin's main at 8f35cb72). Then devbin-vc's landing, hv's history rewrite and hv's cut of devbin 0.1.23, hv's brew steps, and devbin-vc's sweep into Intent's tree; cc gives the sweep a diff check for a packaging/homebrew change (hold 36). (2) dc's one Desk slot on rhadamanth, booked from devbin-vc's Desk after the sweep, every step one after the other and none beside a kovacs run (hv decision 45): (a) the WP-06 (c) to (e) proof on refs/bank/dc/0093-06/cde-draft-unproved-v3-on-fdd9fa2df rebased onto the sweep, its arms red then green and the bats suite at 24 jobs; (b) 0682's by-hand bin/devbin gate rust with TMPDIR on a 1.7G disk image; (c) dc banks, and vc lands WP-06 with 0682's Resolution and close; (d) bin/devbin release gates --leg kovacs; (e) the dry run, whose pre-flight runs no gate. Before (e): intentd on rhadamanth is stopped, and hv's answer is owed on who stops it, dc just before (e) or hv. It has to be down for (e) as for the cut: the pre-flight refuses a live intentd, a dry run stops at that step as the cut does and prints no step after it, and the cut's last step, brew smoke --at-tag, reinstalls brew's intent, the binary today's daemon runs (read by vc on 2026-11-12; sync --to-disk itself has run beside a daemon since issue 0500). Also before (e), vc calls a freeze: every seat commits its board, then nothing writes to Intent's tree until hv's cut has finished, not even a pickup, and vc confirms git status is empty. (3) At that HEAD, which vc sends to ic, ic reads that reference_current_check.sh exits 0 and that DIRT_SCOPE has not moved since 9500cb77a, the pair docs/known-defects.md was driven on at 52c251f0a; ic regenerates docs/reference only if a landing moved the register or ALL_VARIANTS. (4) hv's cut of v3.3.1 (hv decision 44); the macOS stage first runs in the cut, before the tag; hv starts intentd again at the end of releasing.md step 4; then vc tells agentic-engineering-vc it is installable. (5) Other projects' memory folders get ST0093 WP-01's prune when the plan rolls out to the estate. (edited)

## Holds

_(none)_

## Watch-outs

- NEVER truncate the output of `git remote -v` (hv, 2026-10-23): read and quote it whole. (edited)
- Client names in Intent's committed record: only the client's own name is confidential (hv 2026-11-10; narrowed 2026-11-11: an initiative's name is not). Names already on upstream are fixed forward only. (edited)

## Decisions

- (hv, 2026-10-23) Molt never touches Intent on rhadamanth except `intent issues add|edit`, committing only what the verb wrote; no repo commits Python. (edited)
- (hv confirmed, 2026-10-25) Intent adopts devbin 0227's shared formula renderer after 3.2.7, once it takes the assets shape. (edited)
- (2026-10-25) Every Intent judge runs `intent critic` on each changed .rs and shell file as a leg; red on warning or above; no planted positives. (edited)
- (hv, 2026-10-25) Brew by default; dev or opt only while working on that project, and switch back to brew after its release. (edited)
- (hv, 2026-10-25) devbin-vc runs Intent's pushes to local and upstream, fast-forward only, on vc's "push" after build all at HEAD; cuts stay hv's. (edited)
- (2026-10-27) Before a worktree is removed, its node points refs/attic/<node>/<worktree> at its HEAD; attic refs are never deleted. (edited)
- (hv, 2026-10-27) Every localfold and EOD fold archives what is spent, so a pickup prints only live state; ST0089 is built after the cut. (edited)
- (hv, 2026-11-10, first-hand in prose in vc's session, read by 08:48Z on vc's date -u) THE TREE HOLDS ONLY WHAT IS LIVE OR TO BE DONE. hv, verbatim: "We _should_ tidy things up. If issues and ST/WPs are resolved, then lets clean them out of the tree now. We should only have stuff that is live or to be done, old stuff can be dehydrated." So a resolved artefact's realised files leave the tree, by `intent organize --apply` once its whole preview has been read. vc's reading of the boundary: a Not Started thread is to be done and stays, listed with `intent st hydrate` before the apply; ST0056 and ST0057 are Completed and stay, because tests, the reference docs and the pre-commit guards read their files, until ST0084 relocates them; and before an apply every authored file in the removal list is checked byte for byte against tracked canon. First done on 2026-11-10: 41 closed-issue views and the trees of ST0081, ST0086 and ST0087. Closing an issue or completing a thread unlists it and leaves its files, and no step of a fold reconciles the tree, so views had gathered since 2026-10-23. This replaces the practice of 2026-10-25, when an apply's closed-issue removals were put back as an accident.
- (vc, 2026-11-10; relayed by devbin-vc from its own window, NOT heard first-hand by vc) hv answered two questions in devbin-vc's menu at about 19:55Z. (1) The eight old event files that carry a client's name: "Leave them (Recommended)". They stay as they are; the scrub of hv decision 35 is done once cc edits its two archived board items. (2) Whether the estate name the rewrite's new message keeps is a client's name: "Fine, keep it". The banked message at refs/bank/vc/rewrite/new.msg stands. devbin-vc read the rewrite script and reports it touches only c1d7d163d's message, guards on main, a clean tree and ancestry, keeps an attic ref, and checks the tip tree unchanged. ALSO, vc's own under the pen: ic read WP-03, 04, 05, 06, 07 and 09 against the dispatch table at d6f56d7e6 and vc ruled each read as ic recommended; the rulings are in each package's body under "Rulings on the second row read". Among them: INV-12 (only the mediator's session or a sessionless shell writes as the human seat on a profiled board) rides cc's WP-05 bank; out-of-date README is its own class, roster-readme-behind, and not StaleRender; finding-class ids live in FindingClass::as_str alone; and the launcher half of WP-06 is ic's, built once cc banks wb brief.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
