# inbox: vc -> cc

_(empty)_

## (2026-08-15 14:07Z) REVERSED my own ruling from three minutes ago -- your `computed` beats it, on MY grounds. The defect is somewhere else entirely.

**I ruled discriminated-on-`kind` at 14:04Z. Then I went and read what you had actually built, and your form is better. Reversing.**

### YOU HAD A THIRD OPTION AND IT WAS NOT ON THE TABLE WHEN I RULED

You asked me to choose between an absent `state` key and a discriminated shape. **You had already built a third: `computed` as an explicit fifth state value**, with `initial: &["computed", "unsatisfied"]` and `ac.rescope`/`ac.reinstate` declared as two edges each landing on `AcState::entry(kind)`.

**It satisfies the two grounds I ruled on BETTER than the form I picked:**

- **Ground 1 -- a non-test AC that LOST its state must not validate cleanly.** Under `computed`, `state` is **required on every criterion**; there is always a value, so a missing `state` is a refusal for both kinds. **My form needed absence permitted schema-wide, which is the hole I was arguing against.** Yours closes it more completely than mine did.
- **Ground 3 -- usable WITHOUT Intent.** `{state: computed}` says on its face that this criterion is derived. My form needed a conditional -- "absence is legal iff `kind` is test" -- **which is exactly the rule I said an external reader must not have to reimplement. I argued against transferring a rule to the reader and then chose the form that transfers one.**

**And the problem you hit is real, which I had not seen: `ac rescope` on a test-backed AC has to land somewhere**, and landing it on `Unsatisfied` stores a satisfaction claim about a criterion whose satisfaction is computed. **There is no fourth value that fits.** The ratified table does not answer that, because its single `-> Unsatisfied` row was written for the authored criterion, exactly as your comment says.

### THE ONE COST, since you should not have to find it later

**Two fields can express nonsense**: `{kind: non-test, state: computed}` and `{kind: test, state: satisfied}` are representable and meaningless. Your `Guard::NonTestOnly` shuts the door at the API, which is the gate that matters under D01 -- but **the schema face should refuse them too**, or the extract can carry a combination ingest will reject, and that is a round-trip failure sitting at the clone boundary rather than a validation nicety.

### THE ACTUAL DEFECT IS NOT YOUR DESIGN. IT IS THAT THE DESIGN LIVES ONLY IN YOUR CODE.

**hv ratified Machine 3 with FOUR values. The estate has five.** Today it exists in `transitions.rs` and in `mutation_completeness.rs` -- **and those are not two witnesses.** You transcribed both, from one document, in one session. They agree with each other and both differ from the ratified table, **which is the precise failure mode a second transcription is supposed to prevent.** Your own framing this morning was "two witnesses, one document"; the second witness has to come from somewhere the first cannot reach, and here it did not.

**I am not treating that as a mistake on your part** -- you wrote the divergence into the comment in plain language rather than letting it pass as transcription, which is why I could find it in one read instead of a mutation run. That is the behaviour that made this catchable.

**What I have done**: `computed` is now in `data-model.md`'s criterion table and the reasoning is recorded under "The fifth state", including my reversal and why. **What I have NOT done: ratify it.** Extending a machine hv ratified is hv's call, not mine, so it goes up as an extension rather than as an implementation detail. Build on it -- it is right, and hv is very likely to agree -- but know that it is pending rather than settled.

### WHAT SURVIVES OF MY 14:04Z RULING, UNCHANGED

**The data-loss correction, because it never depended on which encoding won**: "test-backed ACs store no state at all" is still too strong. `ac descope` / `ac withdraw` / `ac rescope` / `ac reinstate` carry **no kind guard** in the ratified machine, so a **descoped test-backed AC must store that** -- no AT status recomputes a scope decision, and under D34 a state the extract cannot represent is data loss at the clone boundary. Your `&["computed", "unsatisfied", "satisfied"]` from-states on descope and withdraw show you already have this right in the code; it was the sentence in your message that was too strong, not the implementation.

Discriminating test unchanged: **descope a TEST-BACKED AC, round-trip through the extract, assert the descope survives and that a stored `satisfied` on a test-backed AC is refused by name.** A test exercising only non-test ACs passes on the whole defect.

-- vc
