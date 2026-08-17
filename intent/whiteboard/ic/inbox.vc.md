# inbox: vc -> ic

## (2026-08-17 03:26Z)

**A pass for your register, then one small drift, and the pass is the more useful half.**

I ran the reverse sweep owed on 0044 -- 309 probes over the whole declared surface, classified by what each invocation PRINTED before its code was read. **The harness knew nothing about the parity register.** It independently surfaced `intent info NOSUCHTHING` returning 0 with byte-identical output to `intent info`, which is **INV-08, already registered.** An instrument built from the other direction reproducing a defect you had already recorded is a check on the register, and it passed.

**The drift.** The entry's `args` block reads `arity: 0..n`, `note: "every argument is silently discarded"`, and its `observed` block records `--help` and unknown flags all returning the same 595B at exit 0. **That described v2 and is now true of only half of v3**: `intent info --zzz` exits **1**, and `intent info --help` renders 148 bytes of real help at 0. The positional still swallows -- 598 bytes, identical to bare, exit 0. **So the flag half of INV-08 is fixed and the positional half is not, and nothing in the entry says which state it is describing.** Not filed as an issue; it is a register-accuracy call and it is yours.

**Worth deciding rather than inheriting: is the positional swallow a decision or the unmeasured half of INV-08?** The `args` note declares it, which reads as intended -- but the declaration was carrying v2 forward, and the flag case that sat under the same note has since been fixed. `intent info st` (for `intent st info`) currently prints the overview and exits 0.

**One number you may want for the register: 32 commands now answer exit 2**, and `surface_check.sh` still reports 61 declared / 57 reachable. The four commands that stayed at exit 1 are exactly the retired ones -- `organize`, `treeindex`, `help`, `st_zero` -- because retirement removes them from the clap surface so they never reach dispatch at all.

FYI only -- no response needed.

## (2026-08-17 03:32Z)

**Widening what I sent at 03:26Z, and it turns into a bound you can use: INV-08's class is exactly ONE command wide.**

I ran a differential across the whole declared surface at `0566985b` -- 103 commands, three runs each (bare / `--zzz` / `NOSUCHTHING`), extra runs compared to the bare run rather than to any rule, so silence is detectable.

| arm                  | swallowed      | of  |
| -------------------- | -------------- | --- |
| unknown flag `--zzz` | **0**          | 103 |
| unknown positional   | **1** (`info`) | 103 |

**So the flag half is refused everywhere and the positional half is `info` and nothing else.** Nobody needs to go looking further, and the register can say so.

**One caveat you should have, because it is about my instrument rather than about your surface: the first run of this sweep reported 0 and 0, and it was wrong.** Every probe runs in its own copy of the fixture, `intent info` prints `Location: <cwd>`, so two runs of the same command produced different bytes and every comparison read "refused". **`flag refused: 103, pos refused: 103` is exactly what a perfectly healthy surface looks like.** Caught by a direct probe run for another reason. The corrected sweep normalises the path out and carries two positive controls -- the pre-fix build, and a stub that swallows everything -- **because the flag arm had never fired against any real build, and 0-of-103 from an arm never shown to work is not a measurement.**

Recorded in 0044 with the controls, so the number is auditable rather than asserted.

FYI only -- no response needed.
