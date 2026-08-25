# cc -- localfold 2026-08-25 20:53Z

Four sections cut from the live board, verbatim. **ST0057's gate closed at 67 of 67 today**, so the row these were about is done.

- **THE DENOMINATOR IS IN DOUBT** -- ic's find, and it was right: greening limb 1's four ADDRESS forms would not have closed limb 1, because they were never in its population. **Resolved by vc ruling the axis, not by anyone building the four forms.**
- **OUTPUT SHAPE** -- landed `3c2f50d6` + `5473a5cc`.
- **THE ATTRIBUTION SWEEP** -- retracted by hv. Kept verbatim because the retraction is the record; the method lesson survives on the live board.
- **AC-08.5 CLOSED AND THE GATE WITH IT** -- the closing record. The gate figure and the durable lessons stay live; this is how it got there.

---

## THE DENOMINATOR IS IN DOUBT, AND THE FIND IS ic's -- 2026-08-25

**`Issue.body` IS A DECLARED SCHEMA FIELD THAT NEITHER DOOR CAN WRITE.** Verified both halves at source: `0077.json` carries `body: ''` with `body` among its declared keys, and **`settable_fields` matches only `Thread`/`Wp`/`Ac`/`At` -- `Issue` falls to the `other =>` arm and is refused by name.** `issues add` takes `<TITLE>` and `--severity` only.

**AND THE SWEEP CALLS `E::Issue { .. }` `Reachable`, WHICH IS TRUE.** `put` reaches it and refuses BY NAME, so the door exists. **That is an ADDRESS-axis fact. AC-08.5's subject is _every writable FIELD of every entity_.** So this is a field-axis gap on a form the gate's own instrument marks green, **and the instrument cannot see it by construction** -- `declared_reach` answers a question about addresses.

**SAME SHAPE AS THE DOOR-BLINDNESS vc RULED, ONE AXIS OVER: an instrument internally consistent, correct in its own printed output, and scoped to a NARROWER QUESTION than the criterion it serves.** And `Issue` will not be the only one -- **every form the sweep calls `Reachable` on refusal-by-name grounds is unexamined on the field axis.**

**CONSEQUENCE FOR MY OWN BUILD, AND IT IS WHY THIS IS AT THE TOP: closing limb 1's four ADDRESS forms would not close limb 1.** Do not write an arm until the denominator is settled. Going in with vc's dispositions; ic filed it as an exhibit and it is more than one.

## OUTPUT SHAPE -- LANDED `3c2f50d6` + `5473a5cc`, AND THE FILED BUG WAS AGAIN THE SMALL HALF -- 2026-08-25

**hv FILED `issues list` IGNORING THE TERMINAL WIDTH. IT WAS NOT IGNORING IT** -- `terminal_width()` existed and `issues list` already called it. **`fill` WAS A MINIMUM WITH NO MAXIMUM, SO ONE OVERSIZED CELL SET THE WIDTH OF EVERY ROW**: 312 columns into an 80-column terminal from a single 287-character title. **BOTH IMPLEMENTATIONS CARRIED THE RULE IN NEAR-IDENTICAL WORDS** -- v2 `content-fit is the floor, so nothing is ever truncated`, v3 `a narrow terminal never truncates, it just stops padding` -- **and neither comment states the consequence.** Not a regression: a shared design decision nobody had re-read. **Same shape as `%04d` being a minimum width, twice in one day.**

**THE WIDEST TITLE WAS MINE**, 287 chars, filed twenty minutes earlier with the whole finding in the TITLE because `Issue.body` is unwritable. Median is 152 and 0043 was already 269, so I was the worst offender rather than the cause -- **but the reason titles run long at all is AC-08.5's denominator surfacing as a UI defect.**

Shipped: `--format={terminal,md,json}` and `--width` through one resolver, replacing four spellings across eleven flags. `--json`/`--markdown` KEPT as aliases -- **v2 parity obligations, which is not the same as a compatibility shim, so fail-forward does not reach them.** A disagreeing pair refuses. `export --format` deliberately NOT collapsed (own projection registry, own refusals); `--quiet`/`--verbose` are volume rather than shape and stayed out; **`todo` correctly has no `--width`** -- it prints a document, so a width would mean wrapping prose and would break it as a persisted artefact.

**FIVE DEFECTS OF MY OWN, EVERY ONE FOUND BY DRIVING AND NONE BY READING.** Three are classes:

- **A DECLARED `default` IS NOT A CHOICE.** Adding `default: terminal` made clap supply `--format` unasked, so every `--markdown` read as two formats and refused. **The refusal was correct about its inputs and wrong about the world.**
- **`value_source` PANICS ON AN UNDECLARED ID WHERE `try_get_one` RETURNS `Err`** -- which is the entire reason `opt` and `flag` exist, stated in their own doc comments. I reached for a sibling API that did not share the property and put a panic on every verb without `--format`.
- **A MUTATION THAT FAILS TO MUTATE READS AS A PASSING CONTROL.** One clip mutation returned GREEN because `sed` silently no-op'd on a pattern containing my own delimiter. **A broken mutation reports the safe-looking answer.**
  And two slips: `--width 0` is the declared contract for _use the terminal_ and my first draft refused it; my clean hand-driven pass was of a binary predating my own next edit.

**THE GAP THAT REACHED hv BEFORE IT REACHED A TEST: `intent issues --width 80` REFUSED WHILE `issues list --width 80` WORKED.** The family row declares `default: list`, so the bare form is documented, shorter, and **the one anybody types first.** A default verb whose flags the default route cannot accept is this change's own inconsistency one level up. `issues` is the only family with a bare table form -- st/wp/ac/at all refuse without a subcommand. Pinned and mutation-proven at `5473a5cc`.

**AND `remedy_coverage` CAUGHT MY NEW ERROR TYPE WITH NO PROOF LINE.** A test that scans source for `thiserror` derives and demands a hand-written roster entry -- **the shape that survives a new module by an author who never read it.**

## THE ATTRIBUTION SWEEP -- RETRACTED BY hv, AND THE METHOD LESSON OUTLIVES ITS SUBJECT -- 2026-08-25

**hv RULED THE `(C)` LINE WAS NEVER REQUIRED**: _this isn't a problem, has never been a problem, and is not something that I suggested we go looking for. The only constraint is that I DO NOT WANT ANY CLAUDE EXHAUST IN MY COMMITS._ **So my census is dropped rather than filed as closed-with-no-action -- a carefully measured fact about nothing is still a fact about nothing.** Only the `^Claude-Session:` half survives, it gates, and it is dc's. Verbatim material in `.history/20260825/wip-fold-1435Z.md`.

**WHAT SURVIVES IS ABOUT METHOD AND IT SURVIVES STRONGER: I APPLIED A CONTROL TO HYPOTHESIS 1 AND NONE TO HYPOTHESIS 2, IN THE SAME MESSAGE -- AND HYPOTHESIS 2 IS THE ONE THAT REACHED A REMEDY.** I nearly sent _a compact drops it_, drove it, and killed it. **Killing the first hypothesis felt like the diligence for that message**, so I reached for a second explanation with the rigour already spent, and nothing in the paragraph distinguished the driven claim from the undriven one. **Running one control was worse than running none, because it vouched for the neighbour it sat beside.** vc falsified the second cleanly: the state my mechanism requires (`(C)` absent, trailer present) has a population of ZERO.

**AND IT LANDED ON THE WORST POSSIBLE OUTPUT.** The undriven half did not stay on my board -- **it went into a fleet message telling vc their remedy was the one thing that would not work.** The unevidenced half was the half carrying the recommendation.

**hv's SCOPE POINT IS THE OUTER LAYER AND IT LANDS ON ALL FOUR OF US: THE FIRST QUESTION TO ASK OF A MEASUREMENT IS WHO ASKED FOR IT.** Nobody asked for any of this. **Measuring an unasked question carefully is more expensive than measuring it badly**, and my scope error and my control error compound rather than sit side by side: if the question had been asked, my unrun hypothesis would still have been wrong; because it was not, being wrong cost four nodes an afternoon.

## AC-08.5 CLOSED AND THE GATE WITH IT -- 67 OF 67 -- 2026-08-25

**vc GREENED IT AT `7652f49a` AND RE-DROVE IT ON THE SHARED PAIR AFTER MY `8957261a`. I DROVE IT MYSELF BOTH TIMES AND NEITHER RUN DISCHARGED THEIRS.** `ac status ST0057` 51/51 + 2 withdrawn PASS; `ac status ST0056/03` 16/16 + 1 withdrawn PASS. **ST0056 overall is 63/133 and is NOT this gate's denominator.**

**LIMB 1** -- three entities refused every field they had, BY FORM, naming none, while the address-axis instrument read green because it answers a different question. **And the refusal gave a reason that was FALSE for two of the three**: an operator refused on an issue address was told they had addressed a collection. `fields_of` is now one exhaustive match, so a fourteenth `Entity` variant fails to COMPILE rather than defaulting into a bucket. **`Node` contributes zero rows because `schema_properties::<Node>()` cannot be written, and whether the model should carry one is HELD WITH hv** -- the partition sizes are pinned so WP-14 reifying it reds this and announces itself.

**LIMB 2** -- the thread door restored four children and let nine scalars default; seven CLI lifecycle verbs across two entities cleared `status_reason` on their way past. **One of the seven was ratified and six were not**, and vc's ruled shape absorbed that without amendment: `st resume` passes `Some("")` and spends it as an ACT rather than a default.

**`st attach` CLOSED THE LAST FIELD-AXIS GAP AND IT IS A DOOR RATHER THAN A DUPLICATE.** `place_attachment` is the single row-placement tail; `put_attachment` decides form by DECODING and reaches `Attachment::opaque`, which was already built and tested. **The condition said _no route to CREATE an opaque attachment_ for an hour and that was false** -- the capability existed and only the CLI route was missing.

**THREE CAVEATS ARE ON THE ROW RATHER THAN IN A MESSAGE**, which ic and I both asked for: issue 0082 (canon-first attachments the working tree does not hold), the refusing round trip that makes it safe rather than lossy, and 0084 untouched.
