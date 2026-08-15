---
id: "0028"
title: git commit --only never clears the index, so the whiteboard protocol's own safety rule lets a stale index accumulate unseen
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0028: git commit --only never clears the index, so the whiteboard protocol's own safety rule lets a stale index accumulate unseen

## Tags

whiteboard, git, protocol, skill, measured

## Summary

`intent/plugins/claude/skills/in-whiteboard/SKILL.md:232` prescribes the archive commit as:

> Commit via explicit pathspec (`git commit --only <you>/...`), never `-A`.

That rule is correct and should stay -- it is what stops one node sweeping another node's staged work into its own commit. **But it has a corollary nobody wrote down: `--only` commits the named paths and leaves every OTHER index entry exactly as it found it, forever.** Once any path is staged by anything, no number of `--only` commits will ever clear it, and it becomes invisible to the check people actually run (`git diff HEAD` is clean, because the worktree is clean). It shows only as the left-hand `M` of `MM` in `git status --short`.

In a repository with five concurrent sessions, that stale index is shared state. A single bare `git commit` by any node publishes it.

## Reproduction

Measured 2026-08-15, on the live board first and then in a scratch repo to isolate the mechanism.

**Observed live**, at `3063f8b` with a clean-looking tree:

```
$ git status --short
MM intent/st/ST0056/acceptance.md
MM intent/whiteboard/ic/wip.md
MM intent/whiteboard/vc/wip.md
...11 files, 3 of them peers' boards

$ git diff HEAD --stat
(empty -- worktree IS HEAD)
```

The staged content differed from HEAD only in markdown emphasis markers (`_x_` vs `*x*`) and one stripped blank line: the on-save markdown linter had rewritten the files after they were staged, and the prettified form is what reached HEAD.

**Isolated in a scratch repo** -- stage a change to `a.md`, revert it on disk, then commit an unrelated `b.md` with `--only`:

```
git add a.md              # index: "staged"
printf 'v1\n' > a.md      # worktree back to HEAD content
git commit --only b.md -m "commit b only"

git status --short   ->  MM a.md
git show :a.md       ->  staged      <- index, still there
git show HEAD:a.md   ->  v1
```

`a.md` was never named in the commit, so `--only` left it alone. It survives indefinitely.

**Self-reproduced, unprompted, in the commit that filed this issue.** `ce73e64` committed this very file; `git status` immediately afterwards showed it as `MM` again, with `git diff HEAD` empty and the index holding a pre-linter copy. That closes the last inferential gap: root cause (1) below is no longer two ordinary things that _could_ combine, it is the observed sequence -- `git add` captures the pre-format content, the linter reformats on save, and `--only` then commits the worktree while leaving the index where it was.

## Root Cause

Not a defect in git and not a defect in Intent's code -- both do what they document. The defect is a **gap in shipped protocol guidance**: the skill states the pathspec rule without the hygiene step that rule makes necessary.

Two ordinary things combine to seed the index in the first place:

1. The markdown linter reformats on save, so a file staged pre-format and saved post-format diverges from its own index entry with no user action.
2. Anything that runs `git add` -- a hook, a tool, a habit -- seeds an entry that the node's own `--only` discipline then preserves rather than consumes.

Neither is visible to a node that checks its work with `git diff HEAD`, which is the natural check and is clean throughout.

## Impact

Low in isolation, and the live instance carried no information loss: the index content was byte-equivalent to HEAD apart from emphasis markers the linter would strip again anyway. `git reset` cleared it with the worktree untouched.

What raises it above noise is the multi-node context this protocol exists for:

- **The stale index is shared across all five sessions.** It is not a per-node hazard that a per-node rule can contain.
- **It converts a bare `git commit` from a style error into a publishing event.** THIS REPOSITORY IS PUBLIC and hv has ratified the whiteboards as part of the public record, so the resulting commit -- spurious, touching three peers' boards, attributed to whichever node ran it -- lands in a history nobody can rewrite.
- **The safety rule is what preserves it.** A node following the protocol exactly accumulates this; a node using `-A` would not. That inversion is the reason this is worth writing down rather than treating as an operator slip.

### SEVERITY RAISED to medium on a live instance, 2026-08-15

The filing above says "low in isolation" and "carried no information loss". **Both were true of the instance measured and neither survived contact with a full day of four nodes.**

`22464e5f` (ic): a `git commit --amend -F <file> --no-verify` with **no pathspec** -- `--amend` re-commits the whole index exactly like a bare `git commit` -- took **19 files** and pushed them. Included: another node's 136-line test file, **three peers' single-writer `wip.md` boards**, `.history/` archives from all four nodes, `intent/llm/MODULES.md`, and an open issue. Not rewritten, because four sessions were live on `main` and a force-push would have cost more than the mess.

**Three things this instance establishes that the original did not:**

1. **`--only` protects the commit and NOT the amend.** The two read as the same operation and the second silently widens to the entire index. The documented safety rule does not cover the documented repair.
2. **The sweep is loaded by everyone and tripped by one.** ic's own framing after reading this issue: the amend "did not reach out and grab anything; it published a pile that four nodes had been quietly adding to all day." **Any node that ever runs a bare `git commit` or an unqualified `--amend` publishes everything every other node has staged.** That is a standing property of the tree, not a slip.
3. **THE DAMAGE IS NOT ATTRIBUTION, IT IS A SPLIT CHANGE -- and this is the part that upgrades the severity.** The sweep took a test file **without the `store.rs` methods it calls**, because those were still uncommitted in their author's tree. **HEAD did not build from `22464e5f` until `7257ea68`.** ic initially reported "attribution and process, not data" on the strength of `git show --stat`, then corrected it: a stat says which files moved and cannot say whether the tree compiles.

**The generalisation, and it belongs in the fix: a sweep does not move a FILE, it SPLITS A CHANGE.** A method and its test are one unit; the index holds them separately; the sweep takes whichever half is staged. **Each half reads as finished on its own and only the pair is coherent**, so there is nothing file-shaped for a reviewer to notice. **After a sweep the question is not "whose file is this", it is "does it still build" -- and that question must be asked of HEAD, in a clean clone, never of the worktree**, because the worktree contains the very uncommitted half whose absence broke HEAD.

Verified after repair (vc, 2026-08-15, fresh `git clone --depth 1` of HEAD): workspace builds with `--tests`, and every suite is green with zero failures.

## Proposed Fix

Add the corollary to `SKILL.md:232`, alongside the existing pathspec rule -- one sentence, same place, so the rule and its consequence are read together:

> `--only` commits the paths you name and leaves every other index entry untouched, so a stale one accumulates unseen. Before committing, check `git status --short` for `MM` or staged strays; if the worktree already matches HEAD, `git reset` clears the index without touching a single file.

Two things deliberately NOT proposed:

- **Do not weaken the pathspec rule.** It is load-bearing and correct; this is an addition to it, not a qualification of it.
- **Do not automate the reset.** A guard that silently resets someone's index would destroy real staged work the one time it was real -- the same objection that keeps the clock guard from auto-correcting a stamp (`whiteboard-clock-guard.sh`, "never auto-corrects"). Reporting is the right strength here; the node decides.

Whether the pre-commit gate should additionally _report_ a divergent index is a separate question, and a better one to answer with a measurement of how often this occurs than by argument.

## Related

- ST0045 -- Whiteboard Protocol 3.0, which ships the rule at `SKILL.md:232`
- 0027 -- the other measured finding against shipped whiteboard guidance; same shape (the mechanism is fine, the sentence around it is not)
- Found during a `pickup` on 2026-08-15, in the tree the protocol itself governs.

## Resolutions

{{TBC}}
