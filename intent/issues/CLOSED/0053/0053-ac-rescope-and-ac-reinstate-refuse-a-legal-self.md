---
id: "0053"
title: ac rescope and ac reinstate refuse a legal self-loop at exit 1 -- 0051's mechanism surviving twenty lines from the copy cc fixed, and the refusal names the wrong verb
date: 2026-08-17
reporter: matts
status: CLOSED
severity: high
---

# 0053: ac rescope and ac reinstate refuse a legal self-loop at exit 1 -- 0051's mechanism surviving twenty lines from the copy cc fixed, and the refusal names the wrong verb

## Tags

facade, self-loop, ac, voice, measured, surface, regression-class

## Summary

`Facade::ac_rescope` and `Facade::ac_reinstate` each hand-write a from-state `match` whose `_` arm raises `NotOffScope`, and that arm runs **ahead of** `set_ac_state`, which is where the shared self-loop test lives. So a criterion already sitting in the verb's target state -- `AcState::entry(kind)`, which is `Unsatisfied` for a non-test criterion and `Computed` for a test-backed one -- falls to the catch-all and **exits 1 instead of returning `Outcome::AlreadyThere`**.

**This is 0051's mechanism, and it is the third and fourth live instance rather than a new class.** The first was `Guard::GatePass` hand-run at two call sites. The second was `ac_unsatisfy`, which cc found and fixed at `d0f345b5` -- **in this same file, in this same commit, twenty lines above `ac_reinstate`.** The doc comment cc wrote on the fix states the class in general terms:

> **A hand-written copy of a from-state the table already declares, running ahead of the shared setter's self-loop test**, which is exactly what `Guard::GatePass` was doing at two call sites. Found by driving the verb twice through the real binary (`self_loop_voice.rs`); reading the code did not find it, and neither did `mutation_completeness.rs`, whose walk only ever drives an edge from a state it IS declared from.

Both surviving instances match that description exactly, sit adjacent to it, and were not caught -- **by the sweep that fixed their sibling, by the witness built to find this shape, or by the suite that went green after both.**

**A second, smaller defect rides along: the refusal names a verb the user did not type.** `NotOffScope` carries only `{ ac }`, no verb, so one hardcoded message serves both entry points. `intent ac rescope ST0001 AC-01.1` on an in-scope criterion prints `error: AC-01.1 is in scope, so there is nothing to reinstate` with the remedy `reinstate applies only to a descoped or withdrawn criterion`. **The word `rescope` does not appear; the word `reinstate` appears twice.** The sibling error `WrongOffScopeState` already carries a `verb` field and gets this right, so the fix has a pattern to follow in the same enum.

Found by ic, 2026-08-17, driving all 27 rows of `populations.self_loop` twice through the release binary in order to write `target.no_op` in `surface/dispatch-table.json` from measurement rather than from cc's announcement of the values.

## Reproduction

Measured against a release binary built from a `git archive HEAD` extract at `ae3e308f`, whose `native/` is bit-identical to `b7e60fc5` -- `git diff --stat b7e60fc5..HEAD -- native/ surface/` is empty across all three intervening commits.

**The extract is load-bearing and the first attempt at this reproduction was not sound.** The original build came from a working tree `git status` reported clean, and it was clean; a peer then began editing `render.rs` in the same shared checkout, and a rebuild replaced the binary at the path this measurement was reading. **Two binaries, same path, same name, no record in either of which tree it came from** -- separated only by an mtime 31 seconds apart, which establishes order and not content. Every result below was re-driven against the extract build and reproduced unchanged. **In a shared checkout a clean `git status` licenses a build, not a measurement taken later against its output.**

Fixture is a thread carrying one criterion of each kind, since `AcState::entry(kind)` differs between them and one criterion cannot measure both:

```
AC-01.1  kind: non-test   state: { is: unsatisfied }
AC-01.2  kind: test       state: { is: computed }
```

Each verb is driven twice. The first call is the real movement; the **second** is the self-loop under test.

```
$ intent ac descope ST0001 AC-01.1 --to ST0002     # set up: off scope
$ intent ac rescope ST0001 AC-01.1                 # 1st: real movement, exit 0
$ intent ac rescope ST0001 AC-01.1                 # 2nd: THE SELF-LOOP
error: AC-01.1 is in scope, so there is nothing to reinstate
  remedy: reinstate applies only to a descoped or withdrawn criterion
                                                   # exit 1
```

All four combinations behave identically -- both verbs, both kinds:

| invocation                 | 2nd call                   | exit | expected                   |
| -------------------------- | -------------------------- | ---- | -------------------------- |
| `ac rescope` non-test      | `... nothing to reinstate` | 1    | `already unsatisfied` at 0 |
| `ac rescope` test-backed   | `... nothing to reinstate` | 1    | `already computed` at 0    |
| `ac reinstate` non-test    | `... nothing to reinstate` | 1    | `already unsatisfied` at 0 |
| `ac reinstate` test-backed | `... nothing to reinstate` | 1    | `already computed` at 0    |

For contrast, the fixed sibling in the same family, same run:

```
$ intent ac unsatisfy ST0001 AC-01.1    # 2nd call
ok: AC-01.1 already unsatisfied         # exit 0
```

## Root Cause

`facade.rs:1764-1795`. Both verbs match the current state, handle the one state they are declared from, refuse the sibling off-scope state with a helpful `WrongOffScopeState`, and send **everything else** -- including the target state -- to `NotOffScope`:

```rust
pub fn ac_rescope(&mut self, st: &str, ac: &str) -> Result<Outcome, FacadeError> {
  let criterion = self.criterion(st, ac)?;
  let entry = AcState::entry(criterion.kind);
  match &criterion.state {
    AcState::Descoped { .. } => self.set_ac_state(st, ac, entry, "ac.rescope", json!({})),
    AcState::Withdrawn { .. } => Err(FacadeError::WrongOffScopeState { .. }),
    _ => Err(FacadeError::NotOffScope { ac: ac.to_string() }),   // <- swallows the self-loop
  }
}
```

`set_ac_state` is the only thing that tests `from == target`, and the `_` arm is the reason it is never reached.

**The declared-from reading is the trap, and hv's ruling closed it explicitly.** `ac.rescope` is declared from `descoped` alone, so "already unsatisfied is not a declared from-state, therefore refusing is correct" looks right. The ruling says otherwise: **what decides a self-loop is whether the current state equals the verb's TARGET, not whether the verb is declared from that state** -- which is why the check is placed first, ahead of `check_transition`, `check_gate` and `check_reason`. `AcState::entry(kind)` IS the target these two pass to the setter, so a criterion already holding it is the self-loop by definition.

**Why every instrument missed it, which is the part worth keeping.** `mutation_completeness.rs` only ever drives an edge from a state it is declared from, so it cannot reach this call at all. `self_loop_voice.rs` -- the witness built for exactly this defect -- drives each verb twice from a seeded fixture, but a fixture that starts in scope makes the FIRST call the refusal too, so the pair is `error, error` and the file's actual assertion (the two lines must differ) holds. **The witness's guard is "two identical lines", and a verb that refuses twice produces two identical lines that it does not catch, because it is checking for a repeated success.**

## Impact

**`intent ac rescope` and `intent ac reinstate` are not idempotent, and idempotence is the whole property hv's ruling bought** -- "a caller can rely on the state without checking it first". Any script that rescopes a criterion defensively gets exit 1, and under `set -e` that ends the script. Two verbs in the surface's most safety-critical family behave opposite to the other twenty-five.

**The exit code is the damaging half rather than the wording.** Exit 1 is the code the git gate blocks on (0045), so a defensive `ac rescope` in a pre-commit path fails the commit for a criterion that is already exactly where the caller wants it.

**The wrong-verb message costs a user their next action.** Told `reinstate applies only to a descoped or withdrawn criterion` after typing `rescope`, the reasonable response is to try `intent ac reinstate` -- which fails the same way, with the same sentence, and now looks like the tool disagreeing with itself.

**And the announced values were wrong in a way the announcement could not show.** cc's handoff listed `ac rescope` and `ac reinstate` as `ok: AC-01.1 already unsatisfied | already computed`, reasoned correctly from `AcState::entry(kind)` -- the value the SUCCESS arm hands the setter. The self-loop never reaches that arm. **The subject reasoned about (what the setter would be passed) and the subject reported (what the user sees on a self-loop) came apart, and the reasoning was well-formed either way** -- vc's consolidated class in `parity.md`, arriving through a fifth channel. Had the table been written from the handoff, it would have declared a clean `ok:` for two rows that exit 1.

## Proposed Fix

**Delegate, and map the refusal after the fact -- the shape cc already used on `ac_unsatisfy` twenty lines up.** Handle the declared from-state and the sibling off-scope state as now, then send the remainder to `set_ac_state` and convert its `IllegalTransition` into `NotOffScope`:

```rust
_ => self
  .set_ac_state(st, ac, entry, "ac.rescope", json!({}))
  .map_err(|cause| match cause {
    FacadeError::IllegalTransition { .. } => FacadeError::NotOffScope { ac: ac.to_string() },
    other => other,
  }),
```

The refusal is preserved rather than lost, by the same argument cc made for `ac_unsatisfy`: after `Descoped` and `Withdrawn` are handled above, every `IllegalTransition` these verbs can still produce means "in scope", so the mapping is equivalent by construction -- and the self-loop now returns before the mapping is consulted.

**Give `NotOffScope` a `verb` field**, matching `WrongOffScopeState` which already has one: `#[error("{ac} is in scope, so there is nothing to {verb}")]`. Two call sites, both already know their verb literally.

**And the witness needs an arm it does not have, or this class survives the next fix too.** `self_loop_voice.rs` asserts the two invocations DIFFER, which a double refusal satisfies. It should additionally require that the second invocation exit 0 for every row in `populations.self_loop` -- **the property under test is that a self-loop succeeds, and "the lines differ" is a proxy that a refusing verb passes.** `self_loop_population.rs` binds the population to the ratified machines already, so the row list to iterate is available and checked.

## Related

- ST0056 -- Intent v3.0.0
- 0051 -- the mechanism: a hand-written from-state check ahead of the shared setter
- 0050 -- the sweep that fixed the sibling `ac_unsatisfy` in this same file at `d0f345b5`
- 0045 -- exit 1 blocks the git gate, which is what makes the exit code the damaging half
- `facade.rs:1764-1795` -- both defective verbs
- `facade.rs:1658-1690` -- `ac_unsatisfy`, the corrected shape and the doc comment stating the class
- `parity.md` -- the subject-of-measurement class; this is its arrival in a handoff

## Resolution -- CLOSED 2026-08-17 (vc), verified by execution and by source at `0f87fc2c`

**All three of this issue's closing conditions are met**, checked individually rather than inferred from the fix commit.

**(a) The delegate-and-map shape landed**, exactly as proposed -- `facade.rs`, `ac_rescope`:

```rust
_ => self
  .set_ac_state(st, ac, entry, "ac.rescope", json!({}))
  .map_err(|cause| Self::in_scope(cause, ac, "rescope", "descoped")),
```

The hand-written from-state check ahead of the shared setter is gone; the sibling off-scope state (`Withdrawn`) is still handled explicitly above it, so the refusal is preserved rather than lost.

**(b) `NotOffScope` carries a `verb` field** -- declared at `facade.rs:93`, destructured in the `Display` arm at `:237`, and constructed with a verb at both call sites. The refusal names itself; the issue's _"the refusal names the wrong verb"_ clause is discharged.

**(c) The witness has the arm it lacked.** `self_loop_voice.rs` asserts exit codes directly (`out.status.code()`), and `ac_rescope_and_ac_reinstate_accept_a_self_loop_and_name_themselves_when_they_refuse` passes -- 14 of 14 green in that file. The issue's warning was that _"the lines differ" is a proxy a refusing verb passes_; the test now requires the outcome, not the difference.

**Recorded because it is the more useful half of this issue**: the defect was `0051`'s mechanism surviving twenty lines below the copy cc had already fixed, and cc's fix for it unmasked a filter bug in the walk that should have caught it -- the walk excluded a verb's target PER EDGE, so a kind-dependent verb was asked to refuse from its own other target. **The defect was masking its own detector.** `0051` remains OPEN and is where that class lives; this closure is the instance, not the class.
