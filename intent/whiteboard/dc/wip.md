---
node: dc
name: DevX Claude
role: worker
session_id: baf3a3a8-2d05-4e9a-8170-c1bdf1f0753c
heartbeat_at: 2026-08-20 17:20Z
status: active
focus: "**FOLDED HARD AT END OF DAY (hv's order, relayed by vc). QUIET BUT REACHABLE -- no new build work.** Landed today: AC-04.7, AC-04.6, `int hooks` naming the four shipped guards, fmt+clippy over the prepush CLONE, and `b3018e5b` -- the shell half of matts' suite, a bats assertion pinned to a message the implementation retired on 2026-08-18. **AWAITING vc's SUITE LEG: shell, `bin/`, `.githooks/`, `bin/.devbin/**` and the bats suite come back to me.** The bats leg needs a quiet main tree, so my tree stays clean. **TOMORROW OPENS ON TWO GATE ADMISSIONS, BOTH RULED AND EVIDENCED AND BOTH UNSTARTED BECAUSE THEY ARE matts' CALL AND HE IS AFK.**"
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's full board, watch-outs and decisions are verbatim in `.history/20260820/wip.md` (fold 3 archive). This file is the COLD-SESSION MINIMUM: the rules that govern the next write, and the two things to start on.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.** A time-typed input parameter is a defect by inspection. Not exceptions: fixtures, "only reading it", "but it came from the database".
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE. The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The db is the durable SSOT, files are re-creatable; the typed API is the only door in; migrations are normal.**

## DOING

**Nothing in flight. Tree clean of my work.** Two commits today: `753490ec` (board), `b3018e5b` (the bats fix). Left dirty deliberately and NOT mine: `ic/wip.md`, `native/rust/crates/intentsvcs/tests/migrate_hooks_continuity.rs`.

## TODO

### 1. Tomorrow opens here -- both RULED, both UNSTARTED, both matts' call

- **ADMIT `canon_commit_check.sh` TO `cmd/precommit` (ST0057 AC-03.6).** **TWO DELIVERABLES, NEVER BUNDLED: the remedy edit makes the instruction TRUE; only `--staged` makes it UNNECESSARY.** Evidence: 39 of the last 40 commits rc=0 with the one refusal being the defective commit; my 190-of-286 reservation retired STRUCTURALLY (INHERITED is never examined in narrowed mode); **it cannot wedge**, because prettier writes the WORKTREE and is idempotent, so a refusal leaves the tree holding the bytes its own remedy asks for. **Dispatch UNCONDITIONAL** -- a path trigger would be a second copy of the tool's narrowing, and a skipped arm and a clean arm are both green. Remedy has two readers: before the fact FORMAT-SYNC-COMMIT; after a refusal, re-sync only. **The strongest argument is vc's own `c5320329`: the node that stewards the contract followed the written remedy exactly and diverged anyway, so no rewording closes it.**
- **ADMIT `thread_view_skew_check.sh` TO `cmd/precommit`** (1 gated triple -> 268 views), **CONDITIONAL on a staleness refusal.** It answered `268 view(s) match` rc=0 for me against a binary NINE of its own inputs out of date. **The polarity is the bad one:** a commit changing the generator makes every view skew, and a stale binary renders with the OLD generator, matches the OLD committed views, and greens on exactly that commit. **Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`'s refusal, never a copy** -- that reach list has already been wrong once (named one crate, blind to all of `intentsvcs/src`). The lib owns the BINARY's inputs; a consumer's own extra inputs are a parameter.

### 2. Recorded, not built

- **`intent claude upgrade --apply` IS ALL-OR-NOTHING** -- three of four actions unwanted here, including regenerating AGENTS.md from 3.0.0 DOWN to 2.19.0 on a self-hosted v3 tree.
- **`bin/int SAYS` is with hv**, unbuildable where he put it without forking vendored devbin.
- **`author`/`content` PRINT NOTHING WHERE v2 PRINTS 136 BYTES** -- a clean no-op returns before any output, so a prose language is indistinguishable from one that ran and found nothing.
- **CLI-level tests for `critic`** (the module has 18, the command surface none); **`critic --no-such-flag` EXITS 1 WHERE v2 EXITS 2**, so the gate blocks on a typo -- mine and ic's together.
- **WP-07 hosting sweep**: `agents` (42 call sites), `lang` (44), `claude skills` (23) / `subagents` (25) / `ws` (13) / `prime` (10). `upgrade` (29) and `start` (6) unmeasured, excluded by name -- they write outside the sandbox.
- **AT-11.6's deliverable is unbuilt and mine** (vc resolved the row in my favour, 16:08Z). **AT-11.7** -- positive control is the `intentd` fossil marker. **D37 payload sweep / `AT-00.17`**; `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

**Today's instances are verbatim in `.history/20260820/wip.md`; earlier days under their own dates. These are the CLASSES that will bite the next write.**

- **A GREEN IS ONLY EVER ABOUT THE QUESTION THE INSTRUMENT ASKS.** The family, all measured this week: a cross-check that reconciles because **both sides share an error**; a **true measurement of a different property** offered as proof; a count that mixes _not built_, _built and unverified_ and _verified and unmoved_ when only the first is work; **a zero from an instrument never shown able to produce a non-zero**.
- **A NAME SEARCH RETURNS A FACT ABOUT THE SEARCH, IN BOTH DIRECTIONS.** The pattern can EXCLUDE the answer (`grep 'organize::plan('` found 3 sites, the compiler found 7) or INCLUDE a non-answer (a MENTION inside prose read as a SUBJECT, over-counting a population 4 vs 3). **The over-count is worse: it reads as diligence.**
- **AN INSTRUMENT OR CITATION WHOSE PREMISE EXPIRED IS A LIVE DEFECT, NOT STALE CONTEXT.** Five documents on 08-20 stated a world the estate had left; a bats assertion outlived its message by two months. **No instrument we own catches it -- only a builder trying to satisfy the row does**, and `at lint` exempting `to-write` is CORRECT, which makes this structural rather than an oversight.
- **SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING**, and a skipped gate arm from a clean one. Four instruments were found with no trigger in one day. **A write surface with no named reader is the same shape.**
- **SYNTHESISE THE INSTANCE _AND_ KNOW THE LIVE POPULATION.** A red-first arm needs a SYNTHETIC instance; a green over synthetics alone cannot say whether the feature has ever RUN. **A rig tests the states its author enumerated** -- ten arms covered absent-binary and not stale-binary.
- **DELETE THE BINDING, DO NOT SHIM IT.** The compiler is a population oracle and a shim blinds it. **A fix that changes a type leaves every use unverified, and the ones that still compile are exactly the ones nothing reports.**
- **POLARITY BELONGS IN THE ASSERTION'S SHAPE, NOT IN WHOEVER READS THE OUTPUT** -- `assert_eq!(after, before)` prints `after` as `left`, and I read a failure backwards. **And a POSITIVE assertion cannot notice a retired string coming back; pair it with a `refute`.**
- **CITE THE IMPLEMENTATION, NEVER THE INVOCATION** (a symlinked dispatcher does not contain the command's name). **MARK PROVENANCE PER CLAIM -- driven, read or inferred** -- the cost lands on the READER, which is why the writer never feels it. **VERIFY THE RETRACTION, NOT JUST THE CLAIM.** **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION**, and a timing figure its LOAD.
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it; `grep -c` exits 1 on zero so a `||` fallback fires on a true zero; **`die` calls `exit` and a redirection does not contain one** -- use a subshell. **MEASURE IN THE SHELL THAT WILL RUN THE CODE**: mine is zsh, hooks run bash. **`local a=$1 b=$a` does not see `a`**, and an EXIT trap firing after `local tmp` left scope differs harmless-to-catastrophic by one character.
- **STANDING CONSTRAINTS.** **`git commit --only <paths>` is PATH-scoped, not HUNK-scoped** -- it defends against a peer's STAGED index and does nothing about their UNSTAGED edits; **only a detached worktree catches that class mechanically**. Push `local` only; **`upstream`'s freeze is a CI/CD BUDGET freeze, so pushing spends what it protects**. **DO NOT PUT v3 ON PATH.** NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. **The markdown formatter is a second writer.**

## Decisions

**Today's full set is verbatim in the fold 3 archive. These are the ones that govern the next build.**

- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES -- two doors on one model, and the filter they share is EXTRACTED** so the sigil space cannot change in one door and not the other. **Absence is decided at the filesystem, once, by the caller that touches it** -- inferring it from an empty string collapses it with a manifest declaring NONE, an opposite state.
- (2026-08-20) **A WORKSPACE-WIDE CHECK BELONGS WHERE THERE IS ONE WRITER** (the prepush clone, not the shared tree), **and a fault that can only escape on a push belongs to a PUSH gate.**
- (2026-08-20) **THE ROSTER IS THE RUNNER'S TO ANSWER, AND IT ANSWERS BEFORE IT DISPATCHES.** Never ask a question by running the thing -- `int hooks` once cloned the repository to answer what was wired. **The copied file names no guard and holds no roster**; roster lives in `pre-commit-guards.sh`, read live from `INTENT_HOME`, so a new guard reaches every consumer with no reinstall. **A template is embedded because `init` writes it; a guard is read live because the gate dispatches it.**
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc, retracting their own Highlander argument). Prevention and refusal are different criteria; declining the second because the first exists is a gap, not Highlander.
- (2026-08-20) **AN INSTRUMENT THAT REPRODUCES THE DEFECT IT WAS BUILT TO CATCH IS WORSE THAN ONE THAT UNDER-REPORTS AND SAYS SO.** The conservative instrument wins and its false negatives get named in the file. **A refusal whose reason expired is a live defect, not an owed capability.**
