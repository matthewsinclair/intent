---
node: vc
name: Validation Claude
role: validation
session_id: 7683276e-e120-4768-84a2-a8267004e72d
heartbeat_at: 2026-11-12 07:38Z
status: active
focus: "P0 (hv decisions 39 to 41): devbin 0.1.23, then Intent's next cut. Waiting on devbin's pre-cut and hv's cut of 0.1.23; then devbin-vc's sweep, dc's WP-06 (c)-(e) landed by vc, one Desk booking (--leg kovacs, then the dry run), ic's reference pages, hv's cut (3.3.1 recommended). hv's look at Intent.app closes ST0090."
claims: []
---

# Validation Claude (vc)

## DOING

_(none)_

## TODO

- After 3.2.7: make the critic contract readable to installed critic subagents (deferred, decision 89). (edited)
- NEXT (P0, hv decisions 39 to 41): (1) Once hv has cut devbin 0.1.23 and devbin-vc has swept it into Intent's tree: dc builds ST0093 WP-06 (c) to (e) with one proof run and vc lands it; one Desk booking, dc's: `bin/devbin release gates --leg kovacs`, then a dry run; ic regenerates docs/reference; then hv's cut (vc recommends 3.3.1), and vc tells agentic-engineering-vc when it is installable. The macOS stage first runs in the cut, before the tag. (2) ST0092 WP-11: AC-11.4 waits on arca_supervisor's sc committing its tree, AC-11.6 on molt-vc (Molt's guide cites Intent's terms), AC-11.7 on agentic-engineering's own vc. (3) Other projects' memory folders get ST0093 WP-01's prune when the plan rolls out to the estate. (edited)

## Holds

- ST0090 AC-02.4, issue 0664's last criterion: held until hv has rebuilt the search index in /Applications/Intent.app (3.3.0) and read the second line of its Rebuild Search Index row, reconciled <age> ago. Then vc satisfies AC-02.4 with hv's reading, and dc closes WP-02, ST0090 and issue 0664. (edited)

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
- (hv, 2026-11-10, first-hand by selection in the TUI hv asked for in vc's session, the first four answered between 08:07Z and 08:11Z and the last three by 08:13Z on vc's date -u; wording vc's, each choice hv's and vc's recommendation) SEVEN RULINGS FOR ST0092, Claude Team profiles. (1) A Claude Team is the standing roster a profile registers on a project's board, every member registered whether started or not; a Claude Swarm is whichever of a Team's sessions are running at one time. (2) The Supervisor has its own profile, shipped in ST0092; hv in prose the same hour: "SC is only needed on larger projects where we need multi-project coordination that goes across multiple VCs." (3) The single Claude is gc, Generalist Claude, role generalist. (4) Sleeve agents and estate agents stay out of the profile mechanism and are named in the terms, with Molt and Gtools as their owners. (5) hv's mediation through the VC is enforced by the tool. (6) The roster README is generated, with a preserved block. (7) The sweep writes directly where a project has no running session and through its vc or sc where one is running: all 32 live boards, Intent's own first. AND IN PROSE, the same morning: the default Team is VC, CC, IC, AC and DC, all registered whether started or not; hv's words on the Desk, an extra "hat" that one VC wears, and "we need to institutionalise the way that role works in addition to the other VC roles" (WP-08); and on Pipeliner's sc, "I think that was a mistake and probably needs to be revoked", confirmed with hv before any write there (AC-11.5). hv's go for the thread, verbatim: "You are free to make a new ST and elaborate the WPs for that ST now to make this right." ST0092's context holds each quotation whole.
- (hv, 2026-11-10, first-hand in prose in vc's session, read by 08:48Z on vc's date -u) THE TREE HOLDS ONLY WHAT IS LIVE OR TO BE DONE. hv, verbatim: "We _should_ tidy things up. If issues and ST/WPs are resolved, then lets clean them out of the tree now. We should only have stuff that is live or to be done, old stuff can be dehydrated." So a resolved artefact's realised files leave the tree, by `intent organize --apply` once its whole preview has been read. vc's reading of the boundary: a Not Started thread is to be done and stays, listed with `intent st hydrate` before the apply; ST0056 and ST0057 are Completed and stay, because tests, the reference docs and the pre-commit guards read their files, until ST0084 relocates them; and before an apply every authored file in the removal list is checked byte for byte against tracked canon. First done on 2026-11-10: 41 closed-issue views and the trees of ST0081, ST0086 and ST0087. Closing an issue or completing a thread unlists it and leaves its files, and no step of a fold reconciles the tree, so views had gathered since 2026-10-23. This replaces the practice of 2026-10-25, when an apply's closed-issue removals were put back as an accident.
- (vc, 2026-11-10; relayed by devbin-vc from its own window, NOT heard first-hand by vc) hv answered two questions in devbin-vc's menu at about 19:55Z. (1) The eight old event files that carry a client's name: "Leave them (Recommended)". They stay as they are; the scrub of hv decision 35 is done once cc edits its two archived board items. (2) Whether the estate name the rewrite's new message keeps is a client's name: "Fine, keep it". The banked message at refs/bank/vc/rewrite/new.msg stands. devbin-vc read the rewrite script and reports it touches only c1d7d163d's message, guards on main, a clean tree and ancestry, keeps an attic ref, and checks the tip tree unchanged. ALSO, vc's own under the pen: ic read WP-03, 04, 05, 06, 07 and 09 against the dispatch table at d6f56d7e6 and vc ruled each read as ic recommended; the rulings are in each package's body under "Rulings on the second row read". Among them: INV-12 (only the mediator's session or a sessionless shell writes as the human seat on a profiled board) rides cc's WP-05 bank; out-of-date README is its own class, roster-readme-behind, and not StaleRender; finding-class ids live in FindingClass::as_str alone; and the launcher half of WP-06 is ic's, built once cc banks wb brief.
- ST0092 WP-11, THE ESTATE SWEEP, on hv's go (2026-11-11, rulings: the five stale hv messages cleared; Prodinfra and Riffle solo; Pipeliner converges with sc as it stands and sc retires as a separate step; gyges and kovacs upgraded to 3.3.0 by hv). Converged and committed, one commit per project of what the verbs printed: Intent 69879b5eb, and 29 other boards under ~/Devel/prj, ~/Devel/writing and the client tree; Devbin by devbin-vc (e10e51ee), Webber by webber-vc (10dd2048). Laksa needed its own inbox guard taught that the profile apply's declaration sanctions removing the views of pairs with hv (hv chose that over a --no-verify commit). NOT DONE: Arca's arca_supervisor: the profile is applied in its store, but its tree already held 83 uncommitted paths of its own sessions' work, so nothing was committed; its sc commits the lot (hv: ok). Four Intent projects in the client tree have no board and get none (hv: 'Maybe not'). Pipeliner's sc retirement is still to do. Two Intent defects filed for the next release.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
