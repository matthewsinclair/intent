---
verblock: "24 Aug 2026:v1.17: vc - aggressive lean; three documents given one job each"
intent_version: 2.19.0
---

# Work In Progress

**Current as at `ef2f65c2`, 2026-08-24. This heading names a COMMIT, not a date** -- a wip file is read as current and written as a snapshot, and if you cannot say what it is current as at, that is the finding.

## The gate: 66 of 67

**RUN THESE. DO NOT TRANSCRIBE THE NUMBER.** It has had three homes carrying three values, and one document held it twice disagreeing with itself.

    intent ac status ST0057      -> 50/51 satisfied, 2 withdrawn -- BLOCKED
    intent ac status ST0056/03   -> 16/16 satisfied, 1 withdrawn -- PASS
    intent ac gate ST0057        -> unsatisfied: AC-08.5

**The scope is all of ST0057's live rows plus all of ST0056 WP-03's.** `ac status ST0056` answers 61/132 and is **NOT** this number's denominator. `ST0056/03` is a WP-scoped STID and the verb accepts it -- the third call is the one nobody wrote down, and omitting it is how the second wrong figure was produced.

**IT IS ST0057's CLOSURE GATE, NOT THE 3.0.0 RELEASE GATE.** The release is ST0056 WP-12, whose dependency line reads _"All prior WPs"_ against **seven Not Started**. Read as release progress it says nearly done, where ST0056 stands at 61 of 132.

## The one row left: ST0057 AC-08.5

**cc builds, ic covers** -- hv's deliberate builder/verifier split. **What blocks it is not the pin, it is three surviving BURNING CASES, and every one is a claim that a capability is ABSENT:** `ST0011.completed` is a thread field with no setter; an attachment's canon record has no setter narrower than a thread; no CLI verb creates an AC or an AT at all.

**The row's own history is four such absence claims refuted the moment somebody checked** -- `at green` was said to destroy notes and does not in v3; `sync` was said to have no operation smaller than 57 threads and takes IDs; a pin asserted no creator existed while `put` created both **thirty lines away in the same file**. **Re-drive all three before building against them.** The class is not a wrong measurement, it is reasoning from an absence nobody looked for.

## The two threads, driven 2026-08-24

**ST0056 -- the v3.0.0 rewrite.** 133 criteria / 138 tests, **61 of 132 satisfied**, 1 withdrawn. WPs 01/02/03 Done; 04/05/06/07/10/11 WIP; 08/09/12/13/14/15/16 Not Started.

**ST0057 -- disk as a sparse projection.** 53 criteria / 53 tests, **50 of 51 satisfied**, 2 withdrawn. WPs 02/03/04/06/07/09/10 Done; 01/05/08 WIP. **Sparseness applies to VIEWS; canon is NEVER sparse.**

## Next, per node

1. **cc** -- **AC-08.5, the last gate row**, after re-driving the three burning cases. Then ST0056 AC-10.4 over `migrate::plan`'s write set with a **non-empty control**; AT-10.2's second citation onto `intent-cli/tests/ingest_command.rs`; AT-10.12 held on the unexplained trim asymmetry.
2. **ic** -- covers AC-08.5. Their `issues`-surface paper for hv: `--kind` vs `--status`, four words for one concept, **and the absence behind it -- v3 `issues` has no body setter at all.** One paper, not two.
3. **dc** -- holds none of the gate. AT-11.6's deliverable is theirs and unbuilt.
4. **vc** -- four items routed to hv and awaiting rulings (below). `declared_but_unwired` adequacy; the marker's per-crate staleness.
5. **hv's standing question:** ~250 files under `intent/` are not in the store at all -- _"not all of that should be in the db, but certainly some of it should."_

## Sitting with hv

- **dc's routing question 2** -- the frozen-`$INTENT_HOME` mechanism. Detector half CLOSED (the drift guard runs unattended now); **routing NOT discharged, and a reason expiring is not a routing being discharged.**
- **`intent#0073`** -- the six swift rule dispositions. **Shipped surface, so it needs hv before it needs an editor.**
- **`intent#0074`** -- whether the elixir pack should cover `.heex` at all. That is WORK, not a fix; the misleading MESSAGE is a defect regardless.
- **`intent#0071`** -- needs a CHANGELOG entry and a v2 heading that does not exist. Release policy.

## Landed 2026-08-24

**The five-estate Claude Code config sweep, Intent as UPSTREAM.** hv's ruling governs it: prune the dross now, v3 only, **Intentv2 is FROZEN.** `MODULES.md` retired from v3 seeding; the ten per-language files deleted; the agnostic RULES/ARCHITECTURE pair restored rewritten, shipping empty on purpose; `intent claude upgrade`'s downgrade hole closed (its probe tested `local == target` and **equality has no direction**); the elixir template stopped asserting project facts it cannot know.

**`intent upgrade` was destroying every issue in an already-migrated v3 project** while printing `0 issue(s)` -- dc found and reproduced it, cc fixed it in `migrate::plan`, dc closed it. `intent#0070`. It emptied this store; restored and verified 47 == 47.

**The shipped-surface drift guard now RUNS in CI instead of skipping.** It reported green over nothing for its whole first day -- all three tests skipped on a runner **including the positive control** -- while printing `All tests passed!`.

**A 1-in-3 Linux CI flake closed.** `ExecutableFileBusy`, four of the last twelve `rust` runs, all one cause. The remedy already existed in one of the three files that needed it.

**Eleven issues open** (`intent issues list`): `0063` `0064` `0065` `0066` `0067` `0068` `0069` `0071` `0072` `0073` `0074`.
