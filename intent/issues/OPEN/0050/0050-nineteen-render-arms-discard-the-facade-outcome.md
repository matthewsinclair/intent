---
id: "0050"
title: nineteen render arms discard the facade Outcome, so intent st done prints ok: done for a no-op while intent todo done -- which delegates to it -- reports the no-op
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0050: nineteen render arms discard the facade Outcome, so intent st done prints ok: done for a no-op while intent todo done -- which delegates to it -- reports the no-op

## Tags

surface, voice, self-loop, outcome, render, delegation, measured, parity

## Summary

hv's self-loop ruling (`61069b16`) gave the facade a way to say "nothing moved": `Outcome::AlreadyThere` against `Outcome::Moved`. **One renderer reads it. Nineteen throw it away.**

The nineteen call the facade as a statement -- `open()?.st_done(&id).map_err(fail)?;` -- which propagates the error and discards the `Ok` value, then prints the movement message unconditionally. So `intent st done ST0001` on a thread that is already completed prints `ok: ST0001 done`, having done nothing.

**The reason this is a defect rather than a style preference is that the surface now contradicts itself through delegation.** `todo done` is one of the two arms that reads the outcome, and what it delegates to is `st_done` and `wp_done` -- two of the nineteen that do not. The same facade call, reached two ways, gives two different answers:

```
intent todo done ST0001    ->  ok: ST0001 was already done
intent st done ST0001      ->  ok: ST0001 done
```

The wrapper is the honest one and the thing it wraps is not, which is the wrong way round: a caller reaching for the direct verb gets less information than one reaching through the convenience command.

Found by ic, 2026-08-17, updating the dispatch-table rows for `st done` / `wp start` / `wp done` / `st start` after the self-loop landed -- the rows have to state what each verb PRINTS on the no-op path, and answering that for four rows required measuring all nineteen.

## Reproduction

Measured against `61069b16` by scanning `render.rs` for calls to the eighteen facade verbs that return `Result<Outcome, FacadeError>`, and classifying each by whether the `Ok` value is bound or discarded:

```
DROPPED (Outcome discarded, message printed unconditionally): 19
  render.rs:426  f.st_triage(&id).map_err(fail)?;
  render.rs:427  f.st_start(&id).map_err(fail)?;
  render.rs:437  open()?.st_start(&id).map_err(fail)?;
  render.rs:443  open()?.st_done(&id).map_err(fail)?;
  render.rs:456  open()?.st_cancel(&id, &reason).map_err(fail)?;
  render.rs:474  open()?.st_triage(&id).map_err(fail)?;
  render.rs:481  open()?.st_hold(&id, &reason).map_err(fail)?;
  render.rs:487  open()?.st_resume(&id).map_err(fail)?;
  render.rs:494  open()?.st_reopen(&id, &reason).map_err(fail)?;
  render.rs:501  open()?.st_reinstate(&id, &reason).map_err(fail)?;
  render.rs:580  open()?.wp_start(&st, seq).map_err(fail)?;
  render.rs:586  open()?.wp_done(&st, seq).map_err(fail)?;
  render.rs:599  open()?.wp_reopen(&st, seq, &reason).map_err(fail)?;
  render.rs:605  open()?.wp_unstart(&st, seq).map_err(fail)?;
  render.rs:685  open()?.ac_satisfy(&st, &id, &evidence).map_err(fail)?;
  render.rs:692  open()?.ac_unsatisfy(&st, &id).map_err(fail)?;
  render.rs:765  open()?.ac_rescope(&st, &id).map_err(fail)?;
  render.rs:772  open()?.ac_reinstate(&st, &id).map_err(fail)?;
  render.rs:804  open()?.at_set(&st, &id, status).map_err(fail)?;

BOUND (Outcome read): 2
  render.rs:1128  (st, Scope::Thread) => f.st_done(&st).map_err(fail)?,
  render.rs:1129  (st, Scope::WorkPackage(seq)) => f.wp_done(&st, seq).map_err(fail)?,
```

Both bound sites are inside `todo_done`. Line 426/427 is `st new --start`, where discarding is arguably right -- the composed `st_triage` then `st_start` are setup for a creation whose message is `created:` -- so the honest count of arms that print a movement they did not make is seventeen.

## Root Cause

`Outcome` is deliberately not `#[must_use]`, and the commit that made it so says why: it fired on 65 sites, "nearly all tests putting a fixture into a state where ignoring the outcome is right", and 65 `let _ =` annotations added to silence a warning is how an annotation stops carrying information. **That reasoning is correct about the tests and it does not transfer to the nineteen production arms**, which is the whole gap: the attribute was assessed against its loudest population rather than its most important one, and the population it was right about is the one that does not print anything.

Nothing else covers the field. The type-checker cannot: discarding an `Ok` value is legal Rust. `dispatch-table.json` declares each row's help and exit codes but has no notation for "what this prints when nothing moved", so no parity tool can see it either -- **the surface register is currently unable to state the difference between these two rows**, which is a gap in my own SSOT and not only in the renderer.

## Impact

Two effects, and the second is the one that will cost somebody.

**A user is told a thing happened that did not.** `ok: ST0001 done` is the confirmation of a state change. Issued against a thread that was already closed it is a false report at exit 0, and the point of accepting a self-loop rather than refusing it -- hv's stated reason -- is that a caller can rely on the state without checking first. A caller that cannot distinguish "I closed it" from "it was already closed" has to check first anyway, which gives back what the ruling bought.

**And a script cannot tell the two apart.** Exit 0 both ways, identical stdout. The information exists inside the process and is discarded on the way out.

The self-loop paths where this is most visible are exactly the ones the ruling was made for: `st done` and `wp done` on already-closed units, and `at set` on an already-green row, where v3 correctly writes no envelope but says it moved.

## Proposed Fix

Bind the outcome in the seventeen arms that print a movement, and give each a no-op message in the same voice `todo_done` already uses:

```rust
match open()?.st_done(&id).map_err(fail)? {
  Outcome::Moved => println!("ok: {id} done"),
  Outcome::AlreadyThere => println!("ok: {id} was already done"),
}
```

Two decisions belong to whoever takes it, and neither is a renderer's call alone:

- **Whether `was already ...` is the house phrasing.** v2's one precedent is `intent issues close` -- `ok: issue NNNN already CLOSED` (`bin/intent_issues:282`) -- which is the ONLY `already` arm in v2 and is where the ruling's `already CLOSED` citation actually comes from. Matching it would give `ok: ST0001 already completed`. `todo done` has already shipped a different spelling, so this is a Highlander question about the voice before it is a wording preference.
- **Whether the no-op line is a THIRD prefix.** `st start`'s v2 behaviour uses `skipped:` (`bin/intent_st:634-637`), and the `st start` row already flags that INV-01 names only `ok:` and `error:`. If `skipped:` is revived it should be revived once, here, for all seventeen -- not per-arm.

The dispatch table needs the notation either way, so the parity tools can hold it: a per-row statement of what the no-op prints, which today has nowhere to live.

## Related

- ST0056 -- Intent v3.0.0
- 0044 -- exit-code spellings; same class, one layer down (a caller cannot distinguish conditions the tool can)
- 0046 -- the from-state deviations on these four rows
- `61069b16` -- the self-loop ruling that created `Outcome` and the two honest arms

## Resolutions

{{TBC}}
