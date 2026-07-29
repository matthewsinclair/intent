---
id: "0007"
title: intent ac gate reads a green AT as unsatisfied when its status field carries markdown emphasis
date: 2026-07-29
reporter: matts
status: OPEN
severity: high
---

# 0007: intent ac gate reads a green AT as unsatisfied when its status field carries markdown emphasis

## Tags

acceptance, no-silent-errors, cli, sed

## Summary

`intent ac gate` / `intent ac list` report an AC as unsatisfied when its covering AT is green but the AT's ` -- status:` field carries markdown emphasis -- `status: **green`, `status: **green.**`, `status: **green;`. The AT line is otherwise well-formed, the test behind it is green, and nothing warns. The status value is extracted by a `sed` substitution whose character class cannot match `*`, so the substitution never fires and `sed` returns the input line unchanged; the caller then compares that whole line to the literal `green` and concludes not-green.

This is the read-side twin of issue 0006: the same `sed`-non-match-is-invisible shape, in the same file, but in the gate's extraction path rather than in `ac satisfy`'s write path. It is filed high for the same reason 0006 is -- the tool that records verified state reports a false negative about it -- with one aggravating difference. 0006 fails to write a row. This one misreports a row that was written correctly, so the record is right and the gate disagrees with it, which is the harder failure to diagnose.

## Reproduction

Observed live in the Lamplight project, 2026-07-29, on `ST0276` -- **11 of that thread's 26 ACs read unsatisfied for this reason alone**. Minimal shape, two AT lines differing only in emphasis:

```
- AT-01.1 path/to/test.exs::"some assertion" -- covers AC-01.1 -- status: green
- AT-01.2 path/to/test.exs::"some assertion" -- covers AC-01.2 -- status: **green; mutation-proved rather than red-first**
```

Then:

```
$ intent ac list ST0276
ac: AC-01.1  covered-by: AT-01.1  satisfied: yes
ac: AC-01.2  covered-by: AT-01.2  satisfied: no
```

No warning is emitted for the second row. `warn_malformed` (line 64) validates only the `AC-<n>.<n>` / `AT-<n>.<n>` id prefix via `malformed_acat`, not the status vocabulary, so a line with an unparseable status is well-formed by its lights and passes silently.

The correlation is total across the live estate: every AT written `status: green` gates satisfied, every AT written `status: **green` gates unsatisfied, with no other discriminator between them.

## Root Cause

`bin/intent_acceptance`:

- **`at_status()` line 40** extracts the value as
  `echo "$1" | sed -E 's/.* -- status: ([a-z/-]+).*/\1/'`.
  The capture class `[a-z/-]` cannot match `*`. On `status: **green` the pattern matches nothing, and `sed` with a non-matching `s///` prints the input line unmodified -- so `at_status` returns the entire AT line as if it were a status token, rather than failing.
- **`ac_is_satisfied()` line 164** consumes it as
  `[ "$(at_status "$atline")" = "green" ] && return 0`.
  Comparing a whole AT line to `green` is false, so the AC falls through to unsatisfied with no diagnostic.

Neither layer can distinguish "the status is not green" from "the status did not parse", because the extraction function is total by accident: it always returns a string, and its failure value is a plausible-looking one.

The same latent shape exists in `at_covers()` (line 41) and `line_id()` (line 37), which use the identical print-the-input-on-non-match idiom. `ac_flag()` (line 42) is not affected in practice: its class `[a-z]+` is preceded by `-- satisfied:` and trailed by `.*`, so a qualified marker such as `satisfied: yes (cases 1-3)` still captures `yes` correctly.

## Impact

- **A thread reads less complete than it is, with no signal.** On `ST0276` the gate reported 13/26; 11 of the 13 were this defect and the thread is substantively at 24/26. The two genuinely-open rows were indistinguishable from the eleven false ones without reading the parser.
- **It costs human sequencing decisions, not just a diagnostic cycle.** The understated gate is what routed `ST0276` for triage in the first place, on the reading that a thread the owner believed complete was showing half its criteria open.
- **It is invisible to every existing guard.** `warn_malformed` does not check status vocabulary, `intent st repair` reports the thread clean, and the AT line looks correct to a human reader -- the emphasis reads as ordinary prose formatting, because everywhere else in these contracts it is.
- The failure is silent in both directions of authorship: nothing stops an author writing the bolded form, and nothing tells a reader that is why the gate disagrees with the record.

## Proposed Fix

Three parts. The first is the general one and would catch this class regardless of which field drifts.

1. **Make extraction partial and let it fail loudly.** Have `at_status` (and the sibling extractors at lines 37-41) detect the non-match rather than returning the input -- eg test the line against the expected shape first, and return empty or a sentinel when it does not match. A caller comparing against `green` then sees "no status" instead of a full line masquerading as a token. This is the No-Silent form (`IN-AG-NO-SILENT-001`) and it survives any future pattern drift.
2. **Report an unparseable status as malformed.** Extend `warn_malformed` / `malformed_acat` to flag an AT line whose status is not one of the declared vocabulary (`to-write | red | green | n/a`), so the author is told the marker is wrong rather than left to infer it from a gate that will not move. This is the same recommendation 0006 makes for the `(non-test` marker, and both are instances of "a field the parser rejects should be named, never ignored".
3. **Optionally, tolerate emphasis on read.** Stripping `*`, `_` and backticks before matching would make the existing contracts parse as written. Worth considering but strictly secondary to 1 and 2 -- leniency without a diagnostic just moves the silence, and the vocabulary is documented in every contract's own preamble (`AT status vocabulary: to-write (red-first) | red | green | n/a`).

Note for whoever fixes it: the affected contracts in a consumer estate need correcting too, since fixing the parser alone will flip previously-blocked rows to satisfied all at once. In the Lamplight estate the sweep is small and bounded -- `ST0276` (11 rows, live) plus three completed threads carrying a different out-of-vocabulary value (`ST0298` `GREEN`, `ST0270` `BOTH`, `ST0198` `BUILT`).

## Related

- `0006` -- the write-side twin in the same file (`ac satisfy` reports ok having written nothing). Same `sed`-non-match-is-invisible root shape; fixing them together is natural, and proposed fix 2 here is the same diagnostic 0006 asks for.
- Lamplight `ST0276` -- the live reproduction, 11 rows.
- Lamplight `ST0298` `GREEN`, `ST0270` `BOTH`, `ST0198` `BUILT` -- out-of-vocabulary status values on completed threads; understating harmlessly today, but the same silent path.
- `IN-AG-NO-SILENT-001` -- the principle this violates.

## Resolutions

{{TBC}}
