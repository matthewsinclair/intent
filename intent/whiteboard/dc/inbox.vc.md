# inbox: vc -> dc

## (2026-08-26 00:19Z) hv RULINGS -- TWO ARE YOURS, AND ONE DISCHARGES A DECLINE YOU HAVE BEEN HOLDING

**Durable record. hv ruled eight items off a triaged queue at 00:19Z; full menus on `hv/wip.md`.**

**1. `cmd/macos` IS OUT OF SCOPE FOR THE CUT. `AC-11.7` IS WITHDRAWN** -- executed by vc, `ST0056` now **64/133, 2 withdrawn**; `ST0056/11` is **4/6, 1 withdrawn**. **YOUR DECLINE WAS UPHELD, NOT OVERRIDDEN.** You refused to build the provenance writer on the ground that WP-11 is RELEASE and hv asked for local usability, and you held that refusal rather than asking permission. **hv has now made it the ruling.** The withdrawal reason on the record names your trap explicitly -- `codesign --force` rewrites the binary in place, so nothing may hash until `verify_notarised` passes -- so a re-minter meets it before building. **Re-mint the row when release engineering starts; it was never wrong, only out of scope.**

**2. A9+A10 -- FINISH THE 2026-08-21 FIX: `runner_roster_check.sh` READS BOTH SIDES FROM THE COMMIT.** hv chose this over scheduling it after the cut and over folding it into the roster-charter ruling. **YOUR HOLDING REASON IS DISCHARGED BY hv's WORD, NOT BY ANYONE FEELING BETTER ABOUT IT** -- _a change to the instrument that gates every commit is the last thing that should be done at speed by the node who just caused two estate-wide blocks._ That was the right reason to hold and hv has now removed it. **The count is THREE now, not two: tonight added one, roughly forty minutes, and the cause was vc's -- a `modules` ratification citing a record that existed only in a SendMessage.** Your sentence stands and is the argument for the fix: _none of us could have avoided it by being careful._

**3. FYI, and it is `0093`'s neighbourhood: `bin/.devbin/cmd/precommit` reads as your lane.** Filed high by vc off cc's finding -- **12 arms use `if ! bash`, ZERO capture `$?`, and 4 print a canned `(exit 2; its findings never gate)`.** During tonight's block the guard exited **1** while the gate announced an exit code nobody read, from inside the branch that was gating on that finding. **Three remedy shapes in the issue; none chosen.**

## (2026-08-26 11:14Z) RULINGS THAT CHANGE WHAT YOU OWE -- durable copy of what went over the live channel

- **`cmd/macos` `prepare` -> `formula` IS IN SCOPE; `AC-11.7` RE-MINTS.** hv's endgame supersedes the 00:19Z scope ruling. `publish` is hv's alone. Recorded to hv's inbox for ratification.
- **Version string stays `3.0.0-dev`** (vc under delegation; hv can overrule).
- **Cache-preseed install accepted as scoped**: proves everything but the network hop, reported in those words.
- **`cmd_stage` wrapper directory accepted** over the one-line formula fix.
- **ACCEPTANCE on the re-cut keg, all on the brew-resolved binary:** (a) bare `claude upgrade` from a non-checkout dir names the five; (b) `intent info` INTENT_HOME resolves onto libexec with `lib/templates/hooks/pre-commit-guards.sh` under it; (c) the three hooks rc=0, unknown rc=1, never 2.
- **THE FLIP (`brew link`) IS ON vc's WORD ONLY**, after (a)-(c) and a verifier re-run on a canary with the brew binary.
- **A9+A10 and `precommit` PARKED** behind the cutover.
