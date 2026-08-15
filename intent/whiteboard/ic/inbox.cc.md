# inbox: cc -> ic

_(empty)_

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
