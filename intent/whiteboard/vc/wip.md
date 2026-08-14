---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T11:40Z
status: active
focus: "v2.19.0 SHIPPED (tag 071c612) and the cut verified; globalfold done. Next: measuring the consumer sweeps, which cc has claimed"
claims: []
---

# Validation Claude (vc)

## DOING

- **v2.19.0 SHIPPED and VERIFIED.** Tag `071c612` on both remotes + GitHub release. Cut verified: five sidecars at 2.19.0, CHANGELOG dated 2026-08-14, tag identical across local/upstream/HEAD, tree clean, release body byte-identical to the CHANGELOG `[2.19.0]` section but for one trailing newline. Globalfold done -- `done.md` flipped to shipped, `wip.md` / `restart.md` / `.claude/restart.md` all on post-ship state.
- **Fifteen issues, 0009-0023.** Four (0020-0023) were called in by hv before the cut rather than deferred after it. That batching is the day's process decision and it is what grew the release from eleven issues to fifteen between the build and the tag.

## TODO

- **Consumer sweeps -- cc has CLAIMED these** (their board and their last inbox entry both say Lamplight first). vc does NOT run them; vc measures. My part: baseline what the old AT rows carry, run the delta after cc's sweep, and refuse to call it good until the counts reconcile. The 87-destroyed-name-links loss happened because a sweep was trusted without being measured against git.
- **`credo_checks/` fleet cleanup** -- issues filed 2026-08-14 in the three affected projects (Baize 0001, Lamplight 0003, Conflab 0008); Laksa + Prolix measured clean, nothing filed. Each project fixes its own; `intent doctor` reports the residue in three states. vc's part is done unless a project asks for verification.
- **Post-tag tidy:** the `# Extract created date for index update` block in `intent_st`'s in-progress arm computes a `CREATED` nothing reads. Dead, not wrong. Recorded in 0020's Resolutions, anchored on the comment because cc caught both of us citing line numbers that expired within a day.
- **Carry to hv:** the plugin bins write errors to STDOUT (named in 0023's Resolutions, deliberately left -- it changes what callers capture, not just what they read), alongside `intent_claude_prime:212`, which is the same decision in miniature.

## Watch-outs

- **`bin/release` runs `intent doctor` + the FULL suite as pre-flight, and that block is NOT behind the dry-run guard** (`bin/release:229-247`). So `--dry-run` costs a full suite run, and must never be fired while another suite is running. The upside is that the cut is self-certifying: it re-runs everything and aborts red.
- **cc went active at 08:24Z in a second session** (new `session_id` on their board). `bin/intent_st` is cc's lane and I have edited it -- told them in their inbox. Commit by explicit pathspec, never `-A`, or cc's board sweeps into my commit.
- **This shell is zsh, and MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal instead of discarding it.** It made `st list` look like it wrote its table to stderr. Measure stream separation by redirecting to a file and counting bytes, never by that idiom. The bats tests run under `bash -c`, where it behaves.
- Trusted on cc's record, not independently re-run: the poisoned-consumer upgrade fixtures and the adapted gitignore-idempotence fixture.
- `intent_claude_prime:212` still prints its truncation notice to STDOUT with a capital prefix -- deliberately left (changing its voice means changing its stream); surface if prime output pollution ever bites.

## Decisions

- (2026-08-14) **A mutation battery can lie, and it lies in the direction of looking thorough.** My first 0020 battery reported that deleting the unplaced pass also broke synonym placement -- impossible, since normalisation is untouched by it. Cause: M1's substitution silently failed to match, so the `&&` chain skipped the restore and M2 ran on a half-mangled file. The result was incoherent on its face, which is the only reason I looked. A mutation must hard-fail when the source is unchanged after substitution, and each one must be applied to a restored file. Same family as cc's usage-text false survival: the probe lied, not the code.
- (2026-08-14) **Reproduce against the unfixed code before believing the fix.** 0020's repro was run in a throwaway worktree at `fae90dc` first -- 1 row of 3, exit 0 -- so the fixed run measured a real delta rather than a plausible one. Worktree, never in place: `~/.local/bin/intent` symlinks into this repo, so an in-place mutation is live for every project on the machine.
- (2026-08-14) **My own residual claims get the same refutation discipline as cc's.** "Residual 1" was mine, survived on two boards for a day, and was wrong -- the mechanism it described had already been removed by the fix it was filed against. Refuted by running, and retired.
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs.
