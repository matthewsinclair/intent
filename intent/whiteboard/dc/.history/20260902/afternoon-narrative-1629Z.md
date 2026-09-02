# dc -- afternoon narrative, 2026-09-02 1629Z

Pre-fold board beside this file: `wip-prefold-1629Z.md`, sha `59ea73d9`, 54,899 bytes, `cmp` IDENTICAL against the live file at archive time. The board keeps the rules; this keeps the reasoning, so the fold moves narrative and never a rule.

## What the day actually was

hv said boot and hold. The holding lasted about twenty minutes and then four corrections arrived, three of them against me, and the day became a single long lesson about **populations** -- who counted what, with which instrument, and whether anybody could have noticed.

## The through-line: four defects, one shape

**W47 (morning).** vc inferred a store schema of `16` from ancestry, withdrew it, and found I had already written it onto my board for durability. A retracted number with a second witness is harder to retract than the original. My board then turned out to carry a SECOND wrong `16` -- my own, ordinary decay of a once-true reading -- and the two were indistinguishable on the page.

**W48.** vc drove their own fold class against themselves and found an item that FIRST APPEARED in a fold commit and existed at no prior revision anywhere -- question, provenance and a holder list with my name in it. Their id-diff catches what a fold ATE and is structurally blind to what a fold ADDED. I ran the reverse diff on my own 0809Z fold: fourteen lines exonerated as inherited-or-reworded, and one line with no board ancestor at all, sourced from a cross-session message, which leaves the same trace as no origin (`0099`). **Not fabricated, and indistinguishable from fabricated by the control.**

**W49.** Mid-sweep I published a census of 36 and then ran a transform that hit 46, because its regex asked for a pipe into `grep` rather than a pipe into `grep -q`. Reverted whole. The sting: silently widening a fix is exactly what I told hv in August I was declining to do, and the reason I gave then is why it had to be reverted rather than kept for looking harmless.

**W50, and it is the keeper.** The guard hv ruled found 13 sites the moment it existed. `36` was never a measurement -- it was the count of sites matching a PHRASE I coined in August, and my report, my census, hv's ruling on that number, and vc's independent verification all agreed **because they shared a noun.** vc's check re-ran my pattern and returned my answer.

The sentence that survives all four: **nothing downstream of the naming could have caught it, because every later step takes the name as its population -- so more diligence at any later step produces a better-verified wrong number.** It is not a story about carelessness. Each artefact was diligent about the population it had been handed.

Cure, now standing on both boards: **name the MECHANISM, never the INSTANCE.** _An early-exiting reader downstream of a writer under pipefail_, not _printf piped into grep -q_. The first cannot be narrowed by repetition. The second already had been, before it reached hv. vc expects to use it on criteria more than on defects, which is right -- a criterion named after its first instance has the same failure mode and a much longer life.

## The work that landed

`7f1bfb53` -- the ruled sweep, 36 sites, under vc's canon window, carrying one attachment of ic's that completed `7b08e8b1` rather than pre-empting anything, named as such in the message.

`70e2e48c` -- `pipefail_sigpipe_check.sh`, gated, plus the 13 sites it found, plus the roster row, plus the devbin runner wiring, in one commit because `0210` says a new parity file needs its bytes, its roster row and its dispatch together or the tree red-gates for nodes that touched none of it. ic and I both ate that this morning.

Three things the guard got right, and vc named the third as the one they would not have thought to require:

- The two-sided control runs on EVERY invocation and the tool refuses to report on the real population when either arm fails to fire. A control that ran once at authoring is a claim about a revision nobody re-drives.
- It is a member of its own population -- pattern and fixtures assembled from a variable, so the file does not contain the sequence it hunts.
- **It plants the two false-positive families it had already shipped to me within ten minutes**: a comment describing the idiom, and `||`, which is not a pipeline at all and which the first version reported as a finding -- once against a FILE, and once against **the herestring remedy itself.** A guard whose findings include the remedy it recommends teaches the reader to stop believing it, and planting beats avoiding, because avoidance is untested.

## The one that closed itself, and how I was wrong about it

I told hv the rebuild window was outstanding. It was not: the pair had been rebuilt at ic's `4be902e1`, `currency ok`, zero non-test files changed since. I read a true-when-written line off my own board and repeated it two days later while the arm that decides was one command away. vc was carrying the same stale claim. The board line now carries the COMMAND, and says plainly that the pin-versus-HEAD difference is not the arm.

Then hv asked what `flip` and `burn` meant in a compound I had relayed, and I could not say. I had passed on a name without decomposing it -- W50 inside the hour. vc has since decomposed it (`821e7268`): **flip** = rebind the default `INTENT_BIN`, which today points at the v2 shell script while two other bindings exist; **burn** = re-run `burn.sh`, which has hung for 3.5 hours once; and the ordering is load-bearing, because `burn.sh` classifies RELATIVE TO the binding, so a burn taken before the flip is wall time paid twice.

## What I am carrying into the bounce

Nothing in flight. `ST0057 AT-07.5`'s daemon-down slot is the only open item that is mine to want and hv's to grant. Everything else routed.
