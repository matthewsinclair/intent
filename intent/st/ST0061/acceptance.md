---
st_id: ST0061
title: dehydrate
---

# ST0061: dehydrate -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 **`intent st dehydrate <ID>` is wired and is the inverse of `st hydrate`.** It removes the `STEELTHREAD:<ID>` entry from `intent/.intentfiles` and removes that thread's realised files, reporting each removed path by name and the manifest it changed. **Proven by ROUND TRIP rather than by two separate assertions**: hydrate, then dehydrate, then hydrate again returns the tree byte for byte -- which is the only check that tests the two verbs against each other rather than each against my expectations of it. -- satisfied: no (computed)
- AC-00.2 **THE RAIL: a realised file the store cannot be SHOWN to hold is a refusal NAMING THE FILE, never a deletion.** Every removal passes `organize::gate`, whose match is `Some(rendered) if *rendered == on_disk` with everything else falling to the refusing arm -- so an opaque attachment carrying `None` and a hand-edited file whose bytes differ are both refused, by the same wildcard, and neither is a special case that could be removed by a later simplification. The refusal names the path and its byte count, and every file that existed before the run still exists byte for byte. -- satisfied: no (computed)
- AC-00.3 **THE REFUSAL IS DECIDED BEFORE THE DECLARATION MOVES, AND THIS IS A CONSTRUCTION RATHER THAN AN ORDERING DISCIPLINE.** The plan is computed against the hypothetical unpinned manifest -- `unpin` returns text and `realised_for_action` reads it without writing -- so a refused run leaves `intent/.intentfiles` byte-identical. **A verb that unpinned first would convert a refusal into a DEFERRED DELETION**, performed later by whoever next runs `organize --apply` against a thread the manifest no longer declares, with nobody having decided anything. Asserted by refusing a run and then reading the manifest, not by inspecting the order of two statements. -- satisfied: no (computed)
- AC-00.4 **AN ABSENT MANIFEST REFUSES, AND NAMES THE VERB THAT WRITES ONE.** ABSENT means nobody has said, so everything is realised. Creating a manifest here would declare that everything EXCEPT this thread is realised -- an estate-wide assertion nobody made, arrived at through a single-thread verb. It is the mirror of the hazard `hydrate` already refuses in the other direction, where creating a manifest to hold one entry would declare that entry to be the whole. The refusal is distinguishable from `the thread was not listed`, which is an ordinary exit 0. -- satisfied: no (computed)
- AC-00.5 **THE ESTATE PRECONDITIONS GATE THIS DOOR TOO.** `preconditions::check` refuses the whole run when any declared precondition is unmet, naming every one of them with a denominator. **A per-thread verb must not be a way around the estate gate**: if it were, the gate would protect only the operator who happened to reach for `organize`, and the narrower verb would be the one that deletes. On an estate with no declaration at all the answer is `NoDeclaration` and the refusal stands. -- satisfied: no (computed)
- AC-00.6 **IDEMPOTENT, AND IT NEVER CONFLATES `NOTHING TO DO` WITH `DID SOMETHING`.** `st dehydrate` on a thread already absent from the manifest and absent from disk exits 0 and says so in words distinct from a run that removed files. `unpin` is already idempotent by contract; this criterion is about the REPORT, because a count of removals that reads the same for zero and for one is the class that let `1 refused` speak for 423 files. -- satisfied: no (computed)
- AC-00.7 **WIRING THIS VERB LEAVES A TEST WHOSE NAME ASSERTS SOMETHING IT NEVER CHECKED.** `render.rs`'s `an_unwired_verb_in_a_wired_family_is_sent_to_that_family` uses `st dehydrate` as its fixture and **will keep passing after wiring** -- because `unwired()` is a pure function that interpolates the verb into a message and only ever consults whether the FAMILY ships verbs. The verb's own wired-ness is never read. So the fixture is decorative: the test would pass identically for `st list`. **This is `IN-AG-RED-CONTROL-001` in its quietest form -- not a control that broke, but one that never could break, wearing a name that says it does.** Two changes: the fixture moves to a verb that is genuinely unwired (`bootstrap` or `repair`, the only two left in this family once `dehydrate` lands and with `st organize` retired), and the test gains the assertion its name always implied -- that dispatching that verb through `st()` actually REACHES `unwired`, rather than that `unwired()` formats a string when called by hand. -- satisfied: no (computed)

## Acceptance Tests

### ST-level

- AT-00.1 `native/rust/crates/intent-cli/tests/st_dehydrate_round_trips_with_hydrate.rs` -- covers AC-00.1 -- status: to-write -- hydrate -> dehydrate -> hydrate on one thread; asserts the tree is byte-identical to the start and the manifest entry returns.
- AT-00.2 `native/rust/crates/intentsvcs/tests/dehydrate_refuses_what_the_store_cannot_hold.rs` -- covers AC-00.2 -- status: to-write -- Two arms against the one wildcard: an opaque attachment carrying None, and a hand-edited view whose bytes differ. Both refuse by name; every file survives.
- AT-00.3 `native/rust/crates/intentsvcs/tests/dehydrate_decides_before_the_declaration_moves.rs` -- covers AC-00.3 -- status: to-write -- Forces a refusal, then reads .intentfiles and asserts it is byte-identical. The control is a PERMITTED run on the same fixture, where the entry does go.
- AT-00.4 `native/rust/crates/intentsvcs/tests/dehydrate_refuses_an_absent_manifest.rs` -- covers AC-00.4 -- status: to-write -- No .intentfiles at all: refuses, names organize --default, writes nothing. Control: a present manifest not listing the id exits 0 as an ordinary no-op.
- AT-00.5 `native/rust/crates/intentsvcs/tests/dehydrate_honours_the_estate_preconditions.rs` -- covers AC-00.5 -- status: to-write -- An estate whose declaration is unmet refuses with the denominator; an estate with no declaration refuses as NoDeclaration. Control: the met estate permits.
- AT-00.6 `native/rust/crates/intentsvcs/tests/dehydrate_reports_nothing_to_do_distinctly.rs` -- covers AC-00.6 -- status: to-write -- Second dehydrate of the same thread exits 0 with wording distinct from the first. Asserts the two reports differ, not merely that both exit 0.
- AT-00.7 `native/rust/crates/intent-cli/tests/unwired_is_reached_by_dispatch_not_only_formatted.rs` -- covers AC-00.7 -- status: to-write -- Dispatches the fixture verb through the st family and asserts it reaches unwired with exit 2. The control is the inverse: dispatching a WIRED verb must NOT reach unwired -- which the current test cannot tell apart.

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
