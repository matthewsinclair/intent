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
Why this cannot be left to diligence, measured rather than argued: the instrument built to catch this class -- written by a node that had spent the day cataloguing it, with the rule in front of it -- shipped four defects in one day, and **every one failed in the direction of looking clean**: a directory of threads never looked at; a give-up before the fallback that would have found the source; four projects emitting no rows at all while exiting 0; a nonexistent path returning 0 with "nothing to account for". Not one produced a false alarm. That distribution is not chance. A check that wrongly reports trouble is investigated within minutes, because someone is blocked and comes looking; a check that wrongly reports calm is adopted, then relied on, and the longer it survives the more it is trusted. The selection pressure runs one way, so the surviving population of controls in any codebase is enriched for exactly the failure nobody sees. Diligence is what the asymmetry defeats; a step in the procedure is what survives it.

The mirror image is the same question asked the other way: a control that encodes an expectation which is simply false fails when it should pass, and a working instrument is held back while its author debugs correct code. A control disconnected from its subject ships a false green; a control asserting something untrue discards a true result. Before trusting either verdict, ask what would make this control go red, and whether that is the same thing as the subject being wrong.

## Detection

Two questions, and they are not the same question:

- **WOULD this have failed?** Stash the fix and watch which assertions go red. Valid when the lever is real and only the fix is in doubt. An assertion that stays green with the fix stashed is not a control.
- **COULD this have failed?** Break the control deliberately and watch it go red BEFORE trusting any green. Owed whenever the lever itself is in doubt -- a new harness, a new probe, a negative control, a grep for absence.

Signals to look for in a verification step:

- A suite or gate piped through `tail`, `head` or `grep`, so the exit status read is the filter's.
- `set -u` (or any fail-fast mode) with the assertions after a variable the harness may not have set.
- `pgrep -f <pattern>` where the pattern also appears in the caller's own command line -- or in any long-lived process's arguments on the machine: on a box running LLM sessions, the agents' prompts name the very commands an instrument hunts for, so a full-command-line match reports them forever; match the executable (`pgrep -x`), count pids not lines, and control BOTH arms -- the one that fires and the one that must fall silent.
- A lever (an environment variable, a flag, a stub) asserted to have reached the subject with no evidence that the subject reads it.
- A green suite whose plan line, ok count and exit status were not all read and required to agree.
- A test whose assertions cannot observe the effect its name promises: it asserts the id when the defect loses the qualifier, or reads `covers` and the findings but never the note the span is folded into. **A test must read the thing its name promises, because a test that would stay green under its own defect is indistinguishable from one that holds** -- and by construction it cannot report itself; only a mutation run by someone else finds it (measured 2026-08-26: eight published mutations, one blind test, found on the second operator's run).
- A property whose only enforcement is a side effect of an unrelated mechanism: the refusal holds because an old cut left an unrecognised token, the overlap holds because an allowlist wanted an extension, the span test is green because the truncation ate the part it never read. **A property must be enforced by something that NAMES it; enforcement inherited from an unrelated mechanism expires silently when that mechanism changes, and the green survives the expiry.** Three instances in one evening (2026-08-26, cc), every one found only by removing the accident -- so when a mechanism is replaced for an unrelated reason, list the ratified properties that were riding on it BEFORE the tests go green again.
- A grep for ABSENCE with no positive control showing the pattern can match anything in that source at all.
- A comparison whose "source" side is read from the same place as its "result" side -- a working-tree file that IS the generated view compared against the view -- returns a guaranteed clean for every subject; the source comes from a ref, never from the tree.
- An instrument that names a blind spot without sizing it: "N unmeasured" with no statement of WHICH set is unmeasured lets the reader assume the set is small, when it may be every in-progress subject in the project.

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
intent st show ST0000 | grep -c SENTINEL     # prints 0 -- st show prints four lines and no AC text

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
