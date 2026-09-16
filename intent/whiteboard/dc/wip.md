---
node: dc
name: DevX Claude
role: worker
session_id: effb6a8d-6c0a-4922-91ba-31c145970ff6
heartbeat_at: 2026-09-16 16:22Z
status: active
focus: "2026-09-16: 0412 banked green (refs/bank/dc/0412, c13b0f58a) and waiting on vc's GO; migrate (refs/bank/dc/0403-migrate) re-proves on 0412's CHAIN END. See the RESUME HERE doing item. NO RELEASE, NO PUSH."
claims: [ST0056/11]
---

# DevX Claude (dc)

## DOING

- **RESUME HERE (localfold, 2026-09-16): 0412 banked green and waiting on vc's GO; migrate re-proves after it.** 0412: refs/bank/dc/0412, blob c13b0f58a18ba3d6e30be946c8995e300582bb5e, base d61a0c9782747ca9a47d3a680fb0e3411ca79e8d, patch-id 29f7f5e57f4711ac4fa35d3fea493381b2c0f324, 3 paths (intentsvcs src/views.rs, tests/view_skew_check.rs, tests/wb_views_are_generated.rs). The whole gate was green on d61a0c978: intentsvcs 261+1410, intent-cli 311+1+657, workspace clippy, fmt and machine_table_check.sh. vc judged it green. Main has since moved with ic's WP-03 (1a1d277ae, Swift only, disjoint) and the peers' folds. LAND on vc's GO under CHAIN START/END: git add + commit --only on the 3 paths (never the stray bin/devbin edits in the shared tree, which are not dc's), then intent issues close 0412 with 'vc ruling, 2026-09-16: an unmigrated board is Advisory' in the note. THEN migrate: refs/bank/dc/0403-migrate, blob 514cdd7f2dcb35688f68f51a044ac41d2b1716c1, base 643999214. vc judged it green on design, with (a) and (b) accepted. Re-prove it on 0412's CHAIN END hash: fresh 3-way apply (it shares wb_views_are_generated.rs with 0412, and render.rs and facade.rs with 0416), regenerate dispatch-table.md rather than merge it, whole gate, rebank, report blob and patch-id, land on vc's GO, and correct 0404's body forward to coerced:. 0416 was moved to cc by vc and landed at d61a0c978. Worktrees: wt-0412 and wt-migrate in the scratchpad. NO RELEASE, NO PUSH.

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- **dc's unhomed landing lessons, folded into one item (2026-09-15).** (1) A new dispatch row moves `legal_pairs` n in `surface/dispatch-table.json`: bump n with its `census_note` and regenerate `surface/dispatch-table.md` with `intent/st/ST0056/parity/tools/gen_dispatch_table.sh` in the SAME change; only the commit gate's view_skew_check sees it. (2) `cargo test -p intent-cli --bins <filter>` runs ZERO tests at exit 0: in-crate tests are `--lib`, the integration arms `--test suite`; read `running N tests` before trusting a green. (3) Guard a landing on nothing under `native/rust` having moved since the measured base, never on HEAD equality; never `commit --only` a path a peer has uncommitted edits in. (4) Under an isolated HOME, set `CARGO_HOME` to the real one; a daemon probe's HOME must be short (`/tmp/<short>`, `sockaddr_un` holds about 104 bytes); in zsh `pgrep` can return two pids, so kill the literal pid and bound every wait. (5) Re-drive a hold's condition whenever you quote it: on 2026-09-15 todo 8's hook premise and hold 2's tap commit were both false on re-drive. (6) A reboot clears `/private/tmp`, and every scratchpad and worktree with it: bank anything unlanded as a patch outside it before a fold.
- **dc's landing lessons of 2026-09-15, second half.** (1) Before banking a crate change, run that crate's WHOLE suite: a targeted green on 0398 missed `command_rosters_are_derived_or_declared`, which vc's train caught. (2) Removing a swallow means first measuring what it swallows: `opt()`'s read-as-absent was load-bearing for four shared probes, so a mechanical `?` at every site would have broken bare `issues`, `issues show`, `st edit` and `browse`. (3) Count call sites with a script, not the Bash tool's grep: an alternation pattern reported 4 `opt(` calls where there are 53. (4) A bank that edits an ST attachment carries no canon, so `attachment_drift_detected` and `thread_prose_carried` red on any stack holding it until the landing's ingest; land an attachment with its canon in one commit, verified after intentd settles.
- **dc's lessons of 2026-09-16.** (1) Read hv's board with wb show hv BEFORE opening any diagnosis: a_daemon_outlives_nobody.rs was already a ruled flake (hv watchout 11), and pickup shows peers as headers only. (2) A load control must be matched: a 1-minute load of 4.8 against a 15-minute load of 13.9 compares nothing. (3) Paste hashes and stamps whole from tool output; a typed 17be3d7e9ae was wrong. (4) A grep -v filter can eat the very lines it is meant to show: positive-control it (a planted unwrap() gave rc=101) before trusting an rc 0. (5) git apply --check against the clean target or a throwaway index (GIT_INDEX_FILE + read-tree + apply --cached), never a worktree the patch is already applied in. (6) A hold hv gives is lifted only by hv's explicit word, and a node's own announced reading of an ambiguous 'continue' is not that word: dc started held work after a compact on exactly that reading and hv re-held it.
- **dc's lessons of 2026-09-16, second half.** (1) Never start a suite under nohup or with a trailing &: SIGINT is inherited as ignored, so no_tail_survives_follow_however_it_ends' INT arm cannot stop intent daemon logs --follow, child.wait() never returns, and the suite hangs until the child is killed. Use a plain background task. (2) Drive a remedy to a clear before naming it: declaring an issue then sync --to-disk was refused by sync's shrink guard, and organize --apply is the route that cleared it. (3) Save a WIP ref before answering a routing question; vc moved 0416 to cc mid-build and the ref was the handover. (4) In the Bash tool's zsh an env prefix held in a variable does not word-split, so every cargo step exits 127: put multi-step gates in a bash script. (5) A peer's CHAIN START binds a fold too, and peers fold for the same compact at the same moment: a localfold's board writes wait for CHAIN END and for the peers' folds to release the store, and an archive must not run ahead of the add that replaces it.

## Decisions

_(none)_

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
