---
id: "0006"
title: intent ac satisfy reports ok while writing nothing when the AC row has no satisfied field
date: 2026-07-28
reporter: matts
status: OPEN
severity: high
---

# 0006: intent ac satisfy reports ok while writing nothing when the AC row has no satisfied field

## Tags

acceptance, no-silent-errors, cli, sed

## Summary

`intent ac satisfy <stid> <acid> --evidence "<ref>"` prints `ok: <acid> satisfied by evidence` and exits 0 in cases where it has written nothing at all. The acceptance file is byte-identical afterwards and the gate does not move. It happens whenever the target AC row does not already contain a ` -- evidence:` segment, because the write is a `sed` substitution anchored on that literal and `sed` exits 0 when it matches nothing. The success message is unconditional -- it is printed regardless of whether the substitution fired.

This is a No-Silent violation (`IN-AG-NO-SILENT-001`) in the tool that records verified state, which is why it is filed high rather than medium: the failure mode is a writer reporting success having written nothing.

## Reproduction

Observed live in the Lamplight project, 2026-07-28, on `ST0331` `AC-00.4`.

Any AC row that is a non-test criterion but carries no evidence tail reproduces it. Minimal shape -- a row with the exact `(non-test)` marker and no ` -- evidence:` segment:

```
- AC-00.4 (non-test) some criterion whose text ends without an evidence tail
```

Then:

```
$ intent ac satisfy ST0331 AC-00.4 --evidence "some ref"
ok: AC-00.4 satisfied by evidence

$ intent ac gate ST0331
gate: ST0331 BLOCKED -- 30/33 satisfied; unsatisfied: AC-00.2 AC-00.4 AC-06.1
```

The row is unchanged on disk. Adding a well-formed tail by hand (` -- evidence: pending -- satisfied: no`) and re-running the identical command then works, which is what confirms the anchor is the discriminator rather than anything about the AC id or the evidence string.

## Root Cause

`bin/intent_acceptance`:

- **`cmd_ac_satisfy()` line 333** performs the write as
  `replace_line "$acc" "s|^(- ${esc} .*) -- evidence:.*|\1 -- evidence: ${ref_esc} -- satisfied: yes|"`.
  The pattern requires an existing ` -- evidence:` on the row. A row without one matches nothing.
- **`replace_line()` lines 175-184** treats only a non-zero `sed` exit as failure. `sed` exits 0 on zero substitutions, so a no-op write is indistinguishable from a real one at that layer.
- **`cmd_ac_satisfy()` line 334** then prints `ok: $acid satisfied by evidence` unconditionally. Nothing between the substitution and the message consults whether the file changed.

The guards that do exist are correct and fire properly -- `ac_is_nontest` at line 328 and the `--evidence` requirement at line 329 -- so the gap is specifically the absence of a post-write check, not a missing precondition in general.

### Adjacent observation, same function

`ac_is_nontest()` (line 43) matches the literal substring `(non-test)`. A row written `(non-test, ...` -- eg a marker with a parenthetical note folded into it -- fails that match and is classified test-backed. Two consequences worth fixing together:

1. `intent ac gate` then looks for a covering AT, finds none, and reports the AC unsatisfied while **silently ignoring a `satisfied: yes` the row already carries**. The row is well-formed, so `warn_malformed` does not fire either. An AC in that state can never gate green and nothing says why.
2. `cmd_ac_satisfy` line 328 refuses it with `"<acid> is test-backed; satisfaction is computed from a green covering AT"`. That message states a conclusion the parser reached, not the cause the author needs to act on. The author's actual mistake is the marker text.

Also observed live in Lamplight on `ST0329` `AC-10.5`. One instance across that estate, so it is rare -- but it is silent, and the recovery path is invisible from the error message.

## Impact

- A non-test AC can be reported satisfied when it was not, so a close-gate check that follows a `satisfy` looks like it regressed rather than like the write failed. In practice this costs a diagnostic cycle per occurrence; the gate re-read is what surfaces it, which caps the blast radius but only if someone re-reads the gate.
- Scripted or batched use is worse than interactive use: a caller that trusts exit 0 records satisfaction that never landed and gets no signal at all.
- The adjacent marker issue can strand an AC permanently -- satisfied on the row, unsatisfiable through the tool, with no diagnostic naming the marker.

## Proposed Fix

Two parts; the first is the general one and would have caught this class regardless of pattern drift.

1. **Verify the write instead of assuming it.** After `replace_line`, re-read the row and confirm it now reads `satisfied: yes`; error naming the row and the file if it does not. This is the No-Silent form and it survives any future change to the substitution pattern.
2. **Make a bare row satisfiable rather than refusing it.** When the row has no ` -- evidence:` anchor, append ` -- evidence: <ref> -- satisfied: yes` to the end of the line rather than substituting. That makes `ac satisfy` total over the two row shapes non-test ACs actually occur in, so an author never has to hand-craft a tail to make the tool work. If total-ness is not wanted, the alternative is an explicit refusal naming the required shape -- but a silent no-op is not an option either way.

For the adjacent marker issue: report a marker that is `(non-test` but not `(non-test)` as malformed -- either in `warn_malformed` or as a distinct diagnostic -- so the author is told the marker is wrong rather than told the AC is test-backed. A row carrying a `satisfied:` field while being classified test-backed is a reliable tell and could key the warning on its own.

## Related

- Lamplight `ST0331` AC-00.4 -- the live reproduction of the silent no-op write.
- Lamplight `ST0329` AC-10.5 -- the live instance of the adjacent marker issue.
- `IN-AG-NO-SILENT-001` -- the principle this violates.

## Resolutions

{{TBC}}
