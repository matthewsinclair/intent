---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T11:12Z
status: active
focus: "Booted after compact, holding for hv. One reply owed to cc (who runs the post-sweep counts); ST0056 is on the tree, unclaimed by me."
claims: []
---

# Validation Claude (vc)

## DOING

- (nothing -- v2.19.0 shipped, verified and folded; day archived to `.history/20260814/`)

## TODO

- **Measure cc's consumer sweep.** cc has CLAIMED the sweeps; vc does not run them. Baseline is `intent/analysis/20260814-lamplight-at-sweep-baseline.md` (Lamplight `15dbccc92`, 1639 AT rows). The delta must satisfy three conditions: row count does not fall; `::name` counts drop only where the name survived into a trailing note; the backticked-reference count does not fall at all. Anything failing 2 or 3 is destroyed data, recoverable from git only if someone looks.
- **`credo_checks/` fleet** -- issues filed in Baize (0001), Lamplight (0003), Conflab (0008); Laksa + Prolix measured clean. Each project fixes its own. vc's part is done unless a project asks for verification.
- **Post-tag tidy:** the `# Extract created date for index update` block in `intent_st`'s in-progress arm computes a `CREATED` nothing reads. Dead, not wrong; recorded in 0020's Resolutions, anchored on the comment rather than a line number.
- **Carry to hv:** the plugin bins write errors to STDOUT (named in 0023's Resolutions and deliberately left -- it changes what callers capture, not just what they read), alongside `intent_claude_prime:212`. And the whiteboard inbox is a pickup-time channel only, so a node asking mid-session cannot see an answer appended to its inbox -- that cost two round-trips today.

## Watch-outs

- **`bin/release` runs `intent doctor` + the FULL suite as pre-flight, NOT behind the dry-run guard** (`bin/release:229-247`). A `--dry-run` costs a full suite run. The upside is the cut is self-certifying; the trap is that the documented `--skip-tests` recovery skips the one gate that certifies HEAD.
- **Write the release docs BEFORE the cut** so the tag carries them. Adopted at v2.19.0; the write-them-after habit is why 2.17.x and 2.18.0 have neither.
- **Commit by explicit pathspec, never `-A`.** cc runs concurrently and its board is frequently dirty in the tree; `bin/release` refuses to tag over anything dirty outside its five sidecars, including a peer's board.
- **This shell is zsh, and MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal** instead of discarding it. It made `st list` look like it wrote its table to stderr. Measure stream separation by redirecting to a file and counting bytes. The bats tests run under `bash -c`, where it behaves.
- Trusted on cc's record, not independently re-run: the poisoned-consumer upgrade fixtures and the adapted gitignore-idempotence fixture.

## Decisions

- (2026-08-14) **A mutation battery can lie, and it lies in the direction of looking thorough.** My first 0020 battery reported an impossible result because a substitution silently failed to match and the `&&` chain skipped the restore, so the next mutation ran on a mangled file. A mutation must hard-fail when the source is unchanged after substitution, and each must be applied to a restored file.
- (2026-08-14) **Reproduce against the unfixed code before believing the fix**, in a throwaway worktree -- never in place, because `~/.local/bin/intent` symlinks into this repo and an in-place mutation is live for every project on the machine.
- (2026-08-14) **A line number in a durable record is a fact with an expiry date.** Both cc and I cited line numbers for the same dead block that were stale within a day. Anchor records on a comment string or a symbol, not a number.
- (2026-08-14) **An alarming number from a one-line grep is a finding to check, not a finding.** My unscoped status scan of Lamplight reported 30+ bad statuses; the scoped pass says 9. Implausibility is the signal to look again.
- (2026-08-14) **My own claims get the same refutation discipline as cc's.** "Residual 1" was mine, sat on two boards for a day, and was wrong -- the mechanism had already been removed by the fix it was filed against.
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs.
