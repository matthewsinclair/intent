---
node: cc
name: Control Claude
role: control
session_id: 89be4c37-5e0c-46eb-a8df-6cce6bb2c58b
heartbeat_at: 2026-09-23 09:03Z
status: active
focus: "LOCALFOLDED for hv's compact. On the bounce: re-run 0523 stage 1 v3 on the train stack in wt-train (doing item has the commands), then land after dc's 0524 on vc's verdict; then 0523 stage 2 (messages) and 0525. 0521 open until CI's bats read. NO PUSH, NO RELEASE."
claims: [ISSUE:0521, ISSUE:0523, ISSUE:0525]
---

# Control Claude (cc)

## DOING

- RESUME HERE (cc localfold before hv's compact, 2026-09-23; hv: "localfold+compact now, and then continue on the bounce"). Scratch dir S = /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/89be4c37-5e0c-46eb-a8df-6cce6bb2c58b/scratchpad.
0523 STAGE 1 IS BANKED AS v3, NOT YET RE-RUN. refs/bank/cc/0523-stage1/v3 is blob 36776c993ffc0cf3eb1dbacaf2c67a2f59ab853b, patch-id 5c04d79c2599aa0ca60e66c27c76b0648e3ee205, base 987268553; apply --stat read from the command: 18 files changed, 1125 insertions(+), 30 deletions(-). v1 (refs/bank/cc/0523-stage1/patch) and v2 (.../v2) are spent and KEPT. v3 closes vc's hold on v2: an item with no creator of its own could borrow a LATER item's draft wb.add with the same kind and text and rewrite ITS record. Now the creator must be the one matching event in the first instant at or after the row's stamp, across the whole log (store.events_at_first_instant); none or several means no carrier, so Recorded. The new arm a_carrier_less_item_never_borrows_a_later_items_creator holds it. Source is S/wt-0523r.
FIRST ON THE BOUNCE: S/wt-train already holds 987268553 + ic 0520 + dc 0524 + v3 staged (22 files, 1547 insertions and 36 deletions), and its warm target is in-tree. Census the machine, announce HEAVY START to vc, dc and ic, then in native/rust, with CARGO_TARGET_DIR in-tree, CARGO_HOME=/Users/matts/.cargo and HOME=S/home-train, run exactly train-run2's lines: fmt --check; build -p intentd; test --workspace --no-fail-fast; clippy --workspace --all-targets -- -D warnings; clippy -p intentsvcs -p intent-cli --lib -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic; RUSTDOCFLAGS=-Dwarnings doc --no-deps --document-private-items. Logs go to S/train-run3. Report END with every rc and the stat READ from the command. vc gives the verdict, then land v3 by literal paths AFTER dc's 0524 lands, with the patch-id checked before the commit. The prepared message is S/0523s1_commit_msg.txt: update its blob and patch-id to v3's and say vc judged it only after vc says so.
0523 STAGE 2 (MESSAGES) IS IN S/wt-0523m, ON v2, UNCOMPILED. It has the store (MessageRow, set_message_body, wb_messages_between, wb_text_events widened to ask/announce), board_stamp pub(crate), the facade (wb_edit_message, announce_copies, note_staged, message_origin, message_address, WbNoSuchMessage, WbMessageAmbiguous, WbMessageEdit), the CLI arm with --to and edit_forms, the table row (kind += message, --to, arg help), six facade arms, and error_remedies. STILL OWED: rebase onto v3; apply vc's first-instant rule to message_origin too, because the same borrow flaw exists for a carried message; mandatory_fields entries; CLI arms for --to refused on an item and required for message; CHANGELOG and skill lines for messages; then a run and ic's review. ic accepted the shape with points 1-7, and the blocking one (key a correction by its originating event) is built.
THEN 0525, filed for cc by vc on hv's "Yes, in 3.2.1, own issue": the (edited) mark, after stage 2, with ic reviewing the surface.
Main was aec0c3721 at 08:46Z (0520 landed at 89015b037); dc lands 0524 next. NO PUSH, NO RELEASE.

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
