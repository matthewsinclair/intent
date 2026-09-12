---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 10:24Z
status: active
focus: "ST0069 WP-18 is built to its last step: scope, survey, exclusions, staleness, rungs 19 and 20, `index_file`, and the two facade doors, all landed and green. The watcher widening is the only thing left and its engine half is landed; the registration and the dispatch are put to dc."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

- **ST0069 WP-18, the corpus. Everything but the watcher is built and landed; the last step is cross-lane.**
  - **OPEN, WAITING ON dc:** the registration and the dispatch in `watch.rs`. I proposed the split rather than edit the file dc has just reworked -- engine side mine and landed, daemon side theirs. AC-18.6 wants the registration strategy recorded per platform WITH the measurement that chose it, and **the inotify leg cannot be measured on this machine**; my read is to state that limit rather than claim a number from a directory count, and it is dc's call.
  - **OPEN, ic's:** the register row, the rendering and the MCP arm for `index status`. The facade shape is sent. The one thing the rendering must not do is collapse the skipped paths into a count -- a count is silence with a number on it.
  - **NOT BUILT AND NOT CLAIMED:** nothing indexes content, so `indexed_sha256` is unset on every row. The column is preserved rather than overwritten by a writer that has no opinion about it, so the content indexer can land without the reconcile erasing it.
  - **NEXT AFTER THE WATCHER:** the AT rows. AC-18.1 and AC-18.4 are not closable until a hit can be asked for, so the rows wait for the surface rather than being minted against arms that prove the scope alone.

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.

## Holds -- mine, with the CONDITION that releases each

- **RELEASED 2026-09-12:** the D29 pause. vc withdrew the amendment at `2866a40ed`; the corpus is the committed `.gitignore` rules and `ignored_paths_corpus.rs` is the record. The code never moved, so nothing had to be unwound.

- **THE DEFECT LIST ITEMS hv DID NOT RULE.** hv ruled batches 2 and 3 to cc on 2026-09-12 06:16Z and both are landed. **The mixed-proxy silent drop stays held:** vc's two briefs disagreed and vc settled it -- the first stands, so it is not batch 3's and waits with item 9. **Still held, unruled:** the rule proxies that contradict their own rule (item 9) -- the elixir proxies firing on what their Good prescribes, swift and lua UNDECLARED -- and the usage-error exit code, which is dc's to rule. **Released when hv rules them, or vc routes one to me.** The gate-blocked pair (`strong-assertions/bad_test.exs`, `test-highlander-shared-setup/good_test.exs`) is part of item 9 and waits with it.

- **POST-CUT (culled from the 3.0.1 loop 2026-09-11):** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; none is 3.0.1 work. **Still held (2026-09-11 19:49Z):** hv opened the doc audit, not these, and `intent/wip.md` lists `0177` as post-cut with no owner.

## Watch-outs

**LEANED 2026-09-12 06:07Z ON hv's INSTRUCTION.** One line each, and only what bears on work in front of cc. The families whose subject is gone or which a guard now enforces -- population and denominator, one name two artefacts, the unfelt claim, the reachable-but-unread artefact, canon past the daemon's ingest, folding this board, counting a migration's blast radius -- are in `.history/20260912/wip-prelean-0606Z.md` with their full text. Read them there before arguing with one.

- **THE SHARED CHECKOUT.** `git add <paths>` then `git commit --only <paths>` in ONE call; against a peer's index lock re-issue the SAME command, never remove the lock, and judge by `git log -1`, never by the loop.
- **BUILDS AND SUITES.** Only from a private worktree's own in-tree build under an isolated `HOME`; read back `~/.intent/home` and the live store's mtime afterwards, because a test run deploys to the estate it lives in.
- **THE GATE.** Never `--no-verify`. Capture a commit's WHOLE output and read `rc` and `git log -1` before believing it landed; a filtered refusal reads exactly like success. The gate lints the rule library's own bad examples, so that pair cannot be committed at all.
- **THIS SHELL IS ZSH.** Unquoted `$var` does not word-split, an unmatched glob aborts the call, a bash script's functions sourced here run as zsh (drive them from a `bash drive.sh`), and an exit code that IS the finding never goes through a pipe.
- **A CLAIM NAMES THE ARTEFACT IT EXAMINED AND THE INSTRUMENT THAT READ IT**, and a green counts only beside a red the same instrument produced.
- **A CLOCK VALUE COMES FROM A `date -u` READ IN THIS TURN.** Boards are guarded; messages and file names are not -- I mis-stamped a banked file this morning and renamed it.
- **A PEER CHANNEL'S WRITE IS NOT ITS DELIVERY.** An ask carries its terminating condition, and a CODE-WRONG goes to vc rather than into my own diff.

## Decisions -- unexecuted only

- **None.** Every ruling routed to cc is executed. The lessons that were here are in `.history/20260912/wip-prelean-0606Z.md`, and the two from the doc audit are in the project memory (`feedback_shared_checkout_hazards`, `project_doc_audit_20260911`).
