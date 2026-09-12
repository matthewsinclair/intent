---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-12 08:24Z
status: active
focus: "Directing v3.0.2, which carries ST0069 (hv). Batch 4 silent deletion landed on all lanes and verified. In flight: dc's watcher repair (the daemon-pair cause) then a mid-course rehearsal and the grammar measurement; ic building WP-17 on an approved shape; cc shaping WP-18. Cut is hv's call after the last search package."
claims: [ST0056, ST0057, ST0060, ST0070]
---

# Validation Claude (vc)

## DOING

- **Directing v3.0.2, which carries ST0069 on hv's ruling.** Batch 4 (silent deletion) is landed on all three lanes and verified by file list; the gate test covers every verb in the sweep; the calls made under the pen are listed in `intent/wip.md` for hv to overrule. In flight: dc repairs the watcher defect behind the daemon pair (a directory event is answered by reconciling), then a mid-course `--dry-run` rehearsal, then WP-20's grammar-size measurement; ic builds WP-17 on the shape I approved; cc brings me WP-18's shape before code. I verify every landing by file list against its report and rule every shape before code. Then the remaining search packages in the design's order, dc's final rehearsal on the last HEAD, and the cut on hv's go with the confirm question asked specifically; after the tag, ic's one reference regeneration and the tap publish as its own action.

## TODO -- waiting on hv, nothing claimable

- **ST0056 AC-00.5 and AC-11.1** still need a brew install on a clean Mac. Then WP-11 and ST0056 close.
- **hv's rulings** on the cut's surfaced items and on the audit's defect list, both in `intent/wip.md`.

## Holds

- None. The hydrate that was held for hv landed as a proven banner-only patch from a scratch clone (8a2f2273a); doctor reports nothing at live scope and organize's preview removes nothing.

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

- (2026-09-11) **hv's doc audit ran and is DONE**, every lane verified and pushed; its orders are in `intent/history/20260911-doc-audit.md`. **hv then ruled fix batch 1 (release + install) to dc**; the rest of the defect list in `intent/wip.md` is unruled. The 3.0.1 rulings this section carried, all executed in the cut, are in `.history/20260912/`.

- (2026-09-11) **NOT WORKABLE IN 3.0.1, kept open as constraints, lane `--`:**
  - `0177` -- all of `ext` is unbuilt;
  - `0141` -- no instance today, and the only fix is a guard.
    Closing either would lose the finding.

- (2026-09-11) **`0084` put to hv as decision 11, recommending (a), the byte write for opaque attachments.** vc drove the migrator hole: a Latin-1 `notes.txt` migrates with no sidecar, and the next restore refuses `broken-reference`.

- (2026-09-11) **`0114`, `0220` and `0065` put to hv as decisions 12, 13 and 14.** Each issue reserves its question for hv, or needs a policy value.
