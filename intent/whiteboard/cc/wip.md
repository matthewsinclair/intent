---
node: cc
name: Control Claude
role: control
session_id: 76c0e702-e5b4-4cdc-9bd4-f12ea5965985
heartbeat_at: 2026-08-14T10:49Z
status: paused
focus: "v2.19.0 shipped (tag v2.19.0, 071c612, both remotes). Next for cc is the consumer sweep -- Lamplight first, measured against git before it is trusted."
claims: []
---

# Control Claude (cc)

## DOING

- Nothing. Released to hold after the tag. Day archived to `.history/20260814/`.

## TODO -- next session, in this order

1. **Consumer sweep, one `intent upgrade` per project. Lamplight FIRST.** That single pass now sweeps AT grammar, converges AGENTS.md, rewrites settings.json to the portable hook form, ignores the treeindex cache, and PRINTS (never runs) the `git rm` for a tracked one. Expect BLOCKED-until-swept on Lamplight's 314 rows -- **that is the fix working, not a regression**, and the residue is named rather than guessed. Then Utilz, then Baize.
2. **Push the two local-only fleet commits** in their own repos: Utilz `0171297`, Lamplight `7058fd3a8`. Carry-over: the utilz-side `generator: utilz todo` marker + symmetric guard.
3. **Post-tag tidy:** the dead `CREATED` block in `intent_st`'s in-progress arm -- computed, never read, and it greps the pre-move path. Anchor on the comment `# Extract created date for index update`, NOT a line number; that number has already expired twice. Recorded in 0020's Resolutions.

## TODO -- hv decides

- **`Error:` on STDOUT in the three plugin bins** (0023's named-and-left half, in 0023's Resolutions). An error on stdout interleaves with captured command output, which is how a voice becomes data -- the same class that let a failing `sync` look clean in 0019. It changes what callers CAPTURE, not merely what they read, so it is a different decision from the voice fix. Sits beside `intent_claude_prime:212`, which is the same shape.
- **Still owed from earlier:** the per-project `.claude/scripts/*.sh` copies, inert since U5 (pruning deletes files from consumer trees); a `javascript` language pack to complete 0009's Node exception; issue 0004 item 4 (`ac status` exit code -- the premise does not reproduce, so it wants a close ruling rather than work).

## Lamplight's own backlog, sequenced by them, not by us

- ~97 rows across 6 contracts needing the two-ended AT migration -- cite the file, put the id in the test. `--fix` will not and should not do it. Plus ST0276 (11 bolded `**green`), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`.

## Watch-outs

- **NEVER run a mutation battery against `bin/**` while anyone else is using this tool.** The batteries `cp` over live files, and `~/.local/bin/intent` is a symlink INTO this repo, so every other project on this machine runs whatever state the file is in at that instant. A consumer session hit exactly that. Same rule while a suite is running. Use a sacrificial worktree.
- **A sweep that rewrites the estate must be measured against git before it is trusted.** The U1 `--fix` sweep destroyed 87 test names in this repo's own contracts and nobody noticed for a day; the loss was recoverable only because `f28938c^` still held them. Before any estate-wide rewrite, count what the old rows carried and what the new ones do.
- **The gate blocks unswept estates from the day v2.19.0 ships.** Every named row was already contributing no coverage, silently. The CHANGELOG says so explicitly -- do not soften it when a consumer complains.
- **`intent upgrade` short-circuits when the project is already at the target version** (`intent_upgrade:107`). The fix reaches consumers because v2.19.0 IS a version boundary, NOT because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe.
- **AGENTS.md convergence must stay AFTER the canon apply** in `bin/intent_upgrade`, never as a ledger step. Canon creates `usage-rules.md`, which AGENTS.md's own file map lists. Verified by running it, not by reading.
- **`bin/release` stamps all five sidecars BEFORE the tag.** Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. `DEPRECATIONS.md` is NOT a sidecar -- its verblock is hand-maintained.
- **A dirty tree does not abort the cut early -- it aborts it half-done.** The leftover-dirt check is at `bin/release:437-447`, AFTER the sidecars are stamped and committed, so anything dirty outside the five yields a `release: vX.Y.Z` commit with no tag. Every node pickup writes a heartbeat, so the board itself is the likeliest offender: commit your own dir before handing over. Recovery is a `--skip-tests` re-run, not a revert -- **and that recovery is exactly where a stale "suite green" record turns dangerous**, because the re-run skips the one gate that would have re-established the claim.
- **A `--dry-run` is not cheap.** The pre-flight doctor + full suite are not behind the dry-run guard, so previewing the cut costs a full suite run. The GitHub release body is the CHANGELOG `[X.Y.Z]` section extracted verbatim (`bin/release:500`) -- not `history/` and not `docs/releases/` -- so the CHANGELOG is the one that has to read well in public.
- **Do not use `git stash` in this repo** -- it carries two pre-existing 2025 stashes and a pop once dumped 522 lines of long-pruned migration code into the tree. Use `git show HEAD:<file>` or a throwaway `git worktree`.
- **The markdown linter normalises whitespace and will win.** It collapses leading/trailing spaces inside an inline code span, so `` ` + ` `` silently becomes `` `+` `` and can invert a sentence; it also collapses the multi-space separators the `/in-whiteboard` message format documents (`## (ts)   Re:` becomes `## (ts) Re:`). Rephrase around it; do not fight it. Commit the linted form or every commit reopens the same diff.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case; a `claude` subcommand needs an explicit arm in `bin/intent`. Register in MODULES.md FIRST.
- Node clocks are skewed across sessions (cc read 10:49Z while vc stamped 11:08). Immaterial against a 7-day reclaim rule, but board timestamps are not a reliable cross-node ordering -- use commits for that.

## Decisions

- (2026-08-14) **A record must name the commit it covers, never "HEAD".** `intent/wip.md` claimed "Full suite GREEN at HEAD" while three code commits postdated the run; it was in four documents, and it is a claim that decays silently every time anyone commits. Same class as `steel_threads.md` and the AT reference -- a reader answering confidently from stale evidence, which is what this whole release was about. **The corollary is the sharp part: a stale green is cheap while it is redundant and expensive at the single moment it is not.** Pre-flight normally re-establishes it, but `--skip-tests` is the documented recovery from a half-done abort, so the written record becomes the sole evidence of a run exactly on the occasion something already went wrong. Fixed at `dde7b59`; now a step in the release checklist.
- (2026-08-14) **A test that passes is not a test that works.** Seven guards this release would have guarded nothing, every one caught by mutation and none by review: an invalid ERE whose error `|| true` swallowed; a bats helper called inside `bash -c`; an assertion matching the fixture's own prose; a scope test whose decoy could never have been selected; a probe matching an unrelated comment; a probe hitting usage text instead of code; and one asserting the defective behaviour AS the contract. Break it, watch the right test fail, restore -- every time. **Corollary (vc, same day): the mutation harness is itself a probe, and it can lie too.** A `perl` substitution that silently matched nothing let the `&&` chain skip its restore, so the next mutation ran on a half-mangled file and reported a result impossible on its face -- which is the only reason it got looked at instead of filed. A mutation must hard-fail when the source is unchanged after substitution.
- (2026-08-14) **A tool that cannot finish a job must not start it.** `at lint --fix` half-migrated rows: it stripped the test name before the id existed in the test, breaking the only link the row had. The SUGGESTION was lossy before the fixer was -- it named one file of several, so every human following it lost the same data. A lossy fixer damages what it touches; a lossy suggestion damages everything touched after it.
- (2026-08-14) **Grep for a Highlander rule; do not read for it.** 0011 was fixed by repointing the call sites found by reading, and vc's audit read the same way. A mechanical guard found two more live instances at once. Corollary: a guard scoped to what is already clean certifies the status quo -- widen the needle until it would have caught the bug, then fix what it finds.
- (2026-08-14) **A verifier of results may not state conclusions about mechanism.** `assert_written` checks the post-state and then claimed "the file was NOT updated" -- false in the case that mattered. Its pre-write permission siblings keep those words, because a write refused before it began genuinely did not happen.
- (2026-08-14) **Report the findings you killed, not just the ones you kept.** Checking `RELEASE_NOTES.md:7` and saying so cost three lines and saved vc a re-derivation; a finding already refuted is worth as much to a peer as one that survived, and it stops the same line arriving again from a third node. Reciprocal with vc.
- (2026-08-14) **Announce before editing `bin/`, both directions.** Standing agreement with vc after it worked three issues deep in cc's lane under hv's pre-cut batching. The ownership line did not move -- the queue moved fast because the reasoning was already written down in a form another node could act on, which is the board working as designed rather than an exception to it.
- (2026-08-13) **Run the real path in a sacrificial copy; the dry-run path diverges.** Four defects surfaced only by exercising and none could have been read: `--fix` skipping the commonest migration, the AGENTS.md ledger-step ordering, an upgrade test invalid because the fixture was already at target, and the 65GB per-row scan.
