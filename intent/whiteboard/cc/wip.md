---
node: cc
name: Control Claude
role: control
session_id: 32974d4a-0175-4bfb-b198-cdac20b4d58b
heartbeat_at: 2026-09-24 17:48Z
status: active
focus: "LOCALFOLDED for hv's /compact, at vc's order. Resume state is doing 74: every cc landing is in at its judged patch-id, the 0551 draft and three fold snapshots are banked, the fix set waits on hv, and the root files follow vc's build all. NO PUSH, NO RELEASE."
claims: []
---

# Control Claude (cc)

## DOING

- RESUME STATE (cc, localfold on 2026-09-24 for hv's /compact). (edited)

  LANDED, each at its judged patch-id:
  - 0548: code 73bc232df (3d8ebca06), CHANGELOG line 556e34374, issue closed 19dd2b71b.
  - The team guide, intent/docs and intent/llm: 7dc62eb34.
  - .claude/agents/elixir.md and the root .intent_critic.yml: adf7d1096.
  - Templates v3: 450bcc0f8 (8f064805a).
  - Rules audit: e2b6b33f7 (cb89c12f2).
  - Board: 8512644c0.
  The final-tree run on d66b3c1c2 was green on run 2: every CI line rc 0, cargo 3077/0/5, bats 747/747. vc ACCEPTED the lane report.

  BANKED, NOT LANDED:
  - The 0551 draft, unbuilt and unjudged: refs/bank/cc/0551/draft-unbuilt-on-cc34b16e4 (blob c253f5a1f, patch-id 076114ec7). It applies to HEAD.
  - Fold snapshots of three older worktrees whose diffs are only partly on main: refs/bank/cc/fold-20260924/wt-0523b-on-e1d784dbc, wt-0523m-on-987268553 and wt-train-on-ca936e6c4.
  - Templates v1 and v2 are superseded and never land.

  WAITS ON hv: the fix set.
  - cc's 0551, 0564, 0570 (size S; the design is in 0570's own body) and batch 1 (0554(b), 0556, (b3)).
  - ic's five fix banks and dc's CI bank.
  - If ruled in: build each red-first on HEAD in its own worktree, driving 0570's NoResolvableInstall remedy first. Re-cut and re-drive the team page's two sections for 0551 and batch 1. Then one final-tree heavy run on top, and land on vc's word.

  NEXT AFTER vc's BUILD ALL, in the main tree:
  - A dry `intent claude upgrade`, which should name only AGENTS.md and CLAUDE.md.
  - Then --apply, and commit those two by path.
  - Then a second dry run, which should write nothing. The release pre-flight refuses anything outside the sidecar list.

  AFTER THE CUT: todo 39. NO PUSH, NO RELEASE.

## TODO

- After the cut: remove cc's leftover worktrees -- tmp/wt-cc-{0548,0551,final,rules,tpl}, the older scratchpad worktrees of session 89be4c37 (wt-0521, wt-0523, wt-0523b, wt-0523m, wt-0523r, wt-0525, wt-critic, wt-f1f2, wt-train) and ../Intent-wt-0546 -- each checked against a landed or banked patch-id first (every one was, at the 2026-09-24 fold), then `git worktree prune` for the three registrations whose directories are already gone. Never delete a bank ref. (edited)

## Holds

- 0551 (tracked store), batch 1 (0554(b), 0556 and (b3)), 0564 (malformed .intent_critic.yml seed), 0570 (gate-not-running wording) and the critic.rs comment fixes, together with the team page's two sections that 0551 and batch 1 change -- HELD UNTIL hv rules them into 3.2.1. If ruled in, they stack with ic's five fix banks and dc's CI bank for one more final-tree run on top of HEAD.

## Watch-outs

- SEVERAL DIRTY BOARD RENDERS AT ONCE IS NOT A CHURN PROBLEM. 10 of 307 board-touching commits in the week to 2026-09-22 have a whole-board diff that is `heartbeat_at` and nothing else -- about 3%, because a render is nearly always dirtied alongside content that was going to be committed anyway. Do not spend a design change on it. AND IF YOU WANT A RULE's HOLD RATE, COUNT OCCASIONS AND CLASSIFY THEM; incidents cannot give it, because a violation becomes a message to a peer and a correct application usually leaves nothing behind. Where a violation WOULD leave a persistent artefact the occasions are countable from git -- but classifying one needs OWNERSHIP, and git carries none here: every session commits as hv, and the only subject convention that names a node is `wb(<node>)`. Measured by ic on 2026-09-22: of 42 commits that day, 30 named a node and the 12 that did not were the substantive work. SO THE COUNT IS AVAILABLE FOR BOARD COMMITS AND MISSING FROM THE COMMITS THAT MATTER, and for work the owner is in the chain announcements, which are live-channel messages that die at a compact. CUT ON 2026-09-22 FROM THREE LONGER VERSIONS, on vc's rule that a caution needing this much care to state is close to the boundary where its precision stops being usable at the moment of use. The derivation, the worked example (zero violations in at least six occasions, one-sided at both ends) and what each revision cost are in archived watch-outs 52, 54 and 55 and in this board's commit messages.
- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
