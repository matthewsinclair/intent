---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-15 09:32Z
status: active
focus: "HOLDING for hv, wrapped for the day 2026-09-14 22:04Z: the pass at c4c4880c2, 70 closed today, 11 open (+1 filed at the wrap), every lane released and holding, main clean and green. Next session's order in restart.md line 3 and wip.md. NO RELEASE, NO PUSH."
claims: [ST0056, ST0057, ST0060, ST0070]
---

# Validation Claude (vc)

## DOING

- THE AGGRESSIVE ISSUE PASS (hv, 2026-09-14), wrapped for the day at c4c4880c2 with every node holding: 70 closed today, 11 open plus the st show file-argument issue filed at the wrap. Next session, one lane on the tree AND the store at a time on vc's word: cc's matched control on the index event-wait arm then 0338 (ii) banked on d2491639b, 0338 (i), 0377, 0331 comments, 0375 (directive kind); ic's 0334 (ruled in full), 0339, 0396; then the quiet-window build all, daemon restart, app-install and the one view re-render commit on hv's word. NO RELEASE, NO PUSH.

## TODO

- **ST0056 AC-00.5 and AC-11.1** still need a brew install on a clean Mac. Then WP-11 and ST0056 close.
- **hv's rulings** on the cut's surfaced items and on the audit's defect list, both in `intent/wip.md`.
- hv's items at the wrap: 0321's ruling (vc's lean (c)); the rulings under the pen (0303 0347 0350 0374 0375 0382 0389, decisions 16 and 17, 0334's readings, the store rule); 0177, 0331's six deletions, 0338's contract rows, 0345, 0344; brew pin; the v3.0.2 annotation; the stranger-machine install; ic's explorer TUI list.

## Holds

_(none)_

## Watch-outs

- **`git stash` is repository-wide across every worktree of one repo**, so a stash in a worktree can pop a peer's; two nodes met it in one day. A patch file is the instrument for lifting a diff. **A TODO on a board is a pointer, not the subject**: dc nearly re-ran a settled measurement from a stale row; the register and the design are the subject. **A socket message is not a delivery**: three rulings went unregistered by a busy peer; the durable inbox is the record.
- **Results come back as a patch, never a whole-file copy.** **A cost measurement is not a consequence measurement.** **Every drive is `--no-fail-fast`.** **"Alone" is one target, not an idle host; this host has no idle (floor above ten from its own daemons).** **A claim wider than the thing is a defect.** **Fix a fixture that discards what it is handed, never the assertion.** **Two defensible rulings can loop when they meet; the second measurement decides.**
- **Diff a shared aggregator before `git add`**; a registration lands with its file. **In-tree target dir** in a worktree, never an out-of-tree CARGO_TARGET_DIR. **The register never meets prettier**; its markdown regenerates in the same commit. **`${=var}` to split a list in this shell.** **A refused destructive write goes through a scratch clone and a proven patch, never to a peer.** **Canon carries every attachment's text**, so dehydration loses nothing.
- **TESTS REACH LIVE MACHINE STATE.** Twice today:
  - an intentsvcs test opened the LIVE store via doctor (store 17 -> 18, 15:10Z; d0777bc8);
  - cc's `cargo test -p intent-cli` under the real HOME ran `intent bootstrap`, which republished ~/.intent/home to cc's worktree (15:34Z; restored by vc at 17:07Z with `intent bootstrap`).
    Every suite runs in a private worktree under an isolated HOME. Check ~/.intent/home names /Users/matts/Devel/prj/Intent after any peer run.
- **A parent build cannot read a newer build's fixture** (store 18 vs 17). Build each arm's fixture with its own binary.
- **An out-of-tree CARGO_TARGET_DIR makes intent-cli tests fail** with 'cannot locate the Intent install'. It is the setup, not the code.
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
- THE STORE LOCK IS SHARED LIKE THE GIT INDEX. During a peer's landing chain no other lane runs any intent wb write, pickups included; a refused write gets ONE re-issue after lsof -- intent/.cache/intent.db lists intentd alone, never a retry loop (two lanes' loops refused ic's close three times on 2026-09-14). After a landing commit intentd's ingest holds the lock for tens of seconds and lsof cannot see it: the next chain waits for the daemon to go cool. Attribute a holder only by lsof on THIS store's path; a machine-wide ps shows every estate's wb processes (a laksa-cc pickup was read as ours). The word names the store as well as the tree.

## Decisions

- **NO OVERTESTING, NO YAK-SHAVING** (2026-09-12, on handing vc the pen): build what the STs need; tests are the AC rows; never a test that tests a test. Applies to every lane and to the director.
- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **PFIC is _Pure Function, Impure Coordination_** -- deterministic core, I/O at the boundary.
- (2026-09-12) **hv handed the pen over without ruling the six ST0069 decisions, so they are taken under it and listed in `intent/wip.md` for hv to overrule with a line:** four grammars on by default; omnibox out; `intent modules find` stays as the registry fallback (AC-20.6 withdrawn naming AC-24.5); the Local runtime ruled on cc's table with the lean being no new runtime in 3.0.2; ST0073's AC-05.1 re-keyed to AC-00.1; no TLS for the HTTP embedder. hv's own rebuild order the same hour was executed by vc: the delivered pair at 92df64d, the daemon restarted on it, the app reinstalled.
- (2026-09-11) **NOT WORKABLE IN 3.0.1, kept open as constraints, lane `--`:**
  - `0177` -- all of `ext` is unbuilt;
  - `0141` -- no instance today, and the only fix is a guard.
    Closing either would lose the finding.
- (2026-09-13) hv, verbatim: NO UNNECESSARY OVERTESTING, NO TESTING TESTS, NO YAK-SHAVING. Expeditious delivery of working code; do not relitigate the speed of light for every decision; be RUTHLESS. New code, good code, good tests, done. Applied: build the cheap fix and let the live system judge it; one run decides a question; never a positive control of an instrument; ACs one line per user-facing behaviour, ATs the test that proves it once.
- (2026-09-14) EIGHTH RULING UNDER THE PEN, for hv to overrule with a line: a register State row for Criterion.kind (0346, d9304ef54) and for AcceptanceTest.kind (ic's 0324+0337) is a faithful transcription of hv's 2026-08-17 fold (one machine, kind a dimension of its state, a re-kind the pair's transition into the entry state), because the register classifies fields and has no closer word; each row's comment cites the fold by name and data-model.md's as-built column is corrected forward in the same landing. The other seven: 0303 0347 0350 0374 0375 0382 0389.
- (2026-09-14) NINTH RULING UNDER THE PEN, for hv who ratified the table: data-model.md's Machine 5 (AcceptanceTest.status) moves its green edge from (any) -> green to red -> green under a dated note naming 0337 and the batch ruling (red first); at.set is a mixed verb walked edge by edge. HELD FOR hv, NOT RULED: 0321's log stamp collides with one_clock (SELECT strftime is banned outside INSERT/UPDATE, EXEMPT is hv's and empty); cc's options (a) INSERT RETURNING into a one-row memory table, (b) a real lifecycle table with a DEFAULT stamp (M, schema bump), (c) an exemption for the daemon log with a stated reason, (d) no stamp; vc's lean (c).

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
