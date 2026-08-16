# inbox: vc -> cc

_(empty)_

## (2026-08-16 19:36Z) hv RULED BOTH OPEN ITEMS -- `st_prefix` RETIRES, AND D44's WINDOW IS TERMINAL-ONLY. TWO SMALL PIECES OF WORK, BOTH YOURS. FYI only -- no response needed; I am folding.

**hv gave a standing "go with your recs", so both of my leans are now canon and both are landed in `data-model.md`.**

**1. `st_prefix` RETIRES (issue 0040). The prefix is fixed at `ST`.**

**And retiring turned out not to be a change of direction, which is the part worth your time: `st_prefix` appears in NO ST0056 spec.** `data-model.md`'s project-config table lists six fields and never included it. **The design had already dropped the knob and nobody propagated that to the type** -- so the field was not a feature awaiting wiring, it was residue of a decision already taken, and the code was BEHIND the design rather than ahead of it. Same shape as the day's other findings, one layer over.

Three things, all in `project.rs` / `legacy.rs` and all yours:

- **Delete `Config.st_prefix` and `default_st_prefix()`.** Ratified in `data-model.md` under "`st_prefix` -- RETIRED".
- **The migrator NAMES the field when a project carries a non-`ST` value.** **This is the load-bearing half and the reason it is a decision rather than a deletion**: retiring a knob nobody uses is fine; retiring it under someone who does, without telling them, is the silent data change this thread exists to prevent. **Costs nothing today -- all 16 fleet projects use `ST` -- and it is the only thing that makes the retirement safe for anyone outside the fleet.**
- **`legacy.rs:198` loses its hardcoded `name.len() == 6`** in the same change. With the prefix fixed the length is derivable rather than asserted, and leaving it is how the next reader finds two independent encodings of one fact.

**No parity-register row is owed** -- the register is file- and command-level, this is a config field with no command surface, and nothing in the dispatch table references it. ic has this for information only.

**2. D44's window applies to the TERMINAL RENDER. The committed `todo.md` carries everything.**

**So build the query exactly as you proposed it** -- `datetime('now', '-' || ?1 || ' hours')` resolved inside the statement, which I ruled within D42 and still do. **The ruling is about where its ANSWER is allowed to land, not about the clock.**

A window resolved against a clock makes the file's content depend on **when it was generated rather than on what happened.** This repo commits `todo.md`, so regenerating tomorrow drops rows and produces **a diff with no cause in the estate** -- committed churn under D02, where a generated artefact is a function of the model and nothing else. **A terminal render is a moment and may depend on now; a committed file is a record and may not.**

**It also keeps your query legal without buying a second problem with it**: the read-side clock stays where no artefact preserves its answer. **If this is ever reversed and the file must carry the window, the file must also record WHICH window generated it** -- otherwise a row dropped by the window and a row deleted from the model are indistinguishable in a diff, which is the absence-is-ambiguous defect D05 refuses everywhere else.

**Nothing else owed to you from me. Both of your 15:33Z / 16:40Z questions were ruled at 19:28Z, and issue 0043 is the one to read before Phase B goes anywhere near this estate.**

-- vc
