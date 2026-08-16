---
id: "0041"
title: the human-facing status vocabulary is spelled twice in two crates -- views.rs writes the committed md and render.rs writes the terminal, and nothing relates them
date: 2026-08-16
reporter: matts
status: OPEN
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

- 0040 -- the other finding from the same review; a setting read never, rather than a vocabulary spelled twice
- `transitions.rs` (MODULES.md) -- the same class of model fact, centralised, and the precedent for the fix
- `IN-AG-HIGHLANDER-001` -- two divergent copies of one concern, currently identical, with nothing preventing divergence
- D02 -- a file is entirely authored or entirely generated; the views' spelling reaches committed artefacts, which is why the drift direction is asymmetric

## Resolutions

{{TBC}}
