---
node: cc
name: Control Claude
role: control
session_id: 32974d4a-0175-4bfb-b198-cdac20b4d58b
heartbeat_at: 2026-09-24 19:17Z
status: active
focus: "LOCALFOLDED for hv's compact, at vc's order. Resume state is doing 74: member 8b is banked and accepted, the 0551 draft is banked unbuilt, the fix set waits on hv's ruling in words, and the root files wait on vc's build all. NO PUSH, NO RELEASE."
claims: []
---

# Control Claude (cc)

## DOING

- RESUME STATE (cc, localfold for hv's second compact of 2026-09-24). (edited)

  LANDED, each at its judged patch-id:
  - 0548: code 73bc232df (3d8ebca06), CHANGELOG line 556e34374, issue closed 19dd2b71b.
  - The team guide, intent/docs and intent/llm: 7dc62eb34.
  - .claude/agents/elixir.md and the root .intent_critic.yml: adf7d1096.
  - Templates v3: 450bcc0f8 (8f064805a).
  - Rules audit: e2b6b33f7 (cb89c12f2).
  The final-tree run on d66b3c1c2 was green: cargo 3077/0/5, bats 747/747. vc ACCEPTED the lane report.

  BANKED:
  - Member 8b, the principle-rule mapping, JUDGED AND ACCEPTED by vc; it lands on vc's word with the stack: refs/bank/cc/0584/member-8b-on-00fd19045, blob 1866684ee5a408cfd88f45833de5bda12c436b1b, patch-id d2a44385a32bf31a9a5773ffcc10d448081c376e, 19 files. `references:` is the home of record, `concretised_by:` its exact inverse, and `principles:` names the principle of each agnostic rule a rule references; issue 0584 is the check gap. vc has its CHANGELOG line (Changed) and the lane-report miss.
  - The 0551 draft, UNBUILT AND UNJUDGED: refs/bank/cc/0551/draft-unbuilt-on-cc34b16e4, blob c253f5a1f, patch-id 076114ec7. It applies at c48f8e872.
  - Fold snapshots: refs/bank/cc/fold-20260924/wt-0523b-on-e1d784dbc, wt-0523m-on-987268553 and wt-train-on-ca936e6c4.

  WAITS ON hv: the fix set (hold 28). hv has still not ruled in words. vc's split if it is ruled in: cc takes 0551, batch 1 and the team page; ic takes 0570 with cc's code read (`intent bootstrap` is not the no-install remedy) and runs the stacked final-tree run; dc takes 0564; vc judges and writes every CHANGELOG line.

  cc's NEXT UNITS IF hv RULES IN, in order:
  1. The scopes into the issue bodies FIRST. Batch 1 into 0554, 0556 and 0559: (b1) 0554(b), a board view pulled ahead of the store names `sync --to-store`; (b2) 0556, a cover edit the store can carry names `--to-store` and keeps the edit; (b3) sync --apply's views step leaves a view it could carry and reports it as left. 0551's into 0551: the tracked-store finding and the upgrade both use gitstate::is_tracked's Result, and doctor says when it could not ask git; doctor.rs's bool is_tracked and its callers at :2191 and :2206 stay and are named in the bank report.
  2. 0551, red-first on HEAD in a fresh worktree, from the draft.
  3. Batch 1, red-first, one predicate for "carriable" serving doctor's skew() and sync's views step.
  4. The team page's two sections (working-in-a-team.md:378 and :399-426), re-cut and re-driven.
  Each bank goes to vc with blob, patch-id, logs and a one-line Fixed entry.

  NEXT AFTER vc's BUILD ALL, in the main tree: a dry `intent claude upgrade`, which should name only AGENTS.md and CLAUDE.md; then `--apply`, committed by path; then a second dry run that writes nothing.

  WORKTREES KEPT: tmp/wt-cc-mapping, until member 8b lands. Every other cc worktree is todo 39's, for after the cut.

  NO PUSH, NO RELEASE.

## TODO

- After the cut: remove cc's leftover worktrees -- tmp/wt-cc-{0548,0551,final,rules,tpl}, tmp/wt-cc-mapping once member 8b has landed, the older scratchpad worktrees of session 89be4c37 (wt-0521, wt-0523, wt-0523b, wt-0523m, wt-0523r, wt-0525, wt-critic, wt-f1f2, wt-train) and ../Intent-wt-0546 -- each checked against a landed or banked patch-id first (every one was, at the 2026-09-24 fold), then `git worktree prune` for the three registrations whose directories are already gone. Never delete a bank ref. (edited)

## Holds

- The 3.2.1 fix set, HELD UNTIL hv rules it into 3.2.1. cc's part if ruled in: 0551 (tracked store) and batch 1 ((b1) 0554(b), (b2) 0556, (b3) sync --apply's views step), with the team page's two sections they change, and the critic.rs:343-344 and :703 comment fixes. Under vc's split 0570 goes to ic and 0564 to dc. It stacks with ic's five fix banks, dc's CI bank and member 8b, and ic runs one final-tree run on top that vc judges. (edited)

## Watch-outs

- SEVERAL DIRTY BOARD RENDERS AT ONCE IS NOT A CHURN PROBLEM. 10 of 307 board-touching commits in the week to 2026-09-22 have a whole-board diff that is `heartbeat_at` and nothing else -- about 3%, because a render is nearly always dirtied alongside content that was going to be committed anyway. Do not spend a design change on it. AND IF YOU WANT A RULE's HOLD RATE, COUNT OCCASIONS AND CLASSIFY THEM; incidents cannot give it, because a violation becomes a message to a peer and a correct application usually leaves nothing behind. Where a violation WOULD leave a persistent artefact the occasions are countable from git -- but classifying one needs OWNERSHIP, and git carries none here: every session commits as hv, and the only subject convention that names a node is `wb(<node>)`. Measured by ic on 2026-09-22: of 42 commits that day, 30 named a node and the 12 that did not were the substantive work. SO THE COUNT IS AVAILABLE FOR BOARD COMMITS AND MISSING FROM THE COMMITS THAT MATTER, and for work the owner is in the chain announcements, which are live-channel messages that die at a compact. CUT ON 2026-09-22 FROM THREE LONGER VERSIONS, on vc's rule that a caution needing this much care to state is close to the boundary where its precision stops being usable at the moment of use. The derivation, the worked example (zero violations in at least six occasions, one-sided at both ends) and what each revision cost are in archived watch-outs 52, 54 and 55 and in this board's commit messages.
- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
