---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T09:30Z
status: active
focus: "0020 + 0021 fixed and closed pre-cut on hv's instruction, release docs written; tree ready for bin/release --minor once cc's board is committed"
claims: []
---

# Validation Claude (vc)

## DOING

- **Holding for the cut.** 0020 and 0021 both fixed, guarded, closed and committed pre-cut on hv's instruction; release docs written (`intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`, both practices had lapsed and are resumed not backfilled). Release is thirteen issues, 0009-0021. **Blocker for hv: `intent/whiteboard/cc/wip.md` is dirty and `bin/release` refuses to tag a tree dirty outside its five sidecars.** Not mine to commit.
- **0021 (credo_checks) is the one to watch on the sweeps.** Intent shipped a second Elixir enforcement mechanism that duplicated the rule library and rotted unrun; the shipping side is deleted and consumers get a three-state `intent doctor` report that names the `elixirc_paths` end too. Every Elixir consumer wants `intent doctor` run after upgrading.

## TODO

- **Fire on the cut.** `bin/release --minor`, interactive, NEVER `--no-confirm`. Verify after: five sidecars at 2.19.0, CHANGELOG dated, tag on both remotes, GitHub release body == the CHANGELOG section, tree clean.
- **Post-cut narrative:** `intent/history/v2.19.0.md` + done.md flips to shipped + tag. hv confirmed: v2.19.0.md only, no backfill of the lapsed 2.17/2.18 (same ruling as the 2.10-2.12 lapse).
- **Post-cut estate sweeps**, Lamplight first (`intent upgrade` converges AT grammar via `at lint --fix` -- 314 rows, expect BLOCKED-until-swept, residue named never guessed -- plus AGENTS.md, settings hooks, gitignore, printed-never-run treeindex `git rm`). vc's part is MEASUREMENT: count what the old rows carried against what the new ones do, before trusting the sweep. Utilz / Baize follow.
- **Post-tag tidy:** `bin/intent_st:731-741` computes a `CREATED` in the in-progress arm that nothing reads -- residue of the arguments 0019 pruned. Dead, not wrong. Recorded in 0020's Resolutions.

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
