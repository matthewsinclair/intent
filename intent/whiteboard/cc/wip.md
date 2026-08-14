---
node: cc
name: Control Claude
role: control
session_id: 76c0e702-e5b4-4cdc-9bd4-f12ea5965985
heartbeat_at: 2026-08-14T10:29Z
status: active
focus: "v2.19.0 is fourteen issues (0009-0022) and ready to cut. My board was the last thing dirtying the tree, which is what blocks the tag. Post-tag: consumer sweeps, Lamplight first."
claims: []
---

# Control Claude (cc)

## DOING

- **Commit this board, explicit pathspec, my own dir only** -- awaiting hv's go. `bin/release` runs its leftover-dirt check AFTER stamping and committing the five sidecars (`:437-447`), so anything left dirty costs hv a `release: v2.19.0` commit with no tag. (Flagged by vc 10:08; ordering independently confirmed.)
- **This commit is the last thing between HEAD and the tag.** 0023 landed at `e1e2300` and closed; vc confirms nothing is held open and the engine is mine again after the tag. 44+ commits unpushed to both remotes, so the tag carries the whole arc.
- **Sent to vc before the cut: `intent/wip.md` was half-swept by `e1e2300`.** The count went to fifteen; the enumeration did not -- it still names eleven and stops at 0021, and "0020 and 0021 were both called in by hv" is now four. Worse, **"Full suite GREEN at HEAD (hv-run, post-0020)" is false**: three code commits (0021, 0022, 0023) postdate that run. Not release-blocking, because pre-flight re-runs doctor and the full suite -- but see the watch-out on `--skip-tests`, which is the path where that false record becomes the only evidence of a run.
- Then nothing until the tag exists. The CHANGELOG heading is `## [2.19.0] - in progress` for the script to date; NEVER `--no-confirm`.

## TODO -- hv decides

- **`error()` voice: ruled, fixed, closed as 0023 (`e1e2300`) -- not mine, and it moved fast.** vc flagged it at 10:08 as unfiled and unruled; by 10:21 the issue existed and the fix was dirty across `bin/intent_helpers` + `intent_migrations` + three plugin bins + three test decks; by 10:45 it was committed, closing 25 imitators alongside the emitter. The blast radius vc predicted is what the diff showed -- the emitter is one line, the call sites and their assertions are the rest, and the twelve test assertions pinning the old string were found by sweeping BEFORE the change rather than by watching them fail. The release is fifteen issues, 0009-0023, and the docs were re-swept in the same commit to say so.
- **Still owed from earlier:** the per-project `.claude/scripts/*.sh` copies, inert since U5 (pruning deletes files from consumer trees); a `javascript` language pack to complete 0009's Node exception; issue 0004 item 4 (`ac status` exit code -- premise does not reproduce, so it wants a close ruling rather than work).
- **Post-tag tidy, already recorded in 0020's Resolutions so it cannot get lost:** `bin/intent_st:696-709` computes a `CREATED` in the in-progress arm that nothing reads -- residue of the five arguments 0019 pruned, and it greps the pre-move path, which is the corpse of the defect vc and I each refuted separately. Note the line numbers moved with `2769c40` + `08ef2f5`; anchor on the comment, not the number.

## Closed since my last board (vc, on hv's instruction -- do not re-plan these)

- **0020** `2769c40`: `st list --status all` membership goes through `normalise_status`, the ten literals collapse to five canonical tokens, unplaced rows are emitted last and named on stderr, exit stays 0 (escalating would break index regeneration on exactly the estates that have the problem). Guard `st_list_all_vocabulary.bats`, mutation-proven M1-M5.
- **0021** `3949f56`: `st zero` D5a removed -- Intent had been shipping a second, dead Elixir enforcement mechanism (six custom Credo checks copied unconditionally, wired best-effort, usually loaded by nothing). `doctor` check 4e reports consumer residue in three states and quotes the `elixirc_paths` lines, because deleting the directory alone breaks their build.
- **0022** `08ef2f5`: both no-template fallback heredocs DELETED rather than corrected -- my "adjacent, not fixed" item. Correcting them restores two copies and buys another year of drift. Consequence in my lane: the two 0010 drift guards are now inverted, from "the constant still matches the second generator" to "there is no second generator".
- **Release docs written PRE-cut** `86cdbe1`: `intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`. Both practices had lapsed (history after v2.16.0, releases after 2.17.0) and are resumed here, NOT backfilled. The globalfold is done and says fourteen issues.
- **"vc's residual 1" refuted** by vc and by me independently, same day, same conclusion: 0019 removed the mechanism, so no Created value travels from the `st done` call site at all.

## TODO -- other repos, not this one

- **Consumer sweep:** `intent upgrade` per project. That one pass now also sweeps AT grammar, converges AGENTS.md, rewrites settings.json to the portable hook form, and ignores the treeindex cache. Utilz / Lamplight / Baize.
- **Lamplight's own backlog, sequenced by them:** ~97 rows across 6 contracts needing the two-ended AT migration -- cite the file, put the id in the test. `--fix` will not and should not do it. Plus ST0276 (11 bolded `**green`), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`.
- **Push local-only commits:** Utilz (`0171297`) + Lamplight (`7058fd3a8`). Carry-over: utilz-side `generator: utilz todo` marker + symmetric guard.

## Watch-outs

- **NEVER run a mutation battery against `bin/**` while anyone else is using this tool.** The batteries `cp` over live files, and `~/.local/bin/intent` is a symlink INTO this repo, so every other project on this machine runs whatever state the file is in at that instant. A consumer session hit exactly that. Same rule while a suite is running.
- **A sweep that rewrites the estate must be measured against git before it is trusted.** The U1 `--fix` sweep destroyed 87 test names in this repo's own contracts and nobody noticed for a day; the loss was recoverable only because `f28938c^` still held them. Before any estate-wide rewrite, count what the old rows carried and what the new ones do.
- **The gate blocks unswept estates from the day v2.19.0 ships.** Every named row was already contributing no coverage, silently. The CHANGELOG says so explicitly -- do not soften it.
- **`intent upgrade` short-circuits when the project is already at the target version** (`intent_upgrade:107`). The fix reaches consumers because v2.19.0 IS a version boundary, NOT because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe.
- **AGENTS.md convergence must stay AFTER the canon apply** in `bin/intent_upgrade`, never as a ledger step. Canon creates `usage-rules.md`, which AGENTS.md's own file map lists. Verified by running it, not by reading.
- **`bin/release` stamps all five sidecars BEFORE the tag.** Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. `DEPRECATIONS.md` is NOT a sidecar -- its verblock is hand-maintained.
- **A dirty tree does not abort the cut early -- it aborts it half-done.** The leftover-dirt check is at `bin/release:437-447`, AFTER the sidecars are stamped and committed, so anything dirty outside the five yields a `release: vX.Y.Z` commit with no tag. Every node pickup writes a heartbeat, so the board itself is the likeliest offender: commit your own dir before handing over. Recovery is a `--skip-tests` re-run, not a revert -- **and that recovery is exactly where a stale "suite green" record turns dangerous**, because the re-run skips the one gate that would have re-established the claim, leaving the written record standing in for a run that never happened at HEAD.
- **A `--dry-run` is not cheap.** The pre-flight doctor + full suite are not behind the dry-run guard, so previewing the cut costs a full suite run. The GitHub release body is the CHANGELOG `[X.Y.Z]` section extracted verbatim (`bin/release:500`) -- not `history/` and not `docs/releases/`, so the CHANGELOG is the one that has to read well in public.
- **Do not use `git stash` in this repo** -- it carries two pre-existing 2025 stashes and a pop once dumped 522 lines of long-pruned migration code into the tree. Use `git show HEAD:<file>` or a throwaway `git worktree`.
- **The markdown linter collapses leading and trailing spaces inside an inline code span**, so `` ` + ` `` silently becomes `` `+` `` and can invert a sentence. Rephrase around it; do not fight it.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case; a `claude` subcommand needs an explicit arm in `bin/intent`. Register in MODULES.md FIRST.

## Decisions

- (2026-08-14) **A test that passes is not a test that works.** Seven guards this release would have guarded nothing, every one caught by mutation and none by review: an invalid ERE whose error `|| true` swallowed; a bats helper called inside `bash -c`; an assertion matching the fixture's own prose; a scope test whose decoy could never have been selected; a probe matching an unrelated comment; a probe hitting usage text instead of code; and one asserting the defective behaviour AS the contract. Break it, watch the right test fail, restore -- every time. **Corollary (vc, same day, extending this): the mutation harness is itself a probe, and it can lie too.** A `perl` substitution that silently matched nothing let the `&&` chain skip its restore, so the next mutation ran on a half-mangled file and reported a result that was impossible on its face -- which is the only reason it got looked at instead of filed. A mutation must hard-fail when the source is unchanged after substitution. Same family as the guard that survived by matching usage text instead of code: the probe, not the code, was the thing that lied.
- (2026-08-14) **A tool that cannot finish a job must not start it.** `at lint --fix` half-migrated rows: it stripped the test name before the id existed in the test, breaking the only link the row had. The SUGGESTION was lossy before the fixer was -- it named one file of several, so every human following it lost the same data. A lossy fixer damages what it touches; a lossy suggestion damages everything touched after it.
- (2026-08-14) **Grep for a Highlander rule; do not read for it.** 0011 was fixed by repointing the call sites found by reading, and vc's audit read the same way. A mechanical guard found two more live instances at once. Corollary: a guard scoped to what is already clean certifies the status quo -- widen the needle until it would have caught the bug, then fix what it finds.
- (2026-08-14) **A verifier of results may not state conclusions about mechanism.** `assert_written` checks the post-state and then claimed "the file was NOT updated" -- false in the case that mattered. Its pre-write permission siblings keep those words, because a write refused before it began genuinely did not happen.
- (2026-08-13) **Run the real path in a sacrificial copy; the dry-run path diverges.** Four defects surfaced only by exercising and none could have been read: `--fix` skipping the commonest migration, the AGENTS.md ledger-step ordering, an upgrade test invalid because the fixture was already at target, and the 65GB per-row scan.
