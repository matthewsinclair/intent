---
node: dc
name: DevX Claude
role: worker
session_id: effb6a8d-6c0a-4922-91ba-31c145970ff6
heartbeat_at: 2026-09-16 14:15Z
status: active
focus: "2026-09-16: the devbin twin (17be3d7e9f5526c1f0779ac5acffc97278d6c77d) and Decision A (fd9ff7f0a69737835525f65f2905428ce3537735) are landed and verified. HELD on hv's word: 0403-0409, 0412 and 0416 -- migrate is partly built and banked at refs/bank/dc/0403-migrate-wip, see hold 11. NO RELEASE, NO PUSH."
claims: [ST0056/11]
---

# DevX Claude (dc)

## DOING

_(none)_

## TODO

_(none)_

## Holds

- **0403-0409 (migrate), 0412 (doctor remedies), 0416 (pickup gap) -- routed to dc by vc at aa80e6917e6b97c94bccd303c17d1719b77a996d, HELD until hv lifts hv's own hold given at the terminal on 2026-09-16.** CORRECTS hold 10, which said nothing was edited: after hv's compact dc read 'continue on the bounce' as lifting the hold and started 0403-0409 before hv confirmed, and hv re-held it. That partial work is BANKED at refs/bank/dc/0403-migrate-wip (blob 4a2ad63d2f6e8e50fe13220fea29f5357ec9d874, diff against 12bfde803): wbmigrate.rs, facade.rs and project.rs only -- vc decision 20 in the library (sub-headings, tables and heading qualifiers refused as units, section prose flagged coerced, WbUncarried refusal before any write unless drop_uncarried, the board kept verbatim at .history/pre-migration/wip.md, WbSnapshotInTheWay, left_in_place for non-markdown .history files). intentsvcs type-checks; NOT done: the CLI flag and report words, the dispatch row and dispatch-table.md, the error_remedies and mandatory_fields enumerations, the two migrate test files, and every suite run. Recover with git cat-file -p refs/bank/dc/0403-migrate-wip | git apply in a worktree at 12bfde803 or later. On release, the rest as hold 10 set out: migrate as ONE change under vc decision 20 (own ref), then 0412 doctor (own ref), then 0416 pickup (own ref); cc's vc-decision-21 store door meets migrate and pickup in facade.rs's wb ops, and whichever banks green second proves on top of the first. Whole intentsvcs and intent-cli suites before each bank. NO RELEASE, NO PUSH.

## Watch-outs

- **dc's unhomed landing lessons, folded into one item (2026-09-15).** (1) A new dispatch row moves `legal_pairs` n in `surface/dispatch-table.json`: bump n with its `census_note` and regenerate `surface/dispatch-table.md` with `intent/st/ST0056/parity/tools/gen_dispatch_table.sh` in the SAME change; only the commit gate's view_skew_check sees it. (2) `cargo test -p intent-cli --bins <filter>` runs ZERO tests at exit 0: in-crate tests are `--lib`, the integration arms `--test suite`; read `running N tests` before trusting a green. (3) Guard a landing on nothing under `native/rust` having moved since the measured base, never on HEAD equality; never `commit --only` a path a peer has uncommitted edits in. (4) Under an isolated HOME, set `CARGO_HOME` to the real one; a daemon probe's HOME must be short (`/tmp/<short>`, `sockaddr_un` holds about 104 bytes); in zsh `pgrep` can return two pids, so kill the literal pid and bound every wait. (5) Re-drive a hold's condition whenever you quote it: on 2026-09-15 todo 8's hook premise and hold 2's tap commit were both false on re-drive. (6) A reboot clears `/private/tmp`, and every scratchpad and worktree with it: bank anything unlanded as a patch outside it before a fold.
- **dc's landing lessons of 2026-09-15, second half.** (1) Before banking a crate change, run that crate's WHOLE suite: a targeted green on 0398 missed `command_rosters_are_derived_or_declared`, which vc's train caught. (2) Removing a swallow means first measuring what it swallows: `opt()`'s read-as-absent was load-bearing for four shared probes, so a mechanical `?` at every site would have broken bare `issues`, `issues show`, `st edit` and `browse`. (3) Count call sites with a script, not the Bash tool's grep: an alternation pattern reported 4 `opt(` calls where there are 53. (4) A bank that edits an ST attachment carries no canon, so `attachment_drift_detected` and `thread_prose_carried` red on any stack holding it until the landing's ingest; land an attachment with its canon in one commit, verified after intentd settles.
- **dc's lessons of 2026-09-16.** (1) Read hv's board with wb show hv BEFORE opening any diagnosis: a_daemon_outlives_nobody.rs was already a ruled flake (hv watchout 11), and pickup shows peers as headers only. (2) A load control must be matched: a 1-minute load of 4.8 against a 15-minute load of 13.9 compares nothing. (3) Paste hashes and stamps whole from tool output; a typed 17be3d7e9ae was wrong. (4) A grep -v filter can eat the very lines it is meant to show: positive-control it (a planted unwrap() gave rc=101) before trusting an rc 0. (5) git apply --check against the clean target or a throwaway index (GIT_INDEX_FILE + read-tree + apply --cached), never a worktree the patch is already applied in. (6) A hold hv gives is lifted only by hv's explicit word, and a node's own announced reading of an ambiguous 'continue' is not that word: dc started held work after a compact on exactly that reading and hv re-held it.

## Decisions

_(none)_

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
