---
id: "0013"
title: intent ac has no descope verb, so an AC moved to another thread holds its own thread BLOCKED forever
date: 2026-08-04
reporter: matts
status: CLOSED
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

FIXED + CLOSED (2026-08-14), shipped in v2.19.0. Confirmed as filed: the verbs were `list` / `status` / `satisfy` / `gate` only, satisfaction was binary, and the gate counted every AC. The measured instance had been sitting BLOCKED for days with no outstanding work.

**An AC has four states, not two.** Beyond satisfied and unsatisfied: **descoped** (`intent ac descope <ID> <AC> --to <ID>`) and **withdrawn** (`intent ac withdraw <ID> <AC> --reason "..."`), each with an undo (`rescope`, `reinstate`). The two representations previously available were both wrong -- `satisfy` is a lie because the work was not done, and leaving it unsatisfied is honest but permanent, holding a genuinely finished thread open forever.

**Each verb carries the audit payload that justifies it existing.** `descope` requires `--to` and validates it against a real thread (a descope to a thread that does not exist is a strike with extra steps) and refuses a descope to self; `withdraw` requires `--reason` (a withdrawal with no reason is a deleted line with extra steps, and deleting the line is exactly the practice it replaces). Who ruled, when, and which thread now owns it all land on the AC line -- greppable, diffable, and reportable by `ac list`.

**Off-scope states are detected by MARKER and checked BEFORE satisfaction.** Both were required by the issue and both are load-bearing. `ac_flag`'s `([a-z]+)` cannot read `n/a`, so reusing `satisfied:` for the state would have mis-parsed it; and a descoped *test-backed* AC whose covering AT went with it would otherwise find no cover and report unsatisfied -- reintroducing the false BLOCKED this issue is about, through the fix for it. Counts are reported separately (`29/29 satisfied, 1 descoped -- PASS`), never folded away, so a thread that descoped half its contract looks like one.

### Judgement calls, recorded

**hv added a withdrawal verb by direct instruction**, overtaking vc's deferral of `struck` (deferred for want of field evidence -- the request *is* the evidence). Recorded as considered-and-superseded rather than silently dropped.

**A contract emptied entirely by off-scope moves is REFUSED, not passed.** This is not in the issue. Passing on an empty set would make the new verbs a trivial gate bypass, so the refusal points at the existing `acceptance: exempt` declaration instead. ST0048's rule is that an exemption is announced and never inferred from emptiness, and a contract emptied one withdrawal at a time is still emptiness. Reversible in one line if hv prefers the other reading.

**Later correction (vc audit F2).** `ac satisfy` did not refuse an off-scope AC: on a descoped one it printed `ok:`, exited 0, and wrote a row contradicting itself, while `ac list` and the gate went on correctly reporting it descoped. Reported success with no effect -- issue 0006's shape, reachable through the very verbs added here, and precisely the dishonest bookkeeping descope exists to replace. The refusal already existed inside `ac_offscope_prepare`; it now has one home (`ac_refuse_if_offscope`) and three callers, and names the undo.
