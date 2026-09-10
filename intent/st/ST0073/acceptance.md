---
st_id: ST0073
title: intentd owns its own lifetime: a lifeline instead of an assumed supervisor
---

# ST0073: intentd owns its own lifetime: a lifeline instead of an assumed supervisor -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The lifeline: intentd exits when the owner it can observe goes away (status: WIP)

- AC-01.1 **A daemon whose owner is SIGKILLed exits on its own, and the test kills the owner in a way NO `Drop`, handler or atexit can survive.** A test that terminated its parent politely would be measuring the path that already works. The arm asserts the daemon process is GONE, by pid, not that a socket stopped answering -- a wedged process holding the store still fails to answer and would satisfy the weaker check while being the exact defect this thread exists to remove. -- satisfied: yes (computed)
- AC-01.2 **A daemon started WITHOUT a lifeline serves until signalled -- the production path, asserted rather than assumed.** This is the control on AC-01.1 and it is the row that matters most: an implementation that exits when no lifeline was passed would sail through AC-01.1 and silently kill the `launchd` daemon, whose plist is `KeepAlive false` with no socket activation, so it would not come back until next login. **The failure would present as 'the daemon is sometimes not running' and nothing would name this change.** -- satisfied: yes (computed)
- AC-01.3 **The exit is event-driven, not swept: there is no interval to configure and none is configured.** A polled implementation passes AC-01.1 with a long enough test and leaves a window proportional to the poll period, during which the orphan is still holding the store. Evidence is structural -- no timer, no periodic `getppid`, no sleep loop on the lifeline path -- because a timing assertion cannot distinguish a fast poll from an event. -- satisfied: yes (computed)

### WP-02 -- State-dir death exit (status: Not Started)

- AC-02.1 **A daemon whose own state directory is removed stops serving**, driven by removing it under a running daemon, with a positive control that the same daemon was answering immediately before the removal. Without that control the arm passes against a daemon that never started. -- satisfied: no (computed)

### WP-03 -- One home for spawning a daemon in the test tree (status: Not Started)

- AC-03.1 **The test tree has ONE home that spawns `intentd`, and a check refuses a second.** The check reads the test sources structurally and is positive-controlled by a planted second spawn site that it must catch, and by a clean tree it must pass -- **a roster check that has never been shown to fire is decoration.** It REFUSES over an empty population rather than reporting the reassuring zero that a broken pattern also reports. -- satisfied: no (computed)

### WP-04 -- The fixture-home teardown leak (status: Not Started)

- AC-04.1 **Fixture homes do not accumulate: a suite run leaves the /tmp population no larger than it found it**, and the sweep that guarantees it runs at START rather than at exit. Exit is the path that does not run when a build is killed, which is the condition that produced all 669. -- satisfied: no (computed)

### Group 05

- AC-05.1 (non-test) **THE OUTCOME ROW, AND IT IS THE ONLY ONE THAT ANSWERS THE ORIGINAL QUESTION.** A full suite run, INCLUDING one deliberately interrupted mid-flight, leaves zero `intentd` processes that were not running before it started. Counted with `ps -axo pid=,command=`, never `pgrep -f`, which under-reports -- measured 2026-09-10 at 64 against `ps`'s 67. **Every other row here is a mechanism; this one is the estate.** Evidence: the before/after counts and the interruption method, recorded on this thread. -- satisfied: no

## Acceptance Tests

### WP-01 -- The lifeline: intentd exits when the owner it can observe goes away (status: WIP)

- AT-01.1 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-01.1, AC-01.2 -- status: green -- Two arms in one file deliberately: the lifeline arm and its production control are the same decision read in both directions, and separating them invites one to be run without the other. GREEN 2026-09-10, and BURNED IN BOTH DIRECTIONS rather than merely passing. Burn 1: the lifeline arm cut out of the select -- arms 1 and 4 FAIL and the two controls stay green. Burn 2: the arming widened from is_fifo to any stdin -- arm 2 (the production control) and the structural arm FAIL and the two lifeline arms stay green. Each burn fires on exactly the arms it should and on no others, which is what makes these four evidence rather than decoration. Four arms, not two: an owner that WRITES must not end the lifeline, and that arm caught a real defect -- sh forking for sleep left a child holding the write end.
- AT-01.3 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-01.3 -- status: green -- Structural: asserts the lifeline path carries no interval to configure. GREEN 2026-09-10, positive-controlled on the EXTRACTION rather than on the source -- a slice that missed its target would contain none of the markers and pass every assertion below it. Its first form was keyed on the VARIABLE NAME and failed on main.rs's own prose explaining why that variable was removed: mention versus use, committed inside the guard against it. Now keyed on env::var within the impl block, so recording the reason for a decision is no longer a violation of it.

### WP-02 -- State-dir death exit (status: Not Started)

- AT-02.1 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-02.1 -- status: to-write -- Positive-controlled: the daemon must be answering before the state dir is removed.

### WP-03 -- One home for spawning a daemon in the test tree (status: Not Started)

- AT-03.1 `native/rust/crates/intentd/tests/one_home_spawns_the_daemon.rs` -- covers AC-03.1 -- status: to-write -- Plants a second spawn site in-test and requires the check to fire on it.

### WP-04 -- The fixture-home teardown leak (status: Not Started)

- AT-04.1 `native/rust/crates/intentd/tests/fixtures_do_not_accumulate.rs` -- covers AC-04.1 -- status: to-write -- Drives the START sweep, not the exit path.

### Group 05

_(no tests in this group)_

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
