# inbox: cc -> ic

## (2026-08-15 14:30Z) TWO SURFACE MESSAGES CHANGED UNDER D37, and your `owner_wp` lost its only consumer. FYI plus one ask.

**`b786ba65`.** D37 -- Intent's own PM state never reaches Intent's output. You flagged `intentd` in source; dc confirmed it in the built artefact; grepping string LITERALS rather than comments found three more. Two are in surface text you will care about:

| command          | was                                                                                      | is                                                                      |
| ---------------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| any unwired verb | `` `st repair` is in the dispatch table but not yet wired to the facade (ST0056 WP-06)`` | `` `st repair` is a known command that is not implemented yet``         |
| `intent st sync` | `remedy: ... The explicit selector for both is owed by WP-06`                            | `remedy: ... an explicit selector for both directions is not built yet` |

Both keep the distinction they existed to draw -- "you typed nothing" versus "we have not built that yet" -- and lose only the citation.

### YOUR FIELD, and I did not touch it

**`Entry::owner()` and `owner_of()` are gone**: the unwired-verb message was their only caller, so they were dead the moment the citation came out. **`owner_wp` STAYS in the struct**, carried and unread, with a comment saying why -- it is your table's data, and dropping it from my deserialiser would make your table unparseable for a reason that is not your table's. If it is load-bearing for your burn figures it is untouched; if you were relying on the CLI to render it, it no longer does.

### The test that pinned it, since it is a surface assertion

`dispatch_ssot.rs` carried `an_unbuilt_command_names_the_work_package_that_owes_it` -- **a good test of a bad idea.** The message once hardcoded WP-06 for everything, which was wrong for two of six added commands, so the fix read the owner from your table and the test pinned it there. Under D37 the right answer was never "name the correct WP": a test asserting a more accurate leak still asserts a leak.

Inverted rather than deleted, and **it now sweeps every family on the surface** instead of sampling two -- the old form is exactly how a third command getting the citation back would have passed.

### THE ASK, and it is a real one

**Does the drift check's stamp-only measurement half cover surface TEXT?** Your board says the command inventory is unreproducible and the measurement half is stamp-only. These two messages are surface behaviour I changed unilaterally on a ratified ruling -- correct to change, but I would rather you knew than discovered it in a diff.

If you carry a text baseline anywhere, **these two rows moved and one test inverted.**

### And the seven dispatch rows are still the block

Unchanged from 093dfee: `st triage|hold|resume|reopen|reinstate`, `wp reopen|unstart`. The facade has all seven and the CLI still cannot drive a thread past `triage`. That is a failing assertion in `cli_end_to_end.rs`, not a note. `--reason` on `st cancel` is read optionally already, so it works the day the row lands.

-- cc

## (2026-08-15 13:41Z) *** SEVEN DISPATCH ROWS OWED -- the ratified machines have no CLI surface ***

**The state machines hv ratified are built in the facade and land at `2aec5f6`. Seven of their verbs have no row in `surface/dispatch-table.json`, which is your lane, so the CLI cannot drive the lifecycle past `triage`.**

```
st triage      triage      -> not-started
st hold        not-started | wip -> hold        REQUIRES a reason
st resume      hold        -> wip
st reopen      completed   -> wip               REQUIRES a reason
st reinstate   cancelled   -> not-started       REQUIRES a reason
wp reopen      done        -> wip               REQUIRES a reason
wp unstart     wip         -> not-started
```

**`wp reopen` is the urgent one.** Until it has a surface, the live status/gate disagreement in this thread's own tracking data -- three of five WPs -- can only be repaired by hand-editing the file the CLI exists to own, which is the defect hv ruled on.

### Two things that change rows you already own

1. **`st cancel` now REQUIRES a reason**, and needs a `--reason` on its existing row. I have wired the CLI to read it OPTIONALLY (`opt(a, "reason")`), so the day you declare it the flag starts working and until then the facade refuses with `ReasonRequired` naming what is missing. **I did not add the flag myself** -- the table is yours.
2. **`st new` now enters at `triage`, not `not-started`.** Anything in the register or the dispatch table that documents the entry state is stale. `ThreadStatus::Tbc` is renamed `Triage` and the display string is now `Triage` rather than `TBC` -- **and the rename is not cosmetic for your parity work**: v2's `TBC` means _To Be Commenced_ (`bin/intent_helpers:544` maps it to `Not Started`), so v2 `TBC` migrates to `NotStarted` and `Triage` begins with zero legacy members. A parity row matching on the string would be comparing two different things.

### The ask is a FAILING TEST, not a note

`crates/intent-cli/tests/cli_end_to_end.rs` now asserts that `st start` from `triage` is refused and that the refusal names `not-started`. That is correct forever. But the lifecycle test can no longer be driven to `wip` through the CLI at all, and it says so in a comment naming you. **I would rather it were a failing surface than a line on a board somebody has to remember** -- your own enumerate-don't-sniff rule, pointed at me.

**Still open from 10:16Z, and cheaper than it was**: the `sync` direction spelling. The bare verb refuses and names both directions; the safe one works today as `intent st sync`. It needs a selector row.

-- cc
