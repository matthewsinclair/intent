---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-11 11:34Z
status: active
focus: "BACK ON THE BOUNCE after the compact (folded 2026-09-11 11:04Z). hv: no new work; the open defects in intent/wip.md, in 3.0.1 priority order, are the whole of it (`intent issues list` is the live count). vc keeps the list, drives every fix before closing its issue, and holds hv's pen for the ten decisions. Pre-fold board verbatim at .history/20260911/wip-prefold-0914Z.md. RUN THE VERBS; every figure here rots."
claims: [ST0056, ST0057, ST0060, ST0068, ST0070, ST0073]
---

# Validation Claude (vc)

**AGGRESSIVE LOCALFOLD 2026-09-11 09:15Z, `active`.** Pre-fold verbatim at `.history/20260911/wip-prefold-0914Z.md`, `cmp`-verified byte-identical before the first edit. Rulings, hv items, open defects, decisions and the long watch-out record are there and nowhere else.

## DOING

**NOTHING IN FLIGHT ON vc.** Peers, as last reported (re-read each off its own board):

- `dc` -- #12 `0216`, daemon ingest reverting a landed store write. **RULED (vc, 2026-09-11, reversing vc's own earlier "both paths"): the keep rule applies to the daemon's BACKGROUND ingest only, as a `Load` mode on cc's `resync_inner` engine; explicit `sync --to-store` stays `Load::Restore`, unchanged, because ST0056 AC-03.9 (satisfied, AT-03.10) rules the explicit restore declared-destructive.** `sync_direction.rs` tests and AC-03.9's text stay as they are. Commit bar: dc's six-run harness 0 lost in 6 of 6 AND AT-03.10 green; else banked on dc's board. Then #13 `0212`.
- `cc` -- #17 `0226` held until `0216` lands (same collision class). `0268` closed on re-drive; #38 `0097` now in flight. `0100` is back in cc's column.
- `ic` -- #35 `0223` in flight under vc's ruling (Decisions, 2026-09-11 11:34Z). `0194` closed on re-drive. `0146` is back in ic's column.

## TODO -- on the bounce, in this order

1. Keep `intent/wip.md` true: strike an item only when `intent issues list` no longer shows it open.
2. For each item a peer reports fixed: drive the fix myself against the issue's own reproduction, then `intent issues close <id>`. A fix I cannot reproduce as fixed goes back to its lane, not closed.
   **How:** a detached worktree at the fix sha, `cargo build -p intent-cli --bin intent` (and `-p intentd --bin intentd` separately -- one `--bin` filters both packages) into this session's scratchpad `tgt/`, an isolated short HOME (`/Users/matts/.vc<id>h`, removed after), my own fixture rather than the peer's. For an issue whose fix is already in the PATH binary, drive that binary. Then drop the row from `intent/wip.md` (numbers do not shift), `prettier --write`, and commit the row plus `intent/.canon/issues/<id>.json`.
   **Between reports:** search `git log` for fix commits naming a listed id after its filing, and re-drive those. About half of everything closed so far was fixed weeks ago and never closed.
3. Act on each of hv's ten decisions as its word arrives. Items 3, 4 and 5 are code and need hv's explicit go.
4. At the cut: satisfy the nine cut-time rows by evidence, close ST0056 WP-07/11/12, then ST0056, ST0058, ST0068.

**NO NEW WORK.** Nothing gets added to the list. A defect found while fixing goes in the commit message.

## Holds

- None.

## Standing directives from hv

- **`0196` RULED BY hv 2026-09-05 15:23Z, TWO RULINGS, `authority: hv`.** (1) REMEDY: **staging + atomic mv** -- build into a third target dir, `verify_pair` THERE, rename the verified pair into `target/release/` only on pass. (2) SEQUENCING: **fix FIRST, then rebuild through it** -- the fix's own first real run IS the rebuild to HEAD, and **there is no separate supervised window.** Assigned to dc as WP-11's precondition. **THIS RETIRES THE PHRASE `the rebuild window` FROM MY BOARD AND FROM EVERY MESSAGE I HAVE SENT ABOUT IT.**

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **PFIC is _Pure Function, Impure Coordination_** -- deterministic core, I/O at the boundary. NOT the idiom gloss six documents carried until 2026-09-02.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs -- only what bears on working the list

- **Shared checkout.** `git add <paths> && git commit --only <paths>` in ONE call; `--only` does not add untracked files. Never remove a peer's `.git/index.lock`. Never `--no-verify`.
- **The Bash tool is zsh.** Unquoted `$var` does not word-split; an unmatched glob aborts the whole command. Name paths literally.
- **A canon write can report `ok` and be reverted about a second later (`0216`).** Verify past the ingest, never at the `ok`.
- **`sync --to-store` REPLACES the store from the extract.** Edit canon first, drive state verbs after.
- **Count `intentd` by executable**, `ps -axo pid=,command=` on argv[0]'s basename. `pgrep -f` is unsound in both directions.
- **A timestamp goes on a board or in a message only from a `date -u` read in the same turn.**
- **`git status` shows what is dirty, never what was.** Check `git log` for what a command actually did.
- **A test run can rewrite the machine's install pointer.** `dual_path_conformance` runs `intent bootstrap` in-process under the REAL HOME, and `publish_home()` records the TEST BINARY's install -- so a target dir inside a worktree pointed `~/.intent/home` at that worktree, and deleting the worktree made the pre-commit shim refuse every commit in the repo (2026-09-11, ~10:58 local, restored by ic via `intent bootstrap`). Every build and test run, mine and every peer's, goes under an isolated HOME. If commits start refusing with "the recorded Intent install root is not an install", read `~/.intent/home` first.
- **In zsh, a line starting `===` is `=cmd` expansion** and aborts with `== not found`. Use `echo '---'`.

## Decisions

- (2026-09-11) **The work is the list. hv's rule, verbatim: _THERE IS NO NEW WORK TO BE DONE._** Every node, every item.
- (2026-09-11) **dc's lane emptied, so per the list's own rule `0299` moved ic -> dc and `0216`, `0212` moved cc -> dc.** The lane column in `intent/wip.md` is the authority.
- (2026-09-11) **vc stopped dc maintaining the ST0068 AC-02.3 manifest and its disposition check.** A docs item is the page edit plus one commit; a stale quote in that manifest waits for the cut.
- (2026-09-11) **`0194` leak half ruled CLOSE, not send-back.** The hyphen case no longer refuses; a really malformed FTS5 expression is headed in Intent's voice with a remedy, and its `caused by:` chain is the one renderer every refusal uses. Stripping it at one site is the silent-error class. Closed at ba338f37.
- (2026-09-11) **`0223` remedy ruled: refuse a whole-title bare token equal to a subcommand or long-flag name of the verb's own group, unless a literal `--` precedes the title in the argv dispatch already holds.** clap cannot tell `-- help` from `help` (ic, from clap_builder 4.6.6 source), so the check is one positional comparison, not a reparse. Population read from the built Command tree; the refusal writes nothing; one test, seen red first, with the trailing-`--` arm. Not a `--title` flag (new surface); not an outright refusal (the issue protects the single-word title).
