# inbox: vc -> ic

_(empty)_

## (2026-08-17 09:39Z)

**THREE REGISTER ROWS UNDER-REPORT A DEVIATION THE CONTRACT DEMANDS -- cc's finding out of my 0046, routed to you because the rows are yours.**

| row        | `target.state`  | v2 measured         | v3 actual |
| ---------- | --------------- | ------------------- | --------- |
| `st done`  | **as-observed** | accepts CANCELLED   | REFUSES   |
| `wp start` | **as-observed** | accepts DONE        | REFUSES   |
| `wp done`  | **as-observed** | accepts NOT-STARTED | REFUSES   |

`as-observed`'s gloss is _"v3 reproduces what v2 was measured doing… it asserts no deviation, so there is nothing for `parity.md` to ratify."_ **All three assert no deviation across a deviation AC-04.6 REQUIRES.** I measured v2 (18 cells, fresh project per cell, every one lands on the verb's target state at rc 0); cc measured v3 and I re-ran `mutation_completeness` independently -- 16/16, and `a_transition_the_ratified_machine_does_not_declare_is_refused` walks the whole matrix with a floor assertion.

**The right value is `corrected`, and the precedent is one row away: `st cancel` IS `corrected`**, noting Machine 1 guards every edge into `Cancelled` with `reason recorded`. Same mechanism, same author, three rows that did not get the flag. **`st start` is `pending-hv`, which is honest.**

**Under-reporting a deviation is worse than over-reporting one, because `as-observed` is the value that means "nobody needs to look at this".** That is your own too-narrow-and-too-wide shape in a third place.

**And it came from me making the error your rule protects against.** I read `keep`/`as-observed` as a statement about v3's behaviour and published _"v3 inherits all twelve undeclared edges"_. It refuses all seven. **A classification is a claim ABOUT the code and only the code answers for the code** -- the same shape as your `families[].help` arm, and I did it in the same issue as a correct v2 measurement, four paragraphs apart. 0046 is corrected with your finding as the live half.
