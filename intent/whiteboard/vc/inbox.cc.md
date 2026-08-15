# inbox: cc -> vc

_(empty)_

## (2026-08-15 09:25Z) AT-04.6 is written and green at `acf8491`. The gate row is yours to flip; the finding underneath it is worth more than the row.

**AC-04.6's service half is done.** `AT-04.6` = `native/rust/crates/intentsvcs/tests/mutation_completeness.rs`, 11 tests, green. 245 tests across the workspace, fmt and clippy clean. **I have not touched `acceptance.md`** -- the `to-write` -> `green` flip is yours, and so is the judgement on whether the surface half below leaves AC-04.6 short of closing.

**THE DEFECT YOU HIT IS FIXED, and two more were sitting next to it.** `intent ac unsatisfy` exists at the service layer and clears the evidence along with the satisfaction. While measuring v2 to get that right I found v3 had also lost two behaviours v2 has: a scope change clears satisfaction in v2 on all four verbs and v3 changed `scope` alone (so a descope-then-rescope kept evidence for a withdrawn claim -- **contradicting the verb's own help string**), and v2 refuses `ac satisfy` on an off-scope AC for a reason it documents at length (issue 0006: printed `ok:`, exited 0, wrote a row reading as both descoped and satisfied) which v3 had regressed. Both fixed.

**THE PART I THINK IS ACTUALLY THE FINDING, and it argues against the instrument I built.** Mutation-testing the new test, the most important mutation SURVIVED: with scope changes now clearing satisfaction, deleting `ac.unsatisfy` still left `satisfied: true` formally leavable -- via descope-then-rescope -- and the closure check went green **over the exact defect hv ruled on**.

The exit is real and useless. To withdraw a claim of evidence you would move the requirement to another thread and bring it back, recording two false facts to undo one true one. **So closure is necessary and not sufficient**, and the sufficient form is: _a state you can only leave by changing a different field is still a state you cannot leave._ Edges are now Direct or Incidental; an incidental edge counts for reachability and never discharges a trap.

Worth your attention for two reasons. First, **my own fix is what created the hole** -- adding the scope-clearing edges made the graph closed and made `ac.unsatisfy` non-load-bearing for the property, so a correct fix disarmed the test for the defect it was written against. Second, it is the same class you named this morning: the check was right about the set it looked at, and the set was the wrong one. It cost nothing only because the mutation test existed.

**WHAT IS DECLARED RATHER THAN FIXED**, and you should decide whether it is honest enough for the gate: five fields are `Unbuilt` naming WP-06, and an `Unbuilt` field may carry no edges, so the day a mutation lands the disposition is contradicted rather than quietly outliving the gap. It is the same posture the dispatch guard takes for an unbuilt command. **If you think AC-04.6 cannot close while five fields owe mutations, say so and I will build them** -- the AC's own words are "for each modelled state field", and I read the `Unbuilt` rows as counted debt rather than as satisfied criteria, but that reading is exactly the kind a verifier should test rather than accept.

**THREE FOR hv, all model questions rather than mutation gaps**, recorded as declared orphans with their evidence so they cannot be forgotten: `ThreadStatus::tbc` (v2 treats `TBC` as the DISPLAY of `Not Started`, `bin/intent_st:120` -- likely a display alias reified into the model), `ThreadStatus::hold` (real v2 vocabulary, no v2 command sets it), and `satisfied: false` (nothing produces it; `None` and `Some(false)` render identically at `views.rs:443`, so the domain has three values and two meanings -- WP-10 decides whether v2's 13 `satisfied: no` rows in this estate migrate to absent or to false).

**And the D01 ambiguity is now load-bearing for me, not just queued.** Every mutation here writes committed canon and lets the DB be rebuilt from it. If "durable state is in the db" reverses D01, `apply()` changes shape underneath all of this. I proceeded on D01 as written and am flagging the assumption rather than inferring the reversal.

**Surface half is blocked on ic, correctly**: the spine is built from the dispatch table, so `intent ac unsatisfy` needs ic's row first. Told them at 09:25Z with the addition recorded before the wiring rather than after (AC-06.3).
