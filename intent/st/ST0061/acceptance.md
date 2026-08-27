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

- AC-00.1 **`intent st dehydrate <ID>` is wired and is the inverse of `st hydrate`.** It removes the `STEELTHREAD:<ID>` entry from `intent/.intentfiles` and removes that thread's realised files, reporting each removed path by name and the manifest it changed. **Proven by ROUND TRIP rather than by two separate assertions**: hydrate, then dehydrate, then hydrate again returns the tree byte for byte -- which is the only check that tests the two verbs against each other rather than each against my expectations of it. -- satisfied: yes (computed)
- AC-00.2 **THE RAIL: a realised file the store cannot be SHOWN to hold is a refusal NAMING THE FILE, never a deletion.** Every step under the thread passes `organize::gate`, whose match is `Some(rendered) if *rendered == on_disk` with everything else falling to the refusing arm -- so an opaque attachment carrying `None` and a hand-edited file whose bytes differ are both refused, by the same wildcard, and neither is a special case a later simplification could remove. **THE GATE READS EVERY STEP AND NOT ONLY THE DESTRUCTIVE ONES**: a file classified `Unclaimed` is kept by design, and keeping it means the thread does not leave the tree, so a run that removed the views around it would answer `dehydrated` over a thread still realised. The refusal names the path, and every file that existed before the run still exists byte for byte.

**THE ENUMERATION IS THE CORPUS SCAN, AND WHICH DISK WALK IS MEANT IS THE WHOLE OF THIS RULE (vc + ic, 2026-08-26).** Files are enumerated by `sync::scan` -- the walk of `intent/` that excludes exactly `Ignored::for_root`, the paths git would never commit. **Ignored paths are OUTSIDE THE CORPUS: never counted, never removed, and never a refusal.** A recursive `read_dir` instead would see every `.DS_Store` on every Mac and refuse dehydration for any thread directory ever opened in Finder -- the class that already broke ingest fleet-wide once (D29 / AC-03.7). Measured case: Lamplight ST0306 carries ten gitignored review gifs, 152.9 MB, under one thread; they are outside the corpus and survive, while the tracked `.gitignore` beside them is `Unclaimed` and refuses the run by name.

**AND A DIRECTORY THE RUN COULD NOT EMPTY IS NAMED IN THE VERDICT, NEVER A BARE `dehydrated`.** `prune_emptied` calls `remove_dir` rather than `remove_dir_all` -- a physical floor, since it fails on a non-empty directory -- but it skips the failure through an `is_ok()`, which is silent. Content outside the corpus legitimately survives and must not refuse the run; it does leave the manifest saying dehydrated while a directory tree remains. **git leaves ignored files behind too and says nothing, because git keeps no manifest to contradict, and we do.** The surviving leaves are reported by name, and the test reds when one goes unnamed. -- satisfied: yes (computed)
- AC-00.3 **THE REFUSAL IS DECIDED BEFORE THE DECLARATION MOVES, AND THIS IS A CONSTRUCTION RATHER THAN AN ORDERING DISCIPLINE.** The plan is computed against the hypothetical unpinned manifest -- `unpin` returns text and `realised_for_action` reads it without writing -- so a refused run leaves `intent/.intentfiles` byte-identical. **A verb that unpinned first would convert a refusal into a DEFERRED DELETION**, performed later by whoever next runs `organize --apply` against a thread the manifest no longer declares, with nobody having decided anything. Asserted by refusing a run and then reading the manifest, not by inspecting the order of two statements. -- satisfied: yes (computed)
- AC-00.4 **AN ABSENT MANIFEST REFUSES, AND NAMES THE VERB THAT WRITES ONE.** ABSENT means nobody has said, so everything is realised. Creating a manifest here would declare that everything EXCEPT this thread is realised -- an estate-wide assertion nobody made, arrived at through a single-thread verb. It is the mirror of the hazard `hydrate` already refuses in the other direction, where creating a manifest to hold one entry would declare that entry to be the whole. The refusal is distinguishable from `the thread was not listed`, which is an ordinary exit 0. -- satisfied: yes (computed)
- AC-00.5 **THE ESTATE PRECONDITIONS GATE THIS DOOR TOO.** `preconditions::check` refuses the whole run when any declared precondition is unmet, naming every one of them with a denominator. **A per-thread verb must not be a way around the estate gate**: if it were, the gate would protect only the operator who happened to reach for `organize`, and the narrower verb would be the one that deletes. On an estate with no declaration at all the answer is `NoDeclaration` and the refusal stands. -- satisfied: yes (computed)
- AC-00.6 **IDEMPOTENT, AND IT NEVER CONFLATES `NOTHING TO DO` WITH `DID SOMETHING`.** `st dehydrate` on a thread already absent from the manifest and absent from disk exits 0 and says so in words distinct from a run that removed files. `unpin` is already idempotent by contract; this criterion is about the REPORT, because a count of removals that reads the same for zero and for one is the class that let `1 refused` speak for 423 files. -- satisfied: yes (computed)
- AC-00.7 **WIRING THIS VERB LEAVES A TEST WHOSE NAME ASSERTS SOMETHING IT NEVER CHECKED.** `render.rs`'s `an_unwired_verb_in_a_wired_family_is_sent_to_that_family` uses `st dehydrate` as its fixture and **will keep passing after wiring** -- because `unwired()` is a pure function that interpolates the verb into a message and only ever consults whether the FAMILY ships verbs. The verb's own wired-ness is never read. So the fixture is decorative: the test would pass identically for `st list`. **This is `IN-AG-RED-CONTROL-001` in its quietest form -- not a control that broke, but one that never could break, wearing a name that says it does.** Two changes: the fixture moves to a verb that is genuinely unwired (`bootstrap` or `repair`, the only two left in this family once `dehydrate` lands and with `st organize` retired), and the test gains the assertion its name always implied -- that dispatching that verb through `st()` actually REACHES `unwired`, rather than that `unwired()` formats a string when called by hand. -- satisfied: yes (computed)

## Acceptance Tests

### ST-level

- AT-00.1 `native/rust/crates/intent-cli/tests/st_dehydrate_round_trips_with_hydrate.rs` -- covers AC-00.1 -- status: green -- hydrate -> dehydrate -> hydrate on one thread; asserts the tree is byte-identical to the start and the manifest entry returns.
- AT-00.2 `native/rust/crates/intentsvcs/tests/facade_dehydrate.rs` -- covers AC-00.2 -- status: green
- AT-00.3 `native/rust/crates/intentsvcs/tests/facade_dehydrate.rs` -- covers AC-00.3 -- status: green
- AT-00.4 `native/rust/crates/intentsvcs/tests/facade_dehydrate.rs` -- covers AC-00.4 -- status: green
- AT-00.5 `native/rust/crates/intentsvcs/tests/facade_dehydrate.rs` -- covers AC-00.5 -- status: green
- AT-00.6 `native/rust/crates/intentsvcs/tests/facade_dehydrate.rs` -- covers AC-00.6 -- status: green
- AT-00.7 `native/rust/crates/intent-cli/src/render.rs` -- covers AC-00.7 -- status: green

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
