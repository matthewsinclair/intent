---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 17:14Z
status: active
focus: "**AC-08.4 IS GREEN AND IT WAS NEVER BLOCKED -- I linked it to the D57-8 finding by putting them in ADJACENT BULLETS, and the short denominator was in AT-07.1's file, not AT-08.4's.** D57-8 amended at `c5320329`; AC-05.3 satisfied, its Highlander clause covered all along by an UNCITED green test in a file I was editing today; AC-08.5 correctly still red. **Nothing of mine is with vc and nothing is in flight.** NEXT: **AC-07.7** -- the four COLLECTION forms resolve, and the red-first arm MUST be `AcCollection` because the POST clause reaches the other three; then `doctor --json`, then `declared_but_unwired`'s synthetic member."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. NOTHING UNCOMMITTED.** Board folded 2026-08-20 16:07Z; the pre-fold board and both handled inboxes are verbatim at `.history/20260820/*-1607Z.md`. **A trimmed board is a reading decision, not a record.**

## ON RESUME -- read this first

1. **THE LANDING IS `e7d038c3` AND IT WAS VERIFIED THE ONLY WAY THAT COUNTS**: detached worktree AT the commit, `git status --porcelain` 0 lines, `CARGO_TARGET_DIR` inside it -- **139 targets / 982 passed / 0 failed, clippy `-D warnings` rc=0.** Ten files in ONE commit because `address.rs` and `address_resolution_single_home.rs` each carry two bodies of work and **`--only` is PATH-scoped**; hunk staging is unavailable here.
2. **ALL THREE THINGS THAT SAT WITH vc ARE CLOSED, AND ONE CLOSED AGAINST MY OWN FRAMING** (vc 17:12Z; every claim re-read here off `at list ST0057` at zero hops).
   - **AC-08.4 IS GREEN, AND IT WAS NEVER BLOCKED ON D57-8. I am the one who linked them** -- I put the hold and the document finding in ADJACENT BULLETS on this board, and vc adopted the link without either of us checking which FILE held the short denominator. `d57_8_forms()` lives in `address_resolution_single_home.rs`, which is **AT-07.1's** file; AT-08.4 cites `mutation_create_splits_two_ways.rs`, whose `every_server_assigned_population_posts_to_its_collection` drives all three populations. **The build met the row's bar from the moment it landed.**
   - **AC-05.3 IS SATISFIED BY EVIDENCE**, and its Highlander clause had been covered the whole time by a green test that CITES NOTHING -- `edit_prints_a_path_that_exists.rs:300`, naming no AC and no AT. **I was editing that file today and did not see it**, which is cc's fold-8 class landing a second time, in my own hands.
   - **D57-8 IS AMENDED AT `c5320329`** and its fence now carries all four collection forms. **AC-07.1 IS NOT REOPENED** -- its population is every ENTITY form and it is faithful against nine -- so **no green of mine rests on a falsehood.** The gap moved to AC-07.7, which is mine.
   - **AC-08.5 STAYS RED**, exactly as vc and I both read it. **Nothing of mine is with vc.**
3. **A GATE COUNT MIXES THREE KINDS AND ONLY ONE IS WORK**: _not built_, _built and unverified_, _verified and unmoved_. AC-05.3 is the third. Ask which kind before a number drives a day.

## TODO -- LIVE ONLY

1. **AC-07.7 / AT-07.7 -- THE FOUR COLLECTION FORMS RESOLVE.** Minted by vc at `c5320329` and it is mine. `Threads`, `AcCollection`, `Issues` and `WpCollection` are addressable and **nothing asserts their resolution**; AC-07.1's population is every ENTITY form, so it was never going to reach them. **THE RED-FIRST ARM MUST BE `AcCollection` SPECIFICALLY** (vc's trap, written into the row): the other three ARE the POST clause's server-assigned populations, so any test sourced from that paragraph reaches them and passes **without ever touching the fourth.** An AC id is author-assigned -- `AcCollection` is the one collection the POST clause cannot reach, so it is the only honest red-first.
2. **`doctor --json`** (cc's ask). `Finding` already derives `Serialize`, so `Report` needing it is the whole model change. **cc's gate arm parses TEXT as a workaround and must be DELETED when the face lands, not kept beside it.** Trap: declare `--json` at BOTH family and verb level.
3. **`declared_but_unwired.rs` gets a SYNTHETIC member** per cc's ruling; the borrowed `st dehydrate` stops being load-bearing.
4. **NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives at all. hv deliberately left it out of ruling 4; vc raises it.

## Watch-outs

### The shared checkout

- **`git commit --only` IS PATH-SCOPED, NOT HUNK-SCOPED.** It protects against a peer's STAGED work and does nothing about their UNSTAGED work in a file you also touched -- that is how `main` stopped compiling with my caller at HEAD and my callee in a working tree. **It also means two bodies of work in one file cannot be split into two commits.**
- **ONLY A DETACHED WORKTREE AT THE NAMED REVISION CATCHES IT.** _Build first_ CANNOT: in a shared tree everything compiles because the missing half is sitting there, and **only the published tree is broken.** `git diff --cached` catches it and needs a human to recognise a stranger's hunk.
- **NEVER `cp` A SHARED SOURCE ASIDE TO MUTATE IT.** A restore silently reverts anything written inside the window -- no error, nothing in `git status`. Mutate in a worktree with your own diff applied on top, then discard.
- **A PEER'S HALF-WRITTEN FILE BLOCKS YOUR BUILD.** Five times today. Not a defect, not worth escalating -- **read WHOSE file before diagnosing**, then retry or verify in a worktree.

### Instruments

- **A CROSS-CHECK RECONCILES WHEN BOTH SIDES SHARE THE SAME ERROR, AND IT THEN READS AS CONFIRMATION.** dc reported the workspace at 139 / 978 and derived it correctly from my 138 / 974 -- both sums true. **HEAD alone, clean worktree, is 139 / 971.** The extra 7 were my uncommitted tests, **present on BOTH sides**. The arithmetic was right and the SUBJECT was wrong. **And it self-heals: 978 became true the moment I committed, so nothing afterwards could show it had been false when written.** A number measured in a shared checkout is a number about a tree nobody else has.
- **AN ABSENCE IS ONLY EVIDENCE WITHIN THE SCOPE YOU ACTUALLY READ, AND A CALL BOUNDARY IS NOT A SCOPE BOUNDARY.** I read `apply`'s last thirty lines and reported a property of the whole write path; the manifest read was one call down. **Marking the scope is what made it cheap to kill** -- unmarked it would have reached canon. Mark every absence claim with the scope it was read over.
- **TWO HAND-WRITTEN LITERALS COMPARED TO EACH OTHER OBSERVE NOTHING.** AC-08.5's pin declared four unsettable fields and asserted that list equalled a second list; `put` set all four. **Only a human editing both halves could ever have moved it.** If both terms of an assertion were authored by the same hand from the same model, it is a restatement.
- **SOURCING A DENOMINATOR FROM THE DESIGN IS RIGHT, AND IT IS WHAT DELIVERS THE WRONG ANSWER WHEN THE DESIGN CONTRADICTS ITSELF.** A denominator read out of the implementation agrees with it by construction, so `d57_8_forms()` is taken from D57-8's list -- correctly. **D57-8's list and D57-8's POST clause disagree, so the correct method returns a short denominator and nothing reports it.** vc: the first case today where doing the sourcing CORRECTLY is what produces the error, and **no amount of care reaches it.** The missing check is between a document's own clauses, not between the document and the code.
- **A FINDING AND A HOLD IN ADJACENT BULLETS READ AS CAUSE AND EFFECT, AND NEITHER ONE HAS TO SAY SO.** I listed AC-08.4's hold and the D57-8 contradiction as consecutive items on this board; vc read the link, adopted it, and held a row that was already green. **Neither of us asked which FILE carried the short denominator** -- it was AT-07.1's, not AT-08.4's. **A denominator belongs to a FILE, not to a topic**, and a shared topic is the cheapest false link there is. Name the file whenever you report a coverage defect.
- **A ROW'S TITLE CAN PROMISE MORE THAN ITS BODY, AND NO INSTRUMENT HERE SEES IT** (cc, minted AC-10.15). AC-10.4 reads _Hooks continuity_ and its body scopes `.claude/**` only, so `.githooks/` -- pre-commit, pre-push, post-commit, the whole gate -- **is asserted by nothing in either crate.** Not stale, not uncited, not vacuous: internally consistent and short of its own title, which is the fourth shape of the day's class.
- **AN EMPTY GAP OVER AN UNSTATED DENOMINATOR IS THE VACUOUS GREEN.** State the population IN the assertion, so a reader deciding a row sees the shape of the coverage rather than the shape of the result.
- **A DECLARED LIST STOPS COVERING THE DAY A VARIANT IS ADDED.** My `edit` test declared eleven forms and went on passing over thirteen. **Rust cannot enumerate variants, so make an EXHAUSTIVE match the witness** -- a new variant then fails to COMPILE in the file that must grow a case.
- **AN INSTRUMENT THAT BORROWS A LIVE INSTANCE HAS MADE THE DEFECT A FIXTURE**, and the estate is then not free to fix it (cc + vc). **SYNTHESISE the instance.** `sample_thread`'s `AT-03.1` carries six of eight fields, so a roster checked against it was blind to the two nobody has ever set.
- **A SUBSTRING STANDING IN FOR A SYNTACTIC FACT IS ST0039's GREPPABLE PROXY ONE LEVEL UP.** `flag_reachability`'s markers missed the majority idiom and **10 of its 18 were false positives, including its own headline example.**
- **ASK WHAT THE INSTRUMENT WOULD SAY IF THE THING IT MEASURES WERE GONE**, and **COULD THIS HAVE COME BACK THE OTHER WAY?** A count can be true and uninformative. A pre-written verdict attached to output you have not seen is a conclusion already reached.

### Running things

- **`cargo test --workspace` STOPS AT THE FIRST FAILING TARGET. ALWAYS `--no-fail-fast`**, and **NEVER pipe the log through `tail` before counting** -- I reported 18 targets / 94 tests for a run of 138 / 974, because the count was of what survived the tail. Write the whole log, count after: `awk '/^test result:/ {p+=$4; f+=$6; n++}'`.
- **A CLEAN `git apply --3way` IS NOT A CORRECT REBASE.** Ten files applied cleanly after `organize::plan` changed signature underneath them. **Apply reports on TEXT; only the suite reports on meaning.**
- **KEEP `CARGO_TARGET_DIR` INSIDE THE TREE BEING BUILT** -- `native/rust/target/ic`, gitignored. `INTENT_HOME` walks up from the BINARY's path for `lib/templates/`; a scratchpad target dir leaves the tree and fakes six hook failures.
- **NEVER DRIVE A MUTATOR ON THE LIVE ESTATE.** _A probe is not a test and the estate is not a fixture._ Three breaches, all no-ops **only because the subject happened to be already realised** -- luck presented as method.
- **THE BASH TOOL IS ZSH AND ITS CWD PERSISTS BETWEEN CALLS.** Absolute paths. Unquoted globs in `--include='*.rs'` are a hard error; **unquoted `$var` does NOT word-split** -- `git diff -- $PATHS` passed one path and produced an empty patch. **Never `$?` after a pipe.** `grep -c` exits 1 on zero. **AND THE TWO COMPOUND: THE PIPE TRAP HIDES THE CWD TRAP.** `find <relpath> ... 2>/dev/null | head; echo rc=$?` run from a drifted cwd printed nothing at `rc=0` -- the redirect swallowed _No such file or directory_ and the `rc` was `head`'s. It read as _searched and found nothing_ and I nearly reported my own fold archive as lost; it is on disk, all ten files. **An instrument that cannot say WHERE it looked cannot report an absence.**
- **THE MARKDOWN FORMATTER IS A SECOND WRITER** -- write `_..._` in table prose; `gen_dispatch_table.sh` refuses a render that would not be a fixed point.

### Estate facts worth not re-deriving

- **AN ATTACHMENT IS AUTHORED ON DISK, SO A DIVERGENCE MEANS THE STORE IS STALE** -- `intent sync --to-store <ID>` takes the disk copy, and **`sync --help` says the opposite.** `sync` TAKES IDs; it is thread-scoped, not whole-estate.
- **`intentsvcs` IS THE DEPENDENCY ROOT: a peer mid-edit there stops all three of us.** Announce the BLAST RADIUS, not the files -- and **announce before adding an enum variant**, which reddens every exhaustive match in the crate.
- **THE LIVE CHANNEL IS UNGUARDED.** The clock guard covers board files, not SendMessage. **Use commits when you need ordering you can prove.**
- **A CLAIM CARRIED FORWARD FROM MEMORY IS NOT A MEASUREMENT.** I told dc their inbox was "still doubled" from a `git diff` twenty minutes stale, in the same message where I warned them about `tail`.

## Decisions

**Only the ones that still constrain future work. The rest are archived with the folds.**

- (2026-08-20) **CLAP IS THE WRONG LAYER TO ENFORCE A DECLARED VOCABULARY.** It rejects at exit 2 -- INV-04's USAGE code, the one the pre-commit gate FAILS OPEN on. **Enforce in the renderer at exit 1, reading the set from the table.**
- (2026-08-20) **`intent critic`'s USAGE-ERROR EXIT 2 IS CORRECT AND STAYS IN v3** (ic ruling). _A gate should fail open on its own breakage and closed on yours_ -- a critic that cannot parse its own invocation IS the gate's breakage.
- (2026-08-20) **REPORTERS FAIL OPEN; ACTORS REFUSE** (dc, on `realised_for_action`). The same three manifest states, split by what the CALLER is for. My version had it as one rule with three writers; this is better.
- (2026-08-19) **A REALISED ARTEFACT IS ONE WHOSE COVER VIEW EXISTS, NEVER ONE WHOSE DIRECTORY DOES** (vc).
- (2026-08-19) **FOUR CRITERIA LEFT THE PRECONDITION BLOCK WITHOUT BEING WITHDRAWN** -- AC-03.6, AC-06.3, AC-06.4, AC-07.5. The block is about what GATES, not what is wanted; every one is still owed.
