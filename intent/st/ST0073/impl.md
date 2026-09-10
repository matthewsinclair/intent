# Implementation notes -- ST0073

## AC-05.1: the estate measurement

Driven 2026-09-10 at revision `36352b216291f25615f982cea1c1feb08bfd74eb`. All three arms are at that one revision; the two arms driven before it are recorded at the bottom and are not the claim.

### The instrument, and why it is not `pgrep -f`

The count is a SET of pids whose EXECUTABLE is `intentd` -- `ps -axo pid=,command=` with the basename of the argv[0] field, compared exactly:

```
ps -axo pid=,command= | awk '{ n = split($2, seg, "/"); if (seg[n] == "intentd") print $1 }'
```

`pgrep -f` matches command-line TEXT and is unsound in BOTH directions, which is AC-05.1's own wording and ic's measurement (4 true against 7 reported, the three extras being a peer's shell wrapper and the measurement's own subshells). AC-05.1 is a ZERO claim, so an instrument that can read non-zero at a true population of zero makes a satisfied row look unsatisfiable.

Survivors are a SET DIFFERENCE, not a difference of counts: `comm -13 before.pids after.pids`. A count difference of zero is also produced by one daemon dying while another leaks, and that is exactly the case this row exists to catch.

### Both controls fired, before any arm ran

A zero from a blind instrument is worth nothing, so the instrument was driven to both verdicts against a planted daemon:

| control  | action                                          | instrument reported |
| -------- | ----------------------------------------------- | ------------------- |
| negative | before planting                                 | 0                   |
| positive | a real `intentd` spawned under a private `HOME` | 1 (pid 44630)       |
| negative | after `kill -9` on it                           | 0                   |

The first attempt at this control FAILED and is recorded because it is the trap: the daemon refused to bind, `path must be shorter than SUN_LEN`, because the scratchpad `HOME` made a 144-byte socket path. The instrument correctly reported 0 -- against a daemon that had already exited. **A zero that agrees with the expected answer is where a broken instrument hides.**

### Non-vacuity: the run has to CREATE daemons for a zero to mean anything

A 1-second sampler recorded the live set throughout each arm. Its purpose is to establish that daemons existed while the suite ran; it is NOT a census and does not bound the true peak, because a daemon whose whole life falls between two samples is never seen.

### The three arms

| arm                     | interruption                              | peak live seen | before | after | SURVIVORS |
| ----------------------- | ----------------------------------------- | -------------- | ------ | ----- | --------- |
| clean                   | none -- the suite ran to completion       | 9              | 0      | 0     | **0**     |
| interrupt, first daemon | `SIGKILL` at the first daemon seen, 6s in | 2              | 0      | 0     | **0**     |
| interrupt, crowd        | `SIGKILL` with 6 daemons live, 15s in     | 6              | 0      | 0     | **0**     |

The suite is `cargo test --workspace --no-fail-fast`, which is CI's own invocation. The clean arm ran 14 targets, 2301 passed, 1 failed.

**THE INTERRUPTION IS `SIGKILL` ON THE OWNER, NOT `SIGTERM`**, and it goes to `cargo` AND its direct children -- the test binaries, which are the processes that actually own the lifelines. No destructor, no handler, no `atexit`. The write end of every inherited pipe is closed by the kernel because the process is gone, and for no other reason. That is the whole mechanism ST0073 added, exercised at the only moment it matters.

**THE SECOND ARM WAS RUN BECAUSE THE FIRST ONE WAS TOO WEAK, AND THE FIRST IS KEPT RATHER THAN REPLACED.** Killing at the first daemon seen interrupts a thin moment -- 2 live. The episode this thread exists for was 64 at once, so an arm that never reaches a crowd is not measuring the shape that failed. The crowd arm waits for 5 or more and fired at 6.

After each arm the measurement waits 15 seconds before taking the after set, so a daemon that noticed its lifeline and is unwinding is not counted as reaped when it is merely slow.

### What this does NOT establish

- **The before set was empty in every arm**, so the set-difference machinery never had to distinguish a survivor from a pre-existing daemon. The positive control shows the instrument can see a daemon; it does not show the diff discriminating against a non-empty baseline.
- **The sampler is not a census.** Peak live is a lower bound on how many existed at once.
- **One red remains in the suite**: `mutation_completeness::no_service_call_can_set_an_edgeless_field`, which says in its own panic message that it is a DECISION PENDING hv and not a bug to fix. A full suite run is therefore a full run that is not fully green. AC-05.1 asks about survivors, not about green.
- **A residual flake in `a_daemon_outlives_nobody.rs` is open**, roughly 1 in 20 in parallel, mechanism unconfirmed. It fired once during this session's runs and passed alone. It does not bear on survivor counts -- every arm above ended at zero -- but it is not fixed.

### The two arms driven before the revision

An earlier clean arm at `c5ffdcee` also ended at 0 survivors with a peak of 6. It is not part of the claim because it was taken at a different revision, and because it is what SURFACED a defect of mine: `one_clock::nothing_in_this_workspace_reads_a_clock` was red, naming `crates/intentd/tests/a_daemon_outlives_nobody.rs` for reaching `Instant::now`. Fixed at `36352b21` by counting `PAUSE` intervals instead. **The estate row found a defect in the mechanism rows, which is the argument for having an estate row.**
