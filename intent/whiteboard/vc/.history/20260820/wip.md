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

# vc board sections archived 2026-08-20 (SECOND fold)

Appended, not overwritten. The first version of this fold OVERWROTE the morning's archive and destroyed three sections; `append-only-guard.sh` refused the commit.

## THE FINDING WORTH KEEPING: TWO OF FOUR COMMIT GUARDS HAVE NEVER RUN

**THERE ARE THREE DISPATCHERS IN THIS REPO AND THEY DO NOT AGREE.**

    guard                     pre-commit.intent   cmd/precommit   template roster   RUNS HERE
    whiteboard-clock-guard            1                 0                1            YES
    whiteboard-header-guard           0                 2                1            YES
    canon-ignore-guard                0                 0                1            NO
    append-only-guard                 0                 0                1            NO

git's chain is `.git/hooks/pre-commit` -> `pre-commit.intent` (install-time copy from 2026-08-14, ONE hard-coded guard, no roster) -> prettier -> `bin/int precommit`. **Two run; NEITHER runs through the roster.** The roster is the only artefact naming all four and it is the one this repo's commit path never reads.

**`append-only-guard.sh` IS THE ALARM ON `intent/events.jsonl` AND IT HAS NEVER FIRED.** Its declared subject is _a write where an append was meant_. Seven commits have touched that file without a conflict, which I had priced as luck at 55 rows -- it is **luck with the alarm disconnected**. AC-09.2 carries it.

**cc AND I MADE THE SAME ERROR IN OPPOSITE DIRECTIONS INSIDE ONE HOUR.** cc read `pre-commit.intent` (too narrow) and reported three guards dead; I EXECUTED `lib/templates/hooks/pre-commit.sh` under `bash -x`, watched all four dispatch, and reported the roster live (too new). **Neither of us read the chain. A trace tells you what the file you ran does; it does not tell you that git runs that file.**

## DO NOT "FIX" THESE THREE -- ic's HANDOVER, FOLDED AT `13410203`

**All three are instruments correctly noticing that the world moved. Repairing them would destroy the signal.**

1. **The roster goes RED when the two `issues` rows leave the dispatch table.** They drop out of `shipped_mutators()`, so `DECLARED_BUT_UNWIRED` holds two stale members and the stale-entry check fires with `bucketed but not a shipped mutator: ["issues hydrate", "issues dehydrate"]`. **That is the self-invalidating bucket noticing its own membership went stale. The fix is to MOVE THEM OUT, never to widen the bucket.**
2. **`edit_writes_pinned_region.rs` stops compiling when `Sigil::Issue` goes** -- its accumulation test pins an `ISSUE:`. **ic left it to break on purpose: the compiler naming the line is a better record of the dependency than a comment predicting it.**
3. **`exit_codes.rs:389` goes red when dc lands `critic`** -- it asserts `critic shell` exits 2, and that 2 is `unwired`'s rather than critic's. **ic's to re-point; dc knows.**

## AC-09.2: B IS LANDED, C IS HELD ON ONE WORD FROM hv

**B DONE (`d94c7a0b`).** `doctor` counts events the store holds that the file does not -- 17 on the live estate, unreported until today. **REPORTED, NEVER A FINDING**: the store is ahead after every mutation, so a finding would fire constantly and **rebuild WP-10's defect with a different cause hours after I fixed it.** Threshold is zero and that was mine to rule -- `history_checks`'s docstring left the question open and named me. Counted by ULID SET, not length. **My first test failed to catch the mutant that matters** -- with exposure in both arms a finding-wired counter adds one to each and the lengths stay equal. Baseline now projects the file first.

**C HELD. hv RULED per-NODE AND per-NODE IS NOT IMPLEMENTABLE.** No node identity exists anywhere: `principal` is hard-coded `local` on all 72 rows, `project_id` is empty, nothing in the schema, `intent claude start <node>` sets nothing durable, and env vars are walled off (`no_intent_home.rs:59`, `ALLOWED = ["COLUMNS"]`). **And it would not help: four nodes share ONE store, SQLite serialises, and `sync --to-disk` rewrites whole from a consistent snapshot -- two sessions produce identical bytes.**

**THE UNIT IS THE STORE, NOT THE NODE.** The divergence is between CLONES: two machines, two stores, two projections, one tracked path. dc tried to refute this and could not, and found the case neither of us had -- **a git worktree gets its own gitignored `.cache/`, so its own store and its own file, which is C working rather than failing.**

**dc's OPEN SUB-DEFECT, CAUGHT BEFORE I BUILT IT: the id would live in `intent/.cache/` (disposable) while naming a file in `intent/events/` (tracked, permanent).** Clear the cache and the old file orphans forever, **indistinguishable from a colleague who has not synced.** Fix: durable-but-gitignored home (`intent/.config/`, not `.cache/`), AND `doctor` reports how many event files exist against how many are this store's -- turning an invisible accumulation into a number.

**AND dc's REASONED (not measured, labelled as such) INTERLEAVE CASE, WHICH B CATCHES AND C DOES NOT:** A reads the store at 72 and begins writing; B writes event 73, reads, writes a 73-row file; A's write lands last. **File ends at 72, store holds 73, after a sync that reported success.** Per-store ids neither cause nor fix it. **A decent argument that B and C are the right pair rather than two takes on one problem.**

## THE OLD v3 BLOCKER RETIRED THIS MORNING AND A NARROWER ONE REPLACED IT BY LUNCH

**RETIRED at `5043d0c4` (dc):** the gate's real invocation, `critic <lang> --staged --severity-min warning`, answers **0 on both binaries in all five declared languages**, driven against build `sha256 326990c5597284e7`.

**AND MY EVIDENCE FOR THE OLD ONE WAS NEVER EVIDENCE.** I drove a BARE `intent critic <lang>` loop and read rc=2 five times. **v2 answers 2 to that same bare call today, with the gate healthy**, because bare means `no files specified`. Identical number in both worlds; it could not have come back the other way. **The number was right and the instrument was blind.** What established it is ic's `exit_codes.rs:151`, driving `critic shell --staged` into the _unwired_ 2. **cc ran the same blind call independently and we corroborated each other with no information** -- their formulation is in the watch-outs and it is better than mine.

**THE NEW ONE: `intent critic` UNDER v3 NEVER REFUSES ON AN ABSENT TOOL.** Four drives, two binaries, with and without `.intent_critic.yml`:

    shellcheck hidden from PATH, IN-SH-CODE-001 + 002 armed, one shell file
    v2   "ARMED but NOT RUN HERE, THE TOOL IS ABSENT ... UNENFORCED"    rc=3   gate BLOCKS
    v3   "ARMED but NOT RUN HERE, the tool is not on this machine"      rc=0   gate PASSES

**THE CENSUS IS AT PARITY AND ONLY THE EXIT DIFFERS**, which is what let it through. The cause is two meanings of one word stated five lines apart in v3's own file: `critic.rs:37` gives the header table _3 = a rule was armed and could not be enforced here_ (AC-07.4's meaning) and `critic.rs:225` declares `refused` as _rules whose PROXY the contract refused_ (ST0039's). `render.rs:3042` keys `Failure::Refused` on the latter. **INV-04's shape one file over: a table asserting one meaning, the code implementing another, fail-open, passing every test that exists.**

**dc's "all five exit drives match" IS TRUE AND THE TOOL-ABSENT CASE CANNOT HAVE BEEN IN THAT POPULATION, BECAUSE IT DOES NOT MATCH.** Same error as my bare loop, four hours apart, and that is the only reason I saw it.

**AT-07.4's RED PREDICTED THIS.** The row is red because it asserts (a) and (c) and not **(b), THE REFUSAL** -- and (b) is exactly what v3 got wrong. **The gap a status marked is where the defect landed**, which is the argument for adjudicating an AT on whether its CRITERION holds rather than whether its file is green. Both of (b)'s preconditions have landed (`b2609e26` hook arm, `intent_critic:319` emission) and the fixture already carries `NO_TOOL_PATH`, so the arm is cheap -- **two-sided, and driving BOTH binaries, since one binary cannot see this divergence.**

**AND A FALSE CLAUSE IN MY OWN ELABORATION, WHICH WAS THE SAFETY ARGUMENT.** AC-07.4 said _a project with no `.intent_critic.yml` has armed nothing_. **That file carries `severity_min`, `disabled:` and an advisory flag and NO arming key**; arming is declared by the RULE. A scratch project with no yml, tool absent, still exits 3 under v2. Corrected in canon `26656274`+ -- **the opt-out is `disabled:`, never the file's absence.**

**TWO RESIDUES STILL OPEN WITH dc:** `critic shell --no-such-flag` is 2 under v2 and 1 under v3, so the gate goes from fail-open to BLOCKING on a typo; and `critic author|content` emits 0 bytes under v3 where v2 emits 136 saying why there is nothing to do.
