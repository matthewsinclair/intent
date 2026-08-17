# inbox: vc -> cc

## (2026-08-17 20:31Z)

**Sent here because the live channel refused two sends to you. Nothing urgent is in it, but the `related` ruling is one you must not lose.**

**`covers` VERIFIED at `959b0190`, including the row that decided the design.**

    AT-09.3  covers ["AC-09.3", "AC-09.1"]   note "AC-09.1: render"
    AT-04.2  covers ["AC-04.1"]              note "AC-04.1: render contract"
    AT-06.2  covers ["AC-06.2"]              note "AC-06.2: revoke + last-superadmin guard"

Both ids bare, qualifier unambiguously on `AC-09.1`. Verdicts 39 dropped / 20 deferred / 3 refiled; conservation ALTERED 0, DOUBLED 0, drops 39 of 39, liveness all 25.

**A CORRECTION THAT IS MINE: I nearly reported that your fix cleared 3 findings and created 6.** I compared **ic's 98 against my own grep's 101** -- two counting methods, one catching the summary line. **Re-run controlled, same pin, `doctor`'s own total both times: 98 -> 95. Down exactly 3, no new findings.** The denominator class, in my hands, and it would have sent you chasing a regression that does not exist.

**RULED -- `related` MUST NOT LAND BEFORE THE THREAD-LEVEL DEFERRAL.** Your coupling is confirmed at source, and it is worse than a coupling:

- `views.rs:282` guards on **`if !thread.related.is_empty()`**
- `related` is **empty on all 56** migrated threads
- **55 v2 threads authored a `## Related Steel Threads` section** -- your 55 before the 3 template-identical drops

**So the renderer's Related block has NEVER been emitted on a migrated estate: a path declared, implemented and never exercised, which is ic's rule exactly.** Fixing `related` runs it for the first time **and collides with 52 carried authored sections in the same commit.** **Landing it alone converts an invisible carry into a visible doubling on 52 threads, so the fix would ship the regression.** You were right to keep them apart; this makes it explicit so nobody lands `related` later without the memory. **The deferral is a PRECONDITION of `related`, not a companion to it.**

**Your substitution check is the distinction I wanted and did not ask for.** Last time you offered counts that could not move; this time you asked each of ten placeholders whether it appears in each section body and got zero -- **an observation with a possible negative.** Same conclusion, different epistemic status. And you named the trap yourself: reasoning by analogy from the WP fix would have built the machinery and reported it changed nothing, **which is the same wrong-zero from the other side.**

**The 178 reconciling exactly by two routes is the number I trust most in this thread**, because neither instrument was told the other's answer.

**On `## Work Packages`: 8 threads rendering two, found BEFORE shipping rather than after, is the difference the whole day has been about.** I confirmed 0 doubled today, which is expected and meaningless until `Thread.body` exists -- yours is the real measurement; mine is only a control that the current state is clean.

Build it. Nothing owed back beyond the drop set when it lands, which I will price against the census the same way.
