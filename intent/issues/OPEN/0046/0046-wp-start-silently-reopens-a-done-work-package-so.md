---
id: "0046"
title: wp start silently reopens a Done work package, so v3's wp reopen -- whose whole point is that the transition carries a reason -- ships beside a keep-classified verb that performs it without one
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0046: wp start silently reopens a Done work package, so v3's wp reopen -- whose whole point is that the transition carries a reason -- ships beside a keep-classified verb that performs it without one

## Tags

parity, lifecycle, silent-state-change, two-writers, ST0056, measured

## Summary

**`intent wp start` on a `Done` work package moves it back to `WIP`, at exit 0, with no warning, no confirmation and no record of why.** The message it prints is `started:`, identical to starting a Not-Started one. `bin/intent_wp:208` writes the status with an unconditional `sed` and reads nothing first.

**The dispatch table declares `wp reopen` as `new-surface`, help `Reopen a done work package back into Wip, with a reason`.** That declaration says the capability does not exist today. **It does. It exists under a verb named `start`, and it does the one thing `reopen` was invented to require -- carrying a reason -- not at all.**

`wp start` is classified `keep` / `as-observed`, so **v3 reproduces it faithfully and then ships `wp reopen` beside it.** Two doors to one transition, one enforcing the new requirement and one silently bypassing it.

Found by vc, 2026-08-17, while measuring the WP-status-vs-gate disagreements on ST0056 -- and found only because a claim on my own board (_"the verb is the fix"_) was checked instead of repeated.

## Reproduction

Measured against v2 (`2.19.0`) in a throwaway project, since ST0056's own work packages must not be mutated to test this.

```
$ intent wp done ST0001/01
gate: ST0001/01 EXEMPT -- acceptance.md declares 'acceptance: exempt'
done: ST0001/WP-01
$ grep '^status:' .../WP/01/info.md
status: Done

$ intent wp start ST0001/01
started: ST0001/WP-01
  title: Fixture wp
                                                    -> rc=0
$ grep '^status:' .../WP/01/info.md
status: WIP
```

**Nothing in that output distinguishes reopening closed work from starting new work.**

The mechanism, `bin/intent_wp:208`:

```sh
sed -i.bak "s/^status: .*$/status: WIP/" "$WP_FILE"
```

No read of the current status, so no branch on it. Compare `cmd_done`, which consults the acceptance gate before writing (`intent_acceptance`, ST0044) -- **the close is gated and the re-open is not.**

**The register rows, for contrast:**

| path         | disposition   | help                                                        |
| ------------ | ------------- | ----------------------------------------------------------- |
| `wp start`   | `keep`        | Mark a work package as WIP                                  |
| `wp reopen`  | `new-surface` | Reopen a done work package back into Wip, **with a reason** |
| `wp unstart` | `new-surface` | Return a started work package to NotStarted                 |

## Root Cause

**Two independent facts that are only a defect together.**

1. **`wp start` is a state SETTER, not a transition.** It writes `WIP` over whatever was there. That is defensible for a small tool and it means the verb's name describes one of its effects rather than its behaviour.
2. **`wp reopen` was designed as though the transition did not exist.** Its help promises a reason because a reason is what a reopen ought to record -- a correct design, made without measuring whether anything already performed the move.

**Neither is wrong on its own and the pair is cc's two-writers rule exactly: _a ruling enforced on one of two writers is enforced on neither reliably_ -- the uncovered writer wins whenever it runs last.** Whoever reaches for `wp start` out of habit gets the transition with no reason attached, and nothing anywhere reports that the reason requirement was skipped.

**It is also the class ic named on `backup` and it is why no arm will find the next one: a row SILENT about what it does has nothing on itself to contradict.** `Mark a work package as WIP` is true. It is the whole truth only for a work package that is not Done.

## Impact

**Live on ST0056 right now, and it is what made this measurable.** Three of the seven started work packages disagree with their own acceptance gate, in two opposite directions:

| WP  | status   | gate            | direction                                    |
| --- | -------- | --------------- | -------------------------------------------- |
| 03  | WIP      | **PASS** 11/11  | done by contract, not closed                 |
| 04  | **Done** | **BLOCKED** 4/6 | closed against a contract it no longer meets |
| 05  | WIP      | **PASS** 6/6    | done by contract, not closed                 |

**WP-04's history is the case worth reading.** It closed `5/5` at `1fcf35e7` (2026-08-14) with every criterion satisfied. Since then **AC-04.6 was ADDED** on hv's D32 ruling (`4c376434`, 2026-08-15) and **AC-04.1 was STRENGTHENED** -- its text gained _"the DB write transactional and all-or-nothing, and the file projection unwound on failure"_ and `AT-04.1` moved `green -> red`. **So a `Done` status is describing a contract that no longer exists, and two different events produced that: one criterion added, one criterion tightened.**

**The remedy is available today and is the defect.** `wp start ST0056/04` would reopen it -- silently, at exit 0, recording nothing about D32 or the strengthened AC-04.1. **The one thing anybody would want written down is the one thing the available verb cannot write.**

- **`IN-AG-NO-SILENT-001`**, in the state-machine rather than the error path: a durable state change that reports a different, smaller event.
- **AC-04.6 / D32 is satisfied by this transition and satisfied badly.** _"Every state an entity can enter, it can leave, by a service call reachable from every surface"_ -- it can, so the criterion holds. **D32 says nothing about the leaving being recorded**, and this is the case that shows why it might want to.

**Not claimed: that anyone has been bitten.** No reopened WP in this repository is known to have lost a reason, because until today nobody had established that `wp start` reopens at all. **The measurement is what makes the exposure knowable, and it is the reason to fix it before v3 reproduces it under `keep`.**

## Proposed Fix

**In v3, where `wp reopen` is being built anyway -- and the cheap half is a refusal, not a feature.**

1. **`wp start` REFUSES a `Done` work package and names `wp reopen`.** One branch. It converts a silent transition into a signpost, and it is the whole fix for the two-doors problem: the uncovered writer stops being a writer.
2. **`wp reopen` records the reason it already promises**, and the same treatment is owed to `wp unstart` for `WIP -> NotStarted`.
3. **`st start` wants the same check** -- not measured here, and the setter shape is shared, so assume nothing and measure it before deciding.

**For v2: nothing.** The verb is `keep`/`as-observed` and `bin/**` is not mutated in place while sessions are live. **Recording the observed behaviour on the `wp start` row is the useful v2-side act**, because `as-observed` currently points at a description that is true and incomplete.

**The canary is one line and needs no fixture beyond a closed WP: assert that `wp start` on a `Done` work package does NOT return 0.**

## Related

- ST0056 / AC-04.6 / D32 -- mutation completeness; satisfied by this transition, and the case for whether "reachable" should imply "recorded"
- `wp reopen` / `wp unstart` -- both `new-surface`, both designed as though the transition were absent
- 0044 / 0045 -- the same estate-wide shape from the exit-code side: a verb, a code or a state change whose declared meaning is narrower than its behaviour
- ic's `backup` finding -- a row SILENT about what it does, the class no arm can detect, of which this is a second instance
- cc's two-writers rule -- a ruling enforced on one of two writers is enforced on neither reliably

## Resolutions

{{TBC}}
