---
node: cc
name: Control Claude
role: control
session_id: 42695459-0b6e-4f49-a7b4-96cfd9ab52ab
heartbeat_at: 2026-10-27 07:33Z
status: active
focus: "Light work while L1 holds the desk (vc decision 97): 0657 v2 (doing 97) and ST0081 WP-02 v1 (doing 98) banked over e22661b78, both verified and booked by vc into the train after Part B's; Part B's six-bank train (doing 96) waits for L2; worktree removals wait for molt-vc's Spotlight word, under decision 98; nothing built, run or installed, no slot ask; no push, no release"
claims: [ST0081, ST0086/04, ISSUE:0657]
---

# Control Claude (cc)

## DOING

- ST0086 WP-04 Part B, a six-bank train in vc's order: refs/bank/cc/0086/wp04b-status-v2 (4ae2585f7479), AC-04.6's CLI share; refs/bank/cc/0086/wp04b-wait-v2 (2421b3d9719d), AC-04.4, with slots wait, slots watch and the hook's slots pending; ic's refs/bank/ic/0086/wp04b-page-v2 (8126a568631b), the page's half; refs/bank/cc/0086/wp04b-init-v1 (e8aee9ac86d6), AC-04.5; refs/bank/cc/0086/wp04b-ats-v1 (f649be3fa3ea), on vc's ruling of 2026-10-26: the AT markers on Part A's tests, a test for AC-04.2's lane re-open and LaneBusy close, and the NamedMismatch text asserted; and ic's pipeline-cell-width-v4 (e058514c0734), hv's ask on ic, CSS only. On main 00eb61a2e the tree after five is 2ca7692e71da and after six c45a889677a8 (ic's and vc's measurement, 17:17Z by their clocks); fmt, gen and critic were green over the five. The heavy judge was granted at 15:01:09Z and revoked at 15:01:34Z by vc's clock for load (hv: one project at a time), and runs when molt-vc grants Intent's slot, behind Devbin and Utilz, straight through with build all and the kovacs preflight. A red in cc's banks comes back to cc in tmp/wt-cc-0086b, detached on scratch tips dfb14c98c (four banks) and ddc43caba (with ats); a fix-forward bank carries no CHANGELOG hunk, its line goes to vc as text, and a bank that moves is told to ic. Owed after a green landing, in one chain: AT rows for AC-04.1 to AC-04.6 and AC-04.8 to AC-04.13 (slots.rs for all but 04.5, coordinator.rs for 04.5), with `intent at lint ST0086` before the commit; design.md through `intent st attach` with controller_unreadable, lanes[].queue and line 67's UNREADABLE, plus pending's cost measured with the judge's binary under `--lane none`, as dated figures, never as a bound; ic's todo 58 owes the pipeline cells' sentence in the same file, so the two attaches go in sequence. No node removes a worktree until molt-vc's Spotlight word; then cc removes tmp/wt-cc-0086b at the landing's END or the next fold. Light work only under decision 92. No push, no release. (edited)
- ISSUE 0657 BANKED AND BOOKED (S): refs/bank/cc/0657/v2, blob 7d1f02d89685, patch-id 60b3c7c55ad9. Base: Part B's tree, scratch tip e22661b78, which is main 5d9b5e2be with the six banks applied (tree af8e2cf31dbc). The bank gives tree fdec35c0a120, in tmp/wt-cc-0657. v1 (caee73fc1d8b) is superseded on vc's two items and kept. vc verified v2 at 07:17Z by vc's clock and booked it into the train after Part B's, with ST0081 WP-02 and ic's 0658. The shape, as ruled in 0657's record: source_commit.rs declares UNSHIPPED beside DIRT_SCOPE, each crate's tests/ and benches/ as two top-anchored glob exclude pathspecs, and emits both as INTENT_DIRT_SCOPE and INTENT_UNSHIPPED onto install::Build. inputs_moved and currency.lib's _build_inputs_changed hand git the same specs. PairBehindHead names the first input that moved, with the build-all remedy. The support refusal names its path. The manifest's source_commit is unchanged, and a note: line names HEAD. MODULES.md's two rows carry the clauses. The judge owes the whole crate suites with intentsvcs's, the shell half, fmt, both clippy legs, the generator check and CI's doc command. At landing vc runs `intent sync --to-store ST0056` for the guard attachment, rewords ST0087's AT-02.1, and writes the CHANGELOG line cc sent. If Part B's judge moves a bank, cc re-banks over the new tree. No CHANGELOG hunk. No push, no release. (edited)
- ST0081 WP-02 BANKED (S): refs/bank/cc/0081/wp02-v1, blob 6f72fe1da6a8, patch-id 901e9ee541d5, over e22661b78 (tree af8e2cf31dbc). It gives tree 9d43e270c0d2 in tmp/wt-cc-0081; stacked after 0657 v2 the tree is 67761344455e. READY went to vc at 07:31Z by cc's clock, and vc verified and booked it at 07:32Z by vc's clock: the train after Part B's stacks 0657 v2, 0658 v1 and WP-02 v1 on e22661b78 to tree 9e124c7e0ade. Built on vc's ruling of 2026-10-27, in the package's record. The order: build release, whose tag push starts linux-release.yml through its new tag trigger; then macos prepare and macos publish; then `bin/devbin release attach vX.Y.Z`, which uploads the Linux packages and writes SHA256SUMS last over every asset; then `bin/int brew publish`, the one writer of the formula and cask; then brew smoke and `intent use brew`. macos publish writes no formula or cask: it ends with the cut's "Finish by hand, in order:" line and exits 0. config.yaml enables devbin's release core for check and attach only, with repo, assets.workflow and assets.wait 15m (from the v3.2.4 and v3.2.5 runs). releasing.md step 4 takes the order and names AC-02.1's evidence. The light checks were release check rc 0 with no second writer, doctor rc 0 with the config validating, shellcheck equal to the base, and the critic clean where asked. vc took both of cc's additions: publish names the attach whatever the release carries, and the STILL OWED note moved to brew publish. Judged in the train after Part B's, with 0657 and 0658, owing the whole crate suites and the shell half. No CHANGELOG hunk; the line went to vc as text. AC-02.1 is proven at the 3.2.7 cut. No push, no release. (edited)

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc's bank refs stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on how many there are. A spent bank ref costs nothing where it sits. (edited)

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
