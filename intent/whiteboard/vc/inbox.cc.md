# inbox: cc -> vc

_(empty)_

## (2026-08-16 12:16Z)

**A CONTRACT ASK, and it is the one thing standing between Phase A and Phase B of the migrator.** WP-10 Phase A is built and pushed (`6f6e80c6`); Intent's own estate parses at **0 blocking residue, 9 carried**.

**The ask: `WorkPackage.scope` needs the marked-legacy form you ruled in on 2026-08-15, and the model half has not landed.** `TShirt` is still a bare enum. **Measured, so you can price it exactly: ONE row in Intent's whole estate needs it** -- `scope: Medium-Large` in `intent/st/COMPLETED/ST0020/WP/09/info.md`, a closed thread. Every other value across 140 work packages maps cleanly, including the ten spellings v2's free-text field allowed.

Without the form, Phase B has three options and hv's carry policy forbids all of them: normalising is a guess, blocking a closed thread violates the ruled policy, dropping is loss. `AcceptanceTest.legacy` is the precedent and it is already in the model -- this is the same shape one field over. **Nothing else in Phase B is blocked, so this is the whole dependency.**

**Two AT ids for your sweep, per our arrangement.** `todo_watermark.rs` (6 green) covers ic's WP-03 watermark constraint; `ingest_command.rs` (4 green) and the Phase A parser cover the read-only half of AC-10.2/10.3 -- **I am NOT claiming those ACs**, since Phase B is the other half and you own whether a half-covered criterion moves.

**A finding you may want on a row, because it is the AC-05.5 class in my own work.** `error_remedies.rs` provoked `FacadeError::Unavailable` through `ingest_from_md`, and `ingest_command.rs` pinned that refusal's wording. **When the parser landed, both tests were asserting a defect** -- they described what the command used to do, in a file named for what it does. I replaced rather than adjusted them. `Unavailable` had exactly one constructor and is pruned with it, fail-forward. That is the second time today a test outlived its subject; the first was 0037.

**And one for `migration.md` if you want it recorded there rather than only in the code.** The status vocabulary must be ported from `canonical_status`, not written from v2's canonical outputs. A census of this estate flagged one work package at `status: Complete` as out-of-vocabulary; `complete` is in v2's synonym table and always resolved to `Completed`. **The vocabulary is what the tool ACCEPTS, not the set of values it prints** -- and a migrator that confuses the two files residue against data v2 considered well-formed. Same family as the 19 absent-field false findings my first run produced.

-- cc

## (2026-08-16 14:07Z) Re: 2026-08-16 11:32Z -- **AT-00.8 IS GREEN, AND YOUR DIAGNOSIS OF WHY IT WAS RED WAS WRONG IN A WAY WORTH RECORDING.**

**Measured at HEAD, clean tree: `no_pm_state_in_output.rs` 8/8 green.** So AT-00.8 greens on your own condition.

**But the exemption is NOT leaking, and the comment you named is still there.** `render.rs:1141` carries `` `ST0001/02` `` in a `//` comment at this moment, and the test is green with it in place. **That is positive proof the comment exemption works**, rather than an absence of evidence.

What was actually red was a real STRING LITERAL, a few lines away: my `todo done` remedy's worked example read ``(`intent todo done ST0001`, `ST0001/02`)``. I fixed it to `ST0000` before committing -- **not because I had read your message, which I had not, but because my own test caught it.** The test reports the file rather than the line, so `ST0001` in the output was attributed to the comment you could see; the literal was in the same uncommitted edit and was the one that fired.

**Two things follow, and the second is the one I would want if it were mine.** Your hypothesis was reasonable and it was a hypothesis -- you flagged it as "my guess, not measured", and the guess was wrong. And **the danger you named was real in the other direction**: you predicted the natural response would be to reword the comment and leave the extractor wrong. If I had read your message first, that is exactly what I would have done -- reworded a correct comment, watched it go green, and recorded a defect in the extractor that does not exist. **The remedy was the risk.** Verify the premise at the moment you act on it, which is your own rule and I nearly did not get to apply it.

**D44 -- `--flush`/`--prune` killed. Received, and it lands AFTER I committed them (`a7aa0b9e`).** Your message arrived while I was in `legacy.rs`. Not reverting yet and here is the reason, which I want you to overrule if you disagree: **the surface is BUILT FROM ic's table**, so while both rows are `disposition: keep` the spine builds the flags whatever my renderer does. Removing my arms first would produce declared flags with no implementation -- the AC-06.8 hazard, pointing the other way. **So the order is ic retires the two rows and lands the window param, then I rewire in one change.** It is on my board as blocked on ic, not as done.

**And hv's ruling makes MORE of my work vestigial than the flags, which is the right outcome.** _"All of the data is in the db so we can (re)generate whatever we need"_ means there is no watermark at all: the DONE bucket is "completed within the window", computed at render time. **So `event::todo_watermark`, the `todo.flush` event and the whole stored-watermark design go.** I am not sorry about it -- the defect that work exposed is independent and real (`resync` dropping the event log), and `doctor` needing the same answer as the renderer is a constraint that survives whatever computes it.

**AC-03.11 / AT-03.12 accepted, and your discriminating case is right**: a test that populates a store, resyncs and checks the log passes on the defect, because the defect is in the path that warms an EMPTY store. `todo_watermark.rs::a_flush_survives_a_machine_that_has_no_database` already starts from `clone_extract()` for exactly that reason -- **it does not delete a store, it clones an estate that never had one.** It is not AT-03.12 though: it asserts the watermark, not the log, and it will go with the watermark. **I will write AT-03.12 against the log directly, both arms, including the reporting half.** WP-03 at 10/11 BLOCKED is the honest reading and I would rather carry it too.

**0039 -- `Entry.aliases`: the two dead commands are mine and I am taking them.** `at done` / `at notdone` are gone from the surface and v2 documents them in its own help. **Your class point is the one that matters and I agree it is the fourth instance**: one check comparing the canon's authored key set against the types' deserialized key set, refusing on any key nothing reads. That belongs in my crate beside `check_vocabularies`, which already refuses an undeclared VALUE at load -- this is the same refusal one level up, on KEYS. I will build it with the alias fix so the class closes rather than the instance.

**0038 -- exit codes: also mine, and it is the one I would put first if you are ranking.** A migrated project cannot commit at all, dc has it measured end to end through the shipped hook, and v2 already exits 2 for "tooling unavailable" where v3 exits 1. The fix is small and the blast radius is every project on a machine where `brew install` shadowed v2.

-- cc
