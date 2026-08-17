---
id: "0061"
title: the at na help string still spells the state n-a, and the same row's target.no_op still records the pre-fix no-op -- the two fields of that row nobody corrected when the value moved
date: 2026-08-17
reporter: matts
status: CLOSED
severity: low
---

# 0061: the at na help string still spells the state n-a, and the same row's target.no_op still records the pre-fix no-op -- the two fields of that row nobody corrected when the value moved

## Tags

parity, register, surface, help, vocabulary, stale, measured, 0056-residue

## Summary

Issue 0056 is closed: `AtStatus::display()` landed at `d14cd0b5`, four call sites take it, and v2's own linter accepts the v3-generated `acceptance.md` at 4/4 conform (verified by vc at `55e540df`). **Two fields of the `at na` row were not carried with it, and both are in `surface/dispatch-table.json`.**

**One is shipped output.** The row's `help` field is the string clap prints:

```
$ intent at --help
  na     Set a non-test AT to n-a (the doc / eyeball / gate status)

$ intent at na --help
Set a non-test AT to n-a (the doc / eyeball / gate status)
```

After the 0056 fix this is **the only authored site in the estate where the wire spelling reaches a human**. Swept across `native/` and `surface/` at `55e540df`, every other `n-a` is legitimate: `legacy.rs:608` (the liberal reader), `model.rs:754` (the serde rename itself), `transitions.rs:518` and the event payload (the wire, correctly), `view_determinism.rs:374` (the negative assertion), and prose in this thread's own records describing the defect.

**The other is a stale measurement.** The same row's `target.no_op` reads:

> `` `ok: <AT> already n-a`, exit 0 -- **SHIPPED at `d0f345b5`** ``

HEAD does not print that. `facade.rs:1962-1968` returns `Outcome::AlreadyThere { state: from.display().to_string() }`, so the self-loop now says `already n/a`. The field describes `d0f345b5` and is labelled as describing HEAD.

## Reproduction

At `55e540df`, `surface/` clean:

```
$ ./target/release/intent at na --help
Set a non-test AT to n-a (the doc / eyeball / gate status)

$ grep -n 'Set a non-test AT to n-a' surface/dispatch-table.json surface/dispatch-table.md
surface/dispatch-table.json:2468
surface/dispatch-table.md:1006
surface/dispatch-table.md:1119
```

One authored home, two generated faces -- so the fix is one field and a regeneration, not three edits.

## Root Cause

**A correction reached the fields that were reported and stopped there**, and the row says so about itself. `observed.notes` on this same row, written by ic hours before the fix landed:

> **A correction has to reach every field that repeats the value, not the field that was reported.**

That sentence was written about `observed.notes` itself, which had carried the v3 token in the column that records v2. The rule was stated correctly, applied to the field that had been found, and **the `help` field two keys above it repeats the same value and was not looked at.** A rule discovered while fixing one field does not sweep the others unless someone sweeps them.

**For `target.no_op` the mechanism is different and worth separating: the field was CORRECT when written and its subject moved.** It cites `d0f345b5`, which is honest provenance; what makes it stale is the register's own invariant that `target` describes HEAD. The `no_op_note` states it in capitals:

> **EVERY `target` VALUE DESCRIBES HEAD, AND EVERY ONE OF THEM IS NOW MEASURED RATHER THAN READ.**

**A field that cites the commit it was measured at cannot say whether that commit is still HEAD.** The citation makes the value auditable and does nothing to make it current -- which is the `instant` variant in `parity.md`, at rest.

## Impact

**Low, and it is worth saying why rather than just grading it.** Nothing here enters a file, fails a lint, or blocks a close. The `help` string is read by a person choosing a verb and by any agent reading the surface; it tells them the state is called `n-a` when every other surface of that command -- the movement line, the no-op line, `at list`, the generated row, and every authored row in the estate -- now says `n/a`. That is a documentation defect in the tool's own output, one command wide.

**The `target.no_op` staleness matters more than its severity suggests, because of what consumes it.** `literal_stdout_parity.rs` executes `stdout_exact` templates; `no_op` is prose and is executed by nothing. So a stale `no_op` is invisible in exactly the way `observed.stdout` was until ic wired it -- **and this row is the one that taught that lesson.**

## Proposed Fix

**The `help` field: one edit plus `gen_dispatch_table.sh`.** The state word is `n/a`; the row is `disposition: keep` and v2 has no per-verb help line for `na` to be faithful to (`intent at` prints `red|green|na <stid> <atid>  Set AT status`), so this is v3's own authored string and there is nothing to break by correcting it.

**The `target.no_op` field: re-measure and restate, do not edit the token in place.** The value should be obtained the way every other `no_op` in this register was -- by driving the verb twice through a real binary -- and should cite `d14cd0b5` or later. Rewriting `n-a` to `n/a` by hand would produce a correct-looking value nothing measured, which is the failure mode this register has already been bitten by twice.

**The class-level fix, offered and not insisted on: nothing joins a `target` field to the commit it describes.** Each such field cites its own measurement commit, which is right and is not enough -- there is no check that says "this row was measured before HEAD moved past the file that produces it." A cheap version exists: for each row carrying a measurement sha, refuse if `git log <sha>..HEAD -- <the row's implementing paths>` is non-empty. It would have flagged this row the moment `d14cd0b5` landed. **Whether that is worth building is a judgement about how often the register lags, and nobody has measured that** -- so it is recorded here as the shape of the answer rather than proposed as work.

## Related

- ST0056 -- Intent v3.0.0
- 0056 -- CLOSED; this is its residue, split out because the mechanism differs (an authored literal, not a rendering through `enum_str`)
- `surface/dispatch-table.json:2468` -- the `help` field, the one authored site left
- `surface/dispatch-table.md:1006,1119` -- its two generated faces
- `native/rust/crates/intentsvcs/src/facade.rs:1962-1968` -- what the self-loop actually prints at HEAD
- `parity.md` -- the `instant` variant, of which the stale `target.no_op` is the at-rest form

## Resolutions

{{TBC}}
