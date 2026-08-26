---
id: IN-AG-RED-CONTROL-001
language: agnostic
category: test
severity: critical
title: A control is only a control if it can go red
summary: >
  Before trusting any green, name what would make this control go red,
  then make it go red. A check removed from the causal path still reports:
  a piped tail that hides an exit status, an unbound variable that ends a
  script before its assertion, a liveness probe that matches its own
  command line, a lever that never reaches the resolver it was aimed at.
  Each returns a well-formed green, and red-first work emits the two
  shapes most exposed to it -- assert-failure and assert-absence -- first.
principles:
  - honest-data
  - no-silent-errors
applies_when:
  - "Any verification step whose result you are about to act on: a test run, a gate, a probe, a grep for absence, a liveness check"
  - "Red-first work, because the first assertions written are assert-failure and assert-absence, and a script that produces nothing satisfies both"
  - "A control introduced to prove a fix, when the lever itself has not been shown to reach the subject"
does_not_apply_when:
  - "A measurement whose positive control was demonstrated in the same run and whose instrument has not changed since"
  - "Read-only reporting from which no decision follows"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-AG-HIGHLANDER-001
aliases: []
status: active
version: 1
---

# A control is only a control if it can go red

Before trusting any green, name what would make this control go red, then make it go red. A test, gate or probe that has been removed from the causal path between the subject and the verdict still reports -- and it reports green, in a well-formed shape, with nothing to distinguish it from a control that ran.

## Problem

Four instances in one day, on two estates, each caught only after it had been read as a green:

- A suite run as `bats tests/unit/ | tail -6`: the pipeline's exit status is the tail's, and six lines cannot show a failure at test 200.
- A test harness that omitted a variable the script reads under `set -u`: the script died before its assertion, and two of four tests passed anyway -- one asserting failure, one asserting a string's absence. A suite satisfied by the script never starting is green against any fix at all.
- A liveness check `pgrep -f 'bats tests/unit'` that matched its own command line and reported "still running" indefinitely.
- A negative control that set `INTENT_HOME` to a directory without templates, expecting a refusal: the hook under test resolves its home by parsing `intent info`, not from the environment, so the lever never reached the resolver and the control passed for the wrong reason.

The class was already written down on the board that found it, and the note stopped none of the four. It has to be a STEP performed before a green is trusted, not a catalogue entry.

The sharp edge is that red-first discipline, done correctly, emits the vulnerable shapes first: before the fix exists you write the assertion that the bad path fails and the assertion that the bad output is absent, and both are satisfied by a script that produces no output at all.

## Detection

Two questions, and they are not the same question:

- **WOULD this have failed?** Stash the fix and watch which assertions go red. Valid when the lever is real and only the fix is in doubt. An assertion that stays green with the fix stashed is not a control.
- **COULD this have failed?** Break the control deliberately and watch it go red BEFORE trusting any green. Owed whenever the lever itself is in doubt -- a new harness, a new probe, a negative control, a grep for absence.

Signals to look for in a verification step:

- A suite or gate piped through `tail`, `head` or `grep`, so the exit status read is the filter's.
- `set -u` (or any fail-fast mode) with the assertions after a variable the harness may not have set.
- `pgrep -f <pattern>` where the pattern also appears in the caller's own command line.
- A lever (an environment variable, a flag, a stub) asserted to have reached the subject with no evidence that the subject reads it.
- A green suite whose plan line, ok count and exit status were not all read and required to agree.
- A grep for ABSENCE with no positive control showing the pattern can match anything in that source at all.

Greppable proxy (not authoritative; a critic confirms by reading the step):

```bash
grep -rnE '(bats|cargo test|mix test|pytest)[^|]*\| *(tail|head|grep) ' scripts/ bin/ tests/
grep -rnE 'pgrep -f' scripts/ bin/ tests/
```

## Bad

```bash
# the exit status is tail's; a failure at test 200 is invisible and rc is 0
bats tests/unit/ | tail -6 && echo "suite green"

# a grep for absence with no evidence the pattern could ever match here
intent st show ST0001 | grep -c SENTINEL     # prints 0 -- st show prints four lines and no AC text

# a liveness check that matches itself
while pgrep -f 'bats tests/unit' >/dev/null; do sleep 5; done   # never exits: this loop IS a match
```

Each line produces a plausible, well-formed result that is independent of the subject.

## Good

```bash
# run unpiped to a file; read the plan line, the ok count and the exit status, and require all three to agree
bats tests/unit/ > "$out" 2>&1; rc=$?
grep -E '^1\.\.[0-9]+$' "$out"; grep -c '^ok ' "$out"; grep -c '^not ok ' "$out"; echo "rc=$rc"

# COULD it have failed: plant the finding and assert it SURVIVES; stash the fix and list which tests went red
git stash; bats tests/unit/gate_critic.bats; git stash pop     # the tests that stayed green are not controls

# a grep for absence, positive-controlled first with a phrase known to be present in the same source
grep -c 'FOURTEEN handlers' canon.json      # 1 -- the instrument can see this source
grep -c 'Drift is already measurable' canon.json   # 0 -- now the zero means absent

# liveness by pid, not by pattern
kill -0 "$pid" 2>/dev/null
```

The green is read off a plan line, an ok count and an exit status that had to agree; the control was shown red before its green was trusted; the zero was earned by a one.

## When This Applies

- Every gate, suite and probe whose result gates a commit, a release step or a claim to a peer.
- Every negative control and every grep for absence.
- Red-first work, at the moment the first assert-failure or assert-absence assertion is written.

## When This Does Not Apply

- A measurement whose positive control was demonstrated in the same run, with the same instrument.
- Read-only reporting from which no decision follows.

## Further Reading

- Intent cutover runbook, 2026-08-26 (`intent/whiteboard/vc/cutover-runbook.md`): the four instances, and the memory that a precision figure is a claim about the corpus.
- Devbin issue 0018 and its addendum: a verdict scoped to the world it was measured in.
- IN-AG-NO-SILENT-001: the production-code face of the same principle -- a failure that produces no signal is not a success.
