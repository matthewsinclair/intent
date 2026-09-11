---
node: vc
name: Validation Claude
role: validation
session_id: e089236a-72ea-4b23-87e7-c318ef8f0ac5
heartbeat_at: 2026-09-11 09:27Z
status: active
focus: "ON THE BOUNCE, 2026-09-11 09:27Z. hv: no new work; the 92 open defects in intent/wip.md, in 3.0.1 priority order, are the whole of it. vc keeps the list, drives every fix before closing its issue, and holds hv's pen for the ten decisions. Pre-fold board verbatim at .history/20260911/wip-prefold-0914Z.md. RUN THE VERBS; every figure here rots."
claims: [ST0056, ST0057, ST0060, ST0068, ST0070, ST0073]
---

# Validation Claude (vc)

**AGGRESSIVE LOCALFOLD 2026-09-11 09:15Z, `active`.** Pre-fold verbatim at `.history/20260911/wip-prefold-0914Z.md`, `cmp`-verified byte-identical before the first edit. Rulings, hv items, open defects, decisions and the long watch-out record are there and nowhere else.

## DOING

**NOTHING IN FLIGHT.** The work list is `intent/wip.md`. Waiting on the bounce.

## TODO -- on the bounce, in this order

1. Keep `intent/wip.md` true: strike an item only when `intent issues list` no longer shows it open.
2. For each item a peer reports fixed: drive the fix myself against the issue's own reproduction, then `intent issues close <id>`. A fix I cannot reproduce as fixed goes back to its lane, not closed.
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

## Decisions

- (2026-09-11) **The work is the list. hv's rule, verbatim: _THERE IS NO NEW WORK TO BE DONE._** Every node, every item.
