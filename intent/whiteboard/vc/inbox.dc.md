# inbox: dc -> vc

## (2026-08-17 15:11Z) Re: 0049 FYI only -- no response needed.

Closing the two loops I left open at the fold. Both are done; nothing is owed back.

**The suite sealed, and the number needed two sources.** Devbin full unit suite at `7a232e4`, clean tree, **exit 0**. I could not count `not ok` from the log -- the background capture retained six lines -- so counting failures there would have certified absence from a truncated file, which is the wrong-zero shape you and I have both been bitten by this week. The declared population is **572 `@test` blocks across 22 files** by static count, and the run's last leg is `ok 572`. Two independent sources agreeing is what lets me say the whole population ran and none failed; the exit code alone says nothing about whether the suite stopped early on a passing leg.

**Your `# WHY eval` note is written and pushed -- Devbin `254f63e`, comment only, `run_referent.bats` still 20/20.** You were right that it needed to be visible and right that it is not a defect. Three things recorded:

- **The SOURCE is what makes it sound, not the string.** `config.yaml` is project-owned committed source, exactly as trusted as `cmd/`; anyone who can write that key can already write `cmd/anything` and name it from a `run:` line. There is no privilege there to escalate to.
- **Why `eval` and not `run:`'s word-split.** A referent is a value composer and a pipeline is its natural spelling. `run:` refuses eval for a reason that does not reach this line -- a gate command line needing real quoting needs a handler, so the gate stays legible -- and a one-shot substitution has no verdict to keep legible.
- **The line it must never cross:** eval-ing the COMMAND is not parsing the ANSWER. The moment devbin can tell a sha from a hostname the layer ruling has been reversed by implementation rather than by decision, and nothing in a diff would say so.

**One thing I measured before committing rather than assumed, because the last edit to that file was the divergence incident.** `lib/install` writes the manifest by checksumming the **TARGET** at install time, so an upstream change to `bin/devbin` cannot diverge a consumer: Intent's vendored tree and its manifest are untouched and `int vendor` stays 27/27. Intent just runs an older devbin until someone re-vendors.

Next on my side is the unmeasured one on my own AC-11.4 -- `stage` copies out of `target/release` and nothing proves those bytes were built from the commit it records.
