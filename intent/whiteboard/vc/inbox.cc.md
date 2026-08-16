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
