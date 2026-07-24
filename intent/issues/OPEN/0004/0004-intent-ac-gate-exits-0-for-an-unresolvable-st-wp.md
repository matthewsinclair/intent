---
id: "0004"
title: intent ac gate exits 0 for an unresolvable ST/WP: the close-gate passes when it evaluated nothing
date: 2026-07-24
reporter: matts
status: OPEN
severity: high
---

# 0004: intent ac gate exits 0 for an unresolvable ST/WP: the close-gate passes when it evaluated nothing

## Tags

acceptance, ac-gate, ci-gate, no-silent-errors, exit-codes

## Summary

`intent ac gate <stid>[/NN]` is documented as the close-gate -- "exit non-zero + BLOCKED if unsatisfied" -- and is the command a CI step or pre-commit hook would call to refuse a close. It behaves correctly for targets it can resolve: a satisfied WP exits 0 silently, an unsatisfied one prints `gate: <id> BLOCKED -- N/M satisfied; unsatisfied: ...` and exits 1. But for a target it CANNOT resolve -- a non-existent steel thread, a non-existent WP number, or a garbage argument -- it prints nothing and exits 0. The gate reports success for work it never evaluated. A typo in a hook or CI invocation (`ST0290/4` instead of `ST0290/04`, a renamed ST, a WP that was never created) turns the gate into a permanent silent pass, and nothing in the output distinguishes that from a genuine green.

Its sibling `intent ac status` errors correctly on a bogus steel thread (`Error: no such steel thread: ST9999`, exit 1), so the two disagree about the same bad input -- and it is the CI-facing one that is wrong.

## Reproduction

In any Intent project with at least one steel thread (observed in project Lamplight, 2026-07-24, at Intent v2.17.2):

```
$ intent ac gate ST9999/01          # steel thread does not exist
$ echo $?
0                                    # <-- silent pass

$ intent ac gate ST0290/99          # real ST, WP 99 does not exist
$ echo $?
0                                    # <-- silent pass

$ intent ac gate not-an-id          # garbage argument
$ echo $?
0                                    # <-- silent pass
```

Contrast the same bad inputs on the sibling commands, and the correct behaviour on resolvable ones:

| invocation                        | `ac gate`                                | `ac status`                                        | `ac list`         |
| --------------------------------- | ---------------------------------------- | -------------------------------------------------- | ----------------- |
| `ST0290/04` (real, 4/4 satisfied) | exit 0, silent (correct)                 | `ac: 4/4 satisfied -- PASS`, exit 0                | rows, exit 0      |
| `ST0268/05` (real, 11/12)         | `BLOCKED ... AC-05.12`, exit 1 (correct) | `11/12 satisfied -- BLOCKED`, exit 1               | rows, exit 0      |
| `ST9999/01` (bogus ST)            | **exit 0, silent**                       | `Error: no such steel thread: ST9999`, exit 1      | --                |
| `ST0290/99` (real ST, bogus WP)   | **exit 0, silent**                       | **`ac: 0/0 satisfied -- BLOCKED`, exit 0**         | **empty, exit 0** |
| `not-an-id` (garbage)             | **exit 0, silent**                       | `Error: no such steel thread: STnot-an-id`, exit 1 | --                |

Note the `ST0290/99` row: `ac status` prints the verdict `BLOCKED` and still exits 0, which is internally inconsistent with its own `ST0268/05` row (BLOCKED, exit 1).

## Root Cause

An unresolvable target degrades to an EMPTY acceptance-criterion set instead of raising, and every command in the `ac` family then reports its own flavour of vacuous success over that empty set:

- `gate` computes "are there any unsatisfied ACs?" over zero ACs, gets "no", and exits 0. Vacuous truth: nothing unsatisfied because nothing exists.
- `status` reports `0/0 satisfied -- BLOCKED` but exits 0, so the verdict string and the exit code disagree.
- `list` prints zero rows and exits 0, indistinguishable from a contract that legitimately has no ACs yet.

The missing step is target RESOLUTION as a distinct, failable operation ahead of evaluation. `ac status` already has half of it -- it validates the steel-thread id (hence `Error: no such steel thread`) -- but nothing validates the `/NN` work-package segment anywhere in the family, and `gate` does not perform even the steel-thread check its sibling does. So "this target does not exist" and "this target has nothing unsatisfied" are the same state internally, and only the second one is ever reported.

This is the general No-Silent-Errors failure mode applied to a gate: a check that cannot find its subject must fail loudly, never pass quietly. A gate whose green can mean "nothing was evaluated" cannot be trusted as evidence, which is precisely what a close-gate exists to provide.

## Impact

- **A silent false pass in automation.** Any CI job, pre-commit hook, or wrapper script that gates a close on `intent ac gate <id>` passes forever if the id is wrong -- a typo, a renamed steel thread, a WP that was never created, or an id built by string interpolation that produced something unintended. The failure is invisible: no output, exit 0, identical to a real pass.
- **It defeats exactly the assurance the command exists to give.** `gate` is the one command in the family whose entire contract is its exit code. The other two are for humans reading output, who would notice an empty table; the gate is for machines, which cannot.
- **Manual use is misleading too.** An operator checking a WP before closing it sees silence-and-zero and reads "green", when the truth may be "you asked about something that does not exist."
- Blast radius is bounded by how widely the gate is wired into automation. It never blocks valid work and never corrupts state -- the harm is a close that proceeds on evidence that was never actually gathered.

## Proposed Fix

Resolve the target before evaluating it, in ONE place the whole `ac` / `at` family shares (Highlander -- the id-resolution rule should not be re-implemented per subcommand, which is how `gate` came to lack the steel-thread check `status` has):

1. Add a single resolver that takes `<stid>[/NN]` and returns either the resolved contract or a typed "no such steel thread" / "no such work package" error. Validate the `/NN` segment, which nothing validates today.
2. Every `ac` / `at` subcommand calls it and surfaces a resolution failure as a non-zero exit with a named message -- `gate` included, so an unresolvable target is a hard fail, never a pass.
3. Distinguish "resolved, zero ACs declared" from "unresolvable". A real contract with no ACs yet is a legitimate state and should report as such (and arguably should not gate green either -- a close-gate over an empty contract is the same vacuous pass, one level up; worth ruling explicitly rather than leaving implicit).
4. Fix the `status` exit-code inconsistency while in there: `BLOCKED` should exit non-zero uniformly, not 0 for the `0/0` case and 1 for the `11/12` case.

A regression guard should assert the exit code for each bad-target shape (bogus ST, bogus WP, garbage arg) across `gate` / `status` / `list`, since the defect is invisible in stdout and only observable in `$?`.

## Related

- Surfaced in project Lamplight on 2026-07-24 while closing ST0290 WP-04. The gate returned exit 0 with no output; because that is also what a genuine pass looks like, the close was verified instead via `intent ac status ST0290/04` (`4/4 satisfied -- PASS`), which is the substantive signal. The close itself was correct -- the defect changed nothing there, it only meant the gate could not be used as the evidence.
- No related Intent issue. Adjacent in spirit to 0003 (a quality gate reporting a pass for a check it did not perform).

## Resolutions

{{TBC}}
