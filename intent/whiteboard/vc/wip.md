---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-11 17:38Z
status: active
focus: "THE 3.0.1 CUT, vc holds the pen (hv 2026-09-11: 'You have the pen. Do it.' and 'You're in charge ... make the release a reality'). Step 1 LANDED at 7c40da0ab; waiting on dc's prune (step 2). RUN THE VERBS; every figure here rots."
claims: [ST0056, ST0057, ST0060, ST0068, ST0070, ST0073]
---

# Validation Claude (vc)

**LOCALFOLD 2026-09-11 17:18Z for hv's compact, `active`.** The pre-fold board is verbatim in `.history/20260911/`. The list is done: every workable row is closed, and 0177 is the one open issue, post-cut with no owner. hv ruled all 14 decisions and every one is verified (see `intent/wip.md`). What remains is THE CUT.

## DOING -- THE CUT, in this order

**THE ORDER IS LOAD-BEARING. Each step gates the next.**

1. **vc: the 75 AT rows -- LANDED at 7c40da0ab (2026-09-11 17:38Z).** Retired by evidence, rehearsed from a reset sandbox, live canon byte-identical to the rehearsal. The same commit cleared the two preflight blockers: ST0069 WP-13/14/16 (my 3221118cc armed doctor's group check) and ST0064/01 done. Live doctor: 0 findings, rc=0. dc has the sha. `--scope all` withholds one finding, ST0073 AC-05.1 naming a WP-05 the thread lacks; the thread is closed and the preflight's scope doesn't read it. Report it to hv, don't act on it. Also OPEN WITH dc: `bin/.devbin/manifest.sha256` is dirty in the shared tree (only its source_commit header moved), which the release's clean-tree check refuses; settle it before step 5.
2. **dc: the AC-00.6 prune.** One atomic commit, staged in dc's worktree:
   - population A deleted: bin/intent plus the 25 bin/intent_*;
   - 51 v2-door bats files and run_v2_suite.bash deleted;
   - run_tests.sh self-contained, test_helper on v3 (decision 6), tests.yml and pr-checks.yml on v3 (86 paths).
     The gate blocked it ONLY on step 1's rows. dc re-applies, re-gates, runs the release dry-run and lands.
3. **cc, then ic: the 16 mixed bats files, rebased onto dc's prune.**
   - cc: FIX `critic --rules <dir>` (declared, silently reads canon). Its own commit, S, and stop if it is bigger. Then the surgery on its eight (rulings A-G). BANKED: fix at S in `wt-cut` (patch ac006-critic-rules.patch, sha256 d208f18fc63598ea). critic_config 6 ruled: the critic WARNS on an unreadable project config (never refuses: the gate runs in every estate), and the fixture gets a valid config.
   - ic: its eight, rulings R1-R12. R3 FIXES the release script's second SIDECAR_FILES home, with a mandatory dry-run diff. rule_pack_rust is ruled (A): a fence column. BANKED N/N in ic's worktree (localfold e1e579b4); the R3 dry-run is byte-identical before and after. It lands on my word after dc's prune.
4. **vc: the full suite, green, in a private worktree with an in-tree target** (an out-of-tree CARGO_TARGET_DIR fakes 36 failures). Rust was 2353/0 at b9fdf0f4. Shell: run_tests.sh must be green. Then dc regenerates the WHOLE docs/reference at that sha (ST0068 AC-04.2).
5. **hv runs `! bin/devbin build release --patch`.** Its confirm gate stays human; never --no-confirm. It stamps 3.0.1, commits, tags, pushes both remotes and creates the GitHub release. The CHANGELOG reads `## [3.0.1] - in progress`, and the script dates it.
6. **Artefacts:**
   - `bin/devbin build all` at the tag;
   - `int macos prepare` (stage, sign, notarize, checksum; creds PRESENT per `int macos doctor`);
   - `int macos formula`;
   - `int macos publish` (hv's word covers it);
   - `int macos smoke --reinstall`.
     CI must be green on both legs.
7. **Close out:**
   - satisfy the nine cut-time ACs by evidence (ST0056 AC-00.5/00.6/07.7/11.1/11.4/12.1/12.4, ST0058 AC-00.1, ST0068 AC-04.2) and mark their ATs;
   - judge ST0058 AC-00.3 (every canon-mandated verb works);
   - close ST0056 WP-07/11/12, then ST0056, ST0058 and ST0068;
   - push; then a globalfold.

## Holds

- None. Nodes hold only for the compact.

## Watch-outs (added 2026-09-11)

- **TESTS REACH LIVE MACHINE STATE.** Twice today:
  - an intentsvcs test opened the LIVE store via doctor (store 17 -> 18, 15:10Z; d0777bc8);
  - cc's `cargo test -p intent-cli` under the real HOME ran `intent bootstrap`, which republished ~/.intent/home to cc's worktree (15:34Z; restored by vc at 17:07Z with `intent bootstrap`).
    Every suite runs in a private worktree under an isolated HOME. Check ~/.intent/home names /Users/matts/Devel/prj/Intent after any peer run.
- **A parent build cannot read a newer build's fixture** (store 18 vs 17). Build each arm's fixture with its own binary.
- **An out-of-tree CARGO_TARGET_DIR makes intent-cli tests fail** with 'cannot locate the Intent install'. It is the setup, not the code.

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
