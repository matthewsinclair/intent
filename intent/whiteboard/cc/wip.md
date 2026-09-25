---
node: cc
name: Control Claude
role: control
session_id: 32974d4a-0175-4bfb-b198-cdac20b4d58b
heartbeat_at: 2026-09-25 11:38Z
status: active
focus: "LOCALFOLDED for hv's compact. 3.2.2 lane complete: 4 landed; 0554a, 0555 and tostore banked for vc's whole-suite judge on 67f222f76. Idle unless vc sends work. Resume is doing 74. NO PUSH, NO RELEASE."
claims: []
---

# Control Claude (cc)

## DOING

- RESUME STATE (cc, localfold for hv's compact, 2026-09-25). 3.2.2 LANE COMPLETE (vc decision 61: every open issue ships in one patch, 3.2.2). (edited)

  LANDED, each at its judged patch-id, on vc's word:
  - 0568, a carry no longer folds the body into Context: 4e4d4cf47.
  - 0559, a write refuses over a carriable cover edit and names `intent sync --to-store <ID>`; hydrate records what it wrote: ae03538fc.
  - doctor (i)(ii), the stale-render verb per view and one line per directory in -v: 231c0bf44.
  - 0580, `at edit --kind` says when it resets a status; help updated: abf066599.
  CHANGELOG lines by vc at 394941865.

  BANKED on 67f222f76, stacked in this order, awaiting vc's one whole-suite judge (running on 67f222f76 at the fold):
  - refs/bank/cc/322/0554a, blob a42837bec9df3383e8447de0577e90713e0160b3, patch-id a47f51c37aec19e867bce87a4908e5bfaf92e08f: a pull takes a teammate's board when its file moved FORWARD and the store is still; both-moved and an older checkout are kept and named (vc ruling (b)).
  - refs/bank/cc/322/0555, blob 6cfbbbe769f635344d61f0550f395e8aa8cf74b5, patch-id 03eb791c23ed092bc8729d6df4000abc80f32ae2: a work package both clones minted is renumbered; its claims follow; any other change still takes a side.
  - refs/bank/cc/322/tostore, blob 4407e6d14004fece8fa594f0f6c7c9a07eb59f88, patch-id f7711e2e523aacc646e44e166859e355a79a967e: `sync --to-store` names a carried cover edit instead of "overwrites nothing".
  Worktree: tmp/wt-cc-322, detached at 53ce49fe7 (the three as local wip commits), with an in-tree target. Drivers are in the session scratchpad, 322/drive-*.sh.

  IDLE unless vc sends work. NO PUSH, NO RELEASE. After the cut: todo 39's worktree cleanup, which now includes tmp/wt-cc-322. Never delete a bank ref.

## TODO

- After the cut: remove cc's leftover worktrees -- tmp/wt-cc-{0548,0551,final,rules,tpl}, tmp/wt-cc-mapping once member 8b has landed, the older scratchpad worktrees of session 89be4c37 (wt-0521, wt-0523, wt-0523b, wt-0523m, wt-0523r, wt-0525, wt-critic, wt-f1f2, wt-train) and ../Intent-wt-0546 -- each checked against a landed or banked patch-id first (every one was, at the 2026-09-24 fold), then `git worktree prune` for the three registrations whose directories are already gone. Never delete a bank ref. (edited)

## Holds

_(none)_

## Watch-outs

- SEVERAL DIRTY BOARD RENDERS AT ONCE IS NOT A CHURN PROBLEM. 10 of 307 board-touching commits in the week to 2026-09-22 have a whole-board diff that is `heartbeat_at` and nothing else -- about 3%, because a render is nearly always dirtied alongside content that was going to be committed anyway. Do not spend a design change on it. AND IF YOU WANT A RULE's HOLD RATE, COUNT OCCASIONS AND CLASSIFY THEM; incidents cannot give it, because a violation becomes a message to a peer and a correct application usually leaves nothing behind. Where a violation WOULD leave a persistent artefact the occasions are countable from git -- but classifying one needs OWNERSHIP, and git carries none here: every session commits as hv, and the only subject convention that names a node is `wb(<node>)`. Measured by ic on 2026-09-22: of 42 commits that day, 30 named a node and the 12 that did not were the substantive work. SO THE COUNT IS AVAILABLE FOR BOARD COMMITS AND MISSING FROM THE COMMITS THAT MATTER, and for work the owner is in the chain announcements, which are live-channel messages that die at a compact. CUT ON 2026-09-22 FROM THREE LONGER VERSIONS, on vc's rule that a caution needing this much care to state is close to the boundary where its precision stops being usable at the moment of use. The derivation, the worked example (zero violations in at least six occasions, one-sided at both ends) and what each revision cost are in archived watch-outs 52, 54 and 55 and in this board's commit messages.
- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
