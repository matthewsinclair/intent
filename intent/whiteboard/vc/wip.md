---
node: vc
name: Validation Claude
role: validation
session_id: 7683276e-e120-4768-84a2-a8267004e72d
heartbeat_at: 2026-11-12 09:54Z
status: active
focus: "P0 (hv decisions 39 to 42): devbin 0.1.23 is not cut; three cuts went red before any tag and nothing was published, and the next carries three P0 fixes (devbin-vc, 09:35Z). Step 2 is in ahead of the kovacs leg (docs/reference def0a8e01, CHANGELOG bd525bae2). Then devbin-vc's sweep; dc's one proof run (WP-06 (c)-(e), 0682, 0687, 0688), landed by vc; ic's known-defects re-drive; dc's one Desk booking (--leg kovacs, then the dry run); hv's Intent cut (3.3.1 recommended). ST0090 closes on hv's look at /Applications/Intent.app once the Debug build is quit."
claims: []
---

# Validation Claude (vc)

## DOING

_(none)_

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- NEXT (P0, hv decisions 39 to 43), in order. (1) Done ahead of the Desk booking, because gate all's reference arm runs on the kovacs leg too (ic's finding, 2026-11-12): ic's docs/reference at def0a8e01 and cc's CHANGELOG pass at bd525bae2; the CHANGELOG heading takes the number hv sets. (2) Now, on gyges, off rhadamanth's Desk (hv decision 43), in a clone dc makes from rhadamanth's main by bundle: dc proves issue 0687 (refs/bank/dc/0687/fix-on-7f9064cc3, red and green as one unit) and issue 0688, no_daemon_required.sh's unbounded inside-the-project guard (refs/bank/dc/0688/fix-unproved-on-923c4c44f), and vc lands them; ic re-drives docs/known-defects.md whole on the pair built there and commits it, makes hold 20's explorer call (any shots on rhadamanth), and before the cut reads that DIRT_SCOPE has not moved since its drive. (3) hv's cut of devbin 0.1.23, then devbin-vc's sweep into Intent's tree. (4) dc rebases WP-06 (c) to (e) (refs/bank/dc/0093-06/cde-draft-unproved-v2-on-fdd9fa2df) onto the sweep and proves it with 0682's by-hand gate run, and vc lands them; ic regenerates docs/reference again only if a landing moved the register or ALL_VARIANTS. (5) dc's one Desk booking: `bin/devbin release gates --leg kovacs`, then the dry run. (6) hv's cut at the number hv sets (vc recommends 3.3.1); the macOS stage first runs in the cut, before the tag; then vc tells agentic-engineering-vc it is installable. (7) Other projects' memory folders get ST0093 WP-01's prune when the plan rolls out to the estate. (edited)

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
