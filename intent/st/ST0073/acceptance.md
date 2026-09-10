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

### WP-01 -- The lifeline: intentd exits when the owner it can observe goes away (status: Done)

- AC-01.1 **A daemon whose owner is SIGKILLed exits on its own, and the test kills the owner in a way NO `Drop`, handler or atexit can survive.** A test that terminated its parent politely would be measuring the path that already works. The arm asserts the daemon process is GONE, by pid, not that a socket stopped answering -- a wedged process holding the store still fails to answer and would satisfy the weaker check while being the exact defect this thread exists to remove. -- satisfied: yes (computed)
- AC-01.2 **A daemon started WITHOUT a lifeline serves until signalled -- the production path, asserted rather than assumed.** This is the control on AC-01.1 and it is the row that matters most: an implementation that exits when no lifeline was passed would sail through AC-01.1 and silently kill the `launchd` daemon, whose plist is `KeepAlive false` with no socket activation, so it would not come back until next login. **The failure would present as 'the daemon is sometimes not running' and nothing would name this change.** -- satisfied: yes (computed)
- AC-01.3 **The exit is event-driven, not swept: there is no interval to configure and none is configured.** A polled implementation passes AC-01.1 with a long enough test and leaves a window proportional to the poll period, during which the orphan is still holding the store. Evidence is structural -- no timer, no periodic `getppid`, no sleep loop on the lifeline path -- because a timing assertion cannot distinguish a fast poll from an event. -- satisfied: yes (computed)

### WP-02 -- State-dir death exit (status: Done)

- AC-02.1 **A daemon whose own state directory is removed stops serving**, driven by removing it under a running daemon, with a positive control that the same daemon was answering immediately before the removal. Without that control the arm passes against a daemon that never started. -- satisfied: yes (computed)

### WP-03 -- One home for spawning a daemon in the test tree (status: Done)

- AC-03.1 **The test tree has ONE home that spawns `intentd`, and a check refuses a second.** The check reads the test sources structurally and is positive-controlled by a planted second spawn site that it must catch, and by a clean tree it must pass -- **a roster check that has never been shown to fire is decoration.** It REFUSES over an empty population rather than reporting the reassuring zero that a broken pattern also reports. -- satisfied: yes (computed)

### WP-04 -- The fixture-home teardown leak (status: Done)

- AC-04.1 **Fixture homes do not accumulate: a suite run leaves the /tmp population no larger than it found it**, and the sweep that guarantees it runs at START rather than at exit. Exit is the path that does not run when a build is killed, which is the condition that produced all 669. -- satisfied: yes (computed)

### Group 05

- AC-05.1 (non-test) **THE OUTCOME ROW, AND IT IS THE ONLY ONE THAT ANSWERS THE ORIGINAL QUESTION.** A full suite run, INCLUDING one deliberately interrupted mid-flight, leaves zero `intentd` processes that were not running before it started. Counted by matching the EXECUTABLE -- `ps -axo pid=,command=` with the argv[0] field, never `pgrep -f`, **which is unsound in BOTH directions**: it matches command-line text, so it counts observers and any process merely mentioning the name, and it can also miss a long command line. Its count is neither a floor nor a ceiling. **The direction is load-bearing rather than pedantic, and this row's first wording had it backwards.** It said `pgrep -f` UNDER-reports, on a 64-against-67 reading whose 67 was itself the over-count. AC-05.1 is a ZERO claim, so a reader told only that the instrument under-reports treats its count as a FLOOR -- and at a true population of zero it can read non-zero, which makes a satisfied row look unsatisfiable. (ic drove the over-count, 4 true against 7 from `pgrep -f`, the three extras being a peer's shell wrapper and this measurement's own subshells; the wording is theirs.) **Every other row here is a mechanism; this one is the estate.** Evidence: the before/after counts and the interruption method, recorded on this thread. -- evidence: ST0073 impl.md '## AC-05.1: the estate measurement' -- three arms at 36352b21 (clean; SIGKILL at first daemon; SIGKILL with 6 live), before/after pid SETS by executable match, 0 survivors in every arm; instrument driven to both verdicts on a planted daemon first -- satisfied: yes

## Acceptance Tests

### WP-01 -- The lifeline: intentd exits when the owner it can observe goes away (status: Done)

- AT-01.1 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-01.1, AC-01.2 -- status: green -- Two arms in one file deliberately: the lifeline arm and its production control are the same decision read in both directions, and separating them invites one to be run without the other. GREEN 2026-09-10, and BURNED IN BOTH DIRECTIONS rather than merely passing. Burn 1: the lifeline arm cut out of the select -- arms 1 and 4 FAIL and the two controls stay green. Burn 2: the arming widened from is_fifo to any stdin -- arm 2 (the production control) and the structural arm FAIL and the two lifeline arms stay green. Each burn fires on exactly the arms it should and on no others, which is what makes these four evidence rather than decoration. Four arms, not two: an owner that WRITES must not end the lifeline, and that arm caught a real defect -- sh forking for sleep left a child holding the write end.
- AT-01.3 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-01.3 -- status: green -- Structural: asserts the lifeline path carries no interval to configure. GREEN 2026-09-10, positive-controlled on the EXTRACTION rather than on the source -- a slice that missed its target would contain none of the markers and pass every assertion below it. Its first form was keyed on the VARIABLE NAME and failed on main.rs's own prose explaining why that variable was removed: mention versus use, committed inside the guard against it. Now keyed on env::var within the impl block, so recording the reason for a decision is no longer a violation of it.

### WP-02 -- State-dir death exit (status: Done)

- AT-02.1 `native/rust/crates/intentd/tests/a_daemon_outlives_nobody.rs` -- covers AC-02.1 -- status: green -- GREEN 2026-09-10 (ic). Two arms in `a_daemon_outlives_nobody.rs`, which is where vc's header had already allocated AT-02.1 -- so this is that file's fifth and sixth arms rather than a second home for the fixture.

DRIVEN BEFORE IT WAS BUILT, which is the difference between a test and decoration. Against the build with no state-directory arm: the daemon ANSWERED (`ok: intentd is answering at <home>/.local/share/intent/intentd.sock`), its home was removed, and it was STILL ALIVE 8 SECONDS LATER. Against the build with the arm: EXITED 3200ms after removal. Same probe, opposite verdicts, which is the burn.

THE CONFOUND THAT MAKES THE OBVIOUS VERSION OF THIS TEST WORTHLESS, and it is recorded in the arm's own text because the next person will reach for the obvious placement. A lifeline FIFO must not live inside the home: put it there and removing the home closes the pipe, the daemon exits ON THE LIFELINE, and the run reads as a clean pass while proving nothing about state directories. The shell probe that first drove this managed the confound by keeping the FIFO outside the home and asserting it was still held at the moment of the verdict. THE ARM REMOVES IT INSTEAD OF MANAGING IT: the daemon is started SUPERVISED with `Stdio::null()`, so there is no owner, no pipe, and nothing for an EOF to arrive on. An exit cannot be the lifeline's doing because there is no lifeline, and that needs no assertion to stay true.

THE POSITIVE CONTROL IS THE ROW'S OWN DEMAND and it is `wait_until_answering`, which waits on the shipped routing predicate rather than on a sleep. Without it the arm passes against a daemon that never started.

THE SECOND ARM IS THE CONTROL AND ITS BUDGET IS WHAT MAKES IT DISTINCT FROM THE EXISTING SUPERVISED ARM. "The daemon exited" is also what a daemon exiting for any other reason looks like. `invariant_a_daemon_whose_state_directory_remains_keeps_serving` changes exactly one variable -- the directory stays -- and waits 5s, comfortably past the two-miss two-second worst case. The existing `serves_until_signalled` arm waits 750ms, chosen against an EOF-on-stdin implementation that dies in microseconds, so it establishes nothing about a 4-second mechanism. It then SIGTERMs and requires an exit, so "still running" cannot pass vacuously.

MECHANISM, AND WHY IT IS AN INTERVAL WHEN THE LIFELINE NEXT DOOR REFUSES TO BE ONE. `AC-01.3` forbids an interval on the lifeline path and `invariant_the_lifeline_is_event_driven_and_carries_no_interval` enforces it structurally over the `impl Lifeline` block. This arm polls at 2s and the two sit in one file, so the reason is written at the function rather than left for a reader to infer that the lifeline could have been polled too. THE DIFFERENCE IS THAT THE LIFELINE HAD AN EXACT ALTERNATIVE AND THIS HAS NONE: a pipe delivers EOF from the kernel on a descriptor that cannot be recycled while open, so it is raceless and free. A directory has no such primitive. `notify` REFUSES A PATH THAT DOES NOT EXIST -- `watch.rs` already records this for the `intent/` case -- so a watcher must be re-registered to notice the very event it exists for, which is an interval wearing a watcher's name, and it would add a second watcher to a process already running a debouncer. The honest trade is a stated bound against a silent miss, and a stated bound wins.

TWO CONSECUTIVE MISSES, NOT ONE, on the asymmetry `Lifeline::observed` already makes: exiting wrongly stops launchd's daemon until the next login (plist is `KeepAlive false`, no socket activation), while staying wrongly leaves one orphan the next tick collects. Those errors are not the same size, so a single unlucky `stat` must not be able to stop a production daemon.

STRUCTURAL CHECK CONFIRMED UNBROKEN: `invariant_the_lifeline_is_event_driven_and_carries_no_interval` extracts the `impl Lifeline` block only, and `state_dir_removed` is a free function outside it. Driven rather than reasoned -- the test was run against this change and passes.

A PRE-EXISTING FLAKE IN THIS FILE IS RECORDED HERE AND IS NOT THIS ROW'S: sampled 6 runs with this change (5 pass / 1 fail) and 6 runs at HEAD without it (5 pass / 1 fail), failing on a DIFFERENT lifeline arm each time. Same rate either side, so the change neither introduces nor worsens it. ic first concluded the break WAS this change on one failing run against one passing run, and withdrew that on the sample -- n=1 each side of a stochastic outcome, which is the trap this thread has now hit three times.

### WP-03 -- One home for spawning a daemon in the test tree (status: Done)

- AT-03.1 `native/rust/crates/intentd/tests/every_daemon_spawn_carries_a_lifeline.rs` -- covers AC-03.1 -- status: green -- Plants a second spawn site in-test and requires the check to fire on it. GREEN 2026-09-10. It knows THREE shapes and the second and third were found the hard way: a direct intentd spawn; a "daemon start", which is a short-lived .output() call leaving a DETACHED daemon behind (render.rs does process_group(0) deliberately) and is invisible to any check keyed on .spawn(); and a roster-driven argv, where a file enumerates the dispatch surface and drives "daemon start" without those words appearing in it at all. It has fired for real twice on this estate -- daemon_run_execs.rs, a site its own author had missed by hand, and remedies_are_reachable.rs, the argv case that leaked four daemons per run. It also produced one false positive on a TABLE of subcommand names, because it asked whether both words were present rather than adjacent: mention versus use, inside the guard against it, now keyed on adjacency. 42 sites, every one armed. It refuses over an empty population, both plants are controlled, and the LIFELINE-EXEMPT escape hatch is itself controlled in both directions so it cannot become a hole.

### WP-04 -- The fixture-home teardown leak (status: Done)

- AT-04.1 `native/rust/crates/testkit/tests/abandoned_fixtures_are_swept.rs` -- covers AC-04.1 -- status: green -- GREEN 2026-09-10 (ic), 4 arms in testkit's single suite target, BURNED IN THREE DIRECTIONS rather than merely passing. The sweep lives in `testkit` -- one home reachable from every crate's tests, std only, no new dependency so `dep_graph_guard` stays quiet. `sweep_abandoned_fixtures()` is the assertable worker returning a SweepReport; `sweep_once()` is the idempotent at-START hook. They are separate deliberately: collapsing them makes the worker unassertable after its first call, because a second call reports zero removals and a test cannot tell that from a sweep that does nothing.

THE DISCRIMINATOR IS STRUCTURAL, NOT A PREFIX LIST, AND THE DISK FORCED THAT. Reading the six creation sites found four families; reading /tmp found twelve name shapes. A candidate is a directory whose name begins `intent` AND ends `-<pid>-<counter>`. THE SUFFIX CLAUSE IS WHAT SAVES THE MACHINE, and both exclusions are asserted arms: `/tmp/intent` is the in-session gate's sentinel DIRECTORY, holding one file per live Claude Code session, and `/tmp/intentfiles.new` is a stray file. Both begin `intent`. A prefix-only sweep -- the one anyone writes first -- deletes the sentinel directory out from under every running session on the machine.

REFUSING BEATS GUESSING: if the live-pid set cannot be read, or comes back EMPTY, nothing is removed and the report says `refused`. An empty `ps` result cannot be true while the sweep itself is running, and reading it as "no pid is live" would sweep every fixture on the machine including in-flight ones. Pid reuse can only make this KEEP an abandoned directory, never delete a live one, because a live fixture's own process is alive by construction -- the failure direction is a leak that persists, which is the safe one.

BURN MAP, each firing on exactly its own arms and no others: (1) liveness check removed so the sweep deletes everything it matches -> `a_dead_fixture_is_removed_and_a_live_one_survives_the_same_sweep` alone FAILS; (2) removal disabled, the silent no-op -> that arm AND `a_run_leaves_no_corpse_of_its_own_behind` FAIL; (3) one of the six creation sites unwired -> `every_tmp_fixture_creation_site_sweeps_at_start` alone FAILS. The live-fixture arm is what makes the dead-fixture arm mean anything: "the abandoned directory is gone" is also what a sweep that deletes everything returns, and that sweep would take out every concurrent test run's fixtures.

THE SITE ARM IS A SOURCE CHECK BECAUSE A RUNTIME CHECK PASSES ON THE BROKEN TREE TOO. `sweep_once` is Once-guarded per process, so once ANY site has called it, a site that never calls it is indistinguishable at run time from one that does.

ONE ARM CAUGHT ITS OWN FLAKINESS BEFORE IT COULD LAND, AND THE LIMIT IS RECORDED RATHER THAN PAPERED OVER. The first population arm counted all of /tmp and failed with `the population grew across a run: 0 -> 0` -- the count moved between the assertion and its own panic message, because sibling test binaries share that directory. /tmp is not this test's to make claims about, and a global assertion would have landed as an intermittent CI red whose cause is a concurrently running sibling. Rescoped to a probe family keyed on this process, where the property is deterministic.

LIVE EFFECT, MEASURED RATHER THAN PREDICTED: 902 abandoned directories / 133.7 MB before, 19 / 3.4 MB after, with the sentinel directory and its 46 live session files untouched. The 19 survivors are other suites' live-pid fixtures, which is the correct outcome.

NOT CLAIMED, AND ROUTED RATHER THAN TAKEN: `testkit::fixture_home()`'s own doc already rules /tmp the wrong home -- "under target/ rather than the system temp directory ... it never accumulates in /tmp where nothing prunes it" -- while all six sites hardcode /tmp anyway. Moving them under target/ would let `cargo clean` do this and need no sweep at all. That changes AC-04.1's named mechanism, which is not ic's to change, so the sweep the row specifies is what was built and the alternative is flagged for vc/hv.

### Group 05

_(no tests in this group)_

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
