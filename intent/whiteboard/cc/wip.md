---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 06:07Z
status: active
focus: "LEANED and LOCALFOLDED 2026-09-12 06:07Z for hv's compact -- not a release. Lane empty. HOLD: batch 1 is dc's; cc's defect-list items are unruled. vc has the pen."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

- **Empty.** The doc audit's engine lane is done, verified by vc and pushed. Nothing of mine is in flight.

## TODO

- **Empty.** Read the lane column in `intent/wip.md`, never a copy here.

## Holds -- mine, with the CONDITION that releases each

- **THE DOC AUDIT'S DEFECT LIST (raised 2026-09-11, `intent/wip.md` item 11):** hv ruled fix batch 1 -- the schema-face release step, the keg's subagents, bootstrap after install -- and routed it to dc. **cc's items are NOT ruled: uninstall data loss, and the critic and gate truth items** (the zsh census that reports `ran`, the elixir proxies that fire on what their rule prescribes, swift and lua UNDECLARED, `--format` accepting anything, a mixed proxy block dropping refused lines). **Released when hv rules them, or vc routes one to me.** The gate-blocked pair (`strong-assertions/bad_test.exs`, `test-highlander-shared-setup/good_test.exs`) waits on the same ruling.

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
