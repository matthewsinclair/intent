# Conformance baseline -- v3 against the v2 BATS estate

Measured at `cb78080` on the core-family subset, via
`tests/conformance/run_v2_suite.bash`. This is a CLASSIFIED baseline, not a
pass rate: a pass rate over a set the measurer chose is a vanity metric, and
the number that matters is how many reds are unclassified.

**The expectation was written before the run** (cc -> vc, before measuring):
_the first honest measurement should be mostly red, and mostly red for
ratified reasons. A harness whose first run is green would be the thing to
distrust, because it would mean it was scoped to what already passes._ The
result matched, so it is recorded as the predicted outcome rather than
discovered as a disappointment.

## The contract this measures against

AC-05.2 as sharpened by vc (2026-08-14), from `design.md:146`: the conformance
contract is **stdout, exit codes and behaviour**. File layout is a ratified
deviation class, decided in advance rather than discovered in triage. A v2 test
may leave the contract ONLY by a register classification of `retire` or
`deviate` recorded at land time for a design-consequence reason -- **never
because it failed.**

## Burn-in

Both files FAIL against `INTENT_BIN=/usr/bin/false`, so both genuinely reach
the CLI and their results are evidence about the v3 binary. A file that stayed
green under a broken binary would be measuring something else.

| File                          | Tests | Failed | Result |
| ----------------------------- | ----- | ------ | ------ |
| `tests/unit/st_commands.bats` | 54    | 52     | RED    |
| `tests/unit/wp_commands.bats` | 30    | 30     | RED    |

## Classification

Four buckets. Only the first is a defect against the narrowed contract.

> **CORRECTION (cc, after vc's D27 ruling).** Bucket 3 below was substantially
> WRONG in the first cut of this file, and the error is worth more than the
> classification it produced. I recorded 35 rows as "verbs the spine dispatches
> and the facade does not implement" -- from the shape of the failing assertion
> (`assert_success`) rather than from reading the tests. `st start`, `st show`,
> `st list`, `st new` and `st cancel` are all WIRED. Reading the tests shows
> what actually fails: **their setup hand-builds a v2 estate** --
> `mkdir -p intent/st/NOT-STARTED/ST0001` with a v2 `info.md` and no
> `thread.json` -- and v3 cannot read an estate nobody's tool wrote.
>
> That is parity.md's **manual-edit workflows** class verbatim: "tests that
> hand-edit structured md and expect the tool to honour it convert to
> mutation-based equivalents or retire (authored-once, D02)". Measured: **26 of
> 54** tests in `st_commands.bats` and **23 of 30** in `wp_commands.bats` build
> an estate by hand in their own body.
>
> So most of bucket 3 is a RATIFIED RETIRE, not pending work -- a better result
> than I first reported, arrived at by checking a classification I had already
> written down. **Classifying by the shape of the failure is a guess that looks
> like a finding**; only reading the test says why it failed.

### 1. Message-text parity -- `deviate`, D27 (16 + 12 rows)

`assert_output_contains` failures where v3 says something different from v2:

| v2 says                            | v3 says                                  |
| ---------------------------------- | ---------------------------------------- |
| `Steel thread command is required` | `'intent st' requires a subcommand ...`  |
| `Steel thread not found: ST9999`   | `no steel thread ST9999 in this project` |
| `Steel thread ID is required`      | (clap's missing-argument text)           |

**RULED D27 (vc, 2026-08-14): message TEXT is not in the parity contract.**
Voice, stdout shape, exit codes, grammar and behavioural semantics are.

The decisive argument is that the other reading is _unsatisfiable_, not that it
is less pleasant: AC-04.4 -- ratified, and WP-04 closed on it -- requires every
facade error to render a remedy with its full cause chain, and
`Steel thread not found: ST9999` has neither. If text were in scope, AC-04.4
and AC-05.2 would contradict for every message v2 wrote without a remedy, which
is most of them. Two ratified ACs cannot contradict, so the reading that creates
the contradiction is wrong.

`deviate`, **not** `corrected`: parity.md:13 -- "deviate is a design consequence
of v3; corrected is a bug fix". v2's message is not wrong, so `corrected` would
misdescribe it. The divergence is forced by AC-04.4, which is what `deviate`
means. Each row cites D27, per parity.md:32's requirement that a deviate row
name a D-number ratified before the port lands.

**No message rewrites. v3's messages stand.**

### 2. File layout -- RATIFIED deviation (3 + 13 rows)

| Assertion                                         | Why it fails                                       |
| ------------------------------------------------- | -------------------------------------------------- |
| `assert_directory_exists intent/st/NOT-STARTED/…` | v3 does not use status directories; status is data |
| `assert_file_contains …/info.md "intent_version"` | `info.md` is a generated view with a banner footer |

`retire`, class = file layout (parity.md). These SHOULD fail; a v3 that passed
them would not have reified the model.

### 3. Manual-edit workflows -- ratified `retire` (the bulk of the rest)

See the correction above. Tests whose own body builds a v2 estate by hand and
then expects the tool to honour it. 26/54 and 23/30 measured.

`retire`, class = manual-edit workflows (parity.md), reason = authored-once
(D02). The v3 equivalent is a mutation-based fixture -- build the estate with
`intent st new` rather than `mkdir` -- which is what `cli_end_to_end.rs`
already does.

### 3b. Genuinely not yet built -- WP-06 (a much smaller set)

`st repair`, `st sync`, `st edit`, `wp show` and the long tail: dispatched by
the spine, not yet wired to the facade.

**Marked `pending`, not `retire`.** Calling a failure a deviation because it
failed is the move the sharpened AC-05.2 fences off, and this is the bucket
where that temptation actually lives.

A defect found while checking this bucket: the renderer answered an unwired
verb with "a steel thread command is required" -- when a command HAD been
given. Reporting a missing command for a present-but-unwired one is the same
same-text-for-different-causes collapse AC-04.4 forbids, and it is why bucket 3
looked like bucket "no command" in the first cut. Fixed: an unwired verb now
names itself and points at WP-06.

### 4. Harness-local (2 rows)

`sed` and `[[` failures inside test setup, not assertions about the CLI.
`pending` until read.

## What this baseline is NOT

It is **not** the whole-estate number. That is AC-00.1's, at the release gate,
and hv runs the full suite externally. Two files out of 98 were run, named
explicitly, and that is stated here so the sample cannot be mistaken for the
population.
