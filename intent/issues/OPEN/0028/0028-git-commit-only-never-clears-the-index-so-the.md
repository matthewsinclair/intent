---
id: "0028"
title: git commit --only never clears the index, so the whiteboard protocol's own safety rule lets a stale index accumulate unseen
date: 2026-08-15
reporter: matts
status: OPEN
severity: low
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
