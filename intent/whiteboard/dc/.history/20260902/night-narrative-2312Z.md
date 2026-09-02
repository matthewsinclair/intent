# dc -- narrative for the 23:12Z fold, 2026-09-02

Companion to `wip-prefold-2312Z.md` (`398db1ff`, 60,086 bytes, `cmp`-verified verbatim). The board keeps rules; this keeps the reasoning.

## What the session actually did

hv held me on the bounce, then sent me to vc for next steps. vc answered four things. Three were mine and all three are disposed.

**The rebuild.** The shipped pair had lagged source all evening and I had been calling that a permission problem. It was not. `0196` is a DEFECT -- `guarded_release_build` deletes the pair before building and no failure path restores it -- so the question was never _may I_ but _is anyone mid-run_, which is answerable by asking. cc and ic both cleared it and held commits entirely.

**cc found the hole in my own ask before I started, and it was not a detail.** I had described an unbounded outage as a window. `~/.local/bin/intent` is a symlink into the release directory, so a failed build leaves every session with no binary -- and the pre-commit gate shells out to `intent`, so nobody can commit a repair either. The ask only became honest once a restore existed. I copied the pair outside the release directory and `cmp`-verified both halves first. That is now W57.

**`0217` got a row.** vc ruled a new row rather than an amendment, on the instrument rather than the subject: checking TABLE -> SURFACE walks the table, checking SURFACE -> TABLE must enumerate the binary. Different walk, different denominator. cc then caught that both rows said _the built binary_ without naming WHICH -- and bound to the shared pair the test would describe whatever was last built, and would be unrunnable while `native/rust` is dirty, which is the normal state while the surface is being changed. Now bound to `env!("CARGO_BIN_EXE_intent")`, with vc's addition of what that binding COSTS: so bound, the row says nothing about what is delivered.

## The thing worth keeping from today

**`0203`'s retarget did not survive its drive, and the premise was mine.** I wrote _the six keep rows can be retargeted once this is ruled_ before the remedy existed. The remedy then made the assertions TRUE rather than retargetable. vc ruled off that paragraph afterwards without re-driving it. The principle held and its predicate dissolved -- W58.

**The one real defect was the inverse of the brief.** `wp_commands.bats` asserted the usage strings and nothing about the exit code, under a comment claiming rc=1. The only row with an opinion about rc kept it where nothing could check it, false since `245dcdbe`, no symptom -- W55.

## My own two errors, both caught by reading back

A canon note citing `245bcdbe`, which resolves to nothing. And a verification loop whose sentinel already occurred five times in the file, so it could not have failed -- inside the check I built to obey the correction about exactly that class. Both are W56.

## What this fold nearly did, again

**W52, W53 and W54 were cited in the TODO standing-rules block and had no Watch-out entry anywhere.** A fold that archives TODO would have deleted three rules by moving them -- the same shape as last fold burying three unexecuted hv items, one level up: not an item keyed on dated, but a RULE living in the wrong section. They are promoted. The control now also checks that every cited W-number has an entry: 58 cited, 58 entries.
