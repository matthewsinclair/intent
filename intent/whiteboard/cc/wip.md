---
node: cc
name: Control Claude
role: control
session_id: 347f2808-51b6-4c3c-90a5-3b43d41f5ecf
heartbeat_at: 2026-08-14T00:26Z
status: paused
focus: "v2.19.0 done and with hv: ten issues closed (0009-0018), suite running externally, release not yet cut. Nothing pushed; no tag; VERSION deliberately still 2.18.0 for bin/release to stamp."
claims: []
---

# Control Claude (cc)

## DOING

- **Nothing. With hv** for the external full-suite run and the cut. `bin/release` stamps all five sidecars before the tag; the CHANGELOG heading is `## [2.19.0] - in progress` for the script to date. NEVER `--no-confirm`.

## TODO

- **Everything after vc's U1-U5 audit is unaudited**: U6, U7, the docs pass, all ten Resolutions, 0018, and the three `at lint --fix` repairs. Their queue, not a blocker.
- **hv rulings still owed:** the per-project `.claude/scripts/*.sh` copies, inert since U5 (pruning deletes files from consumer trees, so it is not mine to do); a `javascript` language pack to complete 0009's Node exception; issue 0004 item 4 (`ac status` exit code -- premise does not reproduce).
- **Adjacent, not fixed:** the two no-template fallback heredocs in `intent_st`/`intent_wp` duplicate template content (rule-6) and the WP one has already drifted -- it still writes `## Acceptance Criteria` with checkboxes, a form the template retired. The 0010 drift guard covers them so the warning cannot silently stop firing; the duplication itself is untouched.
- **Consumer sweep (hv, separate repos):** `intent upgrade` per project. After v2.19.0 that same pass sweeps AT grammar, converges AGENTS.md, rewrites settings.json to the portable hook form, and ignores the treeindex cache. Utilz / Lamplight / Baize.
- **Lamplight's own backlog (theirs, sequenced by them):** ~97 rows across 6 contracts needing the two-ended AT migration -- cite the file, put the id in the test. `--fix` will not and should not do it. Plus ST0276 (11 bolded `**green`), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only. Carry-over: utilz-side `generator: utilz todo` marker + symmetric guard.

## Watch-outs

- **NEVER run a mutation battery against `bin/**` while anyone else is using this tool.** The batteries `cp` over live files, and `~/.local/bin/intent` is a symlink INTO this repo -- so every other project on this machine runs whatever state the file is in at that instant. A consumer session hit exactly that today. Same rule while hv is running the suite: a mid-run overwrite corrupts their result and looks like a real failure.
- **The gate blocks unswept estates from the day v2.19.0 ships.** Every named row was already contributing no coverage, silently; `at lint --fix` does the mechanical half and refuses the rest by design. The CHANGELOG says so explicitly -- do not soften it.
- **`intent upgrade` short-circuits when the project is already at the target version** (`intent_upgrade:107`). "The fix reaches consumers" is true because v2.19.0 IS a version boundary for everyone, NOT because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe.
- **AGENTS.md convergence must stay AFTER the canon apply** in `bin/intent_upgrade`, never as a ledger step. Canon creates `usage-rules.md`, which AGENTS.md's own file map lists. Verified by running it, not by reading.
- **`bin/release` stamps all five sidecars BEFORE the tag.** Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. `DEPRECATIONS.md` is NOT a sidecar -- its verblock is hand-maintained.
- **Do not use `git stash` in this repo** -- it carries two pre-existing 2025 stashes and a pop once dumped 522 lines of long-pruned migration code into the tree. To read or test an old commit use `git show HEAD:<file>` or a throwaway `git worktree`, both of which leave the live tree alone.
- **The markdown linter collapses leading and trailing spaces inside an inline code span**, so `` ` + ` `` silently becomes `` `+` `` and can invert a sentence's meaning. Rephrase around it (name the separator in words); do not fight it.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case; a `claude` subcommand needs an explicit arm in `bin/intent`. Register in MODULES.md FIRST.

## Decisions

- (2026-08-14) **A test that passes is not a test that works.** Seven guards written across this release would have guarded nothing, and every one was caught by mutation rather than by review: an invalid ERE whose error `|| true` swallowed; a bats helper called inside `bash -c`; an assertion matching the fixture's own prose; a scope test whose decoy could never have been selected anyway; a probe matching an unrelated comment; a probe hitting usage text instead of the code; and one asserting the defective behaviour AS the contract. Break it, watch the right test fail, restore -- every time, no exceptions.
- (2026-08-14) **Grep for a Highlander rule; do not read for it.** Issue 0011 was fixed by repointing the call sites found by reading, and vc's audit read the same way. A MECHANICAL guard found two more live instances immediately. Corollary: a guard scoped to exactly what is already clean certifies the status quo -- widen the needle until it would have caught the bug, then fix what it finds.
- (2026-08-14) **A tool that cannot finish a job must not start it.** `at lint --fix` half-migrated rows: it stripped the test name before the id existed in the test, breaking the only link the row had and turning an honest finding into a misleading one. And the SUGGESTION was lossy before the fixer was -- it named one file of several, so every human following it lost the same data. A lossy fixer damages what it touches; a lossy suggestion damages everything touched after it.
- (2026-08-14) **A verifier of results may not state conclusions about mechanism.** `assert_written` checks the post-state and then claimed "the file was NOT updated" -- false in the case that mattered. Its pre-write permission siblings keep those words because a write refused before it began genuinely did not happen.
- (2026-08-13) **Run the real path in a sacrificial copy; the dry-run path diverges.** Four defects surfaced only by exercising, and none could have been found by reading: `--fix` skipping the commonest migration, the AGENTS.md ledger-step ordering, an upgrade test invalid because the fixture was already at target, and the 65GB scan.
