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

### PARITY CLASSIFICATION RULED 2026-08-16 (vc, provisional pending hv): `corrected`, NOT `as-observed` -- and the conflict that made it look hard is empty

ic raised this as **two ratified things pointing opposite ways** (`70a52965`): the dispatch table classifies `ac gate` as `as-observed`, whose own note calls it _"the single highest-value parity row in the family"_ because AC-04.3 requires v3 to reproduce v2's gate verdicts -- while this issue says the verdict rule is an accident. **A reader building v3 from the register would port the early-return and be right to.** ic deliberately did not classify it.

**The classification is `corrected`, and the class definition fits verbatim** -- _"a v2 behaviour that is simply wrong and is fixed rather than faithfully reproduced"_. **`as-observed` is for behaviour we CHOOSE to reproduce, and hv's own wording here is that _"the combining rule was chosen by an early-return rather than by a decision"_. You cannot faithfully reproduce a decision nobody made.** Reproducing it is what `parity.md` forbids in its own words: laundering a v2 defect into a v3 requirement, which is precisely the failure an output-equality suite cannot catch.

**AND THE CONFLICT IS EMPTY, WHICH IS THE PART THAT DECIDES IT. Measured 2026-08-16 across all 109 AT rows: exactly TWO ACs carry more than one covering AT, and OR and AND agree on both.**

| AC        | covering ATs      | states             | OR          | AND         |
| --------- | ----------------- | ------------------ | ----------- | ----------- |
| `AC-00.7` | AT-00.5 + AT-00.7 | `red` + `to-write` | unsatisfied | unsatisfied |
| `AC-03.7` | AT-03.7 + AT-03.9 | `green` + `green`  | satisfied   | satisfied   |

**So correcting this changes ZERO verdicts on the current corpus. There is no parity break to ratify and AC-04.3 is untouched.** The two rules can only diverge on a multi-AT AC holding at least one green beside a non-green, and no such row exists.

**THE ACTIONABLE PART IS TIMING: correct it while it is free.** `AC-03.7` is the near exposure -- two greens, so a single regression scores it satisfied on the survivor. **The moment any multi-AT AC goes mixed-with-a-green, the correction starts moving a verdict and reads as a regression rather than a fix.** The window is open and closes by itself.

**Correction to the measurement above, and it is mine.** ic measured `AC-00.7` as both `to-write`; it is now `red` + `to-write` because I moved AT-00.5 an hour before reading their message, making it the contract's first mixed-state multi-AT AC. **The conclusion is unchanged -- neither is green, so nothing diverges -- but ic's stated basis was stale by my hand while they were writing about it.** _A verification is only as current as the thing it read_ is ic's own candidate rule; this is an instance of it against them, caused by me.

**One consequence for authoring, sharper in ic's words than in mine.** I had declined to add a second covering AT row for `class_vocab_check.sh` on the grounds that it could not strengthen an OR gate and would look more rigorous than it is. **Under OR it is worse than neutral: a second covering row is a place a future green can hide a red, so adding rows to a gate that ORs actively LOWERS the bar it appears to raise.** Until this is fixed, naming extra instruments in one row's note is not a workaround -- it is the correct form.

## THE DEFECT IS NOW IN v3 TOO (vc, 2026-08-17)

**v3's close gate ports it.** `crates/intentsvcs/src/contract.rs`, read at HEAD via `git show` with the file confirmed clean:

```rust
pub fn satisfied_by_tests(thread: &Thread, ac_id: &str) -> bool {
  thread
    .tests
    .iter()
    .filter(|t| t.covers.iter().any(|c| c == ac_id))
    .any(|t| t.status == AtStatus::Green)
}
```

`.any`, not `.all`. **A criterion covered by two acceptance tests is satisfied when one of them is green** -- the same rule as v2's early return, expressed as an iterator adaptor instead.

**And the doc comment above it states the defect as the specification**: _"A test-backed AC is satisfied exactly when a covering AT is GREEN."_ That sentence is an accurate description of the code and a false statement of the requirement, which is the worst pairing available -- **anyone checking the code against its own documentation finds agreement.** The class is already recorded on this estate: a check's own prose is where you learn what it does, and nothing compares that prose to what it should do.

### Blast radius on ST0056's own contract: real, and currently latent

Measured at `21b8f8d0`, all **112 of 112** AT rows parsed (an earlier pass silently matched 93 because it required a backticked file and the `(non-test)` rows have none -- a partial parse reporting zero findings is indistinguishable from a clean one, so the script now refuses unless it accounts for every row):

| criterion | covering ATs     | statuses          | `.any` | `.all` |
| --------- | ---------------- | ----------------- | ------ | ------ |
| AC-00.7   | AT-00.5, AT-00.7 | `red`, `to-write` | unsat  | unsat  |
| AC-03.7   | two              | `green`, `green`  | sat    | sat    |

**Zero criteria are currently mixed, so the two predicates agree today.** The defect is live in the code and has not yet bitten this contract.

### But AC-00.7 is one status change away, and what is holding it is a human decision

AC-00.7 has two clauses -- `rusqlite` in exactly one Cargo.toml, **and** the dual-path conformance suite green -- with one AT per clause. `AT-00.7` is `to-write` and lands with WP-08. **`AT-00.5` is a test that PASSES and is held at `red` deliberately**, its own note saying so: _"asserts the rusqlite Highlander ONLY. Held deliberately at partial coverage: it is green (2 tests) and covers half of AC-00.7."_

So the moment anyone moves AT-00.5 to green -- **the obvious, reasonable, tidying-up thing to do for a row whose tests pass** -- `satisfied_by_tests` returns true for AC-00.7 on one of its two covering tests, with the second unwritten and its work package not started.

**The only thing standing between this contract and a falsely satisfied criterion is an author choosing not to mark a passing test green.** That is the refuse-at-partial-coverage discipline doing load-bearing work _because_ the tool cannot. Under `.all` the discipline would be belt-and-braces; under `.any` it is the belt.

## Related

- ST0056 -- surfaced during WP-06 verification; AT-00.5 / AT-06.4 / AT-06.7 carry the interim workaround
- 0028 -- same family: a safety rule whose mechanism does not do what the rule intends
- AC-05.6 -- the precedent for the shape this ruling avoids needing: a v2 assertion failing against v3 for a ratified reason is the harness working. It does not apply here only because the correction moves no verdict at all

## Resolutions

{{TBC}}
