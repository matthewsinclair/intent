---
id: "0014"
title: An AT covers exactly one AC and the parse fails silently: two ids, or a possessive, leaves the AC reading as uncovered
date: 2026-08-04
reporter: matts
status: OPEN
severity: medium
---

# 0014: An AT covers exactly one AC and the parse fails silently: two ids, or a possessive, leaves the AC reading as uncovered

## Tags

acceptance, parsing, silent-failure

## Summary

The coverage link between an acceptance test and an acceptance criterion is derived by scanning the AT's line in `acceptance.md` for a `covers AC-NN.N` token. Two natural ways of writing that line produce **no coverage at all**, with no warning:

1. Naming two ACs -- `covers AC-09.2 and AC-04.3` -- links only the first.
2. Any possessive or adjacent word fused to the id -- `covers AC-09.1's city half` -- links neither.

In both cases `intent ac list` reports the affected AC as `covered-by: -`, which renders **identically to never having written the AT at all**. The AT itself is listed, green, and looks fine.

## Reproduction

Observed on Laksa ST0086, 2026-08-04. Written in `acceptance.md`:

```
- AT-09.1 `.../snorkeltoast_catalog_test.exs` -- covers AC-09.1's city and artist halves ... -- status: green
- AT-09.2 `.../snorkeltoast_test.exs` -- covers AC-09.2 and AC-04.3: ... -- status: green
```

Both ATs were real, both green, both registered:

```
$ intent at list ST0086
at: AT-09.1  test/laksa_web/themes/snorkeltoast_catalog_test.exs  green
at: AT-09.2  test/laksa_web/themes/snorkeltoast_test.exs          green
```

But:

```
$ intent ac list ST0086 | grep -E "AC-04.3|AC-09"
ac: AC-04.3  covered-by: -        satisfied: no     # named second in AT-09.2
ac: AC-09.1  covered-by: -        satisfied: no     # possessive broke the match
ac: AC-09.2  covered-by: AT-09.2  satisfied: yes    # named first, bare id
```

Rewriting to one bare id per AT, and splitting AT-09.2 into AT-09.2 and AT-09.3, fixed all three immediately.

## Root Cause

Two assumptions in the scan, neither stated anywhere the author of an `acceptance.md` line would see:

- **One AC per AT.** Reasonable as a modelling choice -- arguably the right one -- but it is not documented and not enforced, so the natural reading of "this test covers these two criteria" produces a half-linked contract.
- **The id is matched as a bare token.** `AC-09.1's` does not match. Presumably neither would `AC-09.1,` or `(AC-09.1)`.

The unifying defect is that **a failed match is indistinguishable from an absent AT**. Nothing errors, nothing warns, and the failure only surfaces later, at the close gate, as a coverage number that is quietly too low.

## Impact

- **Silent under-coverage.** A thread can carry ATs that were written, run and greened, while its contract reports the ACs as uncovered. The work is done and the record says it is not.
- **It surfaces late and looks like something else.** The first symptom is `intent wp done` refusing to close a package -- which reads as "the work is not finished" rather than "the line is phrased wrongly".
- **Likely widespread and invisible.** Nobody would grep for it, because nothing indicates anything is wrong. Any project with multi-AC AT lines has been silently under-reporting coverage for as long as they have existed.
- **It punishes the more careful author.** Writing a descriptive AT line -- naming what it covers in prose -- is what breaks the match. A terse line works.

## Proposed Fix

Two changes, and the first matters more than the second:

1. **Fail loudly.** If an AT line contains no parseable `covers AC-NN.N`, that is an error at parse time, not a shrug. Likewise, if it contains **more than one** AC id and the model permits only one, say so and name both:

   ```
   warn: AT-09.2 names 2 acceptance criteria (AC-09.2, AC-04.3); an AT covers exactly one.
         Split it, or the second will read as uncovered.
   ```

2. **Match the id at a token boundary rather than by adjacency**, so `AC-09.1's`, `AC-09.1,` and `(AC-09.1)` all resolve. The strictness buys nothing; it only rejects prose.

Worth also documenting "one AT covers one AC" wherever the `acceptance.md` format is described, since the constraint is invisible until violated.

## Related

- 0013 -- `intent ac` has no descope verb. Same family: acceptance-contract state that the tool models more simply than the practice needs, and which fails quietly rather than loudly.
- Found on Laksa ST0086 while closing WP-04, WP-09 and WP-10.

## Resolutions

{{TBC}}
