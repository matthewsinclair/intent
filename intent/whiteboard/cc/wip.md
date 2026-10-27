---
node: cc
name: Control Claude
role: control
session_id: 42695459-0b6e-4f49-a7b4-96cfd9ab52ab
heartbeat_at: 2026-10-27 08:32Z
status: active
focus: "Holding for L2, light work only while L1 holds the desk (vc decision 97): Part B's six-bank train (doing 96) runs first when molt-vc opens L2; 0657 v2 (doing 97) and ST0081 WP-02 v1 (doing 98, the package WIP) are banked over e22661b78, verified and booked by vc for the train after it; worktree removals wait for molt-vc's Spotlight word, under decision 98; no push, no release"
claims: [ST0081, ST0086/04, ISSUE:0657]
---

# Control Claude (cc)

## DOING

- ST0086 WP-04 Part B, a six-bank train in vc's order: (edited)
  - refs/bank/cc/0086/wp04b-status-v2 (4ae2585f7479): AC-04.6's CLI share.
  - refs/bank/cc/0086/wp04b-wait-v2 (2421b3d9719d): AC-04.4, with slots wait, slots watch and the hook's slots pending.
  - ic's refs/bank/ic/0086/wp04b-page-v2 (8126a568631b): the page's half.
  - refs/bank/cc/0086/wp04b-init-v1 (e8aee9ac86d6): AC-04.5.
  - refs/bank/cc/0086/wp04b-ats-v1 (f649be3fa3ea): the AT markers on Part A's tests, a test for AC-04.2's lane re-open and LaneBusy close, and the NamedMismatch text asserted.
  - ic's pipeline-cell-width-v4 (e058514c0734): CSS only.
  RE-CONFIRMED 2026-10-27 at 05:47Z by cc's clock: every patch-id is as recorded. The six apply in order on main c664ea126 to tree 00c92522e4c3, which equals ic's 229c6053f4a2 outside intent/whiteboard and intent/.canon/events. Scratch tip e22661b78 (main 5d9b5e2be with the six applied) is the base of the train after this one.
  THE JUDGE runs when molt-vc opens L2 for Intent, once L1 (devbin, molt, utilz, gtools) is cut, brewed and reinstalled (hv's layer order, molt-vc decision 22). It runs straight through, with build all and the kovacs preflight.
  A RED in cc's banks comes back to cc in tmp/wt-cc-0086b, detached on scratch tip ddc43caba. A fix-forward bank carries no CHANGELOG hunk, and its line goes to vc as text. A bank that moves is told to ic, and 0657 and WP-02 are then re-banked over the new tree.
  OWED AFTER A GREEN LANDING, in one chain:
  - AT rows for AC-04.1 to AC-04.6 and AC-04.8 to AC-04.13 (slots.rs for all but 04.5, coordinator.rs for 04.5), with `intent at lint ST0086` before the commit.
  - design.md through `intent st attach`: controller_unreadable, lanes[].queue and line 67's UNREADABLE, plus pending's cost measured with the judge's binary under `--lane none`, as dated figures and never as a bound. ic's refs/bank/ic/0086/design-cells-v1 carries the pipeline cells' sentence for the same file, so the two attaches go in sequence.
  Worktree removals wait for molt-vc's Spotlight word, under decision 98 as amended. No push, no release.
- ISSUE 0657 BANKED AND BOOKED (S): refs/bank/cc/0657/v2, blob 7d1f02d89685, patch-id 60b3c7c55ad9. Base: Part B's tree, scratch tip e22661b78, which is main 5d9b5e2be with the six banks applied (tree af8e2cf31dbc). The bank gives tree fdec35c0a120, in tmp/wt-cc-0657. v1 (caee73fc1d8b) is superseded on vc's two items and kept. vc verified v2 at 07:17Z by vc's clock and booked it into the train after Part B's, with ST0081 WP-02 and ic's 0658. The shape, as ruled in 0657's record: source_commit.rs declares UNSHIPPED beside DIRT_SCOPE, each crate's tests/ and benches/ as two top-anchored glob exclude pathspecs, and emits both as INTENT_DIRT_SCOPE and INTENT_UNSHIPPED onto install::Build. inputs_moved and currency.lib's _build_inputs_changed hand git the same specs. PairBehindHead names the first input that moved, with the build-all remedy. The support refusal names its path. The manifest's source_commit is unchanged, and a note: line names HEAD. MODULES.md's two rows carry the clauses. The judge owes the whole crate suites with intentsvcs's, the shell half, fmt, both clippy legs, the generator check and CI's doc command. At landing vc runs `intent sync --to-store ST0056` for the guard attachment, rewords ST0087's AT-02.1, and writes the CHANGELOG line cc sent. If Part B's judge moves a bank, cc re-banks over the new tree. No CHANGELOG hunk. No push, no release. (edited)
- ST0081 WP-02 BANKED (S), the package WIP since 34689f61a: refs/bank/cc/0081/wp02-v1, blob 6f72fe1da6a8, patch-id 901e9ee541d5, over e22661b78 (tree af8e2cf31dbc). It gives tree 9d43e270c0d2 in tmp/wt-cc-0081; stacked after 0657 v2 the tree is 67761344455e. READY went to vc at 07:31Z by cc's clock, and vc verified and booked it at 07:32Z by vc's clock into the train after Part B's, which stacks 0657 v2, ic's 0658 and WP-02 v1 on e22661b78. 0658 v1 is withdrawn, since it calls Instant::now and the one_clock guard bans that, so v1's stacked tree 9e124c7e0ade is not the train's. ic named 0658 v2 in its END of 08:30Z by ic's clock: refs/bank/ic/0658/v2, blob 29282d37ceb3, patch-id 3534a4ebafa3, three files that neither cc bank touches. In a scratch index cc stacked 0657 v2 and WP-02 v1 on e22661b78 to tree 67761344455e, then 0658 v2 on that to 66c60d9bc8a7; vc books the train's tree. Built on vc's ruling of 2026-10-27, in the package's record. The order: build release, whose tag push starts linux-release.yml through its new tag trigger; then macos prepare and macos publish; then `bin/devbin release attach vX.Y.Z`, which uploads the Linux packages and writes SHA256SUMS last over every asset; then `bin/int brew publish`, the one writer of the formula and cask; then brew smoke and `intent use brew`. macos publish writes no formula or cask: it ends with the cut's "Finish by hand, in order:" line and exits 0. config.yaml enables devbin's release core for check and attach only, with repo, assets.workflow and assets.wait 15m (from the v3.2.4 and v3.2.5 runs). releasing.md step 4 takes the order and names AC-02.1's evidence. The light checks were release check rc 0 with no second writer, doctor rc 0 with the config validating, shellcheck equal to the base, and the critic clean where asked. vc took both of cc's additions: publish names the attach whatever the release carries, and the STILL OWED note moved to brew publish. Judged in the train after Part B's, with 0657 and 0658, owing the whole crate suites and the shell half. No CHANGELOG hunk; the line went to vc as text. AC-02.1 is proven at the 3.2.7 cut. No push, no release. (edited)

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
