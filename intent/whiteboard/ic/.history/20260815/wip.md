# ic archive -- 2026-08-15 (session f26f5f7b, post-compact continuation)

Session 4. Opened on "crack on with WP-03" (already Done; the live blocker was WP-05 / AC-05.3) and closed with the register complete and the pending bucket resolvable. Every decision below now lives in a committed artefact; the file that carries it is named.

## What landed

**The register is complete at `cd490be`** -- 98 rows against 98 on-disk `.bats`, zero UNCLASSIFIED, zero TIMEOUT, zero UNSTABLE. `keep` 31 / `pending` 40 / `out-of-scope` 21 / `retire` 5 / `deviate` 1. Committed `f11e200` + `bd5938f`.

**The per-test register at `b697874`** -- `parity/pertest.md`, 487 rows, 40 files split, none refused. `keep` 238 / `out-of-scope` 196 / `deviate` 47 / `UNCLASSIFIED` 6. The 238 keeps match the independently-derived burn total exactly.

**The sweep reproduced byte-identically on a second independent run.** Determinism demonstrated, not assumed.

**Four shared libraries, each created because a SECOND consumer appeared, never on speculation:** `lib_corpus.sh` (corpus coverage + burn-cell arithmetic), `lib_mdfmt.sh` (the table aligner), `lib_classify.sh` (no-burn rules + the OVERRIDES table). Both extractions were proved faithful by regenerating byte-identically.

## The defect chain, in the order it unspooled

It started as a check on vc's diagnosis and became eight findings. Worth keeping as a chain because no single one was reachable from the previous session's plan.

1. **`burn-baseline.tsv` could not reproduce the register it was the provenance for** -- 94 rows against 97. Three files landed after the baseline. Found by doing the check vc's set-difference skipped: comparing file CONTENT, not just the two file SETS.
2. **`gen_register.sh` had no `TIMEOUT` arm and no default arm.** The timeout added that morning to stop a sweep failing silently had installed a second silent failure one stage downstream. Proven against the pre-edit generator: 2 rows out for a 3-file TSV.
3. **The summary claimed "all N tests pass" from a template, not the data.**
4. **Unmeasured tests were averaged into the ratio as zeroes** -- "no measurement exists" reported as "does not reach the CLI".
5. **`coverage_map.sh` SKIPPED files absent from the baseline**, so the three missing files left the arithmetic entirely under a confident verdict. Two consumers, two different wrong behaviours, one missing check.
6. **The register was not idempotent through the formatter** -- 232 differing lines on a regeneration of identical data. Which then exposed that **the summary counted classes by reading `$OUT`**, still holding the PREVIOUS register.
7. **bats unescapes a test name before printing it to TAP**, so `\$INTENT_HOME` in source never matched `$INTENT_HOME` in TAP.
8. **An escaped pipe in a cell was treated as a column boundary** by the aligner, corrupting the row while leaving it looking like a table.

## Decisions, each now carried by a file

- **A guard verified in one harness is verified in THAT harness.** `corpus_require` was green under `set -uo` and DEAD under `set -euo pipefail` -- exiting 1 with empty stderr against a baseline known to be four files short. Now carried by `lib_corpus.sh`'s header and the watch-out on the live board.
- **Read the bytes before theorising about them.** On the escaped pipe I read the symptom (a tally reporting class `yes`), concluded the row was fine and only the count wrong, and was wrong: `od -c` showed the pipe gone and replaced by padding. The DATA was corrupted. Carried by `lib_mdfmt.sh`.
- **When an argument disposes of a concern, check whether it disposes of the NEIGHBOURING one before conceding the neighbour.** Told cc their third-level finding qualified `bats_coverage`. It did not -- same v2/v3 argument that saved the register saves it. I applied the argument once and stopped one line early. Corrected at `06a316c`.
- **The burn ratio is blind to negative-assertion tests, one-directionally.** Now the eighth measurement rule in `parity.md`.
- **Provenance is emitted, never hand-copied.** Carried by `gen_register.sh`, which now writes the baseline it used.

## Peer traffic

vc corrected their own AC-05.3 miscount and was right about it; the check they skipped found the defect underneath. cc landed WP-06 surface and found the third level unbuildable by the spine -- a real v3 defect that touches none of my v2 figures. Told both. `ab351a2` swept my uncommitted MODULES.md row into cc's commit: `--only` gives no protection on a shared file.
