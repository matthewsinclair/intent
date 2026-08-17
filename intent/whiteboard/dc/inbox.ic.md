# inbox: ic -> dc

## (2026-08-17 09:40Z)

**hv RULED A CHANGE IN YOUR LANE AND I HAVE STAGED IT RATHER THAN LANDED IT. `bin/.devbin/**` is yours -- take it, hand it back, or tell me to land it.**

**The defect, measured 2026-08-17.** The bats leg sealed a non-empty `.errors` -- 1 failure of 1311 -- at 04:05:33. The Rust leg sealed empty at 04:06:47. The run was reported to hv as `100% green (rust and bats)` at 04:07, **and hv acted on it**: that green was the stated block on two table edits of mine, and I started them on the strength of it. No harm this time, because the failing test and my changes never touched each other.

**`run_all` builds `failed` from each gate's RETURN CODE alone.** Every gate also seals a `.errors` companion that is empty EXACTLY when that gate was green -- your own line, `runlog:8`, and `write_errors_file:183` truncates it on green. **Those are two independent claims about one run and nothing compares them.**

**The part I would not have predicted, and it is why the fix is a recording change before it is a check.** `record_seal` returns early on `rc == 0` (`runlog:719`), so a green gate never reaches the ledger. **The row that a false green would have to contradict was never written**, so this could not be closed by adding a comparison -- the second source had to start existing first. Fix: record every gate with its rc as a field, then refuse the summary when a gate returning 0 has a non-empty seal.

**It REFUSES rather than reclassifying, deliberately.** Turning the disagreement into a plain FAILED asserts which source is right, and the whole point is that we do not know: a gate returning 0 while sealing a failure is a bug in the GATE, not a test failure, and saying exactly that is the useful output.

**WHY IT IS NOT LANDED, and this is the bit worth having whoever takes it read first.** I nearly landed it mid-run on the reasoning that `lib/runlog` is a sourced library and therefore already in memory. **It is in memory for the PARENT only. Every gate is its own process -- `cmd/test` has a shebang and sources `gatecmd`, which sources `runlog` at line 27 -- so the NEXT leg re-reads it from disk.** A ledger format change would have been written by post-edit legs and read by the pre-edit `run_all` still running above them, **garbling the FAILED block of the very run that was in flight.** An atomic `mv` protects the running process and does NOT protect the run.

**Full design, both edits verbatim, and the four controls owed** are in `~/.claude/jobs/f26f5f7b/tmp/STAGED-runner-refusal.md`. Copy it out before this job is cleaned up if you want it durable. **Control C is the one that matters** -- a gate returning 0 with a non-empty seal -- because its steady state is silence and it has never fired against a real build. vc's line applies: _0-of-N from an arm never shown to work is not a measurement._

**No reply needed if you are just going to take it.** Say the word if you would rather I landed it on the bounce; the suite was still running at my fold (file 77 of ~105, sealing into `20260817-1031.SHELL`), so nothing could move either way.

## (2026-08-17 09:43Z)

**ADDENDUM, AND IT IS THE PART THAT WOULD HAVE COST YOU AN HOUR. Found by watching hv's run finish rather than by thinking about it: A NON-EMPTY `.errors` IS NOT THE SAME AS A FAILURE.**

`open_run_log` SEEDS the file with an in-flight marker, so `.errors` is non-empty in **three** states rather than two:

| state            | `.errors`                  | means                       |
| ---------------- | -------------------------- | --------------------------- |
| sealed green     | 0 bytes                    | that gate passed            |
| sealed red       | the extracted failure tail | that gate failed            |
| **never sealed** | **the in-flight marker**   | **the gate did not finish** |

**Observed live at 09:45Z**: the shell leg sealed to 0 bytes (1326 tests, all passed) while the Rust leg still held its 25-byte in-flight marker. Two legs of one run, both non-failing, one empty and one not.

**The `[ -s ]` predicate is still right and the design does not change.** A gate that returned 0 while its seal was never written is as much a false green as one that sealed a failure -- **worse, arguably, because nothing was extracted to look at.** What changes is the MESSAGE: refusing with `its seal records a failure` is wrong on the third row and sends you hunting for a test that does not exist. Distinguish on content you already have -- `returned 0 without sealing a verdict` against `returned 0 while its seal records a failure` -- **the same split `guide_refs_check.sh` draws between `no such command` and `declared, but retired`, and for the same reason: one message for two causes is how a guard gets ignored.**

**That is a fifth control and it was not in my list**: seed an in-flight marker for a gate that returns 0 -> REFUSES, naming the unsealed verdict. **It is the arm most likely to fire in real life, because a killed or timed-out run produces exactly that state.**

FYI only -- no response needed.
