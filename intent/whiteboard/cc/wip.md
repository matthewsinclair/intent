---
node: cc
name: Control Claude
role: control
session_id: 82a8fe51-f060-4925-8bc4-841cd8a8351e
heartbeat_at: 2026-08-14 15:11Z
status: paused
focus: "EOD/EOW. WP-02 close claimed to vc, gate blocked on AC-02.6 pending their renumber. Next is ST0056 WP-03 (ingest, views, sync engine)."
claims: []
---

# Control Claude (cc)

## DOING

- **Nothing in flight.** Folded hard 2026-08-14 15:11Z for EOD/EOW; the day is in `.history/20260814/`.

## TODO -- next session, in this order

1. **ST0056 WP-03 -- ingest, views and the sync engine.** cc builds, vc stewards the contract, ic has parity. **Read `migration.md` as landed, not as remembered**: hv ruled closed-thread lossless-by-carrying as policy and live threads stay BLOCKED-until-clean, neither class ever getting a lossy path -- that shapes the sync engine's write path directly. The v3 migrator's real input is ~1158 rows in shapes `--fix` must refuse (baseline in `.history/20260814/`), so WP-10's fixture cannot be "post-sweep trees"; there will not be any.
2. **Check WP-02 actually closed.** I claimed it at `94dd922`; the gate was `BLOCKED -- 5/6, unsatisfied: AC-02.6` and the renumber into WP-04's group is vc's. Do not touch the contract; if it is still blocked, ask rather than edit.
3. **`installed-agents.json` is untracked AND unignored.** `intent/plugins/claude/subagents/.manifest/` tracks `global-agents.json` but not its sibling, and `.gitignore` names neither, so anyone running `intent claude subagents install` inside a project gets a permanent `??` holding absolute machine paths. Pre-existing, NOT caused by the 0025 fix. Wants an issue; check the consumer estate first, since a rule that only fixes this repo is the wrong shape.
4. **Push the two local-only fleet commits**: Utilz `0171297`, Lamplight `7058fd3a8`. **Re-verify both are still unpushed at the moment of acting** -- the last board assumption on this was a day stale and wrong, and today a peer pushed inside a nine-minute window.

## PARKED -- v2 maintenance is default-defer (hv, 2026-08-14 bounce)

Show-stoppers only; **0025-class suite-blockers are the whole exception**, and that is the test a candidate must pass -- not "is it small". These are decided, not forgotten; do not re-raise them as new findings.

- `Error:` on STDOUT in the three plugin bins (0023's Resolutions), and `intent_claude_prime:212`, same shape. It changes what callers CAPTURE, not merely what they read.
- The inert per-project `.claude/scripts/*.sh` copies; a `javascript` pack to finish 0009's Node exception; issue 0004 item 4 (`ac status` exit code -- the premise does not reproduce, so it wants a close ruling, not work).
- The dead `CREATED` block in `intent_st`'s in-progress arm. Anchor on the comment `# Extract created date for index update`, never a line number.

## Watch-outs

- **NEVER mutate `bin/**` in place while anyone else is live.** `~/.local/bin/intent` symlinks INTO this repo, so every project on this machine runs whatever state the file is in at that instant. Sacrificial `git worktree` only, and pass `INTENT_HOME` explicitly or the harness silently measures the live tree instead.
- **Announce before editing `bin/`, both directions.** Standing agreement with vc; contract changes route through vc even when a builder proposes them.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes, and a pop once dumped 522 lines of long-pruned migration code into the tree. Use `git show HEAD:<file>` or a throwaway worktree.
- **The markdown linter normalises whitespace and will win.** It collapses spaces inside inline code spans (`` ` + ` `` becomes `` `+` ``, which can invert a sentence) and the multi-space separators in the whiteboard message format. Rephrase around it; commit the linted form or every commit reopens the diff.
- **Release-window mechanics live in `intent/restart.md`'s checklist.** The three that bite: `intent build release` stamps all five sidecars BEFORE the tag (author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let it date them); a dirty tree does not abort the cut early, it aborts it half-done, leaving a `release:` commit with no tag; and `--dry-run` costs a full suite run. The GitHub body is the CHANGELOG section verbatim.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp, including the second one in a turn.** Board stamps are not a cross-node ordering -- use commits. Every cc entry heading before `2026-08-14 14:37Z` is unverifiable and is deliberately not retro-corrected; the full finding is in `.history/20260814/`.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case; a `claude` subcommand needs an explicit arm in `bin/intent`. Register in MODULES.md FIRST.

## Decisions

- (2026-08-14) **Mutation discipline, in three clauses, all earned the hard way this week.** (1) **A test that passes is not a test that works** -- eight guards this release would have guarded nothing, every one caught by mutation and none by review. Break it, watch the RIGHT test fail, restore. (2) **An unexpected green is investigated exactly as hard as an unexpected red**, so the expectation is written down BEFORE the run; the red that did not arrive was the only signal that `grep -q ... && false || true` could never fail. (3) **Applied is not reached** -- a mutation proven live in the file can still sit on a branch the test never walks, and then both arms pass and a correct finding looks wrong. The canary must come from the same fixture and branch the test drives. Corollary from vc: the harness is itself a probe and can lie, so a mutation must hard-fail when the source is unchanged after substitution.
- (2026-08-14) **A record must name what it covers -- the commit, the subject, the revision -- never "HEAD" and never a bare number.** "Full suite GREEN at HEAD" was false by three commits across four documents; "314 AT rows" was wrong by 5x on two boards. Both were true when measured and neither carried what it was measured against, so neither could be spotted as stale, and both were about to be acted on. **A stale green is cheap while it is redundant and expensive at the single moment it is not** -- `--skip-tests` is the documented recovery from a half-done cut, so the written record becomes the sole evidence of a run exactly when something has already gone wrong.
- (2026-08-14) **A queued program is evidence with an expiry date; verify the premise at the moment you act, not when you queued it.** Fired twice in one day. The board's top item was a Lamplight sweep whose every premise had expired invisibly -- already at target, their nodes mid-surgery, their hv having ruled the work dead. Then "30 commits unpushed", true when measured, was a no-op nine minutes later because a peer pushed. **With peers live, the gap between recommending and acting is itself the hazard**, and for anything touching another project the check includes who is live in it.
- (2026-08-14) **Fix the class, not the instances -- and say which mechanism the guard actually proves.** 0025 was three shapes of one bug; fixing the reported instance would have left two. The fix names an authority (`resolve_project_root`) and routes every reader through it. The honest half is the mutation matrix: removing the dispatcher scrub reds NOTHING, so it is fail-safe cover for readers not yet written, and the issue says that rather than implying a test proved it.
- (2026-08-14) **A generic environment variable is not evidence.** Code reading `PROJECT_ROOT` raw asks "did someone set a variable?" while meaning "am I in a project?"; those agree often enough to look correct and differ exactly when it matters. devbin did not create the bug, it made it fire every time -- **a tool that makes a latent defect deterministic is doing you a favour.**
- (2026-08-14) **A tool that cannot finish a job must not start it.** `at lint --fix` half-migrated rows, breaking the only link a row had. The SUGGESTION was lossy before the fixer was, so every human following it lost the same data. A lossy fixer damages what it touches; a lossy suggestion damages everything touched after it. Carries straight into the v3 migrator.
- (2026-08-14) **Grep for a Highlander rule; do not read for it.** A mechanical guard found two live instances at once that two reading passes and an audit had missed. Corollary: a guard scoped to what is already clean certifies the status quo -- widen the needle until it would have caught the bug, then fix what it finds.
- (2026-08-13) **Run the real path in a sacrificial copy; the dry-run path diverges.** Four defects surfaced only by exercising and none could have been read.
