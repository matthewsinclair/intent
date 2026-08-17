---
id: "0051"
title: a guard added to transitions.rs is enforced only where someone wrote the enforcement, and nothing tests that the two agree
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0051: a guard added to transitions.rs is enforced only where someone wrote the enforcement, and nothing tests that the two agree

## Tags

transitions, guards, highlander, declaration-vs-implementation, ac-04.6, mutation-testing, generalisation

## Summary

`transitions.rs` declares a `Guard` list per edge and the facade enforces guards at hand-written call sites. **Three of the five variants are enforced from the declaration; two are not, and nothing distinguishes the two groups.**

`Guard::GatePass` was in the second group until 2026-08-17. It was declared on `st.done` and `wp.done` and enforced by `st_done` and `wp_done` running the gate themselves before delegating, so **deleting `GatePass` from the table changed no behaviour at all** -- which is the precise test for whether a declaration is load-bearing, applied in the file that implements AC-04.6. It was found by mutation testing, not by reading, and fixed by routing it through `Facade::check_gate`, which reads the declaration.

**The instance is closed and the mechanism is not.** The next guard added to the table lands in whichever group its author happens to put it in, and a guard in the second group is decoration: it makes the table say a precondition is enforced when nothing checks that it is.

## Reproduction

Delete a `Guard` from an edge in `transitions.rs` and run `cargo test --workspace`. Today:

```
Guard::ReasonRecorded   -> RED   (check_reason reads the declaration)
Guard::NonTestOnly      -> RED   (check_ac_guards reads the declaration)
Guard::EvidenceRecorded -> RED   (check_ac_guards reads the declaration)
Guard::TargetExists     -> GREEN in the table, refused elsewhere -- see below
Guard::GatePass         -> was GREEN until 2026-08-17; now RED
```

The general case is the one that cannot be reproduced by deletion, because the guard does not exist yet: **add** a variant to the enum, declare it on an edge, enforce it nowhere, and the suite stays green.

## Root Cause

`Guard` is a declaration with no single enforcement seam. `check_reason`, `check_ac_guards` and now `check_gate` each consult `transitions::guard_for`, and a fourth enforcement written without consulting it would be indistinguishable from the first three by inspection -- which is how `GatePass` came to be hand-enforced beside its own declaration.

**`mutation_completeness.rs` cannot currently see this class.** It walks the graph's SHAPE: every declared edge is driven from its from-state with the guard SATISFIED, and the field is asserted to land on the edge's target. A guard is by construction the half of a transition a success-path walk never exercises -- the `Guard` doc comment says exactly this -- so a declared-but-unenforced guard passes the walk. Two guards do have their own targeted tests (`a_verb_declared_to_record_a_justification_refuses_a_blank_one` and the AC guard cases in `facade_acceptance.rs`), which is why those variants happen to be covered; coverage by whoever remembered is what this issue is about.

`Guard::TargetExists` is the honest illustration of the gap's shape. It IS enforced -- `ac_descope` refuses a missing thread -- but the refusal is written directly rather than derived from the declaration, so removing the declaration would leave the behaviour correct and the table wrong. Correct behaviour and a lying table is the outcome this class produces at its most benign, and it is still a defect: the table is what AC-04.6 checks the implementation against.

## Impact

**A precondition the register and the ratified machine both claim is enforced may not be.** That is worse than an absent guard, because an absent one is visible in the table and an unenforced one reads as covered. `Guard::GatePass` is the concrete cost already paid: for two days the close gate's status as a _declared_ precondition was decoration, and the only thing making `st done` consult the gate was two call sites that happened to.

It also compounds with the ordering the self-loop ruling requires. Hand-enforcement at the call site put the gate BEFORE the self-loop test, so `st done` on an already-completed thread re-ran the gate -- which hv's ruling forbids, because a criterion added after the close must not block a thread that is legitimately finished. **A guard enforced from its declaration gets the ordering by construction; a guard enforced by hand gets whatever its author's call site does.**

## Proposed Fix

Two candidates, and the second is the one that closes the class rather than the instance.

**Cheap and partial: an enumeration test.** Assert that every `Guard` variant appearing anywhere in `FIELDS` is named by at least one call to `transitions::guard_for`. Mechanical, catches an added-and-never-enforced variant, and says nothing about whether the enforcement is correct.

**The real fix: drive every declared edge a SECOND time with each of its guards UNMET, and require a refusal.** The walk already drives each edge with guards satisfied; the complement is the guard's entire reason for existing. It needs one thing the walk does not have -- a way to construct the unmet condition per `(verb, guard)` pair, since "unmet" means a blank reason for one variant and a BLOCKED gate for another -- so it is a driver table beside the existing `execute` arms rather than a property of the graph. That table is the thing to review: a pair with no unmet-fixture is a guard nobody can test, and it should refuse rather than skip, the same way `execute` refuses a `State` field with no drive arm.

Machine 4 is the row that makes this worth filing rather than noting. It declares **no guards, deliberately** (hv, 2026-08-17: inventing one would be "a parity break wearing a ratification"), and `set_issue_status` therefore has no guard call at all. So the protection for that row today is that `Guard::ReasonRecorded` is the only variant that could mean anything for an issue and `Issue` has no field to record one in -- ie a guard added there would force a model change, and the model change is what would force the enforcement. **That is protection by accident of the model's shape, not by construction**, and it is the weakest of the five rows.

## Related

- ST0056 -- Intent v3.0.0
- AC-04.6 -- the implemented graph must match the ratified machines exactly
- 0046 -- the from-state deviations; the same declaration-versus-implementation split one layer up
- `61069b16` -- the commit that closed the `GatePass` instance and left the mechanism

## Resolutions

{{TBC}}
