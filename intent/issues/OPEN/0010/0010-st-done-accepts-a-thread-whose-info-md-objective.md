---
id: "0010"
title: st done accepts a thread whose info.md Objective is still the template placeholder
date: 2026-07-31
reporter: matts
status: OPEN
severity: low
---

# 0010: st done accepts a thread whose info.md Objective is still the template placeholder

## Tags

st-lifecycle, close-gate, records-hygiene

## Summary

`intent st done` has exactly one gate: the acceptance contract. Nothing checks that the thread's own record says what the thread was for. A thread can therefore reach `Completed` with `## Objective` in its `info.md` still reading the shipped template text, `[Clear statement of what this steel thread aims to accomplish]` -- so the close is recorded, the acceptance contract is satisfied, and the record does not state the objective that was met.

Raised as low severity deliberately. The measured instance rate in a large live project is ~3%, and a reasonable triage outcome is won't-fix. The numbers below are supplied so that call can be made on evidence rather than on the shape of the complaint.

## Reproduction

```
intent st new "Some thread"          # info.md ships with the template Objective
# ... do the work, satisfy the acceptance contract, never touch info.md ...
intent st done <ID>                  # gate: PASS, thread moves to COMPLETED
grep -c 'Clear statement of what this steel thread aims to accomplish' \
  intent/st/COMPLETED/<ID>/info.md   # 1
```

Observed in the Lamplight project, 2026-07-31, across a 333-thread estate.

## Root Cause

`bin/intent_st` gates the close in one place -- it shells to `bin/intent_acceptance ac gate "$ST_ID"` and refuses on BLOCKED. That gate reads `acceptance.md` only. The thread's `info.md` is never consulted, so no property of the record itself is enforced at any lifecycle transition.

This is a gap rather than a defect: the close-gate was scoped to the acceptance contract and does that correctly. Nothing was ever asked to check the record.

## Impact

Measured over 333 threads (258 Completed, 32 Not Started, 28 Cancelled, 15 active):

- **10 threads carry a template `## Objective`** -- 7 Completed, 2 Cancelled, 1 WIP.
- **The class is live, not a legacy tail.** The most recent instance was created and completed on the same day, three days before the measurement. Instances also date back four months, so it recurs rather than clusters.
- Recovering an objective after the fact is possible but uneven. Of the nine terminal cases backfilled by hand: two already carried an `## Objective` in their own `design.md` (transcription); four were derivable from `design.md` plus the work-package roll; one was only derivable from git history; one was only explicable from the commit that closed it as done-by-other-work; **and one had no evidence anywhere** -- template design, an `acceptance.md` with empty Acceptance Criteria and Acceptance Tests sections, and no work packages. For that one the record now states that no evidence survives, because a reconstructed objective would read as testimony.

## Proposed Fix

Add a record check at the existing close-gate point in `bin/intent_st`, beside the `ac gate` call, refusing or warning when `info.md` still contains the template Objective placeholder.

**The scope discriminator is the important part of this proposal.** Gate the ST-level `## Objective` in `info.md` and nothing else:

- A sweep for _any_ template placeholder anywhere under `intent/st` in the same project returns **725 files across 239 of the 333 threads**. That number is dominated by `tasks.md`, `impl.md` and `design.md` -- optional companion documents that were simply never used. The template ships those sections with placeholder prose, so an unused document looks filled-in-shape while being empty. **That is the default state of a generated record set, not neglect, and gating on it would fire on 72% of threads and be disabled within a day.**
- The WP-level equivalent (`[Clear statement of what this work package aims to accomplish]` in `WP/NN/info.md`) is a genuine third case at **152 files**, but 49 of the 64 on active threads sit inside a single long-lived container thread. If `intent wp done` gains the same check it should probably warn rather than refuse.

If the check refuses, it needs the same escape the acceptance gate has for deliberately-contract-free units, since some threads legitimately close without ever having been worked -- eg the done-by-other-work case, where the honest record states that rather than an objective.

## Related

- 0006, 0007 -- the acceptance-parser fixes; same close-gate family, and the same lesson that a gate reports on what it was pointed at and is silent about everything else

## Resolutions

{{TBC}}
