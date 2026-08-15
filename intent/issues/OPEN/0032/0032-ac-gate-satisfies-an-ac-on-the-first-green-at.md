---
id: "0032"
title: ac gate satisfies an AC on the FIRST green AT covering it, so a criterion with two acceptance tests scores satisfied on one
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0032: ac gate satisfies an AC on the FIRST green AT covering it, so a criterion with two acceptance tests scores satisfied on one

## Tags

acceptance, close-gate, vacuous-green

## Summary

`bin/intent_acceptance:454` short-circuits: `[ "$(at_status "$atline")" = "green" ] && return 0`. The satisfaction of an acceptance criterion is therefore the **OR** of its covering acceptance tests, not the **AND**. One green AT scores the whole criterion satisfied no matter how many other ATs claim to cover it and no matter what state they are in.

The AT grammar shipped in v2.19.0 permits many ATs per AC and says nothing about how they combine, so the combining rule was chosen by an early-return rather than by a decision.

## Reproduction

In a thread whose `acceptance.md` carries two rows covering one criterion:

```
- AT-06.7 `path/a.rs` -- covers AC-06.7 -- status: green
- AT-06.8 `path/b.rs` -- covers AC-06.7 -- status: to-write
```

`intent ac gate <ST>/06` reports AC-06.7 **satisfied**. Expected: unsatisfied, because a test that covers the criterion has not been written.

Found 2026-08-15 in ST0056 while auditing why two criteria scored satisfied whose own prose said they did not close.

## Root Cause

The lookup walks the AT rows covering an AC and returns success at the first green one. There is no accumulator, so a non-green sibling is unreachable code from the gate's point of view.

Worth separating from the finding that surfaced it: in ST0056 the mis-scored criteria each had a **single** covering AT, green, whose file genuinely passes but whose assertions cover only part of the criterion. **That half is not this bug** -- it is a hand-made coverage claim being wrong, and no gate can catch it. This bug is why the natural repair for that (add a second row at `to-write` naming the missing arm) **does not work**: the green sibling keeps the criterion satisfied, so the honest bookkeeping has no effect on the verdict.

## Impact

An AC decomposed across several tests -- which is the normal shape for a criterion with more than one arm -- reports green as soon as its easiest arm lands. The close-gate then passes a work package on partial evidence, which is the precise failure `intent ac gate` exists to prevent.

It is also silent in the direction that matters: the gate over-reports, never under-reports, so nothing ever fails to draw attention to it. In ST0056 the workaround was to hold a passing test's AT at a non-green status with an explanatory note (`AT-00.5`, `AT-06.4`, `AT-06.7`) -- ie to record a lie about the test in order to get the truth about the criterion. That is the tell that the combining rule is the wrong way round.

## Proposed Fix

AND, not OR: an AC is satisfied when **every** AT covering it is green (and it has at least one). `n/a` rows on non-test ATs keep their current meaning.

AND is strictly more expressive here and is never wrong where OR is right -- every existing single-AT criterion scores identically, and the multi-AT ones start scoring what they say. Once it lands, the three notes above should drop their held statuses and go green, with the missing arms carried as their own rows.

Not fixed at the time of filing: `bin/**` is not mutated in place in this repo while sessions are live, this is v2 tooling that ST0056/WP-04 replaces with the Rust facade, and the interim control (hold the AT, explain it in the note) is in place and visible. Whoever builds the v3 close-gate should read this before porting the v2 semantics across, since porting the early-return would carry the defect into the rewrite.

## Related

- ST0056 -- surfaced during WP-06 verification; AT-00.5 / AT-06.4 / AT-06.7 carry the interim workaround
- 0028 -- same family: a safety rule whose mechanism does not do what the rule intends

## Resolutions

{{TBC}}
