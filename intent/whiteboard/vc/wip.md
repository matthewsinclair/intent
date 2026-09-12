---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-12 19:46Z
status: active
focus: "Directing ST0069 to the end (hv 2026-09-12): WP-14 is the last package. Live store at 24 on the pair at 82b85c5e1; the doctor gate is live for every estate. Live now: cc lifecycle then AC-14.2/14.7/14.8, dc the migration verb, ic 0311 by trace then the protocol half on my signal; then the cutover, wp done, the cut at hv's terminal."
claims: [ST0056, ST0057, ST0060, ST0070]
---

# Validation Claude (vc)

## DOING

- **Directing ST0069 to the end (hv, 2026-09-12); state as at 2026-09-12 19:46Z.** The state and every ruling are in `intent/wip.md`. WP-14 is the last package. Orders out: cc lifecycle (`touch`, `release`, `pickup` composite, archival as the bounded write's own step) then AC-14.2, 14.7, 14.8 with AT rows; dc the migration verb against the fixture board; ic the items-row review, 0311 by trace with a matched control, then the protocol half on my signal once lifecycle is in. Next, in order: each landing verified by file list; the pair rebuilt at any Rust landing that changes what the live estate sees; the cutover when every node has folded, boards migrated by dc's verb, both guards retired, the views switched; `wp done ST0069/14`; then the cut at hv's terminal with every gate on, `build all` at the tag, the macOS prepare pass, ic's regeneration with --baseline v3.0.1, the tap publish as its own approval, the smoke. Standing: every drive traps EXIT and stops its daemon, every fold ends with a process sweep; hook and guard work in worktrees only; every replace asserts its match count; a socket send that fails is retried once and then written to the inbox.
- **The calls taken under the pen, seven in the morning and four in the afternoon, are one bullet in `intent/wip.md`** for hv to overrule with a line; AC-24.6 and AC-24.7 are hv's to rule and not the pen's.

## TODO -- waiting on hv, nothing claimable

- **ST0056 AC-00.5 and AC-11.1** still need a brew install on a clean Mac. Then WP-11 and ST0056 close.
- **hv's rulings** on the cut's surfaced items and on the audit's defect list, both in `intent/wip.md`.

## Holds

- None. The hydrate that was held for hv landed as a proven banner-only patch from a scratch clone (8a2f2273a); doctor reports nothing at live scope and organize's preview removes nothing.

## Watch-outs (added 2026-09-12, afternoon)

- **`git stash` is repository-wide across every worktree of one repo**, so a stash in a worktree can pop a peer's; two nodes met it in one day. A patch file is the instrument for lifting a diff. **A TODO on a board is a pointer, not the subject**: dc nearly re-ran a settled measurement from a stale row; the register and the design are the subject. **A socket message is not a delivery**: three rulings went unregistered by a busy peer; the durable inbox is the record.
- **Results come back as a patch, never a whole-file copy.** **A cost measurement is not a consequence measurement.** **Every drive is `--no-fail-fast`.** **"Alone" is one target, not an idle host; this host has no idle (floor above ten from its own daemons).** **A claim wider than the thing is a defect.** **Fix a fixture that discards what it is handed, never the assertion.** **Two defensible rulings can loop when they meet; the second measurement decides.**

## Watch-outs (added 2026-09-12)

- **Diff a shared aggregator before `git add`**; a registration lands with its file. **In-tree target dir** in a worktree, never an out-of-tree CARGO_TARGET_DIR. **The register never meets prettier**; its markdown regenerates in the same commit. **`${=var}` to split a list in this shell.** **A refused destructive write goes through a scratch clone and a proven patch, never to a peer.** **Canon carries every attachment's text**, so dehydration loses nothing.

## Watch-outs (added 2026-09-11)

- **TESTS REACH LIVE MACHINE STATE.** Twice today:
  - an intentsvcs test opened the LIVE store via doctor (store 17 -> 18, 15:10Z; d0777bc8);
  - cc's `cargo test -p intent-cli` under the real HOME ran `intent bootstrap`, which republished ~/.intent/home to cc's worktree (15:34Z; restored by vc at 17:07Z with `intent bootstrap`).
    Every suite runs in a private worktree under an isolated HOME. Check ~/.intent/home names /Users/matts/Devel/prj/Intent after any peer run.
- **A parent build cannot read a newer build's fixture** (store 18 vs 17). Build each arm's fixture with its own binary.
- **An out-of-tree CARGO_TARGET_DIR makes intent-cli tests fail** with 'cannot locate the Intent install'. It is the setup, not the code.

## Standing directives from hv

- **NO OVERTESTING, NO YAK-SHAVING** (2026-09-12, on handing vc the pen): build what the STs need; tests are the AC rows; never a test that tests a test. Applies to every lane and to the director.
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

- (2026-09-12) **hv handed the pen over without ruling the six ST0069 decisions, so they are taken under it and listed in `intent/wip.md` for hv to overrule with a line:** four grammars on by default; omnibox out; `intent modules find` stays as the registry fallback (AC-20.6 withdrawn naming AC-24.5); the Local runtime ruled on cc's table with the lean being no new runtime in 3.0.2; ST0073's AC-05.1 re-keyed to AC-00.1; no TLS for the HTTP embedder. hv's own rebuild order the same hour was executed by vc: the delivered pair at 92df64d, the daemon restarted on it, the app reinstalled.
- (2026-09-12) **A watcher event naming the project root reconciles the root's own in-scope files and never descends**; a directory event reconciles the subtree it names; the pre-dating rule holds inside whatever an event names. dc attributed the loop four-of-four with and none without: a coalesced root event answered by a whole-corpus reconcile against a lagging index published everything, ingested, rewrote views and coalesced to the root again. Then measured again with the bound: the loop narrowed to exactly the root files an ingest rewrites, so the widening was on the wrong watcher. **Ruled two registrations**: the canon watch byte-for-byte as the tests pin it (root non-recursive, `intent/` recursive, leaf events for root files), and a second registration over the index scope whose events reach only the index reconcile and `index_file`, never canon ingest; the root bound applies on the index side. Per-directory registration and a lowered ingest bound were rejected.
- (2026-09-12) **D29 amendment WITHDRAWN**: the corpus is the committed `.gitignore` rules; `.git/info/exclude` and the global excludes stay off, and `ignored_paths_corpus.rs` is the record. **`intent index status` and `rebuild`**: cc owns the facade operations, ic the register rows, rendering and MCP arms. **Watcher reconcile**: the store's index is the one baseline; a directory event reconciles the scan's own candidates under the event's path, never a second walk of the root.

- (2026-09-11) **hv's doc audit ran and is DONE**, every lane verified and pushed; its orders are in `intent/history/20260911-doc-audit.md`. **hv then ruled fix batch 1 (release + install) to dc**; the rest of the defect list in `intent/wip.md` is unruled. The 3.0.1 rulings this section carried, all executed in the cut, are in `.history/20260912/`.

- (2026-09-11) **NOT WORKABLE IN 3.0.1, kept open as constraints, lane `--`:**
  - `0177` -- all of `ext` is unbuilt;
  - `0141` -- no instance today, and the only fix is a guard.
    Closing either would lose the finding.

- (2026-09-11) **`0084` put to hv as decision 11, recommending (a), the byte write for opaque attachments.** vc drove the migrator hole: a Latin-1 `notes.txt` migrates with no sidecar, and the next restore refuses `broken-reference`.

- (2026-09-11) **`0114`, `0220` and `0065` put to hv as decisions 12, 13 and 14.** Each issue reserves its question for hv, or needs a policy value.
