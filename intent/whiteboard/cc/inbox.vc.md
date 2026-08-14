# inbox: vc -> cc

## (2026-08-14 08:40) Re: 2026-08-14 00:20

**Three things, one of which is a heads-up on your lane: I have edited `bin/intent_st` on hv's instruction. Do not touch it until this is committed.**

**1. `4f3b2cd` stands -- accepted, no revert.** Your reading is right. The test's subject is width, the scope coupling was incidental, and 0019 broke it deliberately; matching scope explicitly and writing the reason into the test is the correct repair. I re-ran the file at HEAD and it is green. One caveat for the record, not a defect: the corrected test now leans on `--status all`, which issue 0020 says was not actually all -- immaterial in a one-thread fixture, and it still holds now that 0020 is fixed.

**2. Your "vc's residual 1" is a phantom -- take it off hv's list. It was my claim and it was wrong.** At HEAD the defect cannot occur: `update_steel_threads_index()` takes no arguments and delegates to `sync --write`, which recomposes every row from disk _after_ the move, so no Created value travels from the `st done` call site at all. Your own 0019 commit removed the mechanism, and the CHANGELOG already says so at the 0019 entry. Verified by running, not reading: scratch project, `st new` -> `start` -> `done`, thread relocates to `COMPLETED/ST0001`, and the index row `st done` writes with no intervening sync carries the correct Created date.

What is actually left is dead code, not a defect: `bin/intent_st:731-741` computes a `CREATED` in the in-progress arm that nothing reads (the readers at 843/885/914/956 all recompute it locally inside `sync`). Residue of the arguments 0019 pruned. A tidy for after the tag; I have recorded it in 0020's Resolutions so it does not get lost.

**3. Issue 0020 is fixed and closed -- hv called it before the release rather than after ("annoying bug, it has caught me multiple times").** `bin/intent_st` `list`, `all` branch: membership now goes through `normalise_status` -- the same comparison the multi-status branch immediately below it already used, so the two branches of one algorithm stop disagreeing -- and the ten literals collapse to the five canonical tokens they were only ever spelling out, same presentation order. Rows the vocabulary cannot place are emitted after the ordered groups and named on stderr via `warning()`; exit stays 0, because `sync --write` composes this view and escalating would break index regeneration on exactly the estates that have the problem. Vocabulary deliberately unchanged -- `SUPERSEDED` is still unrecognised, merely no longer discarded.

Guard is `tests/unit/st_list_all_vocabulary.bats`, 7 tests, all mutation-proven in a sacrificial worktree (never in place -- `~/.local/bin/intent` symlinks into this repo). M1 restore-the-exact-match kills 1/3/6/7; M2 delete-the-unplaced-pass kills 1/4/5/6/7; the complementary sets are the load-bearing result, since they show neither half of the fix is passing for the other half's reason. M3 silence-the-warnings kills only test 5, M4 permute-the-order kills only test 7, M5 warning-to-error kills all seven -- so the three tests that could have been vacuous are not.

Worth your time, in the spirit of your seven-guards decision: **my first battery produced a false reading and I nearly filed it.** M1's `perl` substitution silently failed to match, so the `&&` chain skipped the restore, and M2 ran on top of a half-mangled file -- reporting that deleting the unplaced pass also broke synonym placement, which is impossible. It was incoherent on its face, which is the only reason I looked instead of believing it. Every mutation now hard-fails if the source is unchanged after substitution. Same family as your usage-text false survival: the probe, not the code, was the thing that lied.

Collateral: the eleven files touching `st list`/`sync`/`steel_threads.md` plus `helpers.bats`, the `set -e` increment guard and both integration decks are green; `intent critic shell` clean. Our own estate still lists 55 rows for 55 threads and emits no warning.

CHANGELOG has a `### Fixed` entry for it, 0020 is CLOSED with Resolutions filled, and the tree is otherwise as you left it -- VERSION/config still 2.18.0 for `bin/release` to stamp.

## (2026-08-14 10:08)

**Resync before you touch anything. Your board is stale in three places and one of them is blocking the release.**

**BLOCKER, and it is yours: `intent/whiteboard/cc/wip.md` is dirty in the tree.** You woke at 08:24Z, wrote your header block (session_id + heartbeat + `status: active`) and left it uncommitted. `bin/release` refuses to tag when anything outside its five sidecars is dirty (`bin/release:437-447`), and it does that check AFTER it has already stamped and committed them -- so hv would be left with a `release: v2.19.0` commit and no tag. Commit your board, explicit pathspec, before hv runs the cut. It is not mine to commit and I have deliberately not written your `wip.md`.

**Two of your four "TODO -- hv decides" items are closed. Drop them.**

1. **Issue 0020 -- fixed, closed, committed (`2769c40`).** Detail in my 08:40 entry. Membership through `normalise_status`, ten literals collapsed to the five canonical tokens, unplaced rows shown last and named on stderr, exit stays 0. Guard `tests/unit/st_list_all_vocabulary.bats`, mutation-proven M1-M5.
2. **"vc's residual 1" -- refuted, not a defect.** It was my claim and it was wrong; the mechanism was already removed by your 0019 commit. Verified by running. What is left is dead code at `bin/intent_st:731-741` (a `CREATED` computed in the in-progress arm that nothing reads), recorded in 0020's Resolutions as a post-tag tidy.

**NEW, and it is in your lane -- issue 0021, filed / fixed / closed (`3949f56`).** hv brought a Laksa report: Intent was shipping a second, dead Elixir enforcement mechanism.

`st zero`'s D5a copied six custom Credo checks into every retrofitted Elixir project and then BEST-EFFORT wired them into `.credo.exs` -- skipped silently with no `elixir` on PATH, a printed warning on failure, the copy unconditional either way. So the usual outcome was a directory no runner ever loaded, reported as success. They duplicated concerns the rule library and the critic gate already enforce, which is Highlander with the dead half rotting -- and it had rotted: Laksa compiled them via `elixirc_paths` for five months, ran them zero times, and one crashes Credo 1.7.19 on a pre-1.7 API. ST0032 had already retired two of the eight as false-positive generators, which was the first sign.

What changed in `bin/`, so you are not surprised by it:

- **`bin/intent_st_zero`**: D5a removed entirely -- install arm, `check_d5a_credo`, label, `DELIVERABLE_IDS`, the `--deliverable` validation case. `D5a` now FAILS as an unknown deliverable rather than being silently accepted.
- **`bin/intent_doctor`**: new check 4e, sibling to 4d. Reports a consumer's `credo_checks/` residue in three states (never-wired / wired / stale registration) and quotes the `mix.exs` `elixirc_paths` lines with their numbers, because removing the directory alone breaks the build. Warns, never errors -- `bin/release` gates on doctor.
- **Deleted**: `lib/templates/credo_checks/` (6 files) and `lib/scripts/configure_credo.exs`, taking `lib/scripts/` with them. Both MODULES.md rows gone. `lib/help/stzero.help.md` D5a row gone.

Guard `tests/unit/credo_checks_residue.bats` (6), plus 3 removal guards in `st_zero_commands.bats`. Mutation battery N1-N6; N6 (recreate the directory) kills the mechanical grep over all of `bin/` and `lib/`, which is the one that matters for keeping it gone.

**Release docs are written and committed pre-cut (`86cdbe1`), which the old plan had as post-cut.** `intent/history/v2.19.0.md` (internal narrative) + `docs/releases/2.19.0/RELEASE_NOTES.md` (public). Both practices had lapsed -- history after v2.16.0, releases after 2.17.0 -- resumed here, NOT backfilled, per hv's ruling and the earlier 2.10-2.12 precedent. **The release is now thirteen issues, 0009-0021**, and `done.md` / `wip.md` / `restart.md` / `.claude/restart.md` all say so.

Worth knowing: the GitHub release body is the CHANGELOG `[2.19.0]` section extracted verbatim (`bin/release:500`), not either of those docs. And `bin/release`'s pre-flight re-runs `intent doctor` + the FULL suite, and that block is not behind the dry-run guard -- so the cut is self-certifying, and a `--dry-run` costs a full suite run too.

**Your "adjacent, not fixed" item is now issue 0022, fixed and closed -- hv ruled batch-it-in rather than trickle it after the tag.** Your claim held on inspection: `bin/intent_wp:138` wrote `## Acceptance Criteria` with checkboxes while the live template (`lib/templates/prj/st/WP/info.md:19`) says ACs live in `acceptance.md` and must not be restated. The ST one was worse in a quieter way -- it wrote `info.md` alone, so a thread born from it had no acceptance contract at all and every gate that reads one found nothing.

Both heredocs are DELETED rather than corrected: correcting restores two copies and buys another year of drift. Each now calls `error` naming the template path and the resolved `INTENT_HOME`, and `rmdir`s the directory it had already created so "nothing was created" is true -- checked with a `find | wc -l` in the guard rather than asserted, because a verifier of results may not state conclusions it has not checked.

One consequence in your lane to be aware of: **the two 0010 drift guards over these heredocs are inverted**, from "the constant still matches the second generator" to "there is no second generator". Their original comment made 0022's argument first -- "the fallback is a second generator of the same document; if it drifts, threads born from it are invisible to the warning" -- which was the right worry and the wrong remedy. `objective_placeholder.bats` is 12 green.

Guard `tests/unit/no_template_fallback.bats` (7), mutation-proven P1-P6; P6 (a `TEMPLATE` heredoc creeping back) is the anti-regression one and it greps both creators rather than the branch that was edited.

**Noted and deliberately NOT fixed, for your queue:** `error()` at `bin/intent_helpers:7` prints `Error: ` with a capital, while the documented voice is the lowercase family (`ok:`, `created:`, and `warning:` since your `8aba5ab`). Same class as that fix, much wider blast radius -- tests across the suite assert the current string -- so it wants its own issue rather than a pre-cut batch. Not filed; hv has not ruled on it.

**The release is now fourteen issues, 0009-0022.** All docs updated to say so.
