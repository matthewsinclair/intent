---
node: cc
name: Control Claude
role: control
session_id: 89be4c37-5e0c-46eb-a8df-6cce6bb2c58b
heartbeat_at: 2026-09-23 08:18Z
status: active
focus: "0523 wb edit: stage 1 (items) written, waiting on the train run after ic's END and dc's 0522; stage 2 (messages) shape accepted. 0521 landed 38db27487, open until CI's bats run on hv's next push. NO PUSH, NO RELEASE."
claims: [ISSUE:0521, ISSUE:0523]
---

# Control Claude (cc)

## DOING

- 0523, intent wb edit (hv: "I need that fixed right now, please"; vc ruled the name `edit` and the widened scope 2026-09-23). TWO STAGES, both 3.2.1.
STAGE 1, ITEMS, is WRITTEN in tmp wt-0523 and not yet compiled. The surface is `intent wb edit <kind> <id> <text> --node <moniker>`: one-way, off MCP, row after wb archive, legal_pairs 80 -> 81. An uncommitted originating event is AMENDED in place (same id, same stamp, same path). A committed one gets a wb.edit {kind, seq, text}. Committed means present at HEAD (gitstate::blob), never the index. A board or event file staged before the edit is named in a note. Archived items are covered. The fixed doc link was public wb_edit linking the private wb_text_carrier, which CI's doc step would red, as 0511 did.
THE TRAIN (vc): ic's 0520 run ENDs, dc lands 0522 chain 2, cc rebases, then ONE whole intentsvcs and intent-cli suite run over HEAD + 0520 + 0523 stage 1 (+ dc's 0524 if banked). Also owed: clippy, and CI's doc command read from rust.yml. Then bank with the apply --stat beside the blob. ic reviews the surface at the bank.
STAGE 2, MESSAGES, after that. The shape is accepted by ic and vc: `wb edit message <anchor> <text> --to <recipient>`, where `<anchor>#<n>` (1-based, in send order) picks one of several in a minute. The committed correction is keyed by the ORIGINATING event id (ic's blocking point). An announce is edited in every copy, and the ok line names each recipient. The no-match refusal says which stamp is the anchor. Focus stays `wb pickup --focus` plus a test. held_name/held_role are out of scope (vc). The "(edited)" mark is with vc; cc recommends a follow-up issue.
REQ 5 ANSWERED: no production path re-serialises a committed event file. The backfill skips existing paths (facade.rs:412), init writes only a new file, and add_event_files writes only events this process just landed. NO PUSH, NO RELEASE.

## TODO

- 0521 is LANDED at 38db27487 (vc PASS; landed patch-id 4650733e4 = judged; gate rc 0, currency ok). What remains: the issue stays OPEN until CI's Intent Tests workflow reads green on BOTH legs after hv's next push. That workflow runs the whole bats suite and is the judge of record. When hv pushes, read the tests.yml run on that HEAD; if it reds, make it speak first. Then close 0521 on vc's word. NO PUSH, NO RELEASE.

## Holds

_(none)_

## Watch-outs

- SEVERAL DIRTY BOARD RENDERS AT ONCE IS NOT A CHURN PROBLEM. 10 of 307 board-touching commits in the week to 2026-09-22 have a whole-board diff that is `heartbeat_at` and nothing else -- about 3%, because a render is nearly always dirtied alongside content that was going to be committed anyway. Do not spend a design change on it. AND IF YOU WANT A RULE's HOLD RATE, COUNT OCCASIONS AND CLASSIFY THEM; incidents cannot give it, because a violation becomes a message to a peer and a correct application usually leaves nothing behind. Where a violation WOULD leave a persistent artefact the occasions are countable from git -- but classifying one needs OWNERSHIP, and git carries none here: every session commits as hv, and the only subject convention that names a node is `wb(<node>)`. Measured by ic on 2026-09-22: of 42 commits that day, 30 named a node and the 12 that did not were the substantive work. SO THE COUNT IS AVAILABLE FOR BOARD COMMITS AND MISSING FROM THE COMMITS THAT MATTER, and for work the owner is in the chain announcements, which are live-channel messages that die at a compact. CUT ON 2026-09-22 FROM THREE LONGER VERSIONS, on vc's rule that a caution needing this much care to state is close to the boundary where its precision stops being usable at the moment of use. The derivation, the worked example (zero violations in at least six occasions, one-sided at both ends) and what each revision cost are in archived watch-outs 52, 54 and 55 and in this board's commit messages.
- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
