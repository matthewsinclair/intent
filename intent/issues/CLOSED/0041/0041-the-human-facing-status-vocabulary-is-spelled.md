---
id: "0041"
title: the human-facing status vocabulary is spelled twice in two crates -- views.rs writes the committed md and render.rs writes the terminal, and nothing relates them
date: 2026-08-16
reporter: matts
status: CLOSED
severity: medium
---

# 0041: the human-facing status vocabulary is spelled twice in two crates -- views.rs writes the committed md and render.rs writes the terminal, and nothing relates them

## Tags

highlander, model, views, render, vocabulary, measured

## Summary

`ThreadStatus` and `WpStatus` each have exactly one definition in `model.rs`. **Their human-facing spellings have two, in different crates**, written by four private functions that cannot call each other:

| vocabulary     | `intentsvcs` (writes the committed md) | `intent-cli` (writes the terminal) |
| -------------- | -------------------------------------- | ---------------------------------- |
| `ThreadStatus` | `views.rs:72` `status_display`         | `render.rs:1395` `status`          |
| `WpStatus`     | `views.rs:332` `wp_status_display`     | `render.rs:94` `wp_status`         |

The strings are byte-identical today. Nothing makes them so, nothing tests it, and all four functions are private (`fn`, not `pub fn`), so neither crate can reach the other's even if it wanted to.

MODULES.md declares `intentsvcs` the Highlander layer and the CLI "a thin skin over its facade". A skin holding an independent copy of the model's own vocabulary is not thin -- it knows something about the model that the model does not export.

Found by vc during the hv-assigned Highlander review of the v3 Rust tree, 2026-08-16.

## Reproduction

Measured at `ff094157` against a pristine `git archive HEAD` extract.

```
$ grep -rn '"Not Started"\|"WIP"\|"Done"' --include='*.rs' crates/*/src
intent-cli/src/render.rs:97:    W::NotStarted => "Not Started",
intent-cli/src/render.rs:98:    W::Wip => "WIP",
intent-cli/src/render.rs:99:    W::Done => "Done",
intent-cli/src/render.rs:1398:    S::NotStarted => "Not Started",
intent-cli/src/render.rs:1399:    S::Wip => "WIP",
intentsvcs/src/views.rs:74:    ThreadStatus::NotStarted => "Not Started",
intentsvcs/src/views.rs:75:    ThreadStatus::Wip => "WIP",
intentsvcs/src/views.rs:334:    crate::model::WpStatus::NotStarted => "Not Started",
intentsvcs/src/views.rs:335:    crate::model::WpStatus::Wip => "WIP",
intentsvcs/src/views.rs:336:    crate::model::WpStatus::Done => "Done",
```

Both six-arm `ThreadStatus` matches agree arm for arm: `Not Started` / `WIP` / `Triage` / `On Hold` / `Completed` / `Cancelled`.

**Nothing compares them.** `grep -rn 'status_display\|wp_status_display' crates/` returns seven hits, all inside `views.rs` itself. The tests pin each side separately against hand-written literals -- `cli_end_to_end.rs:777` and `:244` assert the terminal's spelling, `facade_st_wp.rs:290` asserts a view's -- so **each copy is held in place by its own test and neither test can see the other copy.**

## Root Cause

**The spelling of a closed model vocabulary is a model fact, and it is the only model fact the model does not own.**

The estate already has the correct pattern registered for the neighbouring concern: MODULES.md names `transitions.rs` "THE declared state-transition table ... Surfaces READ it; never re-derive it". Which values a status can move to is centralised. What a status is called is not, and the two are the same class of fact about the same enum.

The mechanism that will cause the drift is visible in the code today. `views.rs:66-71` carries the reasoning behind the vocabulary -- the deliberate `TBC` / `Not Started` divergence from v2, recorded as a `corrected` register row. `render.rs` carries no such note and no pointer to it. **So one copy holds the rationale and the other holds only the strings**, and whoever edits the copy without the rationale has no way to learn that the vocabulary was decided rather than typed.

## Impact

**`intent st list` and the committed `steel_threads.md` are two answers to the same question, produced by two functions that never meet.** They agree today. A rename applied to one is a green build, a green suite, and a repository whose committed index disagrees with the tool that generated it.

The asymmetry makes it worse than a symmetric duplication: **the md views are committed artefacts.** A terminal-only drift is noticed and corrected in the session it appears; a views-side drift lands in git, propagates to every clone, and shows up in a diff nobody attributes to a status rename.

**Not claimed: that the copies disagree today.** They do not -- I checked every arm. This is a Highlander finding about how the code is held together, not a live behavioural defect, and the severity reflects that.

## Proposed Fix

**The spelling moves onto the model type, beside the enum and beside `transitions.rs`.**

1. `impl ThreadStatus { pub fn display(&self) -> &'static str }` and the same for `WpStatus`, in `model.rs`. The `views.rs:66-71` rationale moves with it -- **the note is the part that must not be left behind**, since one copy keeping the reasoning is the mechanism this issue is about.
2. `views.rs` and `render.rs` both call it; the four private functions go.
3. The canary: change one arm's string and assert that a single edit reddens both surfaces' tests. Today a one-arm change reddens at most one.

**Consider whether `Display` is the right trait rather than an inherent method.** It reads better at call sites, but it puts the human spelling on the same type serde renders in kebab-case for the wire, and two audiences on one type is how a wire format ends up in a terminal. An inherent `display()` keeps them visibly separate. Recording the trade-off rather than deciding it -- the call belongs to whoever owns `model.rs`.

## Related

- 0047 -- found by running this issue's own canary before closing: the consolidated home is only partly guarded, and renaming the `NotStarted` arm reds neither surface
- 0040 -- the other finding from the same review; a setting read never, rather than a vocabulary spelled twice
- `transitions.rs` (MODULES.md) -- the same class of model fact, centralised, and the precedent for the fix
- `IN-AG-HIGHLANDER-001` -- two divergent copies of one concern, currently identical, with nothing preventing divergence
- D02 -- a file is entirely authored or entirely generated; the views' spelling reaches committed artefacts, which is why the drift direction is asymmetric

## Resolutions

### CLOSED 2026-08-17 (vc). The duplication is gone and verified. The canary this issue asked for was MEASURED, holds for some arms and not others, and that residue is filed as 0047 rather than kept here.

Verified against the working tree at HEAD `9361c68a`; `model.rs`, `views.rs` and `render.rs` all confirmed clean first.

**Proposed Fix 1 -- DONE.** `pub fn display(self) -> &'static str` is an inherent method on both enums, in `model.rs`: `ThreadStatus` at `:238` with all six arms, `WpStatus` at `:393` with three. The trade-off this issue recorded rather than decided -- inherent method versus `Display` -- was decided the way it was leaning, and for the reason given: serde writes kebab-case for the wire, these are the words a person reads, and the doc comment now says so in place ("Two vocabularies for two audiences is correct; two copies of one vocabulary is not").

**Proposed Fix 2 -- DONE.** All four private functions are gone: no `status_display` or `wp_status_display` in `views.rs`, no `status` or `wp_status` in `render.rs`. Outside `model.rs` the literal `"Not Started"` does not occur anywhere in either crate's `src`. The thin skin no longer knows something about the model that the model does not export.

**And the part this issue said must not be left behind did not get left behind.** The `views.rs:66-71` rationale moved onto the type: v2's `canonical_status` provenance, and the deliberate `TBC` / `Not Started` divergence recorded as a `corrected` register row rather than a parity break. That was the mechanism named in Root Cause -- one copy holding the reasoning and the other holding only the strings -- and it is closed by the note travelling with the strings.

**Beyond the ask, and correctly.** `is_closed()` moved onto the type in the same change, with its own reason recorded: it was private in `views.rs`, so `doctor` could not ask the question and **would have grown a second copy of the answer**. That is this issue's class caught one move before it happened rather than after.

**Proposed Fix 3, the canary -- MEASURED, AND IT DOES NOT HOLD UNIFORMLY.**

This issue asked for exactly one thing to be proven: "change one arm's string and assert that a single edit reddens both surfaces' tests." Inspection cannot answer it, so it was run -- a `git archive` extract at `9361c68a`, built clean, one arm mutated at a time, each mutation confirmed to have applied before anything was run, and a restored control at the end.

| mutation                          | `cli_end_to_end` (terminal) | `facade_st_wp` (committed md) |
| --------------------------------- | --------------------------- | ----------------------------- |
| `ThreadStatus::Wip` -> `ZZ_MUTANT`        | **FAILED, 2 of 19**         | **FAILED, 1 of 10**           |
| `ThreadStatus::NotStarted` -> `ZZ_MUTANT` | ok, 19 of 19                | ok, 10 of 10                  |
| control (restored)                        | ok, 19 of 19                | ok, 10 of 10                  |

**On `Wip` the canary works exactly as specified: one edit, both surfaces red.** On `NotStarted` the same edit is invisible to both. The rig is sound -- the control is clean and the `Wip` row proves the harness detects what it is pointed at -- so this is a property of the assertions, not of the measurement.

**This is not a regression introduced by the fix, and it is not what this issue is about.** `facade_st_wp.rs:296` pins the arm NEGATIVELY (`!after.contains("Not Started")`), which any rename satisfies; and `cli_end_to_end.rs:142` records that `st new` now creates in `Triage` rather than `NotStarted` since the machines were ratified, so the assertion at `:786` no longer traverses the arm it appears to pin. **A state-machine ratification silently defanged a display assertion, and the assertion still reads as though it pins the vocabulary.**

That is a distinct defect from the one this issue names -- it is a test that has stopped testing, not a vocabulary spelled twice -- and it has its own mechanism, its own blast radius across other arms, and no dependency on this fix. **Filed as 0047.** Keeping it here would leave a closed Highlander finding as the only record of an open coverage hole.

**So: the defect this issue reports is fixed and proven fixed.** One definition, one home, the reasoning carried with it, and a canary that fires on the arms it can reach.
