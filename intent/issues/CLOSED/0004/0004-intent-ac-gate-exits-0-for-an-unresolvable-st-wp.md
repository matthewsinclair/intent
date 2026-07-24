---
id: "0004"
title: intent ac gate exits 0 for an unresolvable ST/WP: the close-gate passes when it evaluated nothing
date: 2026-07-24
reporter: matts
status: CLOSED
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

FIXED + CLOSED (2026-07-24), shipped in v2.17.3. Items 1-3 of the proposed fix are implemented as specified. Item 4 is NOT, on evidence set out below -- it remains open for an hv ruling as its own issue if wanted.

**1 + 2. One resolver, shared by the whole family.** `resolve_target()` in `bin/intent_acceptance` is now THE target resolver: it takes `<stid>[/NN]`, runs `parse_wp_specifier` (which normalises the ST id and zero-pads the WP), resolves the thread via `resolve_st_dir`, and -- new -- resolves the `/NN` segment via `resolve_wp_dir`, which nothing validated anywhere before. Failure sets `TARGET_REASON` and returns 1. Every subcommand resolves through it: `ac list`, `ac status`, `ac satisfy`, `at list` and `at red|green|na` via the `resolve_or_die` wrapper (error + exit 1), and `ac gate` via a `BLOCKED` line + exit 1. Only the presentation differs, which is the point -- the gate is read by machines through `$?`, the readers by humans through stdout. `acc_path` (which each reader re-entered separately, and which the gate never called at all -- the asymmetry that let the gate lack even the steel-thread check `status` had) is retired in favour of `acc_file` over the already-resolved target.

`resolve_wp_dir` is new in `bin/intent_helpers`, the WP analogue of `resolve_st_dir` and composing with it; registered in MODULES.md. The `WP_DIR="$ST_DIR/WP/$WP_NUM"` derivation was repeated at four sites in `bin/intent_wp`; the three that resolve an existing WP now go through it. The fourth (`wp new`) constructs the path of a WP that does not exist yet, so it correctly keeps its own concatenation.

The full bad-target matrix, all exit 1 where every cell was previously a silent exit 0:

| target       | `ac gate`                            | `ac status` / `ac list` / `at list` |
| ------------ | ------------------------------------ | ------------------------------------- |
| `ST9999/01`  | `BLOCKED -- no such steel thread`    | `Error: no such steel thread: ST9999` |
| `ST0055/99`  | `BLOCKED -- no such work package`    | `Error: no such work package`         |
| `not-an-id`  | `BLOCKED -- no such steel thread`    | `Error: no such steel thread`         |

**3. Resolved-but-empty is already distinguished from unresolvable, and already refused.** ST0048 made an empty contract (zero ACs on a real thread) BLOCK, so the ruling the issue asks for explicitly is on the books: a close-gate over an empty contract is refused, `acceptance: exempt` being the sole declared escape. What was missing was only the layer above -- an unresolvable target -- which is what this fix adds. The two now report differently: `BLOCKED -- acceptance.md has zero acceptance criteria (empty contract)` vs `BLOCKED -- no such steel thread` / `no such work package`.

Also fixed under this item: the gate's silence on success. PASS lines now join EXEMPT and BLOCKED, so no exit path is mute. This is what let the defect survive three releases of daily dogfooding -- the vacuous passes were not exotic, they were invisible, indistinguishable from a verified 23/23. The WP-lenient rollup (ST0044) is preserved but is now granted only to a WP that exists and is announced when taken (`PASS -- no ACs in scope; rolls up to the ST0001 contract`), rather than inferred from a zero count.

**4. NOT actioned -- the premise does not reproduce.** The issue reports `ac status` exiting 1 on `ST0268/05` (11/12, BLOCKED) and 0 on the `0/0` case, and asks for uniform non-zero on BLOCKED. In this tree `ac status` exits 0 for BOTH, so there is no inconsistency to fix:

```
$ intent ac status ST0001      # 1/2, verdict BLOCKED
ac: 1/2 satisfied -- BLOCKED
$ echo $?
0
```

`cmd_ac_status` has no `exit` statement on any path and returns the status of its final `echo`. `tests/unit/intent_acceptance_cli.bats:111` ("ac status reports counts and gate verdict") asserts exactly this, calling `assert_success` on a `0/3 BLOCKED`. The design this encodes looks deliberate and worth keeping: `status` is the human-facing REPORTER whose verdict is its stdout, `gate` is the machine-facing GATE whose verdict is its exit code -- which is why `gate` exists as a separate command at all. Making `status` exit non-zero would break that test and blur the two.

The `ST0290/99` observation behind item 4 is nonetheless fixed, by items 1-2 rather than by an exit-code change: that target no longer reaches the point of printing a misleading `0/0 satisfied -- BLOCKED`, because resolution now fails first with `Error: no such work package: ST0290/99`, exit 1. Flagging for an hv ruling: if you want `status` to gate on its verdict too, that is a deliberate behaviour change to a documented reporter plus a test rewrite, and belongs in its own issue.

**Guards.** Six tests added to `tests/unit/acceptance_close_gate.bats`, each verified RED against the pre-fix binary and green after (the file's own subject matter makes a vacuous test the obvious hazard): unresolvable ST blocks; unresolvable WP blocks; the WP-lenient rollup still passes for a real AC-free WP and is announced (guarding against over-tightening in the other direction); PASS is announced; every reader refuses an unresolvable target rather than reporting an empty set; a non-numeric `/NN` is a clean error.

That last one is a defect found while fixing this: `parse_wp_specifier` fed `$wp_part` to a bare `10#` expansion, so a non-numeric WP aborted with raw bash noise (`10#abc: value too great for base`) rather than an Intent error. It affected `intent wp show|start|done <st>/abc` as well as the `ac`/`at` family, and is fixed once in the shared helper. Pad-tolerance is preserved throughout: `ST0055/3` and `ST0055/03` both resolve.

Regression sweep clean across `acceptance_close_gate`, `intent_acceptance_cli`, `wp_commands`, `st_commands`, `helpers`, `intent_todo`, `st_new_acceptance`, `st_zero_commands`. `intent critic shell` clean on all three changed files; `shellcheck -S warning` adds nothing new (`intent_acceptance` is clean; the four `intent_helpers` hits are pre-existing SC2155s on untouched lines). Canon updated: `intent/docs/working-with-llms.md` D11 records that the rollup requires an existing WP and that every verdict is reported.
