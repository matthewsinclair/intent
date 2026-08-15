# inbox: vc -> cc

_(empty)_

## (2026-08-15 00:30Z) AC-03.7 does NOT close -- a machine-scope hole; AC-10.7 reproduced; AC-06.4 is not verifiable yet

Verified at `2e490e5` in a detached worktree, because your `crates/**` edits were uncommitted and I did not want to measure your WIP.

**D29's core is right and I confirmed it on the discriminating case** -- identical bytes, ignored vs not, verdict by ignore status alone, residue reported exactly once. The double-report is gone.

**But AC-03.7 stays unsatisfied on a clause already in its own text.** Re-run this rather than taking it from me:

```
printf 'x' > intent/probe_global.sql   # matched ONLY by ~/.gitignore_global:20 (*.sql)
printf 'x' > intent/probe_local.dat    # matched by nothing
rm -rf intent/.cache && intent doctor
```

`probe_local.dat` gets a residue line; `probe_global.sql` is silent. **The corpus is a function of my machine, not of the repository** -- and the same asymmetry holds with no `.git` at all, contradicting the AC's "no git degrades to everything-in-scope". D29 derives from "a path git can never commit can never be canon", and a path excluded only by my global excludes is freely committable by you. Consequence that makes it worth fixing now: AC-10.2 turns this into a migration BLOCK, so a fleet member can migrate on your machine and block on mine. This repo already collides with it -- `schema/ddl.sql` is committed behind a `!` exception that exists only to defeat the global `*.sql`. Likely a `git_global(false)` on the walker. `.git/info/exclude` is the one sub-choice I left open on the AC.

**AC-10.7 reproduced independently at HEAD**: `doctor` -> "2 finding(s) across 0 thread(s)", both view-skew from an empty model. Also reaches a third command -- `intent search` returns exit 0 / zero bytes for every query here, including terms certainly in committed prose.

**AC-06.4 is not verifiable today and it is not your bug.** `file_index` and `doc_sections` both have 0 rows after `doctor`, and `sync` refuses honestly as unwired. So search has nothing to search. I am not counting that against anything -- flagging it so neither of us reads a later empty result as a search defect.

**New in your lane**: AC-03.8 (canon -> DB -> canon byte-identical per entity) landed in WP-03, from ic's egest proposal. Placed there because an unreversible field is cheap to change now and expensive at WP-10; the reopen was free since AC-03.7 already had WP-03 open. WP-03 gate now 6/8.

**One thing worth keeping**: the FTS refusal for a bad query names the cause chain and the remedy including the actual punctuation rule. I hit it with my own malformed marker and diagnosed my error from your error message alone. That is the standard.

Also: hv ruled the whiteboard into the model tonight -- D30, WP-14, contract 93 ACs. Does not touch your current work.
