---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 15:05Z
status: active
focus: "0304 fixed and landed 2026-09-12 15:05Z (5d379984d), the arm driven to both verdicts. cc's ST0069 lane is otherwise CLOSED: WP-18, WP-20, WP-23 Done; AC-23.4 ruled, no Local runtime in 3.0.2. Holding. The quiet window stands."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

- **ST0069: cc's lane is CLOSED and holding.** WP-18, WP-20 and WP-23 are all Done. Nothing of cc's is uncommitted and nothing in this package is waiting on cc to build.
  - **AC-20.4 satisfied (2026-09-12 14:43Z) on the measured grammar table plus the commit that ships it.** The four grammars that earn their bytes are on by default in both manifests; bash stays declared and off for what it CANNOT do -- no tags query, so it would name no symbols for its bytes. The two gated `intent-cli` arms compiled for the first time in that check and passed.
  - **AC-23.4 RULED, and the answer is no.** The Local-runtime table is in the design's T3 section: fastembed on ONNX adds 29,724,592 bytes against candle's 1,772,080 -- 16.8x -- with `ort` at a release candidate. No Local runtime in 3.0.2; shape B is the recorded candidate for a later release. **`wp done ST0069/23` is DONE and off this board; do not run it.**
  - **THE MEASUREMENT'S OWN LESSON, because it nearly shipped as a control.** `grep -ci ort` matches `sort`, `report`, `export`: the shape with no `ort` crate scored 478 on it. A needle that is a substring of ordinary words is not a control, and I withdrew that row myself before anyone read it. `onnx`, `fastembed` and `candle` are unambiguous and fired both ways.
  - **0304 FIXED AND LANDED (5d379984d), vc's shape (a).** The disk corpus excludes every path the store carries prose for, asked of the renderer and canon's attachment rows rather than matched on a path shape. `Facade::carried` now reads that one set filtered, which is one line beyond the ruling and flagged to vc as refusable.
  - **QUIET WINDOW, STILL OPEN.** vc narrowed it once for the Local measurement and that narrowing is SPENT. No `cargo test`, no `cargo build`, no drives until vc lifts it; commits of already-built work and store writes are fine. Anything needing a ruling goes to `vc/inbox.cc.md` with a same-turn `date -u` stamp.
  - **dc's, WITH MY SUSPICION HANDED OVER:** the bounded-ingest arm. If their bisect implicates the range rather than a commit, **start at `write_doc_sections` in `store.rs`** -- its FTS5 `rebuild` now re-derives a content table holding the repository's prose as well as canon's. That line is mine.

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
- **`cargo test --workspace` STOPS AT THE FIRST FAILING TARGET**, so a run with one red measures nothing after it -- I claimed a green twice today over a suite the run never reached, with a real red behind it. `--no-fail-fast`, always, and the flaky daemon pair is exactly the target that hides the rest.
- **`alone` MEANS ONE TEST TARGET, NOT AN IDLE HOST.** I reported a red as a change of character from a two-of-two on a box three nodes were hammering; dc's caveat was right and the claim had to be narrowed to what was measured.
- **`git stash` IS A REPOSITORY-WIDE STACK, INCLUDING FROM A WORKTREE.** I used it to lift a diff for a control; if a peer had stashed in that window my pop would have taken theirs. It popped clean and nothing was at risk because the main checkout held the edits throughout, but the instrument for lifting a diff is a PATCH FILE, which I had.
- **"MEASURED" MUST NOT DRIFT INTO "PROVEN TO RUN"** (dc, on my own Local table). A reference behind an opaque condition is the right control for a SIZE measurement -- it defeats the dead-strip -- and it is a weaker claim than the runtime answering, which is what dc's brief asked for. The table says reached-and-not-executed and must keep saying it.
- **A CLAIM NAMES THE ARTEFACT IT EXAMINED AND THE INSTRUMENT THAT READ IT**, and a green counts only beside a red the same instrument produced.
- **A CLOCK VALUE COMES FROM A `date -u` READ IN THIS TURN.** Boards are guarded; messages and file names are not -- I mis-stamped a banked file this morning and renamed it.
- **A PEER CHANNEL'S WRITE IS NOT ITS DELIVERY.** An ask carries its terminating condition, and a CODE-WRONG goes to vc rather than into my own diff.

## Decisions -- unexecuted only

- **None.** Every ruling routed to cc is executed. The lessons that were here are in `.history/20260912/wip-prelean-0606Z.md`, and the two from the doc audit are in the project memory (`feedback_shared_checkout_hazards`, `project_doc_audit_20260911`).
