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

### WIDENED THE SAME DAY, AND THE TITLE NOW UNDERSTATES THE FINDING

**`wp start` is not a special case. NO v2 lifecycle verb has a state guard of any kind.** The full transition matrix was measured -- every state, every verb, a fresh project per cell -- and **every one of the 18 cells returns 0 and lands on the verb's target state.**

| entity | from          | `start` | `done`        | `cancel`      |
| ------ | ------------- | ------- | ------------- | ------------- |
| st     | Not Started   | WIP     | **Completed** | Cancelled     |
| st     | WIP           | WIP     | Completed     | Cancelled     |
| st     | **Completed** | **WIP** | Completed     | **Cancelled** |
| st     | **Cancelled** | **WIP** | **Completed** | Cancelled     |
| wp     | Not Started   | WIP     | **Done**      | --            |
| wp     | WIP           | WIP     | Done          | --            |
| wp     | **Done**      | **WIP** | Done          | --            |

**Seven of those movements are edges the ratified machines do not declare** (excluding self-loops, which are no-ops): `st start` from `Completed` and from `Cancelled`; `st done` from `Not Started` and from `Cancelled`; `st cancel` from `Completed`; `wp start` from `Done`; `wp done` from `Not Started`. **Counting self-loops as edges it is twelve.**

**Two are worse than the one this issue was filed about.** **`intent st done` on a CANCELLED thread marks it `Completed`** -- abandoned work is silently recorded as finished. **`intent wp done` on a NOT-STARTED work package marks it `Done`** -- and the acceptance gate still runs, so a unit can pass its contract and be closed without ever having been started. **The gate is consulted; the STATE is not.**

**So the shape is not "one verb has a missing guard". It is that v2's lifecycle verbs are unconditional SETTERS, and the state machine exists only in `data-model.md`.** Every one of these verbs is classified `keep` / `as-observed`, which means **v3 inherits all twelve undeclared edges by default unless somebody decides otherwise, and AC-04.6 forbids every one of them.**

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
- **AC-04.6 / D32 is BREACHED by this transition if `start` ships `as-observed`, and this is the sharpest form of the issue.** The criterion's strengthened clause is _"the implemented graph must MATCH THE RATIFIED MACHINES EXACTLY -- no undeclared edge, no missing declared edge, no undeclared state."_ **`data-model.md`'s Machine 1 gives `st start` exactly `NotStarted -> Wip` and Machine 2 gives `wp start` exactly `NotStarted -> Wip`. `Completed -> Wip` and `Done -> Wip` belong to `reopen`, guarded by `reason recorded`.** So v2 has an undeclared edge at both levels, `start` is classified `keep`/`as-observed`, and **reproducing it faithfully makes AC-04.6 false the day WP-04 reopens.** `AT-04.6`'s closure walk is the instrument that should say so.
- **And it falsifies a premise the machines were drafted on.** `data-model.md`'s finding 3 reads _"`Completed` and `Done` are one-way doors. No `reopen` at either level."_ **They are not one-way; they are unlocked and unlabelled.** Machine 2 further calls `wp reopen` _"the one whose absence is causing the live inconsistency above"_ -- **the transition was never absent.** What produced the disagreement is that criteria changed under closed units. Corrected in `data-model.md` without touching a ratified table.
- **The design survives the correction and gets sharper: what was missing was never the MOVE, it is the RECORD.** A reason-carrying door is worth building only if the unlabelled one closes at the same time.

**Not claimed: that anyone has been bitten.** No reopened WP in this repository is known to have lost a reason, because until today nobody had established that `wp start` reopens at all. **The measurement is what makes the exposure knowable, and it is the reason to fix it before v3 reproduces it under `keep`.**

## Proposed Fix

**In v3, where `wp reopen` is being built anyway -- and the cheap half is a refusal, not a feature.**

0. **THE GENERAL FORM, which supersedes the per-verb items below: every lifecycle verb reads the current state and refuses a transition the ratified machine does not declare, naming the verb that does.** The matrix above is the specification -- seven movements to refuse, twelve if self-loops are refused too. **Doing this per-verb as each is built is how six of the seven get missed**, because each one looks like an edge case on its own and only the matrix shows them as one omission. **One shared guard reading the machines, applied at the facade, is the Highlander answer**; per-verb branches are seven chances to encode the same rule differently.

1. **`wp start` REFUSES a `Done` work package and names `wp reopen`.** One branch. It converts a silent transition into a signpost, and it is the whole fix for the two-doors problem: the uncovered writer stops being a writer.
2. **`wp reopen` records the reason it already promises**, and the same treatment is owed to `wp unstart` for `WIP -> NotStarted`.
3. **`st start` wants the same check, and it is now MEASURED rather than assumed.** Filed with "assume nothing and measure it"; measured immediately afterwards, and **it is the same defect one level up and worse**, because the steel-thread verb also moves a directory:

   ```
   $ intent st done ST0001
   gate: ST0001 EXEMPT ...
     moved: intent/st/COMPLETED/ST0001
   $ grep '^status:' intent/st/COMPLETED/ST0001/info.md
   status: Completed

   $ intent st start ST0001
     moved: intent/st/ST0001
   Marked steel thread as in progress: ST0001: Fixture thread
                                                    -> rc=0
   $ grep '^status:' intent/st/ST0001/info.md
   status: WIP
   ```

   **A completed thread is silently reopened AND relocated out of `COMPLETED/`, at exit 0, and `Marked steel thread as in progress` is the same sentence it prints for work that was never done.** `st reopen` is declared `new-surface` -- _"Reopen a completed thread back into Wip, **with a reason**"_ -- **so the picture is symmetric at both levels: a designed door that requires a reason is being built beside an undeclared one that is already open and records nothing.** The refusal in item 1 is owed here too, and the directory move is why it matters more: the ST verb does not merely rewrite a field, it relocates the thread's whole directory, which is the largest silent side effect in the family.

**For v2: nothing.** The verb is `keep`/`as-observed` and `bin/**` is not mutated in place while sessions are live. **Recording the observed behaviour on the `wp start` row is the useful v2-side act**, because `as-observed` currently points at a description that is true and incomplete.

**The canary is one line and needs no fixture beyond a closed WP: assert that `wp start` on a `Done` work package does NOT return 0.**

## Related

- ST0056 / AC-04.6 / D32 -- mutation completeness; satisfied by this transition, and the case for whether "reachable" should imply "recorded"
- `wp reopen` / `wp unstart` / `st reopen` / `st reinstate` / `st resume` -- all `new-surface`, and the two `reopen` rows both promise a reason, so the same designed-door-beside-an-open-one holds at both levels
- 0044 / 0045 -- the same estate-wide shape from the exit-code side: a verb, a code or a state change whose declared meaning is narrower than its behaviour
- ic's `backup` finding -- a row SILENT about what it does, the class no arm can detect, of which this is a second instance
- cc's two-writers rule -- a ruling enforced on one of two writers is enforced on neither reliably

## Resolutions

{{TBC}}
