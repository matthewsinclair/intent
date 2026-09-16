---
node: dc
name: DevX Claude
role: worker
session_id: effb6a8d-6c0a-4922-91ba-31c145970ff6
heartbeat_at: 2026-09-16 19:50Z
status: active
focus: "2026-09-16 evening: 0426 (a project's own guards, declared in config.json) is in build, WIP at refs/bank/dc/0426-wip; the board trawl is built and runs only on vc's word after the rebuild. See the RESUME HERE doing item. NO RELEASE, NO PUSH."
claims: [ST0056/11]
---

# DevX Claude (dc)

## DOING

- **RESUME HERE (localfold, 2026-09-16 evening): 0426 is in build, and the board trawl is built and waits for vc's word.** 0426 (a project's own guards, routed by vc on hv's word; vc's rulings 1 to 5 of 2026-09-16): WIP at refs/bank/dc/0426-wip, blob a4c89748008ddd72d7652f1bbd3a24ebaeb48035, base c8a8915fd393dc295ee65685ee190414a2ea19de, worktree scratchpad/wt-migrate. DONE in the WIP: lib/templates/hooks/pre-commit-guards.sh reads `guards` from intent/.config/config.json (an argv `run` and an optional `when`, rows separated by US 0x1f). It dispatches them after Intent's roster, blocks on a declaration that is MISSING, not executable, untracked or unreadable, and appends a 5th column (intent or project) to --list-guards. There are 5 new arms in intent-cli tests/migrated_guards_still_refuse.rs, all green (HEAD reads no guards). shellcheck is clean and bash 3.2 parses it. REMAINING: (a) Config gains a typed `guards: Vec<GuardDecl>` in project.rs (HEAD already carries the key through `extra`), with an arm that `intent lang init` and `lang remove` keep the guards; it is green on HEAD and stands as the guard, so tell vc. (b) doctor.rs beside hook_findings, as Advisory findings that never block: (a) non-comment lines in the pre-commit chain file, outside the chain block, that name no declared guard, including the fixture of a hand-wired hook plus a declaration reported as doubled; (b) core.hooksPath unset while a tracked .githooks/ or bin/hooks/ holds a pre-commit. Both are red on HEAD, in intentsvcs gate_not_running_is_reported.rs. (c) Verify the --list-guards consumers: bin/.devbin/cmd/hooks, gate_not_running_is_reported.rs and exit_code_consumers.rs. (d) Docs: the config.json reference and the hooks section of working-with-llms.md. (e) Whole gate, bank, report blob and patch-id, land on vc's GO. Adoption, Intent's own included, is NOT in this change (vc ruling 5). Hook templates serve from the working copy, so build them only in the worktree. The rebuild for 0423, 0424 and 0425 waits for 0426. TRAWL: scratchpad/trawl.sh is built to vc's rulings: every board not in the store is registered in the clone, and it writes prep.txt and trawl-logs/worklist.txt. prep was tested on Conflab. RUN IT ONLY on vc's word, after hv's drive and the rebuild. NO RELEASE, NO PUSH.

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- **dc's unhomed landing lessons, folded into one item (2026-09-15).** (1) A new dispatch row moves `legal_pairs` n in `surface/dispatch-table.json`: bump n with its `census_note` and regenerate `surface/dispatch-table.md` with `intent/st/ST0056/parity/tools/gen_dispatch_table.sh` in the SAME change; only the commit gate's view_skew_check sees it. (2) `cargo test -p intent-cli --bins <filter>` runs ZERO tests at exit 0: in-crate tests are `--lib`, the integration arms `--test suite`; read `running N tests` before trusting a green. (3) Guard a landing on nothing under `native/rust` having moved since the measured base, never on HEAD equality; never `commit --only` a path a peer has uncommitted edits in. (4) Under an isolated HOME, set `CARGO_HOME` to the real one; a daemon probe's HOME must be short (`/tmp/<short>`, `sockaddr_un` holds about 104 bytes); in zsh `pgrep` can return two pids, so kill the literal pid and bound every wait. (5) Re-drive a hold's condition whenever you quote it: on 2026-09-15 todo 8's hook premise and hold 2's tap commit were both false on re-drive. (6) A reboot clears `/private/tmp`, and every scratchpad and worktree with it: bank anything unlanded as a patch outside it before a fold.
- **dc's landing lessons of 2026-09-15, second half.** (1) Before banking a crate change, run that crate's WHOLE suite: a targeted green on 0398 missed `command_rosters_are_derived_or_declared`, which vc's train caught. (2) Removing a swallow means first measuring what it swallows: `opt()`'s read-as-absent was load-bearing for four shared probes, so a mechanical `?` at every site would have broken bare `issues`, `issues show`, `st edit` and `browse`. (3) Count call sites with a script, not the Bash tool's grep: an alternation pattern reported 4 `opt(` calls where there are 53. (4) A bank that edits an ST attachment carries no canon, so `attachment_drift_detected` and `thread_prose_carried` red on any stack holding it until the landing's ingest; land an attachment with its canon in one commit, verified after intentd settles.
- **dc's lessons of 2026-09-16.** (1) Read hv's board with wb show hv BEFORE opening any diagnosis: a_daemon_outlives_nobody.rs was already a ruled flake (hv watchout 11), and pickup shows peers as headers only. (2) A load control must be matched: a 1-minute load of 4.8 against a 15-minute load of 13.9 compares nothing. (3) Paste hashes and stamps whole from tool output; a typed 17be3d7e9ae was wrong. (4) A grep -v filter can eat the very lines it is meant to show: positive-control it (a planted unwrap() gave rc=101) before trusting an rc 0. (5) git apply --check against the clean target or a throwaway index (GIT_INDEX_FILE + read-tree + apply --cached), never a worktree the patch is already applied in. (6) A hold hv gives is lifted only by hv's explicit word, and a node's own announced reading of an ambiguous 'continue' is not that word: dc started held work after a compact on exactly that reading and hv re-held it.
- **dc's lessons of 2026-09-16, second half.** (1) Never start a suite under nohup or with a trailing &: SIGINT is inherited as ignored, so no_tail_survives_follow_however_it_ends' INT arm cannot stop intent daemon logs --follow, child.wait() never returns, and the suite hangs until the child is killed. Use a plain background task. (2) Drive a remedy to a clear before naming it: declaring an issue then sync --to-disk was refused by sync's shrink guard, and organize --apply is the route that cleared it. (3) Save a WIP ref before answering a routing question; vc moved 0416 to cc mid-build and the ref was the handover. (4) In the Bash tool's zsh an env prefix held in a variable does not word-split, so every cargo step exits 127: put multi-step gates in a bash script. (5) A peer's CHAIN START binds a fold too, and peers fold for the same compact at the same moment: a localfold's board writes wait for CHAIN END and for the peers' folds to release the store, and an archive must not run ahead of the add that replaces it.
- **dc's lessons of 2026-09-16, evening.** (1) A tab is whitespace to bash's `read`, so an empty TSV field collapses into the next one. The first project-guard dispatch read `run` as `when` and skipped every always-applicable guard as not applicable, silently. Separate fields with a non-whitespace character (US, 0x1f). (2) A read-only sqlite3 open of a WAL store with no -shm fails with "unable to open database file (14)". `file:<db>?immutable=1` reads a store with no -wal and creates no file; `.backup` is the consistent copy of a live one. Never `cp` a WAL store. (3) A plain `wb migrate` is not read-only: it carries a board with nothing uncarried. A read-only survey runs in a scratch clone. (4) core.hooksPath lives in untracked .git/config, so a tracked hooks directory is lost on a fresh clone exactly as .git/hooks is. (5) An if-let with no else arm can be a silent skip (0424): read what a loop drops, not only what it keeps.

## Decisions

_(none)_

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
