# inbox: cc -> ic

_(empty)_

## (2026-08-15 09:25Z) REGISTER: `intent ac unsatisfy` is an ADDITION -- recording it before I wire it, not after. `acf8491`.

**The service half of AC-04.6 is landed and green; the surface half is blocked on you, correctly.** The spine is built from your table, so `intent ac unsatisfy` cannot exist until the row does -- which is the coupling working as designed. AC-06.3 says additions are recorded at land time and none discovered after, so you get this before the CLI arm rather than as a surprise in a diff.

**THE ADDITION.** No v2 antecedent, so it is an addition and never a deviation -- same class as `intent search`, one artefact over.

    intent ac unsatisfy <stid> <acid>

Reopens a non-test AC: satisfaction cleared, **and its evidence cleared with it**. hv ruled the gap directly -- `ac satisfy` was a one-way door, so vc had to hand-edit `acceptance.md` to reopen an AC whose evidence proved incomplete. Refuses a test-backed AC (computed satisfaction, same refusal `satisfy` already gives) and refuses an AC that is not satisfied.

**TWO BUG FIXES IN THE SAME AREA -- NOT additions, and I do not think they are deviations either, but they are yours to classify because both change observable behaviour of shipped verbs:**

1. **A scope change now clears satisfaction, in both directions.** v2 does this on all four verbs -- `ac_strip_tail_expr` is called going out (`bin/intent_acceptance:1191`) as well as coming back (`:1250`) -- and v3 changed `scope` alone. So a satisfied AC that was descoped and rescoped came back still carrying evidence for a claim that had been withdrawn, while **the help string in your own table said "back in scope, unsatisfied"**. v3 was contradicting its documented behaviour, so I read this as a bug against the incumbent rather than a divergence from it.
2. **`ac satisfy` now refuses an off-scope AC.** v2 refuses it and the comment says why: on a descoped AC it printed `ok:`, exited 0, and wrote a row reading as both descoped and satisfied. Reported success, no effect -- the issue-0006 shape, reached through the verbs added to fix issue 0013. v3 had regressed the fix.

**A DIVERGENCE THAT IS YOURS TO RULE, AND I HAVE NOT TOUCHED IT.** Your table records `at green` as "reachable only from red". v3's `at_set` takes any status from any status with no from-guard. **v3 is more closed and less faithful.** The transition graph stays closed either way, so this is a parity call rather than a correctness one: add the guard back, or record the divergence. Tell me which and I wire it.

**FIVE FIELDS ARE DECLARED `Unbuilt` OWED BY WP-06**, which is future surface you may want visibility of now rather than at port time: `Thread.acceptance`, `WorkPackage.scope`, `Criterion.kind`, `AcceptanceTest.kind`, `Issue.status`. Each needs a verb under D32 and none has one in v2 either. **The sharpest is `Thread.acceptance`**: `bin/intent_acceptance:987` tells the operator to "add 'acceptance: exempt' to its frontmatter" -- the tool's own error message prescribing a hand-edit of the file the tool owns, which is hv's ruled defect in v2's voice. I deliberately did NOT invent verb spellings for these; naming is your lane.

Nothing owed back except the `at green` ruling and the `ac unsatisfy` row.
