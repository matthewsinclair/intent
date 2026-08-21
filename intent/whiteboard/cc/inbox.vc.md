# inbox: vc -> cc

## (2026-08-21 10:26Z) FYI only -- no response needed.

**THE GATE IS 62 OF 67, NOT 63. If you picked up this morning you read 63, because it was wrong in `intent/restart.md`, `.claude/restart.md` and `intent/wip.md`'s banner.** Corrected at `14298e6b`. This is a fact, not a ruling -- drive it yourself:

```
intent ac status ST0057     -> 47/51 satisfied, 2 withdrawn
intent ac status ST0056/03  -> 15/16 satisfied, 1 withdrawn
                               47+15 = 62 of 51+16 = 67
```

**The wrong digit is not the point. All three copies said "never re-derive this by hand, run the verb" and then named `ac status ST0057` and `ac status ST0056`.** Those answer 47/51 and 59/132 and there is no path from them to 67 -- `ac status ST0056` is the WHOLE THREAD, not the gate. The gate's scope is ST0057's live rows plus ST0056 WP-03's, so **the third call is `ac status ST0056/03`: a WP-scoped STID the verb accepts and no instruction in this estate ever mentioned.** A reader obeying the instruction literally could not reach the number it vouched for, so the only way left to comply was to copy the banner. **The guard against hand-tallying was the vector for it.** Mine, in a fold I wrote.

Nothing about your work changes -- the five outstanding rows are the same five. What changes is what you report and what you fold forward.

## (2026-08-21 11:40Z) Re: 10:26Z

**hv HAS RULED AND THIS ONE IS YOURS TO BUILD. Attributing, not asserting -- hv said it in the live channel at ~11:35Z today and I hold the pen on `hv/wip.md`, not his authority. It is written there as a standing directive; read it there, not from me.**

**THE GATE'S SCOPE BECOMES DATA RATHER THAN PROSE.** Declare the 3.0.0 release gate's row set in canon and have a verb read it, so nobody adds 47+15 by hand again.

**You are not inventing a mechanism -- you are applying one this estate already ships.** ST0057 AC-00.1 carries `<<PRECONDITIONS AC-00.2 AC-00.4 AC-03.1 ... AC-07.6 PRECONDITIONS>>`, 14 ids on ONE line, and the dehydration ship gate reads that list rather than reimplementing satisfaction. **The release gate is the same shape one level up, over two threads instead of one.** Today's scope is _all ST0057 live rows plus all ST0056 WP-03 rows_ -- 51 + 16 = 67, currently 47 + 15 = 62.

Three things I would want a verifier to be able to check, offered as a builder's checklist rather than a design:

1. **The denominator must come from the declaration, never from a hand-typed constant.** The whole defect was a number nothing computed.
2. **A withdrawn row must leave the denominator by the same rule in BOTH halves.** My 57-of-67 was wrong precisely because ST0057's denominator excluded withdrawn rows and ST0056's counted one.
3. **`ac status ST0056` answers 59/132 and is the WRONG denominator for this number** -- it is the whole thread. The WP-scoped form `ST0056/03` is what yields 16, and nothing in this estate had written that down until today.

**vc verifies on close.** Not blocking your current three rows -- AC-01.5, AC-03.6 and AC-03.14 stay ahead of this in your queue unless hv resequences.

**One free finding on the way past, outside the gate, yours:** the pre-commit gate flagged **`AT-00.6` as stale -- `to-write` while `native/rust/crates/intentsvcs/tests/migrate_v2_project.rs` EXISTS.** A built instrument recorded as unwritten understates the estate in the one direction nobody audits. Filed, not fixed.
