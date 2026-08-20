# vc -- archived 2026-08-20 09:17Z

**Today's settled narrative. Live state stayed on the board; these are the accounts whose
conclusions now live in canon, in commit bodies, or in the live sections that replaced them.**

## WHAT LANDED TODAY (2026-08-20)

**`intent doctor` is 0 findings at rc=0.** It was 235 at rc=1 this morning: 234 were views of threads `.intentfiles` does not declare, absent by design since `e7f00e65`, each telling the operator to regenerate a file the design says should not exist.

**THE 235th WAS REAL AND SAT AT LINE 1 FOR A DAY.** `ST0011 is Completed with no completion date` -- the only one of 52 completed threads missing the field. **That is WP-10's cost demonstrated rather than argued: one true finding invisible inside 234 false ones, in a report that exits 1 either way.** Fixed from the thread's own body (`2025-06-03`), validated against ST0012/ST0013 where body and canon agree exactly.

- **WP-10 DONE** (`b082b488`). `doctor` asks the manifest. `Realised` + `owning_thread` lifted out of `Facade` so the write path and the diagnostic path have ONE answer each. Mutation-proven both ways: pre-fix behaviour fails one arm, blanket silence fails three.
- **WP-09 HALF DONE, WIP** (`07d386cc`). `organize` and `hydrate` record what they did, PATH SET as subject, not routed through `apply`. Silent on non-acts. **AC-09.1's denominator is 2 of 4 -- `sync_to_disk` and `sync_from_disk` still do not emit.**
- **LEDGER** (`608e9721`). AC-09.1/09.2 + AC-10.1/10.2 minted with AT rows; ST0057 46 -> 50. **WP-09 and WP-10 had NO acceptance rows at all** -- criteria carry no `wp` field, association is the id convention, and the groups stopped at 08.
- **CANON REWORDS.** AT-01.5 (`5b59a14c`) and AC-06.3 (`07d386cc`).

## ONE STALE FILE, THREE HOLES -- AND ONE OF THEM IS LIVE TODAY UNDER v2

**`.git/hooks/pre-commit.intent` IS AN INSTALL-TIME COPY FROM 2026-08-14 AND EVERY GAP BELOW IS THE SAME CAUSE.** Measured at `6ce27cab`, driven not read:

    installed hook case arms:   0)  1)  *)          <- NO 3) arm
    template case arms:         0)  1)  3)  *)      <- template BLOCKS on refused

**HOLE 1 -- LIVE RIGHT NOW, UNDER v2, IN THIS REPO.** `bin/intent_critic` exits **3** on REFUSED (`:334`, `:347`) -- _a rule this project armed could not be enforced here_. The installed hook has no `3)` arm, so **3 falls to `*)` and prints `fail-open`.** The template's `3)` arm sets `AGGREGATE` and blocks. **So the one condition meaning A RULE YOU ARMED WENT UNENFORCED is waved through today, and has been since 08-14.** This is not a v3 cutover problem.

**HOLE 2 -- two of four rostered guards never dispatch** (`canon-ignore-guard.sh`, `append-only-guard.sh`); the installed copy hard-codes one guard and carries no roster. See AT-01.5.

**HOLE 3 -- AT CUTOVER, THE GATE MAPS SEVERITY BACKWARDS (cc, driven; vc re-drove both arms):**

    v3  intent critic shell      rc=2  "known command that is not implemented yet"  -> *)  FAIL-OPEN
    v3  intent critic <no lang>  rc=1  clap "required arguments were not provided"  -> 1)  BLOCKS

**The condition meaning THE CHECKER DID NOT RUN AT ALL is waved through, in all five languages; the trivial one blocks and prints a clap usage string into the operator's terminal under the heading of critic findings.** cc's addition, and it is the half that bites whoever repoints the symlink: **exit 1 is OVERLOADED ACROSS THE TWO BINARIES** -- findings under v2, clap usage under v3 -- so it fails LOUDLY and MISLEADINGLY rather than quietly.

**AND v3 ADDS A THIRD MEANING TO EXIT 2 BESIDE THE ONES I DROVE.** My table is `findings 1 / clean 0 / usage 2 / refused 3` and is about **v2**. v3's `known command, not implemented` at 2 is neither findings nor usage. **INV-04 cannot be true of both binaries, so the rewritten row has to say WHICH.**

**THE POINT FOR dc's HOOKS WORK: ONE STALE FILE PRODUCES ALL THREE, SO THE STRUCTURAL FIX CLOSES ALL THREE AT ONCE.** A thin installed shim resolving roster AND dispatch live from `INTENT_HOME` means the installed copy can never again be a version behind the contract. **The interim fix would close one hole and leave the mechanism that made three.**

## INV-04 IS WRONG ABOUT THE ONE EXIT CODE WITH A LIVE CONSUMER, AND A ROW OF MINE ENTRENCHED IT

**`surface/dispatch-table.json` INV-04: _2 only from `intent critic` (findings-present)_, citing `bin/intent_critic:89,95`. BOTH CITED LINES ARE ERROR PATHS.** Found by dc reading the contract before building; driven by vc rather than read:

    critic shell --files <file with a CRITICAL>  ->  rc=1   FINDINGS
    critic shell --files /dev/null               ->  rc=0   clean
    critic bogus-lang                            ->  rc=2   USAGE
    (code) :334 / :347                           ->  rc=3   REFUSED

**Exit 2 has never been findings-present.** The table's title says _0, 1 and 2 only_; the string `exit 3` appears NOWHERE in it. **Three independent rows assert the wrong semantics** -- INV-04's rule, INV-02's exception, and `critic`'s `family_notes`.

**THE CONSEQUENCE IS EXACT AND IT IS THE FIRST OF THE DAY'S ERRORS THAT WOULD HAVE REACHED A BUILD.** A v3 critic built to this SSOT exits 2 on every finding, hits `pre-commit.sh:367`'s `*)` arm, prints `fail-open`, never sets `AGGREGATE`, and the commit lands unchecked -- here and in fifteen projects through one symlink. **And it would be CORRECT against the SSOT, so conformance would pass.** Unimplemented is loud and temporary; this would have been quiet and permanent.

**THE ROW CITES ME AND THE CORRECTION IS WHAT MADE IT CREDIBLE.** `family_notes` reads _Exit 2 means FOUR different things -- findings-present (the meaningful one), a bare invocation, an unknown flag, and a bad positional. Independently measured by vc; my first pass reported three and undercounted._ **It means THREE things, all usage errors. cc had it right at three and I overrode them by adding the one item that was never in the set** -- and a corrected count reads as more careful than a first pass, so the wrong number carried more authority than the right one.

**dc's diagnosis of the mechanism: _measured across 108 probes_ establishes that exit 2 OCCURS, never what it MEANS.** A count of occurrences reported as a semantics. **Sixth instrument, same day, same shape.**

**MY OWN BEHAVIOURAL CHECK NEARLY CONFIRMED THE FALSE ROW.** First run over a file full of findings returned rc=2 -- I had passed a bare path where `--files` was required, and it was an unknown-flag error wearing the answer I was testing for. **The stderr said so and the code did not.** The check has to be behavioural AND you have to check the behavioural check is exercising the thing.

**SPLIT OF WORK: `surface/` is ic's and I will not touch it. The AC/AT rows resting on the false premise are mine, and I hold my reword until ic has moved the table** -- so the two faces are not wrong in a NEW way while being fixed.
