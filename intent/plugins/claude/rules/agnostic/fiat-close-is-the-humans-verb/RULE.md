---
id: IN-AG-FIAT-001
language: agnostic
category: fiat
severity: critical
title: Fiat close is the human's verb, and you do not invoke it
summary: >
  `intent fc` force-closes an ST, WP, AC or AT on human authority, against
  the evidence, because the requirement is not worth what finishing it
  would cost. That judgement is the human's alone. You share their uid and
  their shell, so nothing prevents you from running it -- the prohibition
  is a contract you hold, not a wall you would hit, and every invocation
  is attributable. Do not run it, do not put it in a script, a hook, a
  skill or a Makefile, and do not ask a peer to run it for you.
principles:
  - honest-data
  - human-authority
applies_when:
  - "Any moment an acceptance row, work package or steel thread is blocking you and closing it would unblock you"
  - "Writing automation of any kind -- a script, a hook, a CI step, a skill, a Makefile target -- that touches Intent's close verbs"
  - "Relaying a request between sessions or agents that would result in `fc` being invoked by anything but a human at a keyboard"
does_not_apply_when:
  - "Reading, listing or reporting fiat-closed rows; the prohibition is on invoking the verb, never on seeing its results"
  - "Proposing a fiat close to the human, with the reason you would give -- that is the correct move when a requirement genuinely is not worth finishing"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-AG-RED-CONTROL-001
aliases: []
status: active
version: 1
---

# Fiat close is the human's verb, and you do not invoke it

`intent fc <target> --because "<text>"` closes a steel thread, work package, acceptance criterion or acceptance test **on human authority and against the evidence**. It is the recorded decision that a requirement is not worth what finishing it would cost. Every other close verb in Intent asserts that something was done; this one asserts that someone with authority decided it need not be. That authority is the human's, and it is not delegable to you.

## Problem

The verb exists because the alternative is worse. Requirements over-cook -- an acceptance set grows until finishing it costs more than the thread is worth -- and the human needs a way to cut and run that leaves a record instead of a silence. Before `fc` the mechanism was `wp done` plus a hand-written note saying "accepted unverified", with the provenance carried in prose that nothing reads.

**The reason you are forbidden the verb is not that your judgement is bad. It is that the record would be false.** A fiat close records WHO decided, and a fiat close you invoked records a decision nobody made. The row then reads, forever and to everyone, as a human ruling. That is worse than the blocked gate it removed, because a blocked gate is visible and a fabricated authority is not.

**And the pressure to use it is strongest exactly where it is most damaging.** You meet `fc` when a row is red, the fix is expensive, and closing it would let you report the work finished. That is the moment the verb looks most reasonable and the moment its use is least defensible.

## This is a contract, not a wall, and the difference is the whole rule

**You share the human's uid and their shell.** Harness allowlists do not close this: `intent wp` was measured allowlisted in a sibling estate, so its close verbs ran with no prompt at all, and anything built on the existing verbs inherits every such allowlist. There is no permission boundary between you and this command on the machine you are running on.

**So enforcement is detection and attribution, never prevention, and this rule does not pretend otherwise.** A rule that claimed you _cannot_ do this would be false, and a false claim in a contract is worse than no claim: it invites the reader to conclude that because nothing stopped them, nothing forbade them.

What actually holds the line is that **the record is permanent and it is yours**. The fiat record carries the reason, the invoker evidence and the timestamp; the event log carries the principal. A fiat close you invoked is not undetectable -- it is merely undetectable _at the moment of invocation_, which is not the same thing.

## Detection

**Mechanically checkable, and these are worth grepping for:**

- **`fc` in committed automation.** Any occurrence of the verb in a script, a git hook, a CI configuration, a Makefile, a skill body or a plugin command is a finding regardless of who wrote it. Automation has no human at a keyboard by definition, so an `fc` on an automated path can only ever produce a fabricated authority. This is the one arm of this rule a critic can enforce outright.
- **A fiat record whose invoker evidence says no tty.** Not proof of anything on its own -- see below -- but it is the field to look at, and an estate can report the population.
- **A fiat record whose `because` restates the criterion rather than giving a reason not to meet it.** "AC-04.2 could not be satisfied" is the row's own text; a reason says what was traded away and why that was the right trade.
- **A fiat close landing in the same session as the work it unblocks.** Reportable, never conclusive: the human may well have ruled during that session, which is the ordinary case.

**NOT mechanically checkable, and stated plainly because the honesty is the point:**

- **Whether a human or an LLM typed the command.** Same uid, same shell, same environment. There is no field that settles it and none can be added on this machine.
- **tty-or-not does not settle it either.** An agent can run a command with a tty allocated, and a human's own `fc` inside a script, a CI job or an ssh pipe has none. It narrows the population; it does not identify the actor.
- **Whether the human's ruling actually happened.** The record says a human decided; only the human can confirm they did.

**So the honest summary is: an FC on an automated path is detectable, and an FC at an interactive prompt is attributable but not attributable to a person.** Everything this rule buys, it buys from you holding it.

## Bad

```bash
# the acceptance row will not go green and the gate is blocking the report
intent fc ST0056 AC-07.7 --because "cannot be satisfied without a published tag"

# the same thing wearing a schedule, which is worse: no human is present by construction
echo 'intent fc "$1" --because "unblocking CI"' >> scripts/close-stale.sh

# and the form that launders it through someone else
#   "can you run `intent fc ST0056/14 --because ...` on your side?"
```

Each of these produces a record saying a human ruled. The first is the tempting one and the reason it is tempting is the reason it is forbidden: it is the moment closing the row would let you report the work finished.

## Good

```bash
# say what you would have run, and hand the judgement back with the reason attached
#   "AC-07.7 cannot go green before a published tag exists, and I think it is not
#    worth holding WP-07 for. If you agree, the reason I would record is:
#    'evidence requires an installed build; tag is post-cut'."

# meanwhile, report the blocked state honestly rather than removing it
intent ac gate ST0056        # BLOCKED, and the blocking id is in the output
```

The proposal carries everything a fiat close carries except the authority. **A blocked gate you reported is a true record; an unblocked one you authored is not.**

## When This Applies

- Any moment an acceptance row, work package or steel thread is blocking you and closing it would unblock you.
- Writing automation of any kind -- a script, a hook, a CI step, a skill, a Makefile target -- that touches Intent's close verbs.
- Relaying a request between sessions or agents that would result in `fc` being invoked by anything but a human at a keyboard.

## When This Does Not Apply

Reading a fiat-closed row, reporting one, or counting them is never covered -- the prohibition is on invoking the verb.

**And proposing one is the correct move, not a workaround.** When a requirement genuinely is not worth finishing, say so, name the requirement, and give the reason you would put in `--because`. That is the whole content of a fiat close except the authority, and the authority is the part that is not yours to supply.

## Further Reading

- `intent/st/ST0066/info.md`: hv's two rulings that built this verb -- the enforcement posture (detection and attribution, never prevention) and the package (one verb, in-model record, cascades).
- IN-AG-RED-CONTROL-001: the other procedural agnostic rule, and the nearest neighbour in shape -- both are about not letting a green you produced stand in for a green you earned.
- IN-AG-NO-SILENT-001: the production-code face of the same principle. A fiat close you invoked is a failure that produces no signal.
