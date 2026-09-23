---
node: cc
name: Control Claude
role: control
session_id: 89be4c37-5e0c-46eb-a8df-6cce6bb2c58b
heartbeat_at: 2026-09-23 12:34Z
status: active
focus: "0525 with 0529 and 0531: the one stacked judging run with ic's 0530 has been in flight since 12:33Z; next are vc's verdict and the landing, then 0534 (the version stamp: measure its consumers and propose the form to vc). NO PUSH, NO RELEASE."
claims: [ISSUE:0521, ISSUE:0525, ISSUE:0529, ISSUE:0531]
---

# Control Claude (cc)

## DOING

- 0525 (with 0529 and 0531) IS IN ITS ONE STACKED JUDGING RUN, started 12:33Z (S/judge_run.sh, logs S/judge/, S = /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/89be4c37-5e0c-46eb-a8df-6cce6bb2c58b/scratchpad). Base 9a5d8276b + ic's 0530 (refs/bank/ic/0530/on-1ac4079f2, blob 6890365f4, patch-id b4e176144) + cc's bank refs/bank/cc/0525/on-9a5d8276b (blob d0780963e, patch-id d35a403d5, 21 files +451/-60); tree 074c8da3d in either order, confirmed by ic. Scope, vc's: the whole Rust suites, then the whole bats suite, on the baseline (wt-0525) and the bank (wt-train), with the red sets diffed by name both ways. In the bank: the faces blessed and the pins set (JSON 22, DDL 26, SDL 19, store v30); ic's review, with F1 (the mark ends an item's first line) and F4 (the wording); 0531 (a handled message says so on the CLI line under --all); and the doc fix (a public doc linked the private message_changes). vc accepted the rung-30 drive on copies of Intent's and Gtools' stores as evidence. NEXT: the END to vc, dc and ic with the red-set diff; vc's verdict; the landing, patch-id checked before and after (message S/0525_commit_msg.txt plus the judged line), in the order cc and ic agree; 0525, 0529 and 0531 close when it lands; dc builds 0532 on top. NO PUSH, NO RELEASE.

## TODO

- 0521 is LANDED at 38db27487 (vc PASS; landed patch-id 4650733e4 = judged; gate rc 0, currency ok). What remains: the issue stays OPEN until CI's Intent Tests workflow reads green on BOTH legs after hv's next push. That workflow runs the whole bats suite and is the judge of record. When hv pushes, read the tests.yml run on that HEAD; if it reds, make it speak first. Then close 0521 on vc's word. NO PUSH, NO RELEASE.
- 0534 IS cc's ONCE 0525 LANDS (vc, 2026-09-23; hv decision 32 puts it in 3.2.1): the version string cannot tell a release from a dev build. cc owns the stamp in native/rust/build-support/source_commit.rs. Before building anything: MEASURE every consumer of `intent --version` (intent daemon status, the pre-push hook, the brew formula test, int macos smoke, the app, and both restart files), then propose the form to vc. vc rules the form.

## Holds

_(none)_

## Watch-outs

- SEVERAL DIRTY BOARD RENDERS AT ONCE IS NOT A CHURN PROBLEM. 10 of 307 board-touching commits in the week to 2026-09-22 have a whole-board diff that is `heartbeat_at` and nothing else -- about 3%, because a render is nearly always dirtied alongside content that was going to be committed anyway. Do not spend a design change on it. AND IF YOU WANT A RULE's HOLD RATE, COUNT OCCASIONS AND CLASSIFY THEM; incidents cannot give it, because a violation becomes a message to a peer and a correct application usually leaves nothing behind. Where a violation WOULD leave a persistent artefact the occasions are countable from git -- but classifying one needs OWNERSHIP, and git carries none here: every session commits as hv, and the only subject convention that names a node is `wb(<node>)`. Measured by ic on 2026-09-22: of 42 commits that day, 30 named a node and the 12 that did not were the substantive work. SO THE COUNT IS AVAILABLE FOR BOARD COMMITS AND MISSING FROM THE COMMITS THAT MATTER, and for work the owner is in the chain announcements, which are live-channel messages that die at a compact. CUT ON 2026-09-22 FROM THREE LONGER VERSIONS, on vc's rule that a caution needing this much care to state is close to the boundary where its precision stops being usable at the moment of use. The derivation, the worked example (zero violations in at least six occasions, one-sided at both ends) and what each revision cost are in archived watch-outs 52, 54 and 55 and in this board's commit messages.
- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
