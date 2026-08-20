---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 16:07Z
status: paused
focus: "**LANDED AT `e7d038c3`, MEASURED ON THE PUBLISHED TREE AT 139 / 982 / 0 WITH ZERO DIRTY.** AC-05.2's payload arm driven (`Fixture::git_init` was the entire blocker); `AddressError::Incomplete` splits too-FEW from too-many; `Entity::Issues` + `Entity::WpCollection` complete D57-8's three POST populations; AC-08.5's two-literal pin is a measurement and STILL RED by its own stated denominator. **Nothing in flight, nothing uncommitted.** Next: `doctor --json`, `declared_but_unwired`'s synthetic member."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. NOTHING UNCOMMITTED.** Board folded 2026-08-20 16:07Z; the pre-fold board and both handled inboxes are verbatim at `.history/20260820/*-1607Z.md`. **A trimmed board is a reading decision, not a record.**

## ON RESUME -- read this first

1. **THE LANDING IS `e7d038c3` AND IT WAS VERIFIED THE ONLY WAY THAT COUNTS**: detached worktree AT the commit, `git status --porcelain` 0 lines, `CARGO_TARGET_DIR` inside it -- **139 targets / 982 passed / 0 failed, clippy `-D warnings` rc=0.** Ten files in ONE commit because `address.rs` and `address_resolution_single_home.rs` each carry two bodies of work and **`--only` is PATH-scoped**; hunk staging is unavailable here.
2. **TWO THINGS ARE WITH vc AND NEITHER IS MINE TO CLOSE.**
   - **AC-05.3 IS A ROW NOBODY MOVED, NOT WORK NOBODY DID.** State `unsatisfied`; its only covering row `AT-05.3` is `n-a` with `file: null`, **so no test can ever move it.** The evidence exists under a different row -- `the_renderer_calls_the_edit_door_exactly_once`, in AT-05.1's file, asserts ONE `.edit(&address` call site on the SOURCE, because Highlander is a claim about how many implementations exist and two that agree today pass every behavioural test. A satisfy decision.
   - **D57-8's URL LIST AND ITS POST CLAUSE DISAGREE, FOUR PARAGRAPHS APART.** The list gives NINE forms, every one an ENTITY, no collections; the clause below requires a collection address per server-assigned population, of which there are three. **The code now carries FOUR collection forms the list does not mention** (`Threads` and `AcCollection` predate me). `d57_8_forms()` -- the denominator `every_d57_8_form_resolves` calls _the WHOLE list_ -- is those nine and **cannot see any of the four.** I changed no denominator to hide it.
3. **A GATE COUNT MIXES THREE KINDS AND ONLY ONE IS WORK**: _not built_, _built and unverified_, _verified and unmoved_. AC-05.3 is the third. Ask which kind before a number drives a day.

## TODO -- LIVE ONLY

1. **`doctor --json`** (cc's ask). `Finding` already derives `Serialize`, so `Report` needing it is the whole model change. **cc's gate arm parses TEXT as a workaround and must be DELETED when the face lands, not kept beside it.** Trap: declare `--json` at BOTH family and verb level.
2. **`declared_but_unwired.rs` gets a SYNTHETIC member** per cc's ruling; the borrowed `st dehydrate` stops being load-bearing.
3. **NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives at all. hv deliberately left it out of ruling 4; vc raises it.

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
- **THE BASH TOOL IS ZSH AND ITS CWD PERSISTS BETWEEN CALLS.** Absolute paths. Unquoted globs in `--include='*.rs'` are a hard error; **unquoted `$var` does NOT word-split** -- `git diff -- $PATHS` passed one path and produced an empty patch. **Never `$?` after a pipe.** `grep -c` exits 1 on zero.
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
