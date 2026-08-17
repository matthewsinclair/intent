---
id: "0049"
title: the runner reports a leg verdict without recording whether the tree changed under it, so a mid-edit skew reads as a landed regression
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0049: the runner reports a leg verdict without recording whether the tree changed under it, so a mid-edit skew reads as a landed regression

## Tags

devbin, runlog, verification-apparatus, false-alarm, seal, measured, concurrency

## Summary

A leg's verdict is written as a claim about the project -- `FAILED: rust` -- but what it measured is one tree at one instant, and **the run pair records neither which tree that was nor whether it still exists.** The `.errors` seal answers two questions well: did the run complete, and was it green. It cannot answer the two that a reader actually needs: which tree was this about, and does anybody still have it.

On a quiescent tree the distinction is empty. On this estate it is not: five nodes edit one working tree concurrently, so the interval between "the compiler read the file" and "a human read the verdict" routinely contains edits. **A verdict that has outlived its tree is indistinguishable, by inspection, from a verdict about landed code** -- which is the same property that makes a fabricated whiteboard timestamp dangerous, for the same reason.

Found by vc, 2026-08-17, after a red leg was escalated as a regression and turned out to be about a tree that had ceased to exist 41 seconds after the leg finished.

## Reproduction

The measured incident, from the artefacts rather than from recollection.

`bash bin/int check && bash bin/int test` reported:

```
FAILED: rust
  check format       tmp/check/20260817-1138.FORMAT.errors
  test rust          tmp/test/20260817-1146.RUST.errors
```

The rust seal named one failing test:

```
---- the_facade_routes_closes_through_the_gate stdout ----
thread ... panicked at crates/intentsvcs/tests/close_gate_parity.rs:732:3:
the same gate refuses once the coverage goes red
test result: FAILED. 20 passed; 1 failed
```

Three measurements, all local BST:

| artefact                                         | modified | relative to the rust leg |
| ------------------------------------------------ | -------- | ------------------------ |
| `tmp/check/20260817-1138.FORMAT.errors`          | 11:38:57 | -- (earlier leg)         |
| `tmp/test/20260817-1146.RUST.out` (leg finished) | 11:46:45 | anchor                   |
| `crates/intentsvcs/src/facade.rs`                | 11:47:26 | **+41s, after**          |
| `crates/intentsvcs/tests/close_gate_parity.rs`   | 11:47:33 | **+48s, after**          |

**Both files the failure depends on were modified after the leg had already finished.** The tree that was compiled is gone; it cannot be re-run, and no artefact of the run identifies it.

The control that makes the above mean something: **`78a12dce` is green.** A `git archive` extract of that commit into a sacrificial directory with its own target dir compiles and passes that binary at **21 passed, 0 failed**, `the_facade_routes_closes_through_the_gate` included. A second node measured its child `b2173b1b` green independently, from a detached worktree, on the same two legs. So nothing that has landed is broken.

**The commit is named rather than called "HEAD" deliberately, and this sentence was wrong on first writing.** It said "HEAD is green", which was true at the time and false within the hour -- `b2173b1b` landed while this issue was being written. See the Proposed Fix: it is the same defect as the one being reported.

And the failing assertion is HEAD's verbatim -- `git show HEAD:.../close_gate_parity.rs` carries `assert!(facade.wp_done(...).is_err(), "the same gate refuses once the coverage goes red")` -- while the working tree's version of the same test has since been rewritten to assert `Outcome::AlreadyThere`, citing the hv self-loop ruling of the same morning. The leg caught a tree mid-transition: the behaviour had moved to the ruled semantics, the test had not yet followed.

**Scope of what was verified, stated rather than implied:** that `78a12dce` passes, that `b2173b1b` passes, and that the measured tree no longer exists. **Whether the current working tree passes was NOT verified** -- that is a separate question, re-runnable at any time by its owner, and this issue makes no claim about it.

## Root Cause

`run_gate` (`bin/.devbin/lib/runlog:896`) does three things in order: `open_run_log` (`:965`) seeds the `.errors` companion with the in-flight marker, the command runs, and `record_seal` (`:1030`) writes the verdict. **At neither boundary is any property of the source tree observed.**

The seal grammar is a closed vocabulary about the RUN -- in flight, completed-green, completed-red -- and it is well designed for that: `open_run_log` seeding the marker is exactly why a killed run reads as not-green rather than as passing. The gap is that the run is not the only thing a verdict depends on. The tree is the other half, and it is unrecorded, so `print_run_verdict` (`:718`) presents a claim about the project with the confidence of a claim about the run.

The idiom needed to close it is already in the file: `purge_run_logs` resolves its keep anchor "by mtime via `-nt`" (`:665`). That is the only mtime comparison anywhere under `bin/.devbin/`; the mechanism exists and is simply not pointed at the source tree.

## Impact

A false regression reached the hypervisor and was escalated to two building nodes. Cost this time was small -- the diagnosis took one extract and four `stat` calls -- but the failure mode is not self-limiting:

- **It is silent.** A stale verdict looks exactly like a live one, so the default reading is the wrong one, and it is wrong in the expensive direction (chase a regression that does not exist).
- **The green case is worse than the red one.** A red verdict at least gets read carefully. A stale GREEN certifies a tree nobody has and is never questioned -- and this incident produced stale greens too, on four other legs, which nobody has any reason to distrust.
- **The false-green half is not hypothetical, and the evidence was already in the file.** `resolve:459-464` records a measured incident from **04:05 the same morning**: the bats leg sealed a non-empty `.errors` (one failure of 1311) at 04:05:33, the Rust leg sealed empty at 04:06:47, and the run **was reported to a human as "100% green (rust and bats)" at 04:07 -- who acted on it.** That was a rc-versus-seal disagreement rather than a stale tree, but the shape and the cost are identical: a verdict asserting more than its measurement supports, believed because green is not read twice.
- **It degrades exactly when the estate is busiest.** The window is the run's duration, so the slowest legs on the most-edited days have the widest exposure.

## Proposed Fix

Two parts, and **the first matters more than the one this issue was originally filed about.**

**1. Name the referent, so the verdict cannot decay.** Report what is being measured -- `git rev-parse HEAD` plus the dirty path list. A verdict that says `measured: 78a12dce +9 dirty` is readable a week later; `FAILED: rust` is readable only while the tree holds still.

**CORRECTED BY dc, 2026-08-17, and the correction is load-bearing. This originally said "stamp it into the log", which is ambiguous in the one direction that breaks the subsystem: IT CANNOT GO IN THE SEAL.** `write_errors_file` truncates `.errors` to empty on `rc -eq 0` (`runlog:210`), and the file says so itself twenty lines later -- _"An empty companion means 'completed green run' to every reader of this seal"_ (`:227`). A sha written there makes **every green run read as red**. Verified from source, not taken on report.

**The right home is `print_run_verdict` (`:718`), and the reason is what actually failed today.** hv read `FAILED: rust` in a SUMMARY and had no way to know the tree was dirty; information reachable only by opening the log arrives after the reader already distrusts the verdict, which is too late to be the thing that creates the distrust. `print_run_verdict` fires on every kept run, single or aggregated, and already has rc, log and errors in hand:

```
verdict:  /Users/matts/.../20260817-1146.RUST.errors
measured: b2173b1b +9 dirty -- THIS VERDICT DESCRIBES NO COMMIT
```

**It must print on GREEN runs too**, as a plain unadorned `measured: b2173b1b` with no warning voice, so the dirty line stands out by being unusual rather than by shouting. There is already precedent for exactly that in the same function: the `verdict:` line is emitted whenever `keep=1`, independent of rc, so a `measured:` line beside it is an addition to a block that already fires on green, not a new behaviour.

**dc's second finding, which belongs here because it is why this location is worth more than it looks.** `DEVBIN_SEAL_LEDGER` is exported only inside `run_all` (`resolve:441`, unset at `:495`) and `record_seal` early-returns without it (`runlog:762`), so **the rc-versus-seal disagreement check fires only on `<cmd> all`** -- a single-gate run gets no check at all. Devbin's own open issue 0015 is a completed run that sealed in-flight, and its invocation was a single gate. **The one recorded upstream instance is on the path that check does not cover.** Putting the referent and the disagreement predicate together in `print_run_verdict` covers it, and makes one coherent change rather than two bolt-ons.

This is the more fundamental defect, and it was found the hard way while filing this issue: vc reported "HEAD is green" to a peer, correctly, and the sentence was false forty minutes later because HEAD had moved -- a fact a second node hit independently in the same window and reported against their own measurement. **`HEAD` is a pointer, so a claim about HEAD is a claim about whatever it points at when read, not when written.** The runner has exactly the same bug in the same shape: a leg name is a pointer at a tree. Stamping the sha costs one subprocess at run start and converts every verdict from perishable to durable.

**2. Record whether the tree moved under the leg.** The sha alone does not cover a dirty tree, where the interesting changes are the uncommitted ones. `open_run_log` already creates a file at run start, so use it as the anchor: at `record_seal`, `find <the leg's source scope> -newer <anchor>`; if anything comes back, annotate the verdict -- `rust FAILED (tree moved during the run: 2 files newer than the run start)` -- and name the files in the `.errors` companion beside the failure.

Properties worth keeping:

- **Part 2 needs no clock.** It compares two files on one filesystem to each other, the same two-sided shape as the whiteboard clock guard's check C. A quiescent tree cannot trip it, so it needs no tolerance and no suppression list. (Part 1 does shell out to git, which part 2 deliberately does not -- they are separable, and part 2 still works in a tree with no VCS at all.)
- **Annotate, never suppress.** The verdict still says FAILED. A guard that downgraded a red to a warning on tree movement would be a bypass, and the movement is a fact about the measurement, not a verdict about the code.
- **The idiom is in-house** (`:665`), so this is not a new dependency or a new concept in the file.

**It narrows the window; it does not close the class, and a green from it should not be read as proof that it has.** A file changed and changed back within the run passes; so does a change landing between the compiler reading one file and the next. The residue is real. It is much smaller than the current state, which is that the question is not asked at all.

**Ownership:** `bin/.devbin/**` is devbin-owned (see 0048), so this is dc's call whether the fix belongs upstream in devbin or in this project's handler. Filed by vc as a finding, not built.

## Related

- ST0056 -- Intent v3.0.0; the concurrent-node working model is what makes the window non-theoretical
- 0048 -- devbin-owned files carry local patches and this project has no detector for them; same ownership boundary
- 0027 -- the clock guard's tolerance rationale; same family of "a measurement compared against the wrong reference"

## Resolutions

{{TBC}}
