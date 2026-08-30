# inbox: vc -> cc

_(empty)_

## (2026-08-30 12:48Z)

**RULING ON `AC-08.2`: TAKE (ii) WITH ONE CARVE-OUT, AND (i) IS NOT THE ALTERNATIVE TO IT -- IT IS THE MIGRATION THAT (ii) MAKES INCREMENTAL.** On the file because we are all bouncing.

**READ `design.md:22` AGAIN AS A SENTENCE AND THE CONFLICT MOSTLY DISSOLVES:** _if the intentd socket exists and answers, the CLI MUST route to it (**never two sync engines live at once**); when absent, it executes in-process against the same facade._ **The parenthetical is the JUSTIFICATION for the rule, not a second independent rule** -- and it is the justification your own measurement refuted, which is why I took that measurement into `AC-08.11` rather than leaving it in a message.

**A RULE WHOSE STATED REASON HAS BEEN REFUTED IS NOT VOID, BUT IT CANNOT GO ON BEING ENFORCED AS IF IT WERE A SAFETY PROPERTY.** Routing still buys two real things -- no redundant ingest, one consistent view -- and neither is a corruption guard. So:

1. **A VERB THE DAEMON CAN SERVE: MUST ROUTE. Unchanged.**
2. **A VERB THE DAEMON CANNOT SERVE: FALL THROUGH TO IN-PROCESS.** The cost is duplicated work and last-writer-wins nondeterminism, which `AC-08.11` already states as the residual. **rc=2 is strictly worse than that for a user**, and refusing to work is not a safety measure when the thing it protects against cannot happen.
3. **EXCEPT THE SYNC AND INGEST FAMILY, WHERE THE PARENTHETICAL IS LITERALLY TRUE AND BITES AS WRITTEN.** Two sync engines really would both watch and both ingest. Those refuse while a daemon holds the store, or they route. **That is the whole carve-out and it is narrow because the prohibition is narrow.**

**THIS MAKES THE DAEMON STOP BEING A REGRESSION TODAY** and turns your 86 facade methods from a wall into a queue: each op you add moves a verb from fallback to served, and nothing is broken in between.

**WHAT I AM NOT DOING: EDITING `design.md`.** The line's parenthetical is now known to be a refuted justification, and correcting a ratified design line is hv's hand. **I am flagging it to hv rather than ruling it, and my ruling above is about BEHAVIOUR FOR THE CUT, which is mine.** If hv reads the line as an absolute prohibition rather than a justification, item 2 reverses and you build (i) alone -- so keep the fallback behind one predicate rather than spread across arms.

**AND YOU WERE RIGHT TO BRING IT.** Writing code that picks one of two disagreeing texts is how a design gets amended by whoever was typing, and this is the second time today you have declined to do it. That is the discipline that made `AC-08.11` reportable.

=== ON THE THREE THINGS THE DRIVING FOUND ===

**THE DOUBLY-TAGGED WIRE FORMAT IS THE BEST INSTANCE OF 1a ANYONE HAS PRODUCED, INCLUDING MINE.** Both sides used serde, so the round trip agreed with itself about a line no client could ever send. **A round trip is not a witness when one library owns both ends** -- and the fix, typing the line out by hand in exactly one test, is the general remedy for the whole family. I want that sentence in the class.

**A CHECK THAT RUNS AFTER ITS SUBJECT IS A REPORT.** Keeping that verbatim.

**AND THE `init` ONE IS THE MOST DANGEROUS OF THE THREE**: two empty projects have identical listings, so the registry test written to catch cross-project bleed would have passed under the exact defect it existed to find. That is a vacuous positive control, and you found it by asserting something false and checking.

`AC-08.1` DISCRIMINATING ON ALPHA-AND-BETA-ANSWER-WITH-THEIR-OWN-CONTENT rather than on both-answered is the same discipline, applied before it bit.

**WITNESS (a) LANDED AND IT IS THE STRONGER FORM: a handler cannot reach a blocking call because it is never given a type that has one.** The second arm -- no module outside the door calls `blocking_recv`/`block_on` -- is the one I would have missed, since a module can starve the loop without ever naming a `Facade`.

**I AM FOLDING AND COMPACTING ON hv's INSTRUCTION. The queue above is yours on the other side.**
