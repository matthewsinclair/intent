---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 12:39Z
status: active
focus: "The lane is complete but for one record: WP-18 and WP-20's integration are closed and cited, WP-23's seams are landed with its two AT rows green, and AC-23.4 waits on hv through vc. Holding after that; vc will say when the quiet window needs suites paused."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

- **ST0069. Everything in cc's lane is landed; one record is outstanding and it is not mine to write.**
  - **WAITING, NOT BLOCKED:** AC-23.4, the Local-runtime decision. Two candidate shapes with their crates are with dc to measure (binary delta only, each crate referenced behind an opaque condition or fat LTO drops it and a zero reads as free) and with vc to put to hv. `wp done ST0069/23` follows the record. **I do not build Local.**
  - **REPORTED, dc's, AND MY CLAIM ABOUT IT CORRECTED:** the bounded-ingest arm. dc measured six runs each way across their own commit -- 3 of 6 red with it, 4 of 6 without -- so it is not their registration. And dc's caveat cuts my claim too: **`alone` means one test TARGET, not an idle HOST**, and my two-of-two was taken on a box three of us were hammering. What I would defend is narrow: at working load, a single-target run reds where this morning it did not, and nothing yet separates "an ingest got slower" from "the box got busier".
  - **THE SUSPICION I OWE dc'S BISECT:** not the rungs. An ingest does not write `src_sections`; what moved on the ingest path is `write_doc_sections`, whose FTS5 `rebuild` now re-derives a content table that also holds the repository's prose. If the range rather than a commit is implicated, that line is mine and it is where to look.
  - **ON hv's LIST, NOT A QUIET ADDITION:** TLS. The HTTP embedder speaks `http://` and refuses `https://` by name; a TLS stack is a dependency with a rationale and it goes to hv beside the Local decision.
  - **QUIET WINDOW, ACKNOWLEDGED, AND IT IS OPEN NOW** (vc corrected the trigger: not WP-22's landing). Until vc lifts it -- **no `cargo test`, no `cargo build`, no drives of any kind on this box.** Store writes and commits of files already built are fine, so `wp done ST0069/23` on 23.4's record is fine.

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
- **A CLAIM NAMES THE ARTEFACT IT EXAMINED AND THE INSTRUMENT THAT READ IT**, and a green counts only beside a red the same instrument produced.
- **A CLOCK VALUE COMES FROM A `date -u` READ IN THIS TURN.** Boards are guarded; messages and file names are not -- I mis-stamped a banked file this morning and renamed it.
- **A PEER CHANNEL'S WRITE IS NOT ITS DELIVERY.** An ask carries its terminating condition, and a CODE-WRONG goes to vc rather than into my own diff.

## Decisions -- unexecuted only

- **None.** Every ruling routed to cc is executed. The lessons that were here are in `.history/20260912/wip-prelean-0606Z.md`, and the two from the doc audit are in the project memory (`feedback_shared_checkout_hazards`, `project_doc_audit_20260911`).
