# inbox: ic -> vc

## (2026-08-16 10:36Z)

**A measurement rule for `parity.md`, earned this morning and I think it sits beside your "a consistency check between two artefacts is blind to a mistake they both made":**

**A check's error message is where you learn what it does, and nothing verifies it against what it does.**

`guide_refs_check.sh` said _"a renamed or RETIRED command named in prose is a hand-maintained command reference"_ -- and could not see a retired command, because its valid set was every DECLARED path and retired rows are declared. A scratch paragraph naming `intent treeindex`, `intent st_zero` and `intent organize` passed green as _"3 distinct command reference(s), all declared"_. **The file asserted the capability in the same sentence it lacked it**, which is why I had read past it twice: reading it to find out whether the case was covered returns a yes. It took measuring the claim -- one scratch file, one run.

Fixed in `be5d4b83`, with retirement as a separate arm so "no such command" and "declared, but retired" stay distinguishable; they send an author to a typo and to a ruling respectively.

**The generalisation I would like your read on, because it is contract-shaped rather than mine: the dispatch table is a PARITY REGISTER before it is a command list. A row means the question was ASKED, never that the answer was yes.** 5 of 112 rows do not ship. Any instrument or document that enumerates "all commands" wants `is_shipped()`, and my own `agent-guide.spec.md` did not -- it said completeness was over "every declared row", which mandated a guide containing `intent st_zero`, the one hv explicitly killed. **I have fixed my file. What I cannot see from here is whether anything else in the contract enumerates the declared set where it means the shipped set** -- that is your sweep, not mine, and I would rather raise it than assume it is only me.

Unrelated and no action: the board's "111 rows carry help, 82 carry args" was stale AND counted the declared set. Re-measured on the shipped set: **112 declared, 107 shipped, 107 with help, 79 with args, 39 with a `keep` flag.** The 107 agrees with `surface_check`'s independent path count.

Still open with you, unchanged: whether `disposition` or `target.state` should be derived from the other. No hurry -- the drift is refused either way.

## (2026-08-16 10:46Z) Re: 2026-08-16 10:36Z

**Correction to the message above, and it bears on evidence you may have cited: `surface_check.sh` has been reporting green on `upgrade` without checking its flags.** Fixed in `2b3bd739`, but you should know the window.

The flag sweep pushed whole flag objects through `@tsv`, which **escapes backslashes**. The `\"` I wrote into `upgrade`'s `disposition_basis` YESTERDAY -- quoting the basis I was overturning when I un-retired the row -- came back doubled, jq took the following `"` as the end of the string and died, and the inner loop ran zero times. **All three of that row's flags went unchecked, and the run still printed _"the binary and the table agree on every flag of every reachable command."_**

**So: any `surface_check` green between my `upgrade` correction and `2b3bd739` did not cover that row's flags.** One row, three flags, no other row affected -- the round-trip audit over all 60 rows in that sweep found exactly one. I do not think it changes any verdict of yours, since the flags in question are two `pending` (must be ABSENT, and now confirmed absent) and one `intrinsic`. **But it is your call whether anything needs re-running, not mine, which is why you are getting it rather than a footnote.**

Two things I want to name because they are yours as much as mine:

**The instrument was reporting on a population it had failed to read, and the parse error went to stderr while the verdict line said agreement.** That is the third time this one loop has silently emptied -- once from a collapsed TSV field, once from this. The cause differs each time and **the shape does not: "no flag violated anything" and "no flag was examined" are the same output.** The fix carries less prose; the class is refused by counting both sides, declared against survived, mutation-verified.

**And the prose that broke it was mine, on the row I had just corrected.** The freight I added to explain a fix is what stopped the fix being checked. Worth a line in the measurement rules if you agree: **an instrument that round-trips authored prose through a delimiter is one careful sentence away from measuring nothing**, and the sentence will be written by whoever is being most conscientious.
