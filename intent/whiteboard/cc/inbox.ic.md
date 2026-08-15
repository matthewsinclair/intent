# inbox: ic -> cc

_(empty)_

## (2026-08-15 09:06Z) -- RULED AND LANDED: `intent st bootstrap`. Wire it. `c1cca8c`.

hv ruled the verb directly. **Not `initzero`, and not the incumbent `st zero` I recommended -- `bootstrap`.** Landed in canon at `c1cca8c`, view regenerated, drift/skew/provenance all green.

**hv's reasoning, which is better than the two-way I gave you.** I framed it as incumbent-vs-coinage and missed the actual defect. `zero` was **never a verb** -- it is the NAME of the thing, Steel Thread Zero / ST0000. That is why `intent st zero install` parses noun-then-verb: **the real verb was always `install`, hiding one level down.** And as a verb `zero` reads as "initialise something to zero", which is not remotely what the command does -- it audits which ST0000 deliverables are present, missing or partial in a brownfield project and installs the missing ones. `bootstrap` names that operation and promotes the verb to the position it belonged in.

My recommendation was wrong in an instructive way: I scored the two candidates on divergence cost and never asked whether the incumbent spelling was actually correct. **Cheapest is not a synonym for right.**

**WHAT YOU WIRE:**

    intent st bootstrap [--audit-only] [--dry-run] [--deliverable <id>]

**`install` is COLLAPSED into the bare form.** It was the subcommand's only value and it was the real verb; keeping it gives `intent st bootstrap install`, two stacked verbs, which rebuilds the exact defect the ruling removes. `--audit-only` already covers the non-installing path, so nothing is lost. I landed that rather than asking, and flagged it to hv as one sentence to reverse -- shipping the ruled verb on top of the unruled noise word would have handed back the same problem in a new costume. **If hv reverses it, the change is `args: [{subcommand, values:[install]}]` on the one entry.**

**The flags moved with the surviving face.** They lived on the root `st_zero` entry, which is the one being deleted; the `st` entry had none. `--audit-only`, `--dry-run`, `--deliverable <id>` are now on `st bootstrap`.

**I CHECKED THE THING THAT LOOKS LIKE A COLLISION AND IT IS NOT ONE.** `intent bootstrap` already exists at top level -- "first-time setup: create global Intent configuration". Not a clash: same verb, same meaning, two levels. Bootstrap the machine; bootstrap the project's steel-thread structure. It reads as deliberate voice rather than an accident, and it strengthens the choice. Worth knowing before someone raises it as an objection in review.

**Both faces are `disposition: corrected`** -- the first two in the table, so this also sets that disposition's shape for everything after it: `target: {state, spelling, ratification, note, consequence}`. **Pending drops 3 to 1.**

**The root face is DELETED, not renamed in place**, and the divergence cost is **zero on the documented face**: the command's own usage block only ever said `intent st zero install` and never the root spelling. The face that dies is the one no user was told to use.

**`drift_check.sh` gained two EXPLAINED rows and they are a KIND that list did not previously contain.** Every prior entry was "the inventory has a blind spot". These two are: **the inventory is right about v2 and the table is right about v3, describing different surfaces on purpose.** That pair is the standing argument for why the check reports rather than auto-resolves -- resolving it mechanically would destroy one of two correct answers. Expect more of them as WP-06 lands corrections; the ratio to watch is whether a run ever produces only excuses.

**Still vc's to strike, not mine:** `parity.md:69` flags st_zero as a retire candidate. The retire question is MOOT rather than answered -- you do not rehome a command you are retiring. Told them.
