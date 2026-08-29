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

| class  | on Intent                              | why                                                                                                                                                               |
| ------ | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0124` | **0 confirmed / 615**                  | measured, source-based, control-verified                                                                                                                          |
| `0126` | **UNMEASURED**                         | detector built and control-verified, but it has **12 false positives / 416** on this estate -- see below                                                          |
| `0127` | **CLOSED -- the class does not exist** | conflab-vc reconciled 114 authored notes against 114 canon notes, identical sets; the specimen never had an authored note. vc closed it. Dropped from this survey |
| `0129` | present, uncounted                     | the 22-segment row above is one specimen; no population sweep was run                                                                                             |

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

## Corrections taken after this leg was written (2026-08-29 12:06Z)

- **`0127` is CLOSED and dropped.** conflab-vc's full reconciliation found 114 authored notes and 114 canon notes, identical sets -- the class does not exist. I had it as UNMEASURED; it is now _not a class_, which is a better outcome than a measurement.
- **The `0126` mechanism is a two-capture ROTATION, not 3x duplication.** The note splits at its own embedded delimiter and is re-emitted TAIL-FIRST with the delimiter reinserted: grammatical, reordered, complete. The 3x case is the overlap subset. **This matters for my splice detector: a rotation is NOT a repeated span**, so a same-field duplication test is aimed at the subset rather than at the class. The detector needs an order-sensitive comparison to reach the dominant shape.
- **The delimiter predictor is validated and is an UPPER BOUND on the REPAIRABLE class only** -- recall 19/19, specificity 93/93, zero false negatives, and canon-side and authored-side signatures selecting the identical 21 rows. All 19 are lossless. **The two false positives are unexplained, so the delimiter is necessary and not sufficient.**
- **vc independently ran the refined evidence detector on Intent and got 0 of 78**, matching this leg's figure with a fully populated control. Two instruments agreeing, not one being trusted.
- **vc's standing caveat is now answered.** vc wrote that Intent's clean result "may mean the ingest did not damage us, or that we never went through it -- those are different facts and I measured only the first." The history recovery settles it: **Intent DID go through the ingest, and 615 of its authored rows are recoverable.** The clean result is the first fact, not the second.

## Residual coverage test (2026-08-29 12:11Z) -- run because a row-level census can conceal a gap INSIDE the rows

conflab-vc reported their class (i) rising 2856 -> 3422 chars after conflab-ic caught a **segment-level blind spot**: their instrument compared an AT row's head and its note, and prose sitting BETWEEN `covers` and `status` was examined by neither. One row cleared as undamaged had in fact lost 464 characters. They had been reporting "141 of 141 rows compared" and treating it as coverage.

**My headline was the same shape -- "615 rows, 615 matched" -- so it needed the same test.** Their recommended control: subtract every examined segment plus scaffolding from the authored row and measure what is left over.

| accounting, at SEGMENT level      | chars  | share  |
| --------------------------------- | ------ | ------ |
| total authored                    | 321244 | 100%   |
| **VERIFIED present in canon**     | 309473 | 96.34% |
| excluded as STRUCTURE, unverified | 11400  | 3.55%  |
| reported LOST                     | 371    | 0.12%  |
| **RESIDUAL UNEXAMINED**           | **0**  | **0%** |

**Residual zero.** Every authored character falls into a category the checker actually decided on -- the row splits on `--` and each segment is judged, so there is no "between two examined regions" gap of the kind that cost Conflab 464 characters.

**The 3.55% passed without verification is stated as a limit rather than folded into the pass.** Its top forms are `yes (computed)` 137x, `no (computed)` 112x, `yes` 71x, `n/a` 24x, and `covers AC-NN.N` -- state tokens and structural links, with no prose among them. The 30-character bound on `satisfied:`/`status:` values is what keeps prose from hiding behind a structural key.

The 371 lost characters are the single confounded `ST0056/AC-00.10` row.

**And a correction from conflab-ic that belongs in any repair plan: LOSSLESS IS NOT INTACT.** Five of the rotated rows now open mid-sentence on a pronoun whose antecedent sits after it. Every character survives and the row says nothing. **Repairability is a claim about the source, not about whether the row currently reads.**

## THE CORRECTION THAT MATTERS MOST, AND IT DEVALUES THIS LEG'S HEADLINE (2026-08-29 12:14Z)

**Intent is the corpus the ingest parser was FITTED TO. Its zero is the least informative result in the fleet, not a representative one.**

conflab-vc refuted the alternative I was carrying -- that Intent's `-- evidence:`/`-- satisfied:` grammar might never reach a destructive path. **Measured on Conflab: 28 authored AC rows carry an evidence clause, ALL 28 use that same grammar, ZERO use `-- status:`, and 15 of the 28 were damaged.** So the grammar demonstrably reaches a destructive path and grammar immunity is dead.

They named three surviving explanations for Intent's zero: a different parser version, a different call site, or genuinely clean. **It is none of those. It is that Intent was the development corpus.**

- `legacy.rs` was created **2026-08-16**, and its creation commit is titled _"Intent's own estate parses with zero blocking residue"_. Its body: _"**Measured on this repository**: 56 threads, 140 work packages, 280 criteria, 227 acceptance tests, 0 BLOCKING residue"_, and _"TWO DEFECTS OF MY OWN, both found by **running it against the real estate** rather than by reading the spec"_.
- Intent's hop is **2026-08-19**, three days later.
- **Every fix to the acceptance-parsing path is dated 2026-08-26/27** -- a week AFTER Intent's hop -- and they carry `fix(laksa)` / `fix(port)` / `cc(migration)` subjects. They are repairs for defects that only appeared once the parser met estates it had NOT been fitted to.

**So Intent ran the earliest, least-fixed parser and came out clean, because Intent's rows are the rows it was debugged against.** A parser tuned until one estate parses cleanly will parse that estate cleanly. The zero is real and it is very nearly guaranteed.

**CONSEQUENCES FOR THE FLEET SURVEY, and they are the reason this is at the bottom of the file rather than in a footnote:**

1. **Intent must NOT be used as the fleet baseline or as a control estate.** A detector calibrated so that Intent scores clean has been calibrated on the training set.
2. **This leg's zero is not evidence that the fleet is largely clean.** It is evidence about the one estate that cannot be evidence.
3. **The informative estates are the ones migrated AFTER 2026-08-19 and BEFORE the 08-26/27 fixes** -- they ran a parser fitted to a corpus that was not theirs, with the defects still in it. That window is where the damage should concentrate, and identifying which estates fall in it is now the highest-value next measurement, ahead of sweeping all sixteen.

**This is my own board's standing watch-out arriving in my own survey: MEASURE WHAT THE DOCUMENT SHIPS TO, NOT WHAT YOU ARE STANDING IN -- the estate's own configuration hides its bugs from it.** I measured the estate I am standing in and it is the one estate whose result was pre-determined.

### Correction to the sha256 claim, at conflab-vc's request and it is the defensible form

Earlier this file treats their byte-identical check as validating the history-recovery method. **It validates ONE INSTANCE of it.** On Conflab the preserved copy and the parser's input are the same artefact; on any other estate the copy was made by a different hand at a different moment, so nothing carries over. **One validated instance, not a validated method.**

## conflab-vc's mechanism, and the test that CAN tell (2026-08-29 12:16Z)

**The mechanism, measured on all 28 of Conflab's evidence-bearing ACs with perfect separation and both arms populated:**

| authored state   | evidence | count    |
| ---------------- | -------- | -------- |
| `satisfied: no`  | **LOST** | 14 of 14 |
| `satisfied: yes` | KEPT     | 14 of 14 |

**The ingest discards the evidence clause on every criterion authored UNSATISFIED and preserves it on every criterion authored SATISFIED.** The embedded delimiter does not drive it -- 13 of the 14 lost rows contain no delimiter at all.

**THIS INVERTS MY EARLIER REFINEMENT AND conflab-vc WAS RIGHT TO STOP ME USING IT.** I refined the detector to `satisfied` AND no evidence, on the reasoning that the raw form's 30 hits were unsatisfied rows carrying no evidence _because they are not satisfied yet_. **That reading is exactly what this damage produces.** After the ingest, a row whose authored evidence was destroyed is indistinguishable from one that never had any -- the only thing that separated them is the text that is now gone. So `satisfied + no evidence = 0` scores zero **by construction**: satisfied rows are the ones the ingest preserves. My refinement removed the entire exposed population and kept only the immune one.

**The store cannot settle it. The git-history route can, and does.**

Pulling the pre-hop authored row for every Intent non-test criterion with no evidence in canon:

| outcome                                                 | count  |
| ------------------------------------------------------- | ------ |
| authored WITH an evidence clause, now empty -- DAMAGE   | **0**  |
| authored WITHOUT one -- convention, confirmed at source | **14** |
| never passed through the ingest                         | 16     |

**All 16 are accounted for and none is a gap:** ST0058 (canon first appears 2026-08-22), ST0060 (08-25) and ST0066 (08-28) were all created AFTER the 08-19 hop, and `ST0056/AC-14.12` and `ST0057/AC-09.2` were added after it. They were authored natively in v3 and never met the parser.

**So Intent's exposed population is 14 rows, all 14 confirmed authored-without-evidence, zero damaged -- established against the authored source rather than inferred from the store.** This also independently reproduces the conservation checker's zero over 615 rows, which had this class covered by a planted arm. Two instruments, two routes, same answer.

**THE FITTED-CORPUS CAVEAT ABOVE IS UNAFFECTED AND STILL GOVERNS.** A source-confirmed zero on the estate the parser was debugged against is still the estate whose result was pre-determined. **This strengthens the claim about Intent and changes nothing about the fleet.**

## RETRACTED: the date-band triage. And the class is LIVE, not historical (2026-08-29 12:18Z)

**I recommended triaging the fleet by migration date -- estates hopping between 2026-08-19 and the 08-26/27 fixes. conflab-vc refuted it with one counterexample and I am withdrawing it.**

**Conflab hopped 2026-08-28**, one to two days AFTER those fixes, running the most-fixed parser available, and **still lost 15 of 28**. A heuristic of that shape needs exactly one counterexample.

**The consequence is worse than a scoping correction.** conflab-cc measured the evidence drop independently of conflab-vc and reached the same 14/14 both-arms split. That makes it **a deterministic conditional field drop, not a parser accident** -- and deterministic behaviour that survived the fixes is behaviour **any estate hopping TODAY will still get**.

**SO THIS IS AN OPEN DEFECT IN THE SHIPPED TOOL, NOT A MIGRATION CLEANUP.** Every framing in this file above that treats the four classes as historical damage to be surveyed and repaired is wrong on that point. The survey's subject is a live parser.

**THE PREDICTOR THAT REPLACES THE DATE BAND IS THE AUTHORING CONVENTION.** An estate whose unsatisfied non-test criteria carry evidence clauses will lose them; an estate whose authors leave them empty has nothing to lose. That is precisely the difference between Conflab and Intent, and it is **computable from the estate's own authored source without any damage assessment**.

**AND IT EXPLAINS INTENT'S ZERO WITHOUT THE FITTED-CORPUS ARGUMENT.** Intent's convention is to leave them empty -- my own measurement, 14 of 14 authored without one -- so Intent has no exposure regardless of which parser ran. **The fitted-corpus finding remains independently correct and the warning against using Intent as a fleet baseline still stands**, but it is not the explanation for this class, and I had it doing work it does not do.

**Revised next measurement, replacing the date comparison:** for each estate, count unsatisfied non-test criteria whose PRE-HOP AUTHORED row carries an evidence clause. That number is the estate's exposure, it needs no damage assessment, and the history route already makes it reachable everywhere.

## ROOT CAUSE, READ AT SOURCE -- it is the MODEL, not the parser

(2026-08-29 12:22Z. conflab-vc closed the "is it the current build" question by fixture; they explicitly had NOT read the source or tested Intent HEAD, which is the half I can do from here.)

**`legacy.rs` is BYTE-IDENTICAL between the build conflab-vc tested (`8177b53e`) and Intent HEAD.** So their caveat -- that the escalation might narrow to whoever runs that build -- is closed. **It does not narrow.**

```rust
let state = match (verdict, evidence) {
  (true, Some(e)) => AcState::Satisfied { evidence: e },
  _ => AcState::Unsatisfied,
};
```

**The `_` arm swallows `(false, Some(e))`: a criterion authored UNSATISFIED WITH an evidence clause, whose evidence is discarded.** And `AcState::Unsatisfied` is a **unit variant** -- it carries no payload, so **the model has nowhere to put it.**

**THIS IS NOT A PARSING ACCIDENT AND THAT IS WHY THE 08-26/27 FIXES DID NOT TOUCH IT.** Those repaired parsing. This is the data model being unable to represent _unsatisfied, with evidence_. Deterministic by construction, exactly as conflab-cc characterised it from the other end.

**THE COMMENT DIRECTLY ABOVE THE MATCH REASONS CAREFULLY ABOUT THE OTHER CASE AND IS SILENT ON THIS ONE.** It explains at length why `(true, None)` -- satisfied with no evidence -- must arrive `Unsatisfied` rather than have evidence invented for it, citing vc's migration ruling. **`(false, Some(e))` is not mentioned anywhere.** The author reasoned about one member of the `_` arm and the other went with it silently. A wildcard arm is where that happens.

**WHAT THE FIX COSTS, stated because it is not a parser patch.** Preserving the clause means `Unsatisfied` must carry optional evidence -- a change to `AcState`, which is the serialised store shape and the GraphQL `AcStateView`. **That is a schema change and therefore a fleet-wide bump**, the same class dc has already routed to hv via vc on ST0066. **It is hv's call and not a defect anyone should quietly patch.**

**Not claimed:** I have read the source and diffed it; I have NOT re-run conflab-vc's fixture here. Their fixture and this reading are two routes agreeing, not one confirmed twice.

## THIRD ROUTE, AND IT RECLASSIFIES THE DEFECT (2026-08-29 12:25Z)

conflab-vc found the coupling **documented in the CLI's own help text** -- `intent ac unsatisfy --help`: _"Reopen a satisfied non-test AC -- clears satisfaction AND its evidence together"_, printing `ok: AC-01.1 unsatisfied (evidence cleared)`. And they measured `ac new` destroying evidence in a **v3-native store with no ingest anywhere in the picture.**

**Both confirmed at HEAD from source, and `facade.rs` was read from `git show HEAD:` rather than the working tree, because dc's uncommitted ST0066 work is in that file and must not be attributed to shipped behaviour.**

- `ac_unsatisfy` (facade.rs:4630): _"Reopen a non-test criterion: unsatisfied, and its evidence gone with it"_ -- deliberate, reasoned, and **announced to the user**.
- `ac_new` (facade.rs:4449): constructs a **FRESH `Criterion`** with `state: AcState::Unsatisfied` for non-test and `put`s it. **It never reads the existing row.** So calling it on a satisfied criterion overwrites the whole record, evidence included, and the outcome reports `"replaced": true` **with no mention of evidence at all.**

**SO THE DEFECT IS NOT "THE INGEST DROPS A FIELD". It is that v2's authored form could express _unsatisfied, and here is what was measured and what would discharge it_, and the v3 model cannot represent that state at all.** Migration destroys it by construction. **That is a representable-state regression between schema versions**, which is why no parsing fix could have touched it.

**THE ASYMMETRY THAT BELONGS IN THE FILING.** One model constraint, three paths, and only one of them tells the user:

| path           | destroys evidence | announces it                    |
| -------------- | ----------------- | ------------------------------- |
| `ac unsatisfy` | yes               | **yes** -- "(evidence cleared)" |
| `ac new`       | yes               | **no** -- reports "replaced"    |
| v2 migration   | yes, 14 rows      | **no**                          |

**Two of the three are silent, and this estate has a named rule for that: `IN-AG-NO-SILENT-001`.** The contrast is sharper evidence than any path alone, because the announced path proves the destruction is known.

### The connection to cc's open `0131`, which is live work rather than a filing

cc holds an hv GO ruling on `0131`: **a verb named `add`/`new` must REFUSE on an existing key.** cc's own board records that this partly overturns `0119` and that **`ac new` is a question to RAISE, not to settle at the keyboard.**

**`ac new` refusing on an existing key would close this destruction path outright.** That does not settle cc's question and is not mine to settle -- but it changes what the question is about: it stops being a consistency argument about verb naming and becomes **a measured data-loss argument**, with conflab-vc's v3-native fixture as the evidence.

**And the operational sting, in conflab-vc's words: the repair path and the damaged population are the same set.** Every contract defect awaiting a ruling is a sentence fix on an AC; `ac new` is the only verb that can reword one; and the ACs whose evidence migration already destroyed are exactly the unsatisfied ones anyone would reach for.

**This also widens the justification for the schema change.** If `AcState::Unsatisfied` gains optional evidence it fixes all three paths, not the ingest alone -- which is an easier bump to argue than an ingest-only framing.

## CORRECTION: "the fleet is QUEUED to receive it" was FALSE (2026-08-29 12:27Z)

**I wrote that the remaining estates are queued to receive this damage, and told vc it was a sequencing question for hv. THE PORT IS CLOSED.** vc read it from the ledger, itself re-derived from every `intent/.config/config.json` on this machine: **all 16 projects are on 3.0.0** -- Anvil, Baize, Cdsync, Conflab, Courses, Devbin, Intent, Laksa, Lamplight, MicroGPTEx, Molt, Molt-flynn, Molt-matts, Prolix, Riffle, Utilz -- with only `Intentv2` frozen at 2.19.0 by standing rule, and that is the tool tree rather than a project. **Conflab was the last to hop, 2026-08-28. Nothing is queued.**

**This is the worst-shaped error I made today and it is worth naming as a shape rather than a slip.** It was an URGENT claim about a thing that cannot happen, and urgency is exactly what gets a fix sequenced ahead of better work. A false premise that merely misinforms gets corrected; **a false premise carrying urgency reorders someone else's queue before anyone checks it.** I inferred "estates still to hop" from the damage being live and never asked the ledger how many estates were left -- **a one-command question I did not run because the inference felt complete.**

**The re-pointed version, which is vc's and is the one that should stand:**

- For the sixteen, **exposure is already incurred**; the question is ASSESSMENT, not prevention.
- The defect is live in a **SHIPPED TOOL**, so it still reaches any new project migrating from v2, any rollback-and-redo, and **every consumer of Intent v3 who is not us.**

**The escalation is not weakened. It is re-aimed from our fleet to Intent's users**, which is a larger population and one that cannot be surveyed from here.

**vc filed `0133` CRITICAL and independently verified all three load-bearing facts rather than relaying them:** the wildcard at `legacy.rs:1707`, `AcState::Unsatisfied` as a unit variant at `model.rs:1233`, and the empty `git diff 8177b53e..HEAD -- legacy.rs`.

**And vc records that they made the by-construction detector error independently and at the same time, reporting their 0-of-78 to hv as a clean result.** Two nodes reaching the same wrong zero is the trap; conflab-vc's both-arms separation is what broke it. **Agreement between instruments is not evidence when both instruments share the blind spot.**

## The other half of `0131`, measured here (2026-08-29 12:29Z)

conflab-vc found that making `ac new` refuse on an existing key would remove the **only** way to reword a criterion. **Reproduced on Intent rather than relayed:**

| verb                                                                                    | `--text`         |
| --------------------------------------------------------------------------------------- | ---------------- |
| `list` `status` `satisfy` `unsatisfy` `gate` `descope` `rescope` `withdraw` `reinstate` | **0**            |
| `new`                                                                                   | **the only one** |

**There is no `ac edit`.** And Intent's own help calls `new` _"Create a criterion (caller-assigned id, IDEMPOTENT)"_ -- **the upsert is the stated contract, not an implementation accident**, which is why refusing on an existing key is a contract change rather than a defect repair, and presumably why cc's board records `0131` as partly overturning `0119`.

**So after `0131`, a sentence fix on an existing AC stops being blocked on a ruling and starts being blocked on the absence of a verb -- which is worse, because a ruling can arrive and a missing verb cannot** (conflab-vc's line, and it is the argument in one sentence).

**I have sent cc BOTH halves and said so**: the data-loss argument for refusing, and the unwritability argument against refusing alone. I sent the first half an hour earlier, so delivering the correction was mine to do -- **a question that only ever received one half would get answered on half the evidence.** I proposed no answer either time; it is cc's ruling and cc's scope.

One scoping note offered to them as consequence rather than recommendation: **an edit verb touching TEXT ONLY is the first path that goes nowhere near `AcState`**, so it sidesteps the schema bump for the common case and need not wait on the `Unsatisfied`-gains-optional-evidence question queued for hv.
