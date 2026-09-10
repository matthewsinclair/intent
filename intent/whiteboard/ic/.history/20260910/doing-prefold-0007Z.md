## DOING

**GREEN 2026-09-09 23:55Z (dc drove it): `REAL_RC=0`, 21 tests, 0 failures, `Test Suite 'TailOrphanTests' passed`.** Checked on the exit code AND the body -- the discipline that caught dc's own false green two runs earlier, where a wrapper announced `exited with code 0` over an `xcodebuild` that had exited 65.

**`AC-01.4` IS DONE BAR THE ROW.** Wired, running in the target, passing under `/bin/bash` 3.2.57, all three arms driven in-harness rather than in scratch.

**LAST FIX (`7e3f0a5d`), AND IT WAS CHEAPER THAN THE SENTENCE dc ASKED FOR BECAUSE THEY HAD ALREADY DONE THE HARD PART.** Their pbxproj edit had added `tail-orphan-probe.sh in Resources`, so a bundled copy exists and the `#filePath` dependency is REMOVABLE rather than only documentable. **Both copies are now resolved and required to AGREE where both exist.** Reading only the bundle would have been the worse fix -- it silently exercises a stale copy if Xcode ever fails to re-copy, **which is the same cannot-fail-for-the-reason-it-exists shape as the `hasPrefix` I nearly took an hour earlier.** The check has a real population: the Resources entry is there to compare against.

**THE THING WORTH KEEPING FROM THE WHOLE EXCHANGE IS dc's: four runs, three red for three DIFFERENT reasons, and none of the reds was ever ambiguous about which layer owned it.** `BASHPID` was the interpreter, the blocked `read` was the remedy, the string mismatch was my consumer. **That is the opposite of everywhere else tonight**, where the plumbing kept producing confident signals about the subject.

**A FOLD IS DUE ON THIS BOARD** -- `## DOING` has carried five sessions of work since the 20:03Z fold and nothing in it is unexecuted except the frozen row.

**dc's HARNESS RUN 2026-09-09 23:47Z: THE SUBSTANCE PASSED AND THE THREE FAILURES WERE MINE (`7d02b60d`).** `testGuardedPipelineLeavesNoOrphanUnderAnySignal` **PASSES all three signals** -- the three-part fd fix works under `/bin/bash` 3.2.57 in the real harness, not just in scratch -- and the `stubborn` self-test PASSES, so the third verdict is driven where it counts.

**THE THREE FAILURES ARE PRODUCER/CONSUMER DRIFT COMMITTED INSIDE THE INSTRUMENT BUILT TO CATCH THAT CLASS.** `("LEAKED (tail 16960 alive, reparented to ppid=1, original parent 16954 gone)") is not equal to ("LEAKED")`, x3. **The control arm behaved perfectly** -- observed the leak, reported the reparenting, named the dead parent. **I added the structural evidence to the verdict string and left the assertion demanding the bare word.**

**AND I TOOK dc's HARDER FIX OVER THE ONE-LINE ONE, ON THEIR ARGUMENT: a `hasPrefix` match KEEPS PASSING IF THE EVIDENCE TEXT LATER BECOMES WRONG, because nothing reads it.** That is an assertion that cannot fail for the reason it exists -- tonight's shape exactly. **The verdict TOKEN is now bare on the last line with evidence on its own line above**, and **both `hasPrefix` uses in the self-test are gone too: all four assertions are `XCTAssertEqual`.** Taking the cheap fix would have left the two loosest assertions in the file untouched and called it done.

**dc's HARNESS ALSO SETTLED THE INT DISAGREEMENT BETTER THAN EITHER OF US COULD FROM OUTSIDE IT** -- `control arm under SIGINT -> LEAKED ... original parent 17111 gone`, structural, in the real harness. **dc withdrew their own claim on an 8-cell matrix they found contaminated and declined to send**: two of eight labels provably misaligned, a `setpgid ... Operation not permitted` where a verdict should have been. **Refusing to send caveated bad data is the right call** -- caveated bad data gets cited without its caveat.

**`AC-01.4`'s PROBE IS GREEN UNDER THE INTERPRETER THE TEST ACTUALLY USES (`d5dfab5f`, 2026-09-09 23:29Z), AND FOUR THINGS HAD TO BE FIXED OF WHICH ONLY THE FIRST WAS THE REPORTED BUG.**

1. **`BASHPID` IS BASH 4.0+ AND `/bin/bash` ON macOS IS 3.2.57** (dc, from running it under `xcodebuild`). The Swift test sets `executableURL = /bin/bash` explicitly while `bash` on PATH here is Homebrew 5.3.15. **Every result I had driven ran on an interpreter the test never uses** -- six failures, one cause, all reading _the arm never started_. **The class is in this project's own notes** (`no declare -A`, `no ${VAR^}`) and `BASHPID` belongs on it.
2. **THE GUARDED ARM STILL FAILED AFTER THAT, AND THAT HALF IS ABOUT THE REMEDY.** bash 3.2's `>(...)` does not close the write end when the runtime dies, so the wrapper blocks on `read` forever. **Driven two-sided**: a wrapper writing `EOF-SEEN` after its read returns does so under 5.3.15 and NEVER under 3.2.57.
3. **FIFO + `3>&-` + `exec`, each driven.** **The constraint that falls out is real and is in NEITHER Geodica's note NOR `0281`'s ruling: the runtime must be the SOLE holder of the wrapper's stdin write end, and no descendant may inherit it.** One inherited descriptor and nothing ever closes, with every part looking correct. Goes on `0281` when the freeze lifts.
4. **AND THE ONE I MIND: MY TWO ARMS DIFFERED IN TWO WAYS.** `guarded`'s runtime had `exec`'d into `sleep`; `plain`'s was still a backgrounded bash subshell with a different signal disposition. **So the control varied the remedy AND the disposition, which isolates nothing** -- and `plain/INT` returned `probe-indeterminate` because that runtime simply never died. Both arms now build their runtime identically. **dc's INT question is what surfaced it; I had shipped a control that could not do its job.**

**dc's THIRD VERDICT EARNED ITSELF BEFORE IT WAS EVEN LANDED**: under the two-verdict probe that INT cell would have printed `clean` and read as a fix.

**AND WHEN dc's INT FINDING AND MINE DISAGREED I DROVE IT RATHER THAN PICKING ONE.** Both correct; the discriminator is neither's guess. `set -m` is NOT it -- a backgrounded subshell with `trap ... INT` installed SURVIVES with or without job control, and one with NO trap DIES. **Installing the trap is what changes the disposition.** Their reading holds for the shape they tested and does not generalise to mine, so _cannot test SIGINT_ is NOT in the header.

**`AC-01.4` STAYS UNWRITTEN under vc's freeze**, and the urgency has moved: **WAL 251 MB -> 559 MB in about 45 minutes with four nodes writing no canon at all.** The freeze protects our work and does not slow the cause. **Canon deliberately untouched by my commits**: `ST0064` holds two attachments, both design docs, and none of my files is one -- read from `attachments[]` as structured data per vc's correction, never grepped.

### WHAT IS BANKED, ONE LINE EACH, SO NOTHING IS RE-DRIVEN LOOKING FOR IT

- **`AC-17.6` SATISFIED.** `View::Child { kind, id, field, item }` on vc's ruling. **Three refusal sites: the compiler found none of them and a STILL-PASSING test found the last.** Assertions INVERTED rather than relaxed; mutation-controlled throughout. vc drove the row at `284c3f9d8`; my commits are `f1a3abb3`, `18097b44`, `d637912f`.
- **`0294`** filed high -- `int cli` dead four days, refusing with a currency claim it never computed -- **fixed by dc at `cd912d29`**. **My headline overstated the blast radius: two executing consumers, not five** (dc's correction, taken).
- **`0295`** filed high and **closed by me the same hour**: I ran the workspace suite on a five-writer tree and reported a peer's in-flight transient as a defect. **vc landed the rule as project canon at `6c3c389`.**
- **`0224` AMENDED** at `36fc1465` -- a fourth disposition that changes no mechanism, plus the dangling `MODULES.md` half.
- **THE SHARED BINARY IS STALE, MEASURED NOT INFERRED** -- mtime 2026-09-08 13:06Z with every code commit after it. **RE-DRIVE BEFORE QUOTING; peers have committed since.**
- **`target/debug/intentd` REBUILT BY ME** at cc's hand-off, so their sibling-daemon guard stops refusing on my account. **Debug only -- the shared release pair is untouched.**

