---
wp_id: WP-14
title: The DONE cutoff is canon state, not history: it travels by git
scope: M
status: Done
---

# WP-14: The DONE cutoff is canon state, not history: it travels by git

## Objective

**The DONE cutoff must survive a git clone, and today it does not.** It is stored only as a `todo.flush` EVENT, D53 took the event log out of the working tree, and so a machine that has never run a command in the project has no cutoff at all. Measured on Intent's own estate: **all 54 completed-or-cancelled threads reappear in DONE on a fresh clone**, and `doctor` reports the committed `todo.md` as hand-edited, permanently, on every project that has ever flushed. That is the v2 defect the watermark exists to prevent, arriving through a different door.

## The distinction that makes this a fix and not a workaround

**"A flush happened at T" is HISTORY. "The current cutoff is T" is STATE.** They were the same field, and filing the cutoff as history is what put it on the wrong side of D53.

- **History stays exactly where D53 put it.** The `todo.flush` event keeps its stamp, every flush is still kept, two machines' logs still merge, and `openness.rs`'s assertion that the disk trip does not carry history is untouched.
- **State goes in the canon**, which is what carries project facts on disk and moves them through git.

hv ruled it on 2026-08-26: the cutoff is canon state.

## Where, and why not the two nearer homes

**`intent/.canon/project.json`, a new project-level canon artefact.** The canon holds `st/` and `issues/` and has no project-level file; this creates one, and it is the right home for project-level recorded state generally rather than a lean-to built for one scalar.

**NOT `intent/.config/config.json`.** Config holds CHOICES a person makes -- `todo.window_hours`, `backup.schedule` -- and is hand-editable. The cutoff is a fact a command recorded. Putting a machine-written value in a hand-edited file gives one file two writers, which is the hazard this project has paid for more than once.

**NOT the thread canon.** The cutoff is project-level; there is no thread it belongs to.

## THE RULE THAT KEEPS THIS FROM BECOMING TWO HOMES

**The canon file is the ONE home for the current cutoff, and NOTHING reads a cutoff out of the log.** `event::todo_watermark` is REMOVED rather than kept as a fallback. A fallback is the gate-figure defect in miniature: two homes, two values, and drift that nobody sees because both answers look plausible.

The one exception is the migration, which reads the log exactly once to derive the value it is writing, and is the last thing ever to do so.

## Shape

```json
{
  "schema": "intent/project/1",
  "todo_watermark": "2026-08-26T22:27:46Z"
}
```

`todo_watermark` is optional. **Absent means never flushed**, which is a fact the model can now hold -- v2 could not represent it, because it read the cutoff back out of the generated file and an absent file had to fall back to a clock.

## Write path

`Facade::todo_flush` records BOTH in one transaction: the `todo.flush` event (history) and the project state (the cutoff). `render_ctx` and `doctor` read the state and never the log.

## What it closes

Both of the tests that have been red since the watermark was restored go green **by the fix rather than by being rewritten to a weaker contract**:

- `a_flush_survives_a_machine_that_has_no_database` -- the clone carries `.canon/project.json` like every other canon file.
- `doctor_does_not_report_a_flushed_view_as_hand_edited` -- doctor reads the cutoff from a file, so it has one with or without a store.

## Migration

An existing project has flushes in its log and no `project.json`. The migration derives the cutoff from the log's maximum `todo.flush` stamp, writes it once, and after that nothing reads a cutoff out of the log again. A project that has never flushed gets a file with no `todo_watermark`, which is the correct representation of never-flushed.

## The acceptance criteria were authored AFTER the code, and from this cover

`AC-14.1` through `AC-14.8` were minted on 2026-08-27, after the build landed at `225b9c88` -- a commit prefixed `wip(`, which was the only record anywhere that the work was unfinished, and which no tool, no gate and no pickup consults. The work package still read `Not Started` with zero criteria while six green tests made it look finished from the code side.

**They are derived from the claims on THIS PAGE, not from what the code does.** Criteria written to fit an existing build are the degenerate case with the answer known in advance: they are satisfied by construction and prove nothing. Where the build happens to satisfy one, `todo_watermark.rs` is cited as EVIDENCE rather than as the criterion's origin.

**This is stated because a reader in six months cannot otherwise tell them from criteria that drove the work, and that difference is the whole of their evidential value.** (ic, on vc's word under hv's pen, 2026-08-27.)

### Two are left unsatisfied and VISIBLE, deliberately

**`AC-14.7` -- one transaction -- is UNCOVERED.** Nothing drives it. A run that wrote the `todo.flush` event and failed before the project state would leave a flush no cutoff reflects, and no test forces that ordering; reaching it needs an injected mid-transaction failure this estate has no harness for.

**`AC-14.8` -- the migration -- is NOT BUILT**, which is a stronger statement than untested. `git log -S TODO_FLUSH -- native/rust/crates/intentsvcs/src/migrate.rs` is EMPTY: the migration this cover names as *the one exception* was never written. The `todo_watermark: None` at `migrate.rs:465` is the v2 -> v3 conversion and is correct there for the reason recorded beside it -- a conversion has never been flushed -- so it is not that migration wearing a disguise.

**And the same gap is visible from the other end.** Measured 2026-08-27: of 14 estates carrying a `.canon/`, **2 have `project.json`** (both tracked, both carrying a watermark) and **12 have none at all**. `ingest::carry_project_state` is absent-leaves-the-store-alone, so a machine that has never run a command in one of those 12 has no cutoff -- which is the defect this work package exists to prevent. The missing migration and the missing files are one gap seen from two ends.

Neither is softened, scoped out, or described as deferred: an unsatisfied criterion naming a real absence is worth more than a green work package.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0057/acceptance.md`, under the `WP-14` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0057.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `the thread canon`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
