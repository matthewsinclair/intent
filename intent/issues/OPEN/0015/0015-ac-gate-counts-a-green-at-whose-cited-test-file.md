---
id: "0015"
title: ac gate counts a GREEN AT whose cited test file does not exist: the citation is never resolved
date: 2026-08-05
reporter: matts
status: OPEN
severity: medium
---

# 0015: ac gate counts a GREEN AT whose cited test file does not exist: the citation is never resolved

## Tags

acceptance, ac-gate, at-coverage, false-green

## Summary

`intent ac gate` decides an AC is satisfied by reading the covering AT's `status:` field. Nothing ever checks that the AT's cited test file exists. So an AT can name a test that was renamed, moved or deleted, keep `status: green`, and the gate keeps counting it as coverage -- reporting a thread closer to done than it is, on the strength of a test that cannot be run.

The citation is parsed. `at_pathname()` (`bin/intent_acceptance:65`) extracts the backticked path, and it is used in exactly one place: printing it in `intent at list` (`:489`). No caller resolves it against the filesystem.

## Reproduction

Verified on a real project (Lamplight, 2026-08-05), not constructed:

```
ST0290 AT-02.2  status=green  native/cli/tests/config_migration.rs
```

That file does not exist. The nearest name on disk is `native/cli/tests/config_profile.rs`, so the test was most likely renamed and the citation was not carried with it.

`intent ac gate ST0290` reports `BLOCKED -- 23/32 satisfied` and **AC-02.2 is not in the unsatisfied list**: the dangling citation is being counted as coverage.

Estate-wide scan of the same project, over 179 AT rows that name a file:

| Class                                     | Count | Verdict                                    |
| ----------------------------------------- | ----- | ------------------------------------------ |
| cited path exists                         | 165   | fine                                       |
| cited path missing, AT is `to-write`      | 5     | **correct** -- the test is not written yet |
| cited path missing, AT is `green`         | 1     | the defect                                 |
| citation is a BARE FILENAME, no directory | 8     | unresolvable by construction (see below)   |

The `to-write` rows matter to the fix: a missing file is the EXPECTED state for an AT that has not been written, so a naive "the path must exist" check would red five correct rows. The rule is narrower -- **a `green` AT must resolve.**

The eight bare filenames (`run_coverage_test.exs`, `validator_test.exs`, ...) are a second, milder case: they carry no directory, so they cannot be resolved at all even though every one of them does exist somewhere in the tree. They are cited in a form the tool could never check.

## Root Cause

`cmd_ac_gate` (`bin/intent_acceptance:375`) walks the AC lines and calls `ac_is_satisfied`, which reads either `satisfied: yes` (non-test ACs) or the covering AT's status. Both are string fields in `acceptance.md`. The AT's path is a third string that no code path validates, so the contract's link to reality is asserted by the author and checked by nobody.

This is a false-green rather than a false-red, which is why it survives: the gate gets _more_ permissive as citations rot, so nothing ever fails to draw attention to it.

## Impact

An ST can reach its done-bar on coverage that does not exist. The failure mode is silent and it compounds -- a renamed test file leaves the AT green forever, and the thread's gate reports a number the tree does not support. It bites hardest exactly where the gate is trusted most: at ST close, where the number is the evidence.

It also degrades over time rather than at a point, so it is invisible to review: nobody re-reads a green AT's path.

## Proposed Fix

Resolve the citation in `warn_malformed` / the gate, gated on status:

- An AT with `status: green` whose cited path does not exist is a **malformed contract line** -- the gate already BLOCKs on those, so this needs no new outcome, only a new check.
- An AT with `status: to-write` (or `na`) is exempt: the file is legitimately absent.
- A citation with no directory component cannot be resolved and should warn on its own terms -- either require a repo-relative path, or resolve by unique basename and fail when it is ambiguous.

Worth pairing with `intent at green`, which is the moment a citation goes load-bearing: refusing to mark an AT green when its path does not exist catches the rename at the point of the lie rather than at the next gate.

## Related

- 0014 -- AT coverage parsing is comma-separated only; the adjacent parsing gap in the same contract
- 0013 -- `intent ac` has no descope verb; the same family of contract-record drift

## Resolutions

{{TBC}}
