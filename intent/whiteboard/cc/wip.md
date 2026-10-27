---
node: cc
name: Control Claude
role: control
session_id: 42695459-0b6e-4f49-a7b4-96cfd9ab52ab
heartbeat_at: 2026-10-27 06:47Z
status: active
focus: "GO for Intent's no-load prep (hv's "Yes, go on." via molt-vc, vc decision 97), light work only while L1 holds the desk: 0657 (doing) banked first over Part B's tree in a detached worktree, then ST0081 WP-02 (todo, released from hold 33) read and reported to vc, then banked; Part B's six-bank train (doing 96) waits for L2; nothing built, run or installed, no slot ask; no push, no release"
claims: [ST0081, ST0086/04, ISSUE:0657]
---

# Control Claude (cc)

## DOING

- ST0086 WP-04 Part B, a six-bank train in vc's order: refs/bank/cc/0086/wp04b-status-v2 (4ae2585f7479), AC-04.6's CLI share; refs/bank/cc/0086/wp04b-wait-v2 (2421b3d9719d), AC-04.4, with slots wait, slots watch and the hook's slots pending; ic's refs/bank/ic/0086/wp04b-page-v2 (8126a568631b), the page's half; refs/bank/cc/0086/wp04b-init-v1 (e8aee9ac86d6), AC-04.5; refs/bank/cc/0086/wp04b-ats-v1 (f649be3fa3ea), on vc's ruling of 2026-10-26: the AT markers on Part A's tests, a test for AC-04.2's lane re-open and LaneBusy close, and the NamedMismatch text asserted; and ic's pipeline-cell-width-v4 (e058514c0734), hv's ask on ic, CSS only. On main 00eb61a2e the tree after five is 2ca7692e71da and after six c45a889677a8 (ic's and vc's measurement, 17:17Z by their clocks); fmt, gen and critic were green over the five. The heavy judge was granted at 15:01:09Z and revoked at 15:01:34Z by vc's clock for load (hv: one project at a time), and runs when molt-vc grants Intent's slot, behind Devbin and Utilz, straight through with build all and the kovacs preflight. A red in cc's banks comes back to cc in tmp/wt-cc-0086b, detached on scratch tips dfb14c98c (four banks) and ddc43caba (with ats); a fix-forward bank carries no CHANGELOG hunk, its line goes to vc as text, and a bank that moves is told to ic. Owed after a green landing, in one chain: AT rows for AC-04.1 to AC-04.6 and AC-04.8 to AC-04.13 (slots.rs for all but 04.5, coordinator.rs for 04.5), with `intent at lint ST0086` before the commit; design.md through `intent st attach` with controller_unreadable, lanes[].queue and line 67's UNREADABLE, plus pending's cost measured with the judge's binary under `--lane none`, as dated figures, never as a bound; ic's todo 58 owes the pipeline cells' sentence in the same file, so the two attaches go in sequence. No node removes a worktree until molt-vc's Spotlight word; then cc removes tmp/wt-cc-0086b at the landing's END or the next fold. Light work only under decision 92. No push, no release. (edited)
- ISSUE 0657, install's HEAD check judged by the build-input scope (S), cc's bank on vc's ruling of 2026-10-27, written into 0657's record before code. DIRT_SCOPE stays on its one line at build-support/source_commit.rs:199, and emit_source_commit() also emits it as INTENT_DIRT_SCOPE, with the tests/ and benches/ exclusion declared once beside it and read from the build the same way. install::Build gains a scope field. A private inputs_moved() mirrors currency.lib's _build_inputs_changed and names it as its twin. The HEAD refusal is its own InstallError variant, naming the first path that moved and giving the build-all remedy, in place of RuntimeSetDirty there. An unreadable git refuses. The support refusal names its first path. The NOTES.md arm in install_publishes_an_opt_tree.rs inverts, since that move installs. The manifest's source_commit keeps its meaning, no manifest key is added, and the HEAD shows only on the note: line. If the arm holding DIRT_SCOPE's shell twin cannot read the exclusion, the bank report says so. Base: Part B's tree, main with the six banks applied in the train's order; re-banked if Part B's judge moves a bank. Light work in a detached worktree (edits and git, nothing built or run) under hv's layer order (vc decision 97). Judged in the train after Part B's, beside ST0081 WP-02, owing the whole crate suites, intentsvcs's whole suite and the shell half. No CHANGELOG hunk.

## TODO

- ST0081 WP-02, Linux assets on each tag's GitHub release with one writer per release object (S). RELEASED FROM HOLD 33 by vc under hv's pen on 2026-10-27, in the package's record: ST0087 has gone first because WP-01 to WP-04 are Done, and WP-05 is proof whose brew and kovacs legs follow the cut. Built for 3.2.7, after 0657's bank. FIRST, before any edit: read intent/docs/releasing.md step 4 and bin/.devbin/cmd/build.d/release as they stand, and send vc two things. Where `bin/devbin release attach vX.Y.Z` sits in the cut. How the one SHA256SUMS that attach writes over every asset meets the SHA256SUMS.txt that macos stage writes, against which the cask's sha256 is checked, with the release object and the formula each keeping one writer. THEN the Intent half per the record: bin/.devbin/config.yaml declares `commands.release.enabled: true`, `release.verbs: [check, attach]`, `release.repo: matthewsinclair/intent`, `assets.workflow: linux-release.yml` and `assets.wait`, and no release.remotes; linux-release.yml gains the tag trigger on `v[0-9]*`; releasing.md's cut order takes the attach and the by-hand interim retires. The vendored devbin is 0.1.14 and offers `release attach`. Light work in a detached worktree. Judged beside 0657, owing the whole crate suites and the shell half. No CHANGELOG hunk. AC-02.1 is proven at the 3.2.7 cut, by the evidence in the spec's item (6).

## Holds

_(none)_

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc's bank refs stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on how many there are. A spent bank ref costs nothing where it sits. (edited)

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
