---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-11 13:14Z
status: active
focus: "LOCALFOLD FOR A COMPACT, 2026-09-11 13:14Z, then back on the bounce. hv: no new work; the open defects in intent/wip.md are the whole of it (`intent issues list` is the live count). vc keeps the list, drives every fix before closing its issue, rules fix shapes under the pen, and holds hv's decisions (now 1-14). Pre-fold board verbatim at .history/20260911/wip-prefold-1314Z.md. RUN THE VERBS; every figure here rots."
claims: [ST0056, ST0057, ST0060, ST0068, ST0070, ST0073]
---

# Validation Claude (vc)

**LOCALFOLD 2026-09-11 13:14Z, `active`.** Pre-fold verbatim at `.history/20260911/wip-prefold-1314Z.md`; the 09:15Z fold is `.history/20260911/wip-prefold-0914Z.md`. Executed rulings (0194, 0223, 0195, 0224, 0168) are there, with their evidence in each close commit.

## DOING

**NOTHING IN FLIGHT ON vc.** Peers at the fold, each to be re-read off its own board on return:

- `ic` -- `0154` + `0185` FIXED together at `1f2f8f6a` (`intent set`), WITH vc FOR RE-DRIVE. Then #50 `0139` and the rest of its column.
- `cc` -- `0283` half B verified (fe8775ed); now #77 `0176` (moved from ic). `0100`, `0084` HELD on hv (decisions 2, 11).
- `dc` -- `0283` half A (doctor/organize messages) in flight, then #75 `0150`, then #81 `0172`. `0259` half 2 is in its column. `0220`, `0065` HELD on hv (decisions 13, 14).

## TODO -- on the bounce, in this order

1. **Re-drive `0154` + `0185` at `1f2f8f6a` or later.** Check the ruling's conditions against the build, not ic's report:
   - exactly one of value or `--from`;
   - the address goes through `address_of`;
   - WP body and thread title/objective/context/body are settable, with nothing widened;
   - an unsettable field (`status`) is refused with the tree byte-identical;
   - the value reads back from the STORE and survives a real intentd ingest.
     Red arm: 951cbac2 answers `unrecognized subcommand set`. Close both in one commit.
2. Keep `intent/wip.md` true: strike an item only when `intent issues list` no longer shows it open. The row count must equal the open count.
3. For each item a peer reports fixed: drive the fix myself against the issue's own reproduction, then `intent issues close <id>`. A fix I cannot reproduce as fixed goes back to its lane, not closed.
   **How:**
   - The worktree is `$SP/wth`. Move it with `git -C $SP/wth checkout --detach <MAIN's sha>` -- NOT `HEAD`, which inside the worktree is its own HEAD and a no-op. Build into `$SP/tgt`.
   - Then `cp $SP/tgt/debug/intent $SP/wth/native/rust/target/debug/intent`. A binary outside an install tree cannot find `lib/templates`; `claude upgrade` and `upgrade` need that, and `INTENT_HOME` is not honoured.
   - `intentd` is built separately (`-p intentd --bin intentd`).
   - Use an isolated short HOME (`/Users/matts/.vc<id>h`, removed after) and my own fixture.
   - The version string names the last NATIVE commit, so check the fix is an ancestor rather than expecting its sha.
   - Then drop the row (numbers do not shift), `prettier --write`, and commit the row plus `intent/.canon/issues/<id>.json`.
     **Between reports:** search `git log` full messages (`--grep`, not subjects) for fix commits naming a listed id after its filing.
4. **`0283` half A LANDED at `62f2db60` (dc, rebased onto half B) -- re-drive both halves together on the bounce, then close.** dc's test reads the remedy out of doctor's own line, runs it, and checks doctor is clean; drive that same loop on my own closed-thread fixture. When dc reports `0259` half 2: drive it, then close.
5. **`0176`: cc reports it does not reproduce at HEAD** (the unconditional refusal predates the filing, a7aa0b9e 2026-08-16). Drive it myself: `todo notdone`/`toggle` on a done thread and WP each exit 1 with no row, event or file moved; `st reopen` is the control. Then close. The refusal's remedy fails when followed (positional reason; `st reopen` printed for a WP): put it in the close commit, not a fix.
6. **`0066` FIXED at `f74fbf31` (cc).** Phase A refuses a thread directory one level inside a non-bucket directory (`_inbox/`) as `unknown-file-shape`, naming both remedies. Drive it: a v2 estate with ST0001 flat and ST0002 under `intent/st/_inbox/`, committed; `upgrade` gives rc=1 with a residue line for `intent/st/_inbox/ST0002` and the tree unchanged; after `git mv` to `intent/st/ST0002`, both migrate. RED 951cbac2: `migrated: 1 thread(s)`, and every verb refuses on ST0002 (the loop). Watch for cc's unconfirmed lead: a `config.json edited in the working tree` refusal that `git commit` calls nothing to commit.
7. Act on each of hv's decisions (1-14) as its word arrives. Items 3, 4 and 5 are code and need hv's explicit go. Unblocked by a word: decision 2 frees `0100` (cc, shape ruled); 11 frees `0084` (cc, patch banked); 12, 13 and 14 close or free `0114`, `0220`, `0065`.
8. At the cut: satisfy the nine cut-time rows by evidence, close ST0056 WP-07/11/12, then ST0056, ST0058, ST0068.

**NO NEW WORK.** Nothing gets added to the list. A defect found while fixing goes in the commit message.

## Holds

- **`0283` close** -- both halves are in (B fe8775ed verified; A 62f2db60 not yet driven). Held only until vc re-drives A on the bounce.
- **`0259` close** -- held until dc's half 2 lands (doctor must say when it answers from a store the same run flags stale). Half 1 is verified at d984b077.
- **`0100`, `0084`, `0114`, `0220`, `0065`** -- held until hv answers decisions 2, 11, 12, 13 and 14 respectively. Each row names its decision.

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
