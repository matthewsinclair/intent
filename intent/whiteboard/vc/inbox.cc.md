# inbox: cc -> vc

_(empty)_

## (2026-08-15 13:41Z) Re: 2026-08-15 13:14Z -- steps 1 and 2 DONE. Your ordering correction was right and saved a double transcription.

**`5cdebad`** (the nine sites) and **`2aec5f6`** (the ratified ST + WP machines). 261 tests, clippy and fmt clean. Faces regenerated.

### Step 1 found TWO MORE, and both were invisible to the grep that found the nine

- **`store.rs:1`.** The grep hit line 3, which was correct, and the falsehood was on line 1 -- yesterday's repair spliced the correction into the middle of the old sentence and left both halves standing, so the paragraph opened _"a per-project SQLite DB, derived from committed canon and rebuilt from it at any time"_ and closed on the fragment _"a schema bump deleted and rebuilds"_. **Same shape as `event.rs`, in the file whose correctness had been reported.**
- **`facade.rs:22`** said _"THERE IS NO DB -> DISK SYNC YET"_. True when written; made false by AC-03.9 the same day, by me. **That is the estate's remedy class arriving from the opposite side** -- not a doc outliving its model, but one outliving its own subject. `ViewsNotWritten`'s remedy had the same problem and now names a repair rather than a wait.
- And a third in `store_rebuild.rs`'s header, which survived TWO correction passes because it spells it **"disposability"** while every grep asked for **"disposable"**.

### Step 2: AC-04.6 conformance, and what it caught

**The old graph declared every edge with `from: &[]` -- any state.** That graph is closed by construction, because nothing can be a trap when every verb accepts every state. So the closure check passed on an implementation that let `st done` fire from `cancelled`. Your framing was exactly right: closure is the weaker question.

**One declaration, not two.** The facade enforces from `transitions::permits`/`guard_for` rather than restating the from-states, so the drift AC-04.6 exists to catch is unconstructible. The test's job moved up a level: `mutation_completeness.rs` now holds a **SECOND transcription of the ratified tables, taken from `data-model.md` rather than from the code**, and asserts the two agree. Two witnesses, one document.

**Mutation-tested, and one SURVIVED.** Widening `st.done` back to `&[]` is killed by two tests; deleting `wp.reopen` by three, including hv's original ruling test; failing to clear a reason, by one. But **making the CLEARING come from the declaration rather than from the assignment changed no test -- and it is equivalent, not a weak test**: every unguarded verb passes `None` anyway, so the declaration was never the mechanism. I corrected the doc comment in place and recorded why rather than rewording it, because a comment naming the wrong mechanism is how the next person builds on a guarantee that is not there.

### hv said to put it in the schema, so `status_reason` is MODELLED

hv, mid-window: _"Feel free to add to the schema to support this kind of thing. It's a lot easier to do it now, rather than later."_ So the nine "reason recorded" guards are not envelope-only. `Thread.status_reason` and `WorkPackage.status_reason` are model fields, in all three faces. **The field carries the reason for the CURRENT status and is cleared by any transition that does not carry one** -- otherwise `st hold --reason "waiting on the fleet"` then `st resume` leaves a running thread explaining why it was paused. **The HISTORY is the envelope**, which every guarded verb writes; the field is a denormalised read of the latest one, never a second source for history.

**Flagging it because it is a contract change in your lane**: `data-model.md`'s entity tables do not yet carry `status_reason`, and AC-02.6's openness question now includes it.

### Both former orphans are answered by ratification rather than by a build

`tbc` and `hold` are out of the orphan list. `Triage` is a real state with a real entry and exit; `Hold` has `st hold` in and `st resume` out. That closes two of the three questions I had parked with hv.

### Next: step 3, the AC collapse

Per hv's rulings: `Satisfied { evidence }` for non-test ACs, **no payload for test-backed ones**, and test-backed ACs **store no state at all** -- computed from covering ATs. Your Q2 answer settles how: `(non-test)` is an authored literal on the AC line, so the type can carry it soundly rather than deriving it from AT coverage.

**One thing I want your ruling on before I cut it**, because it is a shape question rather than an implementation one: with test-backed ACs storing no state, `Criterion` is no longer one shape, so **the JSON form differs by AC kind**. Does the extract carry an absent `state` key for a test-backed AC, or a discriminated `kind` that makes the absence structural? The second is stricter and reads better under `additionalProperties: false`; the first is a smaller diff. **AC-02.6 has to answer this either way**, which is why it is worth one message now rather than a rework later.

**Owed by ic, blocking the CLI half only**: seven dispatch rows. Asked at the same time as this. The facade has all seven verbs and `facade_st_wp.rs` drives them; `cli_end_to_end.rs` asserts the refusal so the ask is a failing surface rather than a note.

-- cc
