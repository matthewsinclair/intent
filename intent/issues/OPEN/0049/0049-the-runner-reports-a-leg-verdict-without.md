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

**dc's second finding, which belongs here because it is why this location is worth more than it looks.** `DEVBIN_SEAL_LEDGER` is exported only inside `run_all` and `record_seal` early-returns without it, so **the rc-versus-seal disagreement check fires only on `<cmd> all`** -- a single-gate run gets no check at all. Devbin's own open issue 0015 is a completed run that sealed in-flight, and its invocation was a single gate. **The one recorded upstream instance is on the path that check does not cover.** Putting the referent and the disagreement predicate together in `print_run_verdict` covers it, and makes one coherent change rather than two bolt-ons.

**The line numbers differ by tree, and the pair of them is this issue happening inside the conversation about this issue.** In **Intent's patched copy** the export is `resolve:441`, the unset `:495`, and `record_seal` `runlog:762`. In **Devbin upstream `4f8c4b6`** the export is `resolve:441`, the unset `:477`, and `record_seal` `runlog:718`. vc cited the first and dc the second; **neither said which tree, and the export being `:441` in BOTH is exactly what made one of us look wrong instead of the pair look under-specified.** The gap is the patch's net +30/-12, landing between the export and the unset. **A citation naming a line is a claim about a pointer, read at a moment, with the moment omitted** -- which is the whole of this issue, arriving between the two people writing it.

**AND THE SEAL'S CORE INVARIANT IS A WRITE THAT CAN BE REFUSED (dc, measured 2026-08-17).** The `rc=0` arm of `write_errors_file` reads:

```sh
if [ "$rc" -eq 0 ]; then
  if ! : >"$errors"; then
    warn "cannot seal $errors -- the run was GREEN but its seal could not be written"
  fi
  return 0
fi
```

**When the truncation fails -- a read-only seal, a vanished directory -- it warns and returns 0, and the in-flight marker survives.** So "a green run beside a non-empty seal", which the truncation appears to make unconstructible, **is reachable**: dc drove it against a read-only seal with controls either side proving the probe reached the real function, and the gate reported green with the marker still in place. The only defence was a `warn`. This matters to the fix above in both directions: it is the case dc's landed consistency check now refuses, **and it is a caution against any design that leans on "the seal is empty on green" as a property rather than as an attempted write.**

This is the more fundamental defect, and it was found the hard way while filing this issue: vc reported "HEAD is green" to a peer, correctly, and the sentence was false forty minutes later because HEAD had moved -- a fact a second node hit independently in the same window and reported against their own measurement. **`HEAD` is a pointer, so a claim about HEAD is a claim about whatever it points at when read, not when written.** The runner has exactly the same bug in the same shape: a leg name is a pointer at a tree. Stamping the sha costs one subprocess at run start and converts every verdict from perishable to durable.

**2. Record whether the tree moved under the leg.** The sha alone does not cover a dirty tree, where the interesting changes are the uncommitted ones. `open_run_log` already creates a file at run start, so use it as the anchor: at `record_seal`, `find <the leg's source scope> -newer <anchor>`; if anything comes back, annotate the verdict -- `rust FAILED (tree moved during the run: 2 files newer than the run start)` -- and name the files in the `.errors` companion beside the failure.

Properties worth keeping:

- **Part 2 needs no clock.** It compares two files on one filesystem to each other, the same two-sided shape as the whiteboard clock guard's check C. A quiescent tree cannot trip it, so it needs no tolerance and no suppression list. (Part 1 does shell out to git, which part 2 deliberately does not -- they are separable, and part 2 still works in a tree with no VCS at all.)
- **Annotate, never suppress.** The verdict still says FAILED. A guard that downgraded a red to a warning on tree movement would be a bypass, and the movement is a fact about the measurement, not a verdict about the code.
- **The idiom is in-house** (`:665`), so this is not a new dependency or a new concept in the file.

**It narrows the window; it does not close the class, and a green from it should not be read as proof that it has.** A file changed and changed back within the run passes; so does a change landing between the compiler reading one file and the next. The residue is real. It is much smaller than the current state, which is that the question is not asked at all.

**OWNERSHIP -- RULED INTENT-SIDE BY hv, 2026-08-17, reversing dc's earlier upstream ruling. The fix does not go in devbin at all.**

`bin/.devbin/**` is not uniformly devbin-owned, and `manifest.sha256`'s own header says so in one line: **"Files not listed here -- config.yaml, cmd/, help/ -- belong to the project."** `bin/.devbin/cmd/` carries eleven project-owned handlers. So the referent line is buildable in Intent's own handler with **no devbin change, no manifest divergence, and nothing upstream to wait on.**

**The reason it never belonged upstream is sharper than any dependency argument: Intent knows it is a git checkout. devbin does not, and must not have to.** A generic gate runner has no business knowing what a commit is. Verified rather than assumed -- devbin's `lib/` contains **zero** git invocations, the single textual hit at `lib/cmd/version:32` being a comment. (A first pass with `grep -rln 'git '` returned three files and all three were substring matches; the word-boundary form returns one comment. The premise survives a proper test, which is why it was worth running one.)

**THE SITING THAT SENT THIS UPSTREAM WAS THIS ISSUE'S, AND IT IS vc's.** Part 1 originally read "at `open_run_log`, stamp into the log" -- and `open_run_log` is devbin core. That single choice of location made the fix look like an upstream change, dc took the siting as given and carried the framing to Devbin, and it cost a filed issue and a branch before hv dissolved it in a sentence. **Part 1 was wrong about WHERE in two independent ways**: the wrong function within the file (the seal cannot carry it, and a log nobody opens is not where a reader's distrust is created), and the wrong LAYER entirely. dc caught the first within the hour. Nobody caught the second, including the person who wrote it, until hv did.

Filed by vc as a finding, not built. Part 2 gets easier under this ruling, not harder: `-newer` against the run's own log needs no VCS anywhere, and in a project-owned handler git is available freely besides.

## Related

- ST0056 -- Intent v3.0.0; the concurrent-node working model is what makes the window non-theoretical
- 0048 -- devbin-owned files carry local patches and this project has no detector for them; same ownership boundary
- 0027 -- the clock guard's tolerance rationale; same family of "a measurement compared against the wrong reference"

## Resolutions

{{TBC}}

## THE CLOSE CONDITION AS WRITTEN IS UNSATISFIABLE BY DESIGN -- dc, 2026-08-17, correcting a vc verdict from the same day

**vc reported this issue not-done on the grounds that `print_run_verdict` carries no `measured:` line. It cannot get one.** `print_run_verdict` lives in `lib/runlog`, ie in devbin, and **devbin has no git and must not acquire it** -- the layer ruling that sent `int measured` project-side in the first place. A `measured:` line beside `verdict:` would require devbin to know what a commit is.

**So the Proposed Fix above names a home the architecture forbids**, and has since it was written. The correction is dc's; the verdict it corrects is vc's, taken by reading `print_run_verdict` and finding the line absent -- which is a true observation of a function that could never have carried it.

**Recorded as an instance, not a chore.** An issue whose close condition cannot be met is indistinguishable from an issue nobody has got to: both are an open row. **The sweep that reported this had no way to tell those apart**, which puts it in the sibling class in `parity.md` -- a true answer that has stopped discriminating -- alongside 0057's zero-byte seal and 0059's uninvoked instrument.

**What DOES exist, and it is not nothing.** dc's `run:` hook writes the referent into the `.out` LOG via `--exec`, before the gate command execs, and it never touches `.errors`. That ordering is deliberate and canaried: `write_errors_file` truncates the seal to empty on `rc -eq 0`, so a sha written there would be destroyed on green or make green read as red. **dc canaried the opposite property explicitly -- a green run's seal must still be EMPTY with the header present, across all seven grammars** -- because that failure would be unrecoverable.

### RE-SCOPED (vc, 2026-08-17), and the original condition is retired rather than left standing

**This issue's subject is unchanged and still real**: hv read `FAILED: rust` in a SUMMARY and had no way to know the tree was dirty under it. The issue's own argument stands -- information reachable only by opening the log arrives after the reader already distrusts the verdict, which is too late to be the thing that creates the distrust. **`int measured` does not close that**, because it is a separate command a reader has to think to run, which is the same "reachable by asking" the issue rejects.

**The close condition is therefore restated as the design dc offered, which respects the layer ruling instead of breaking it: `run_gate` echoes an OPAQUE STRING the project hands it, beside `verdict:`, with devbin never parsing it and never learning what a commit is.** The project composes `b2173b1b +9 dirty`; devbin prints it. It must print on GREEN runs too, unadorned, so a dirty line stands out by being unusual rather than by shouting -- that half of the original Proposed Fix survives intact and is the part that was always right.

**Bigger than the flag both parties had been calling small, and named as such rather than closed on the cheaper thing.** dc has offered to build it. **The one option deliberately NOT taken is closing on what exists**, because that would resolve the issue by lowering it to what happened to get built.

### RULED (hv, 2026-08-17): BUILD IT, AND THE SEQUENCE IS DEVBIN FIRST

**hv, verbatim: _"Agree with VC. Just do it."_ on the re-scoped design, and _"Yep. Agree."_ on the sequence.** dc raised it rather than slipping it in behind a three-word fix, on the grounds that it is a devbin-**core** change and was named as one.

**The sequence is the ruled half and it is not incidental.** Build in Devbin, merge there, then `int upgrade` here -- the same order the 0016 carry just proved. The alternative is landing a core change in the vendored tree, which would re-diverge this project from stock **one week after hv ruled against exactly that**, and would put `int vendor` back to reporting patches nobody had ruled on. Intent reached `27 of 27 matching, no local patches` at `55e540df` and the point of that state is that it survives the next change rather than being a moment.

**What this does NOT license.** It is a ruling on the referent line, not on devbin acquiring git. The whole design is that **devbin echoes an opaque string and never parses it** -- the moment devbin can tell a sha from a hostname, the layer ruling that sent `int measured` project-side has been reversed by implementation rather than by a decision. That property is the acceptance condition, not a stylistic preference.
