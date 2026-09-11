---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-11 16:15Z
status: active
focus: "BOUNCE, 2026-09-11 16:15Z. hv ruled all 14 decisions. Every workable row on the list is re-driven and closed; the 3 still open are NOT WORKABLE. Left: dc's decision 3 and decision 2's notice fix, then the cut. intent/wip.md is the authority. RUN THE VERBS; every figure here rots."
claims: [ST0056, ST0057, ST0060, ST0068, ST0070, ST0073]
---

# Validation Claude (vc)

**BOUNCE AFTER THE 2026-09-11 COMPACT, `active`.** The pre-bounce queue is closed, and each close commit carries its own drive. Pre-fold boards are verbatim at `.history/20260911/wip-prefold-1314Z.md` and `-0914Z.md`.

## DOING

**NOTHING IN FLIGHT ON vc.** The peers, each to be re-read off its own board:

- `dc` -- decision 3 (strike `st bootstrap`, `agents template` and `claude prime`), ruled RETIRE with `spelling: ""`, st_zero's successor cleared, `init --with-st0000` retired, and B ruled (a). Then the fix to decision 2's notice: c2ea14c1's "only way back" sentence was sent back.
- `cc`, `ic` -- holding; their lanes are empty.

## TODO

1. **Decision 3 when dc lands it.** Each of the three answers rc=2 'was retired' with no replacement and is absent from --help. st zero's refusal names no successor. `init --with-st0000` is retired. The five repoints and five one-sided notes are as ruled, and a kept negative half fails on a planted instance.
2. **Decision 2's notice when dc re-lands it.** Drive both ways back with the 3.0.0 keg (/opt/homebrew/Cellar/intent/3.0.0_1/bin/intent): a snapshot always works; deleting the cache works only while canon carries nothing 3.0.1 alone writes (a WP status_legacy is refused as schema-invalid).
3. **At the cut.**
   - Decision 6 (dc): repoint test_helper at v3.
   - dc: regenerate the whole of docs/reference at the cut sha.
   - Satisfy the nine cut-time rows by evidence.
   - Close ST0056 WP-07/11/12, then ST0056, ST0058 and ST0068.
   - Push upstream (decision 1).
   - The PATH pair must name the cut sha: `bin/devbin build all`.
4. **The three NOT WORKABLE rows (0177, 0141, 0172) stay open** as constraints on future work. Each row says why.

**NO NEW WORK.** Nothing gets added to the list. A defect found while fixing goes in the commit message.

## Holds

- None. Every hv decision is ruled, and every held close is done.

## Watch-outs (added on the bounce)

- **A test that opens `repo_root()` migrates the LIVE store** (tests/attachment_drift_detected.rs -> doctor -> Store::open). Run every intentsvcs suite from a private worktree. The 15:10:30Z incident and its recovery are in d0777bc8.
- **A parent build cannot read a fixture a newer build made** (store 18 vs 17). Make each arm's fixture with that arm's own binary.

## Standing directives from hv

- **`0196` RULED BY hv 2026-09-05 15:23Z, TWO RULINGS, `authority: hv`.**
  1. REMEDY: **staging + atomic mv** -- build into a third target dir, `verify_pair` THERE, and rename the verified pair into `target/release/` only on pass.
  2. SEQUENCING: **fix FIRST, then rebuild through it** -- the fix's own first real run IS the rebuild to HEAD, and **there is no separate supervised window.** Assigned to dc as WP-11's precondition.
- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **PFIC is _Pure Function, Impure Coordination_** -- deterministic core, I/O at the boundary.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs -- only what bears on working the list

- **Shared checkout.** `git add <paths> && git commit --only <paths>` in ONE call; `--only` does not add untracked files. Never remove a peer's `.git/index.lock`; never `--no-verify`.
- **A commit can lose the lock race AFTER the wait loop.** Always read `git log -1` after committing. If it failed, re-issue the SAME command; do not recompose it (0224's close, 2026-09-11).
- **A red arm must predate the fix and postdate the defect.** 951cbac2 is behind HEAD. It predates some refusals (`0259`) and postdates some fixes (`0256`), and it reads templates from the LIVE tree, so it is no baseline for a template change (`0220`). When it cannot discriminate, use the fix's parent, or a one-line mutation in my own worktree. Check that the red arm reproduces BEFORE reading a green.
- **A drive can miss its own case.** 0283's first fixture left the thread declared, and 0259's never reached the gate; both arms then agree, and that proves nothing. A green on the fix only counts beside a red on the baseline.
- **The Bash tool is zsh.**
  - Unquoted `$var` does not word-split, and an unmatched glob aborts the whole command.
  - `$r:lib` is `$r` plus the `:l` lowercase modifier; write `${r}:lib`.
  - A line starting `===` is `=cmd` expansion; use `echo '---'`.
- **A canon write can report `ok` and be reverted about a second later.** 0216 is closed, but verify past the ingest.
- **`sync --to-store` REPLACES the store from the extract.** Edit canon first, drive state verbs after.
- **Count `intentd` by executable**, `ps -axo pid=,command=` on argv[0]'s basename.
- **A timestamp goes on a board or in a message only from a `date -u` read in the same turn.**
- **A test run can rewrite the machine's install pointer** (`dual_path_conformance` runs `intent bootstrap` under the REAL HOME). Every build and test run goes under an isolated HOME. If commits refuse with "the recorded Intent install root is not an install", read `~/.intent/home` first.

## Decisions

- (2026-09-11) **The work is the list. hv's rule, verbatim: _THERE IS NO NEW WORK TO BE DONE._** Every node, every item.
- (2026-09-11) **When a lane empties, the node takes the next unclaimed item in list order, skipping one whose files a peer is about to touch.** The lane column in `intent/wip.md` is the authority: `0083`, `0168` and `0176` moved ic -> cc on it. `0185` stays with ic beside `0154`, so the two body doors are one design.
- (2026-09-11) **vc stopped dc maintaining the ST0068 AC-02.3 manifest and its disposition check.** A docs item is the page edit plus one commit; a stale quote in that manifest waits for the cut.
- (2026-09-11) **NOT WORKABLE IN 3.0.1, kept open as constraints, lane `--`:**
  - `0177` -- all of `ext` is unbuilt;
  - `0141` -- no instance today, and the only fix is a guard.
    Closing either would lose the finding.
- (2026-09-11) **`0100` shape ruled (b): carry the v2 spelling in `status_legacy`, with `status` unchanged.** The build is HELD on the 17 -> 18 store rung (decision 2).
- (2026-09-11) **`0084` put to hv as decision 11, recommending (a), the byte write for opaque attachments.** vc drove the migrator hole: a Latin-1 `notes.txt` migrates with no sidecar, and the next restore refuses `broken-reference`.
- (2026-09-11) **`0114`, `0220` and `0065` put to hv as decisions 12, 13 and 14.** Each issue reserves its question for hv, or needs a policy value.
- (2026-09-11) **`0259` is not closed on half 1.** The issue's second clause is doctor's, and sits in dc's column.
- (2026-09-11) **`0283` ruled A+B, both in 3.0.1, one row.**
  - A (dc): make doctor's and organize's messages true, and name `st hydrate <ID>`, which pins the thread.
  - B (cc, verified): re-render an undeclared view iff its disk bytes equal the prior render.
  - C (a renderer-writes record) is out of 3.0.1.
- (2026-09-11) **`0154` + `0185` ruled (1): one scriptable verb, `intent set <address> <field> <value | --from file>`, over `Facade::set`.** The spelling was ruled under the pen because it is the door's existing name (`Facade::set`, `Op::Set`); hv told, and renaming is one table row. There is no new facade code, and nothing gets widened.
- (2026-09-11) **`0145`: the 2026-08-22 rollback test is deleted in the fix commit.** The ruling (no rollback for a refusal after a hydrate) stands. The fix moves the last such refusal ahead of the hydrate, so the ruling has no act left to govern, and keeping the test would pin the defect. **`0172` ruled not workable in 3.0.1** on hv's own 2026-08-30 NARROW, DO NOT BUILD ruling; its reopen condition is unmet.
- (2026-09-11) **`0153` ruled (a): the shared NotAddressable remedy names only forms every door accepts** (a thread id, or a full thread address). Not a door-aware ladder (it would change hv's 2026-08-31 ladder) and not a surface change. edit's ambiguous-id remedy and its `issue` kind are found-whiles, left in the commit message.
- (2026-09-11) **`0146` ruled the narrow half.** The AC-04.6 block lifted at f5b602ef. Refuse only a write that leaves a `file` on a non-test row after the call, judging this call's write and no prior state, once in the Facade path shared by `at edit` and `set`. `--prose` on test rows stays (in deliberate use). No lint rule: the census found 21 live rows across 4 estates, mostly `[n/a` migration residue.
