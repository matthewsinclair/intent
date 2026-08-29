# Fleet ingest-damage survey -- Intent leg

owner: ic
started: 2026-08-29 12:03Z
scope: MEASURE ONLY. Nothing repaired, nothing staged for repair.
estates measured: 1 of 17 (Intent). **The other 16 are UNMEASURED, not clean.**

## The finding that changes the survey's reach

**The v2 comparison source survives in git history on any git-tracked estate, whether or not anyone preserved a copy.**

`legacy.rs:1273` reads `dir.join("acceptance.md")`. That is the ingest's own input path, and the v3 generated view now sits at **the same path** -- the ingest overwrote its own input. Conflab's `acceptance.v2.md` was a hand-preserved COPY, not the parser's input, which is why it looked like the only estate with a source.

So vc's framing -- that class (i)'s population question "lives or dies on finding an estate that kept a v2 bucket" -- is answerable more widely than it looked. Recovering the source is `git log` on `intent/st/<ID>/acceptance.md`, taking the newest blob that carries `-- status:`/`-- satisfied:` rows and lacks the v3 GENERATED VIEW banner.

**This is a capability claim, and it is measured on ONE estate.** It rests on the ingest reading a tracked path, so it fails on any estate that was not git-tracked before its hop, or that squashed the pre-hop history away. **Neither condition has been checked anywhere.**

## What was measured on Intent

14 threads carry a recoverable v2 source: **615 authored rows, 615 matched to a canon record, 0 rows absent.**

Compared against **canon as it stood at the hop commit** (`16048f82`), not against today's canon:

| thread group                      | rows | findings |
| --------------------------------- | ---- | -------- |
| ST0043-ST0055 (12 frozen threads) | 285  | 0        |
| ST0057                            | 86   | 0        |
| ST0056                            | 244  | 2        |

**Class (i) / `0124` on Intent: ZERO CONFIRMED in 615 rows.** The two remaining findings are both in one row (`ST0056/AC-00.10`) and are **CONFOUNDED, not attributed** -- see below.

**Why comparing against TODAY's canon is the wrong instrument, measured rather than assumed:** the same check against current canon returns 112 findings, 98 of them in ST0057. Against canon-at-hop ST0057 returns **0**. Those 98 were eleven days of subsequent authoring, not parser damage. **An instrument pointed at today's store measures the authors, not the ingest.**

## The one unresolved row, stated as unresolved

`ST0056/AC-00.10`: authored row 7182 chars on a single line; canon-at-hop text **15215 chars**. Two authored spans are absent from canon (`the mtime-ordering pair at 709/711`, `die-calls 42`) while neighbouring spans survive.

**It is NOT the `0126` splice.** The splice signature is a span appearing twice in one field; the longest repeated span in this record at a 100-char floor is **none**. So the +8033 characters are not duplication.

The likely reading is the hop boundary: ST0056 is the thread ABOUT the v3 rewrite and was under continuous edit on the two days either side of the hop, so its canon record almost certainly moved ahead of the last authored `acceptance.md` blob. **I cannot separate that from parser damage with the evidence I have, so it is reported CONFOUNDED and attributed to nothing.**

This row is also the estate's worst `0129` specimen independently of the above: **22 pseudo-segments** on one row, because the author wrote `--` inside their prose and the v2 grammar reads every one as a field boundary.

## Class coverage -- what this leg did NOT measure

| class  | on Intent                | why                                                                                                          |
| ------ | ------------------------ | ------------------------------------------------------------------------------------------------------------ |
| `0124` | **0 confirmed / 615**    | measured, source-based, control-verified                                                                     |
| `0126` | **UNMEASURED**           | detector built and control-verified, but it has **12 false positives / 416** on this estate -- see below     |
| `0127` | **UNMEASURED, not zero** | an ABSENT field, not a short one. Intent's schema has **no `note` field at all** and zero `legacy.raw` stubs |
| `0129` | present, uncounted       | the 22-segment row above is one specimen; no population sweep was run                                        |

## The splice detector fires on healthy prose, and that governs its fleet use

`tools/splice_scan.py` finds a span repeated within one field -- self-contained, needing no comparison source, which is why it was built first. It discriminates on a planted 5-arm control (1 positive fires; clean same-vocabulary prose, a sub-floor repeat, a 120-char rule, and a markdown table all stay silent).

**On Intent's real store it returns 12 hits in 416 fields, and the ones read are authored repetition, not splices** -- a quoted error string deliberately used twice in different sentences. Intent's rows are also the wrong corpus for it: they were ingested by a grammar (`-- evidence:`/`-- satisfied:`) that differs from Conflab's (`-- status:`), and `note()` keys on `-- status:`.

**So the count is an upper bound with a high false-positive rate in this house style, and taking it to 16 estates unread would manufacture findings.** It needs the structural discriminators from `0126` -- unbalanced parens, a field opening mid-sentence, the duplicate sitting in a trailing qualifier-fold segment -- before it is a fleet instrument.

## Instrument defects caught by controls, recorded because the next reader inherits them

- The splice detector fired on a **120-char run of `-`**: two non-overlapping halves of one run are a repeated substring by the letter. Markdown rules and table delimiters are the same shape. Fixed with a prose-diversity guard; both arms kept as standing controls.
- The conservation checker's first run reported **23 lost segments on ST0055 and all 23 were its own artifacts** -- `covers AC-01.1` has no colon, and `(non-test)` is canon structure my parser left glued to the text. After the fix: 0. **Had the count been reported unread it would have sent someone hunting damage that does not exist.**
- A history walk treated **"file absent at this commit" as "no generated-view banner"**, breaking on the wrong blob and printing a `fatal:` that the pipeline swallowed. A check that cannot tell CLEAN from NOT PRESENT.
- `git log --all -- '*acceptance.v2.md'` returned nothing on Intent and **the pattern could not match the subject**: Intent's buckets are plain `acceptance.md`. That near-miss is what led to the history finding at the top of this file.

## Next, in value order

1. **Check the recovery method on a SECOND estate.** The whole reach claim rests on one estate's layout.
2. **Give the splice detector its structural discriminators** before any fleet run.
3. **`0127` needs a schema-shaped instrument** -- an absent field is invisible to both tools here.
4. The 16 unmeasured estates. **Not clean. Unmeasured.**

## Cross-estate exchange with conflab-vc (2026-08-29 12:05Z)

conflab-vc validated their delimiter predictor against a byte-verified source covering 100% of Conflab's 274 rows: recall 19/19, specificity 93/93, precision 19/21. **And all 19 rows it finds are LOSSLESS rotations** -- the note split at its own embedded delimiter and re-emitted tail-first, every authored character surviving. Conflab's 2856 genuinely lost characters sit where the predictor does not look: **14 AC evidence clauses lost entirely** (one truncated 1492 -> 407) and **5 AT head-prose spans before `covers`** (598 chars, one losing eleven test-case names).

**So a delimiter count is a count of the REPAIRABLE damage.** On Conflab it finds 19 repairable rows and is silent on 20 unrepairable ones.

### Is Intent's leg blind to the class conflab measured as real? NO -- controlled, not assumed

Planted arm: strip the `evidence` field from one canon record while the authored row still carries it. **The conservation checker FIRES and names the exact evidence text.** So Intent's 0-findings result does cover evidence-clause loss; it is not a silence produced by looking elsewhere.

### conflab-vc's second detector does NOT transfer to Intent, and their own test is what shows it

Their signature -- `kind: non-test` with empty `state.evidence` -- scores **14/14 on Conflab** and **30 hits on Intent**. Reading the hits rather than the count: they are `unsatisfied` and `withdrawn` rows, which carry no evidence **because they are not satisfied yet**. That is a healthy row scoring positive.

Applying their own warning -- measure what a healthy row scores -- the discriminating form is `satisfied` **AND** no evidence:

| non-test criterion state | has evidence | count |
| ------------------------ | ------------ | ----- |
| satisfied                | yes          | 78    |
| unsatisfied              | no           | 27    |
| withdrawn                | no           | 3     |
| **satisfied**            | **no**       | **0** |

**108 of 108 non-test criteria have evidence exactly when their state calls for it, and there are zero candidates.** The raw signature would have reported 30 findings on this estate; the refined one reports 0. **Both numbers come from the same store, and the difference is entirely in whether the hits were read.**

Recommend the refined form for the fleet run, with the state breakdown printed alongside the count -- the breakdown is what shows a reader whether the estate's convention supports the inference at all.
