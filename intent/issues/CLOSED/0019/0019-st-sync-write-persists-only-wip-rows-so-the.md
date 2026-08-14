---
id: "0019"
title: st sync --write persists only WIP rows, so the canonical steel_threads.md empties as threads complete; the index updater's arguments were never read
date: 2026-08-14
reporter: matts
status: CLOSED
severity: medium
---

# 0019: st sync --write persists only WIP rows, so the canonical steel_threads.md empties as threads complete; the index updater's arguments were never read

## Tags

steel-threads, st-sync, index, silent-failure, dead-code

## Summary

Two halves of one seam, found while acting on an hv "fix it" for what turned out to be a mis-filed residual.

**Half 1 -- the real defect.** `intent st sync --write` regenerates `intent/st/steel_threads.md` -- a document whose own preamble says _"an index of all steel threads in the project"_ -- by composing `st list` with **no status filter**. The default list shows WIP only, so the committed, canonical index has only ever carried the 1-2 threads in flight, and goes **empty** the moment a release closes the last one. This repo's committed index was an empty table over 55 threads at the time of filing. It was born this way: every historical revision carries exactly the WIP rows of its moment (2 → 1 → 2 → 1 → 0 across its last six commits), and `cbf159a` ("Post release todo prune") committed the fully empty table. Nobody noticed, because the file decays to empty exactly when nobody is looking at it -- and because the delegation that writes it ran under `> /dev/null 2>&1`, so a sync failure was also indistinguishable from a clean write.

**Half 2 -- the decoy that hid it.** `update_steel_threads_index()` took five arguments and read none of them; all four call sites computed values the function discarded before delegating to `sync --write`. Two of those sites (`st done`, `st cancel`) grepped a Created date out of `$ST_FILE` -- a path the thread had **already moved away from** -- producing a wrong answer that was then thrown away with everything else. That dead-but-plausible surface is what the original residual finding was about, and it was wrong in an instructive way (below).

## Reproduction

```
# Fixture: three threads, all Completed.
$ intent st list --status all --markdown        # the inner command, run by hand
| ST0003 | Past-created probe | Completed | 2026-01-15 | 2026-08-13 |   # correct
$ intent st sync --write
updated: intent/st/steel_threads.md
$ grep -c '^| ST' intent/st/steel_threads.md
0                                               # the canonical index: header, no rows

# This repo, at filing (55 threads):
$ git show HEAD:intent/st/steel_threads.md | grep -c '^| ST'
0
```

## Root Cause

`bin/intent_st` sync block: `LIST_ARGS=""` composed the default (WIP-filtered) list into the full-index markers. Plus the dead five-argument signature on `update_steel_threads_index` (four call sites), and the `2>&1` swallow on its delegation.

## Impact

- The project's canonical, committed thread index is wrong by omission almost always and totally wrong (empty) after every release -- while reading as authoritative.
- The dead argument surface misdirected a review: effort was spent assessing the correctness of values nothing reads.
- The swallowed stderr meant a genuinely failing sync would have been silent too.

## Proposed Fix

1. Compose the index from `st list --status all` (the document's stated contract).
2. Prune the dead arguments from `update_steel_threads_index` and all four call sites, including both moved-file greps; state the no-arguments contract in the function comment.
3. Let the delegation's stderr flow (`> /dev/null`, not `> /dev/null 2>&1`).
4. Guard: `st done` must land the completed row -- with the thread's own (past) Created date -- in the index.
5. Regenerate this repo's own `steel_threads.md` (55 rows) in the same commit, so the fixed generator and the committed artifact agree.

## Related

- 0011 -- its Impact item 2 ("`sync --write` persists the duplicate row into the committed index") described this same seam; true in mechanism, but understated -- the index was dropping almost everything, not just gaining a duplicate.
- 0018 -- the same lesson one directory over: committed derived state that is stale or empty reads as authoritative and is worse than absent.

## Resolutions

FIXED + CLOSED (2026-08-14), in v2.19.0, by vc on hv's direct "fix it".

### Correction to the record that led here (vc's own)

The residual vc filed after the bounce audit claimed _"the index row's Created goes empty/wrong until the next `sync --write`"_. **That was wrong**: the arguments (including the mis-grepped Created) were never read, and the sync inside the updater regenerates every row from `info.md` immediately -- the Created value was always correct **for the rows that appeared at all**. The real defect was which rows appeared: WIP only, per the missing `--status all`. Diagnosed from reading a call site, not from running the path -- the same failure mode this release corrected twice in the filed record (0014, 0011 item 3), reproduced here by the reviewer who kept flagging it. The empirical probe that exposed it (a past-dated thread closed in a fixture, then the index read back) took three minutes and contradicted the filed residual in both directions at once: no wrong date ever lands, and no row landed either.

### What fixed it

All five items of the Proposed Fix, exactly. The guard's past-dated Created is the mutation-killer: with `--status all` removed the row vanishes and the test fails; with a wrong-source date it shows today and the test fails. This repo's index went from 0 rows to 55 in the fix commit.

`st list`'s own default (WIP) is untouched -- the terminal view and the persisted index are different documents with different contracts, and only the index's composition changed.
