---
id: "0047"
title: renaming the NotStarted display arm reds neither surface -- a machine ratification moved st new to Triage and silently defanged the assertions that still read as though they pin the vocabulary
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0047: renaming the NotStarted display arm reds neither surface -- a machine ratification moved st new to Triage and silently defanged the assertions that still read as though they pin the vocabulary

## Tags

test-coverage, vocabulary, model, views, render, defanged-assertion, measured

## Summary

`ThreadStatus::display()` is the single home for the human-facing status vocabulary (issue 0041). **Changing the `NotStarted` arm's string to anything at all reds neither surface's tests.** Both `cli_end_to_end` (the terminal) and `facade_st_wp` (the committed markdown) stay green on 19 of 19 and 10 of 10.

The same mutation applied to the `Wip` arm reds both, correctly and immediately. So this is not a missing test file or a broken harness -- it is two specific assertions that read as though they pin the vocabulary and no longer do, for two different reasons, neither of which is visible at the assertion.

Found by vc, 2026-08-17, while running the canary that issue 0041's Proposed Fix asked for before closing it.

## Reproduction

A `git archive` extract at `9361c68a`, compiled clean. One arm mutated at a time, **each mutation confirmed to differ from the original before anything was run** -- a mutant that fails to apply is indistinguishable from a passing test -- and a restored control at the end.

| mutation                                  | `cli_end_to_end` (terminal) | `facade_st_wp` (committed md) |
| ----------------------------------------- | --------------------------- | ----------------------------- |
| `ThreadStatus::Wip` -> `ZZ_MUTANT`        | **FAILED, 2 of 19**         | **FAILED, 1 of 10**           |
| `ThreadStatus::NotStarted` -> `ZZ_MUTANT` | ok, 19 of 19                | ok, 10 of 10                  |
| control (restored)                        | ok, 19 of 19                | ok, 10 of 10                  |

**The `Wip` row is the positive control and it is what makes the `NotStarted` row mean anything.** It proves the harness detects a one-arm rename on both surfaces when the assertions actually traverse the arm.

**Coverage stated rather than implied: two of nine arms were measured.** `ThreadStatus` has six and `WpStatus` has three; this run covered `Wip` and `NotStarted` on `ThreadStatus` only. **`Triage`, `On Hold`, `Completed`, `Cancelled` and all three `WpStatus` arms are unmeasured** -- not known-good, unmeasured. The mechanisms below are arm-agnostic, so the expected number of unguarded arms is higher than one and nobody should read this issue as being about a single string.

## Root Cause

**Two independent mechanisms, each of which defangs an assertion without changing it.**

**1. The views-side assertion pins the arm negatively.** `facade_st_wp.rs:296` reads:

```rust
after.contains("WIP") && !after.contains("Not Started")
```

It asserts that after starting a thread the view says WIP and no longer says Not Started. **A rename satisfies the second clause trivially** -- if the string becomes `ZZ_MUTANT`, then `Not Started` is indeed absent, and the assertion passes for the wrong reason. Only the `WIP` half is a real pin, which is exactly why the `Wip` mutant reds this file and the `NotStarted` mutant does not.

**2. The terminal-side assertion no longer traverses the arm.** `cli_end_to_end.rs:786` reads `listed.contains("Not Started")`, which looks like a direct pin. But `cli_end_to_end.rs:142` records the change that removed its reach: **`Triage` rather than `Not Started` since the machines were ratified** -- `st new` now creates a thread in `Triage`, so the fixture this assertion runs against never puts a thread into `NotStarted` in the first place.

**The ratification was correct and the assertion was correct, and nothing compared them.** A state-machine change altered which states a fixture visits; the display assertion that depended on that traversal was not re-examined, because nothing connects "which states does this fixture reach" to "which strings does this test pin". Both files still read as intentional coverage.

**This is the class cc named as their sharpest watch-out on 2026-08-16, arriving in a different material**: a guard can be named for the exact defect it lets through. There it was `exit_codes.rs` asserting `code != 2 || ...` where the first disjunct was always true. Here it is two assertions whose names and shapes describe a pin they no longer apply.

## Impact

**The vocabulary that 0041 consolidated into one home is only partly guarded in that home, and the unguarded direction is the one that reaches committed artefacts.**

0041's Impact section is the live risk and it survives its own fix: the md views are committed, so a views-side drift lands in git, propagates to every clone, and appears in a diff nobody attributes to a status rename. A rename of `NotStarted` today is a green build, a green suite, and a changed `steel_threads.md` -- which is precisely the outcome 0041 existed to prevent, now reachable through the tests rather than through the duplication.

**Not claimed: that anything is currently wrong.** Every arm's string is correct today. This is a hole in what would catch a future change, and the severity reflects that.

**Not claimed either: that the consolidation made this worse.** Before 0041 each copy was pinned by its own test; those tests are the same ones measured here, so the `NotStarted` arm was very likely already unguarded on at least the views side. The consolidation did not create the hole -- it made the hole visible, by giving the canary a single place to aim at.

## Proposed Fix

**One table-driven test over every arm of both enums, asserting each string reaches both surfaces positively.**

1. Enumerate all six `ThreadStatus` arms and all three `WpStatus` arms. For each, drive a thread or work package into that state through the facade and assert the arm's `display()` string appears in the committed view AND in the terminal listing. Positive assertions only -- a negative assertion cannot pin a spelling, which is mechanism 1 above.
2. The canary for the test itself: mutating any single arm must red it. Nine mutations, nine reds. Today two of nine are known to red on one arm and neither on the other.
3. **Fix `facade_st_wp.rs:296` in place rather than only adding coverage beside it.** Leaving a negative assertion that reads as a pin is how the next reader concludes the arm is covered.

**Consider whether the fixture-traversal problem generalises.** Mechanism 2 is not about display strings: it is that a ratified machine change altered which states fixtures reach, and no instrument reports which assertions lost their traversal. If other assertions depended on `st new` landing in `NotStarted`, they are silently vacuous too. That sweep is not this fix and may be worth its own issue.

## Related

- 0041 -- the Highlander finding this was discovered while closing; its Proposed Fix 3 asked for exactly the canary that surfaced this
- 0033 -- unrelated mechanism, same shape: a thing that reports success while doing nothing
- `IN-AG-NO-SILENT-001` -- a test that has stopped testing is a failure that does not surface
- `transitions.rs` / `data-model.md` -- the ratified machines whose `Triage` change removed the terminal assertion's reach
- D02 -- a file is entirely authored or entirely generated; the views are committed, which is why the drift direction is asymmetric

## Resolutions

{{TBC}}
