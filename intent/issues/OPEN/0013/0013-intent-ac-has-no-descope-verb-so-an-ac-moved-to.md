---
id: "0013"
title: intent ac has no descope verb, so an AC moved to another thread holds its own thread BLOCKED forever
date: 2026-08-04
reporter: matts
status: OPEN
severity: medium
---

# 0013: intent ac has no descope verb, so an AC moved to another thread holds its own thread BLOCKED forever

## Tags

acceptance, lifecycle, gate

## Summary

`intent ac` offers exactly four verbs -- `list`, `status`, `satisfy`, `gate`. There is no way to record that an acceptance criterion has been **descoped to another steel thread**, which is a routine and legitimate outcome: the requirement is still real, somebody still owns it, and it is simply no longer this thread's.

The only available representations are both wrong. `satisfy` is a lie -- the work was not done. Leaving it `satisfied: no` is honest and permanent: the AC counts against the thread's total forever, `intent ac gate` reports BLOCKED, and `intent wp done` refuses to close the package. A thread that is genuinely finished cannot be closed by the tool, only by a human deciding to ignore what the tool says.

## Reproduction

Observed on a real project (Laksa) on 2026-08-04, on two threads simultaneously:

```
$ intent ac status ST0085
ac: 34/35 satisfied -- BLOCKED          # AC-00.2 descoped to ST0090 by the hypervisor, 2026-08-03

$ intent ac status ST0086
ac: 29/30 satisfied -- BLOCKED          # AC-08.1 descoped to ST0092 by the hypervisor, 2026-08-04

$ intent wp done ST0086/08
gate: ST0086/08 BLOCKED -- 1/2 satisfied; unsatisfied: AC-08.1
Error: cannot close ST0086/08: acceptance contract is BLOCKED
```

In both cases the descope is a deliberate, recorded, hypervisor-level decision, written out at length in `acceptance.md` prose. The tool cannot see any of it.

ST0086 AC-08.1 is the sharp case, because no wording change could rescue it. It reads _"Shop enquiry and artist join create real records **rather than sending mail**"_, and the ruling was to ship the mailto and build the capability as ST0092. There is no interpretation under which that AC passes, and there should not be -- it correctly describes work that is not being done here.

## Root Cause

An AC's state is modelled as a boolean (`satisfied: yes|no`) when it has at least three states in practice:

| State        | Meaning                                                 | Should it block? |
| ------------ | ------------------------------------------------------- | ---------------- |
| satisfied    | The work is done and evidenced                          | No               |
| unsatisfied  | The work is outstanding **on this thread**              | Yes              |
| **descoped** | The requirement is real and has moved to a named thread | **No**           |

`struck` is arguably a fourth (the requirement was withdrawn entirely, not moved -- ST0084 WP-09's four ACs were handled this way, by deleting the lines and recording the strike in prose). Deleting lines loses the audit trail that a descope most needs: which thread took it, when, and on whose ruling.

## Impact

- **A finished thread cannot be closed.** ST0085 has been sitting at 34/35 BLOCKED since 2026-08-03 with no outstanding work.
- **The gate stops discriminating.** Once a project has descoped ACs, BLOCKED no longer means "there is work to do", so a reader learns to override it -- which is exactly the moment a real blocker gets waved through. A check that is routinely ignored is worse than no check.
- **The audit trail lives in prose the tool cannot read.** `intent ac list` shows `satisfied: no` and says nothing about the ruling, the date, or the receiving thread. Anyone reasoning from the machine-readable view sees an unfinished thread.
- **It pushes toward the dishonest option.** The pressure to just call it satisfied and move on is real, and the tool currently rewards it.

## Proposed Fix

Add a verb that records the move and its destination:

```
intent ac descope <ST> <AC-ID> --to <ST> [--by <who>] [--reason <text>]
intent ac rescope <ST> <AC-ID>                        # undo, back to unsatisfied
```

Serialised on the AC line so it stays greppable and diffable, alongside the existing fields:

```
- AC-08.1 Shop enquiry and artist join create real records rather than sending mail -- descoped-to: ST0092 -- by: hv -- on: 2026-08-04 -- satisfied: n/a
```

Then:

- `intent ac status` reports it separately rather than folding it into the failure count: `29/29 satisfied, 1 descoped -- PASS`. **Descoped ACs should not be silently dropped from the total** -- a thread that descoped half its contract should look like one.
- `intent ac gate` and `intent wp done` treat descoped as non-blocking.
- **`--to` should be validated against an existing thread**, since the whole value of a descope over a strike is that the requirement remains owned. A descope to a thread that does not exist is a strike with extra steps.

Worth deciding at the same time whether `struck` deserves a verb of its own, for a requirement withdrawn rather than moved. It has the same audit-trail problem and is currently handled by deleting lines.

## Related

- 0014 -- an AT covers exactly one AC, and the parse fails silently otherwise. Same family: acceptance-contract state that the tool models more simply than the practice needs, failing quietly rather than loudly.
- Found while closing Laksa ST0086; the two live instances are Laksa ST0085 AC-00.2 and ST0086 AC-08.1.

## Resolutions

{{TBC}}
