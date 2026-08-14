---
id: "0024"
title: at lint and ac gate accept a WP scope and silently drop it -- a scoped --fix rewrites rows outside the scope
date: 2026-08-14
reporter: matts
status: CLOSED
severity: high
---

# 0024: at lint and ac gate accept a WP scope and silently drop it -- a scoped --fix rewrites rows outside the scope

## Tags

acceptance, at-lint, close-gate, scope, out-of-scope-mutation, consumer-report

## Summary

`intent at lint <ID>/NN` and `intent ac gate <ID>/NN` both accept a work-package scope -- `at lint`'s own usage line documents it -- and both ignore it. Every AT row in the thread is linted, counted and reported under a question that named one work package. The reporting half makes a finished WP read as blocked by rows it does not own; the mutating half is worse, because `--fix` under a scope rewrites rows belonging to work packages the user did not name.

Reported from Lamplight (their `7f5c0bd9a`), where four nodes are working an estate of 1639 AT rows. They filed it as a close-gate defect. It is not: the location is `at_lint_report`, which serves both `at lint` and `ac gate`, so the gate merely inherits it -- and the `--fix` half was not in their report at all.

## Reproduction

Against this repo's own ST0056 (60 AT rows over 12 WPs), the control and the defect side by side:

```
$ intent at list ST0056/02   ->  6 rows      # scope honoured
$ intent at list ST0056/05   ->  4 rows      # scope honoured
$ intent at lint ST0056/02   ->  lint: ST0056 ok -- 60 AT row(s) conform
$ intent at lint ST0056/05   ->  lint: ST0056 ok -- 60 AT row(s) conform
```

Two tells in the lint line: the row count is the whole thread's, and the subject is echoed as bare `ST0056` rather than the scope it was given.

The mutating half, run in a sacrificial fixture (`ST9001`, two WPs, one AT row each; WP-01's row unbackticked and therefore fixable, WP-02's already conforming). **The ask was to fix WP-02:**

```
$ intent at lint ST9001/02 --fix
fix: 1 AT row(s) rewritten in intent/st/ST9001/acceptance.md
lint: L3 AT-01.1 ...
lint: L3 AT-02.1 ...
lint: ST9001 FAILED -- 2 finding(s) over 2 AT row(s)

$ diff before.md intent/st/ST9001/acceptance.md
19c19
< - AT-01.1 test/wp01_test.exs -- covers AC-01.1 -- status: green
---
> - AT-01.1 `test/wp01_test.exs` -- covers AC-01.1 -- status: green
```

WP-02 needed no fix and got none. The single row rewritten belongs to **WP-01**, which the scope excluded. Findings were reported for both.

## Root Cause

`cmd_at_lint` calls `resolve_or_die`, which sets `$WP_NUM`, and then never consults it. Three separate consumers of the scope drop it:

- `at_lint_fix "$acc"` -- signature takes the file only; the loop is `while ... done < "$acc"`, ie every `- AT-` line in the file.
- `at_lint_report "$acc" "$completed" "$stream"` -- same shape, iterates `at_lines "$acc"` with no `in_wp_filter` call.
- the success/failure lines print `$ST_ID`, not the scope.

`cmd_ac_gate`'s AC loop does call `in_wp_filter "$lid" || continue`, which is why the two halves of one verdict disagree: the AC half narrows, the AT-lint half does not. `in_wp_filter` already returns 0 when `$WP_NUM` is empty, so it is safe to call on the thread-scoped path.

## Impact

1. **Out-of-scope mutation.** A scoped `--fix` rewrites rows the scope excluded. In a large estate this is the failure mode v2.19.0 was built to prevent, reappearing at a different seam: not a lossy fix this time, but one that does more than it was told, silently. Someone narrowing a `--fix` to one WP _precisely in order to be careful_ gets a thread-wide rewrite.
2. **A finished WP cannot close.** Lamplight's ST0264/15 is 4 ACs satisfied, 4 ATs green, zero findings on its own rows, and the gate returns the thread's verdict -- BLOCKED, 32 findings over 36 rows -- byte-identical for WP-02, WP-06, WP-08 and WP-15. Their ic escalated it at 09:00Z as needing a waiver. No waiver is needed; the WP was never blocked.
3. **A BLOCKED verdict from `ac gate <ID>/NN` is not evidence about that WP** and must not be quoted as such until this is fixed. Same family as the count-vs-capability class: an instrument that accepts a narrowing argument and then answers the wider question reads exactly like a correct answer.

## Proposed Fix

Thread the resolved scope through both functions rather than filtering at the call site, so the two entry points cannot drift apart again:

- `at_lint_report` -- apply `in_wp_filter "$id" || continue` **before** `AT_LINT_ROWS` is incremented, so the reported row count narrows with the findings.
- `at_lint_fix` -- same filter on the `- AT-` arm, so a scoped fix rewrites only in-scope rows.
- both output lines -- print the scope actually resolved, not `$ST_ID`, so the subject of a count is never wider than the count.

Guard: a fixture thread with two WPs where only the out-of-scope WP has a fixable row. Mutation-test it -- remove the filter, confirm the guard goes red, restore. A guard scoped to what is already clean would certify the status quo here, because the thread-scoped path passes either way.

## Related

- ST0056 -- v3.0.0; the parity contract (WP-01/WP-05) should carry scope-honouring as an explicit parity property, and the reified model makes this class unconstructible rather than guarded.
- 0017 -- the AT row grammar, `at lint` and `--fix`, which this scopes.
- Lamplight `7f5c0bd9a`, `3cc9d6a82` -- the consumer report and the ruling that surfaced it.

## Resolutions

**Settled here, because it was raised as an open contradiction and is not one.** Lamplight's cc recorded owing Intent two corrections, one being that `intent at lint --fix` measured byte-unchanged on four of their threads, which they read as contradicting the filing that `--fix` is lossy -- "a no-op cannot lose anything, so one of the two claims is wrong and I have not settled which."

Both claims are true; they measure different revisions of the tool.

- **Byte-unchanged is correct for the shipped `--fix`.** It rewrites only rows that fail `at_row_arm`, and refuses the two shapes that dominate their estate (`path::"name"` citations and multi-file `+` citations -- 1158 of 1639 rows, 70%). Verified in a fixture built from exactly those two shapes: `fix: 0 AT row(s) rewritten`, both rows byte-identical, with the L1 message explaining the refusal. A thread made entirely of refused shapes is a no-op by design.
- **Lossy is correct about the original `--fix`**, before it was hardened three times -- the version that destroyed 87 test-name links in Intent's own contracts, recovered from `ee44f63`.

Neither claim named the revision it was measured against, which is why they appeared to collide. Same lesson as the stale row count and the stale green claim: a measured figure that does not name its subject and revision is a rumour with a decimal point.

**Fixed.** `in_wp_filter` -- which already no-ops on an empty `WP_NUM` -- is now called in both AT loops, the way the gate's AC loop always did:

- `at_lint_report`: the filter runs BEFORE `AT_LINT_ROWS` is incremented, so the denominator narrows with the findings. Fixes the reporting half for all five call sites at once, `ac gate` included.
- `at_lint_fix`: out-of-scope rows are copied **verbatim** rather than skipped -- the loop rewrites the whole file, so dropping the line would delete it.
- Both lint lines and the gate's remedy print `target_label` (`<ST>` or `<ST>/NN`) instead of `$ST_ID`.

Verified against the reproductions that established the defect: `at lint ST0056/{02,05,09}` now reports 6 / 4 / 4 rows under their own names, matching `at list` exactly, with the thread-scoped path unchanged at 60. In the fixture, `at lint ST9001/02 --fix` rewrites 0 rows and leaves WP-01 byte-identical, while `ST9001/01 --fix` still migrates its own row.

**Guard**: `tests/unit/at_lint_wp_scope.bats`, 5 tests. The fixable row deliberately sits in the OUT-of-scope work package -- a fixture with it in scope would pass with or without the filter and certify the status quo.

**Mutation-proven**, three mutations in a sacrificial worktree, each hard-checked to have actually applied before the run:

| Mutation                             | Expected red | Result   |
| ------------------------------------ | ------------ | -------- |
| filter removed from `at_lint_report` | 1, 5         | 1, 5 red |
| filter removed from `at_lint_fix`    | 3            | 3 red    |
| subject reverted to `$ST_ID`         | 1            | 1 red    |

Restored: 5/5 green. Neighbours green and unchanged -- `at_grammar_lint` 19, `acceptance_close_gate` 16, `intent_acceptance_cli` 12, `ac_offscope_states` 12, `st_new_acceptance` 3.

**One process note, and it is the reason the mutation pass earns its keep.** Test 5's negative assertion was first written `echo "$output" | grep -q "..." && false || true`, which can never fail -- the trailing `|| true` swallows it. It passed the baseline and passed M1, a mutation that should have killed it. It was caught only because a mutation failed to produce a red it was supposed to produce, and rewritten as a bare `[[ "$output" != *...* ]]`. A vacuous assertion is invisible to every run except the one designed to break it.
