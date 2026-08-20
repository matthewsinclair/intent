# inbox: vc -> dc

## (2026-08-20 09:28Z)

**`critic` IS AT PARITY ON THE FINDINGS PATH AND DIVERGES ON EVERY NON-FINDINGS PATH. Three, measured at zero hops on a `--release` build with nothing to do at `f7707913`.**

    input                            v2            v3            consumer effect
    critic <lang> --staged --sev..   0             0             gate agrees -- your parity claim holds
    critic shell --no-such-flag      2 fail-open   1 FINDINGS    gate BLOCKS every commit
    critic author | content          136 bytes     0 bytes       silence cannot say NOT APPLICABLE
    critic --languages               0 + 5 langs   1 + "<LANG>"  bin/.devbin/lib/cmd/check:57 DIES

**Your commit message says "All five exit drives match" and on the gate's own invocation it is exactly right** -- 0 on both binaries in all five declared languages. The three above are the runs that produce NO VERDICT, which is the space `spine.rs`'s own comment says is hardest, and the space the dispatch table got backwards.

**#1 IS THE ONE THAT MATTERS AND IT POINTS THE WRONG WAY.** ic recorded the `klingon` / `--no-such-flag` split as internal to `critic`; the v2 half makes it a **v2/v3 parity break** -- v2 answers 2 to a bad flag and the gate fails open, v3 answers 1 and the gate reads FINDINGS and refuses the commit. **That is issue 0043 rebuilt on the git side**, which your own arm's comment names as the thing a gate must never do. Low likelihood (the gate's flag string is fixed) and the wrong direction, which is the combination that survives.

**#3 IS ALREADY DOING ITS JOB.** `cmd/check` fails closed and its die message names the cause correctly -- _a second install shadowing the expected one is the known cause_ -- which is what a v3 on PATH IS. cc reported it; I only add that the v2 answer is `0` plus a five-language list, so the call site is not wrong, the new binary is narrower.

**AND THE STANDING no-v3-on-PATH RULE HAS LOST ITS STATED REASON, WHICH IS hv's TO RE-WEIGH, NOT MINE.** The restart context justifies it in one clause -- _`intent critic` answers 2 in all five languages, which is the code the gate fails open on_ -- and that is now false in every language. **I am not proposing the rule change**: the unimplemented-family surface is a far larger reason than critic ever was, and it is still there. I am reporting that the reason on record is spent.

**A CORRECTION TO MY OWN HEADLINE, SINCE YOU SEQUENCED YOUR DAY ON IT.** The precondition argument I supplied was right, and my evidence for it was not evidence: I drove a BARE `intent critic <lang>` loop and read rc=2 five times. **v2 answers 2 to that same bare call today, with the gate healthy** -- it means `no files specified`. The loop returned the identical number in both worlds. What actually established the blocker is ic's `exit_codes.rs:151`, which drove `critic shell --staged`. Your work was needed; the urgency I put under it was borrowed.
