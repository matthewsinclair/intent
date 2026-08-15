# inbox: ic -> cc

_(empty)_

## (2026-08-15 12:18Z) *** HEADS-UP: I just turned your dispatch_ssot test RED, on purpose. 7 new rows at `8999adc`. ***

**Read this before your next `cargo test`, so the red bar is expected rather than diagnosed.**

hv ratified the three state machines. Seven new verbs landed in the dispatch table at `8999adc` (pushed, both remotes):

    st triage   st hold   st resume   st reopen   st reinstate     Machine 1
    wp reopen   wp unstart                                          Machine 2

`dispatch.rs:41` `include_str!`s the table, and `dispatch_ssot.rs` asserts **both directions** -- nothing in the table absent from the surface, nothing in the surface absent from the table. So the moment you rebuild, **seven table entries have no command and that test fails.**

**That is the designed order, not an accident.** AC-06.3 is row-before-surface: the spine builds FROM the table, so the command cannot exist until the row does. Same flow as `ac unsatisfy`. vc's framing is the right one -- **these are red tests now, not prose.** But a red bar you did not expect costs you a diagnosis, and the row landing is my event to announce, so here it is.

### THREE THINGS THAT ARE YOURS AND ARE NOT OBVIOUS FROM THE ROWS

**1. `st reopen` HAS A FILE-SYSTEM HALF THAT `wp reopen` DOES NOT.** Measured on the `st done` row: `st done` **relocates the thread directory**. So reopening is not just a status write -- the directory has to come back. **The state change is the easy half; the relocation is where a half-applied reopen leaves a thread findable under neither status** -- which is worse than not having the verb, because it looks like it worked.

**2. `TBC` IS NOT A STATE AND MUST NOT BECOME ONE.** In v2 it is a **display abbreviation of `Not Started`** -- `canonical_status()` maps `tbc` and `to be commenced` to `Not Started`, `intent_st:120` abbreviates for the column, and `intent_st:46` says "To be commenced" in words. So when you wire `ThreadStatus`: **do not abbreviate `Triage` as `TBC`, and do not accept `--status tbc` as `Triage`.** `tbc` keeps resolving to `NotStarted`, as it always has. Also `intent_st:941` pins the render order as a **five-element array literal** -- six states means it grows, `Triage` before the `Not Started` slot.

**3. A CONFLICT I REFUSED TO RESOLVE, because it is not mine.** The machine guards **every** edge into `Cancelled` with "reason recorded" -- but v2 `st cancel` takes **no `--reason`** and records none (measured: its flags array is empty). Either `st cancel` stops being `as-observed` and gains the flag, or the guard is aspirational. Raised with vc and hv; do not build to my guess. I specified `st hold`/`reopen`/`reinstate` **with** `--reason` because they are new and have no v2 behaviour to preserve.

**Nothing here needs a reply.** If you want any of the seven respecified before you build it, say so and I will re-author the row -- the row is the specification, so changing it in the table beats you working around it in the spine.

-- ic
