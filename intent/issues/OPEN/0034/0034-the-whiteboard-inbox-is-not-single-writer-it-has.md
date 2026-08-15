---
id: "0034"
title: the whiteboard inbox is not single-writer: it has one appender and one clearer, so a pathspec commit by either node commits the other's act as a split change
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0034: the whiteboard inbox is not single-writer: it has one appender and one clearer, so a pathspec commit by either node commits the other's act as a split change

## Tags

whiteboard, git, protocol, skill, measured

## Summary

Protocol invariant 1 in `intent/plugins/claude/skills/in-whiteboard/SKILL.md` reads:

> **One writer per file.** `wip.md` = the node; `inbox.<sender>.md` = the sender.

**That is false for inboxes, and the file layout section two screens earlier says so itself:**

> `<node>/inbox.<sender>.md` -- appended only by `<sender>`; read + cleansed only by `<node>` (the owner).

Appending and cleansing are both mutations of one file. **The inbox has one APPENDER and one CLEARER -- two mutators, in two sessions, on two schedules.** The property that makes `wip.md` contention-free does not hold here, and the word "writer" covering both acts is what hides it.

The consequence is a commit hazard neither node can see coming: **whichever node commits the file first commits the OTHER node's act**, and because the recipient's clear is one act spread across two files (remove from `inbox.<sender>.md`, capture into `.history/<date>/inbox.<sender>.md`), a sender committing its own append lands the removal half without the capture half.

## Reproduction

Measured live on this board, 2026-08-15, without trying to provoke it.

1. vc (sender) appended two entries to `cc/inbox.vc.md` and two to `dc/inbox.vc.md`.
2. cc and dc each ran a fold in their own sessions: entries removed from the live inbox, captured into `<node>/.history/20260815/inbox.vc.md`.
3. vc committed `13e1b530` with an explicit pathspec naming, among vc's own files, `cc/inbox.vc.md` and `dc/inbox.vc.md` -- files vc is the documented writer of.

What that commit actually contained:

```
0    352   intent/whiteboard/cc/inbox.vc.md
15   216   intent/whiteboard/dc/inbox.vc.md
```

**Zero insertions on cc's inbox.** The two entries vc had just appended are not in the commit; what is in the commit is cc's removal of them. Immediately afterwards:

```
$ git status --short intent/whiteboard/cc/
 M intent/whiteboard/cc/.history/20260815/inbox.vc.md     <- the capture half, NOT in HEAD
```

So at HEAD: the entries were gone from the live inbox and their archive copy was uncommitted. Recoverable from two peers' worktrees, present in the repository nowhere.

**The pathspec was correct by every rule in the skill.** vc named only files vc writes, used `--only`, and named no peer's board. The rule that is supposed to prevent exactly this class permitted it.

## Root Cause

**The correct rule already exists, is stated once, and is scoped to the wrong operation.** Measured across the whole shipped skill: `SKILL.md:232` is the ONLY commit instruction in the entire protocol.

```
232:  5. Single-owner: you only ever touch your own <you>/ directory ... Commit via
      explicit pathspec (git commit --only <you>/...), never -A.
```

That is directory ownership, and it is right. But it lives inside step 5 of the `archive` subcommand, so it reads as guidance about archiving. **The two subcommands that write into a PEER's directory -- `ask` and `announce` -- carry no commit step at all.** Both are two steps: append the entry, touch your heartbeat. A sender is told to write into `<peer>/` and told nothing about committing what it wrote, so it improvises, and the natural improvisation is "I wrote it, so I commit it".

**Which the invariant at `:276` then confirms in so many words:**

> 1. **One writer per file.** `wip.md` = the node; `inbox.<sender>.md` = the sender.

`:58-59` already contradicts it -- appended by the sender, cleansed by the recipient -- but `:276` is the line in the section called "Protocol invariants", and it says the inbox is the sender's file. **A sender committing its own inbox file is following the invariant.**

Two properties follow, both invisible from either side:

1. **Two mutators with no coordination.** Each node correctly believes it is the only one touching the file, because each is the only one performing ITS act on it, and `:276` confirms that belief for the sender.
2. **The clear is atomic in intent and non-atomic on disk.** Removal and capture are two files. Only the pair is coherent -- the removal alone is data loss, the capture alone is a duplicate. **Any commit taking one without the other is a split change**, and a sender's pathspec commit takes exactly the removal, because the capture is in a file the sender does not write and would never name.

**Stated against the reporter's own instance rather than in the abstract**: `13e1b530` named `cc/inbox.vc.md` and `dc/inbox.vc.md` -- paths outside `vc/`. Held against `:232` that is already wrong, and no separate rule had to be invented to see it. What the protocol does not say anywhere is that `:232` governs anything other than `archive`.

## Impact

Bounded, because the archive is captured before it is removed and both halves exist in a worktree until someone commits them. **The failure is a window, not a deletion.** But the window is real and this repository is public, so what lands in it is a published state where a message exists nowhere in the repo.

What makes it worth filing rather than absorbing:

- **The `-A` mitigation does not reach it.** `--only` with a hand-checked pathspec is the strongest discipline the protocol names, and it is what produced this: every path in `13e1b530` was one the reporter is the documented writer of. **The rule that would have caught it (`:232`, own-directory) exists but governs a different subcommand**, so a sender applying `--only` faithfully still lands the hazard. This is not "the pathspec was right" -- it was wrong against `:232` -- it is that nothing in the protocol says `:232` applies here, and the invariant at `:276` says the opposite.
- **It scales with how well the protocol is followed.** A node that folds promptly (correct) shortens its own window and widens its senders'. The more nodes fold, the more often a sender's routine commit lands somebody's removal half.
- **Third instance of split-change today, from three unrelated mechanisms** -- ic's unqualified `--amend` (`22464e5f`, HEAD did not build until `7257ea68`), vc's own hv-inbox fold (`2b3a8961` committed the emptied inbox and left the `.history/` capture untracked), and this one. **Three mechanisms producing one failure shape is a property of the layout, not a run of carelessness.**

## Proposed Fix

**PROMOTE `:232` OUT OF `archive` AND MAKE IT THE PROTOCOL'S COMMIT RULE: `<node>/` is committed by `<node>`, whatever is in it and whoever wrote it.**

Nothing new has to be designed -- the sentence is already written and already correct, it is simply filed under one subcommand. Promoted to a protocol invariant it says: a sender appends to `<recipient>/inbox.<sender>.md` and does NOT commit it; the recipient commits it, along with its own clear and the `.history/` capture. Because all three are inside the directory the recipient owns, **removal and capture land in one commit by construction rather than by discipline.**

**cc's statement of it is better than the above and is the one to ship** (2026-08-15, undertaking it unprompted after reading this issue): _"when I next clear your inbox I will commit BOTH halves myself in one pathspec commit, because the pair is only coherent together and the clearer is the node that can see both."_ **Directory ownership is the mechanism; "the clearer is the only node that can SEE both halves" is the reason**, and the reason is what makes the rule survive contact with someone who has not read this issue. A sender cannot commit the pair even in principle -- the capture is not a file it writes and would never occur to it to name.

The cost, stated plainly: **a message is on disk but not in HEAD until its recipient next commits.** The live channel is the disk, so delivery is unaffected; what is delayed is the public record. Given the alternative is a published state where the message is in no tree at all, that is the better failure.

Rejected alternative -- **"the sender must commit its append immediately, in the same step as appending."** It narrows the window rather than closing it, leaves the removal/capture pair split across two nodes' commits, and is a rule that must be remembered every time. **A control refuses; a reminder reminds.**

Also fix the invariant at `:276`, because the false sentence is what makes the sender's improvisation feel correct:

> **One writer per file.** `wip.md` = the node; `inbox.<sender>.md` = **one appender (the sender) and one clearer (the recipient) -- two mutators, so it is committed by the directory's owner, never by the sender.**

**ONE NAMED EXCEPTION, and the fix is wrong without it: `hv` never runs a session, so `hv/` has no committer.** The hypervisor node is human-driven -- no `pickup`, no fold, no commit -- so "the directory's owner commits it" leaves `hv/inbox.<sender>.md` and `hv/.history/` in nobody's hands. **Measured: that is exactly how `2b3a8961` shipped hv's inbox emptied with its `.history/` capture untracked** -- the reporter's own instance, and the reason it went unnoticed is that the rule being violated had not been written yet.

So: **for `hv`, the SENDER commits both halves**, including the archive when it clears hv's inbox on hv's instruction. The general rule and this exception have the same justification -- **whoever can see both halves commits both halves** -- which is why the exception is not a special case so much as the rule stated in terms of capability rather than of directory.

Not proposed: changing the file layout. Inboxes belong in the recipient's directory -- that is what makes `clear` and `archive` single-owner operations, and moving them would trade this hazard for a worse one.

## Related

- ST0045 -- Whiteboard Protocol 3.0, which ships the invariant at `SKILL.md`
- 0028 -- same family, same session: `--only` never clears the index. **That one is about what a commit CARRIES; this one is about what a correct pathspec MISSES.** Both make the documented safety rule the thing that produces the hazard
- 0027 -- the other measured finding against shipped whiteboard guidance; same shape (the mechanism is fine, the sentence around it is not)
- ST0056 -- found at a pickup on 2026-08-15, in the tree the protocol governs, on a five-node board

## Resolutions

{{TBC}}
