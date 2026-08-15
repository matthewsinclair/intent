---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 19:30Z
status: active
focus: "AT-00.8 built (four surfaces, faces 37 ids -> 0) and AC-06.8/EXP-05 landed -- ic's parity check is at ZERO findings, from 21 this morning. 340 tests, clippy and fmt clean, both remotes at eb2e4dde. NEXT: AC-06.10(b) reader (ic's --versions row is in the binary), then the empty-category remedy on nine verbless leaves."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **Migrations are NORMAL, and the ladder now exists.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6). **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: snapshot = same-schema rollback ONLY; the recovery path for an outdated store is the EXTRACT. **D36**: `rm intent.db` is not an operation. **D37**: our ST/WP/AC ids never reach Intent's output, including the published schema faces.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

**You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either**. Asking SQLite and then writing the answer is still writing a time you obtained: the read and the write are two acts with a gap, and a write retried or deferred inside it is stamped when it was PREPARED. **The record is stamped BY the write.**

**hv's sharpening, verbatim and the most testable form of the rule:** _"intent3 won't have any cli or intentsvcs functions that TAKE a time. There will be cli and intentsvcs functions that RETURN times, but those will have gone end-to-end thru the db where the time was SET BY SQLite, not confected in an LLM hallucination."_ **That is a property of the API surface, not of the call sites** -- and it is a sharper guard than `one_clock.rs`, which bans `::now` and would not catch a time-shaped parameter.

**Four things that are NOT exceptions**, each already used by one of us to reintroduce a clock: a test fixture; "I'm only reading it"; "the value came FROM the database"; "it's just a board label".

**Creating vs restoring is the split that makes it workable**: create -> the DB stamps, no caller supplies anything; restore -> the recorded stamp is carried. **Re-stamping on restore or migration destroys history and every stamp still looks valid.**

**Why load-bearing**: under D34 two machines MERGE event logs. A merge needs a time nobody could have typed.

## DONE TODAY -- AC-02.8 whole, three commits, with vc

`04c6813a` schema, `075ebb13` the clock, `c2ba44fd` the signature guard. 314 tests, clippy `-D warnings` and fmt clean, both remotes. **Q1 and Q2 both ruled my way by vc and built as ruled.**

- **Record timestamps on every table**, DB-written via DEFAULT. `created_at`/`updated_at` + upsert on `threads`/`issues`/`file_index`; **`written_at`** on `related`/`wps`/`criteria`/`tests`, because a row deleted and re-inserted with its parent can only honestly record when THIS VERSION was written. `event_log.ts` IS its record timestamp and the DDL says so. `SCHEMA_VERSION` 3, rung 2->3 rebuilding eight tables, FKs off per SQLite's recipe and re-checked inside each rung's transaction.
- **`Store::now`/`today` DELETED.** `st_new` hands in an empty `created`; the store fills it inside the INSERT and RETURNS it. `write_thread` gained the same two doors `write_event` has. **`apply()` now writes the DB first and renders files from what landed** -- the projection used to be computed before the write, which was harmless only while the application knew the dates.
- **hv's signature form enforced**: no shipped function TAKES a time. Name AND type, so `stamp: Stamp` (which door) survives while `today: String` does not.

## DONE TODAY -- second half

`28fd5721` **AC-06.10 (a)+(c)** -- `INTENT_VER` + a per-type `SCHEMA_<TYPE>_VER` in all five faces, each in its own idiom, constants injected by the generator. AT reads the PUBLISHED files; mutation-tested by dropping the SDL injection and re-blessing, which `schema_faces_drift` PASSED -- only a test that opens the artefact sees a generator that stopped injecting. A pinned per-type contract hash stops a version sitting at 1 forever. **(b) needs one flag row on `schema` and is with ic; the reader lands with the row, never before it.**

`9122f4e5` **three of ic's four spine parity breaks**, all measured by their `surface_check.sh` and none findable by reading. **ARITY 8 of 8** -- `subcommand_required` hardcoded `true` against the declared slot arity; v2 exits 0 on `intent todo`, v3 exited 1. **FAMILY FLAGS** -- `with_args` ran only on the verbless branch, so `todo`'s own `--json` reached every leaf and never `todo`. **SHORT-ONLY FLAGS** -- a bare `continue` dropped three declared `keep` flags with no diagnostic. **Their check goes 21 findings -> 7.**

`cff33c77` **`event_log.ts`'s shape is PUBLISHED** -- format + pattern, so the millisecond move is visible in the contract. The version guard then fired unprompted and only on the JSON contract: `SCHEMA_JSON_VER` -> 2.

`70f1fc52` **AC-03.10(d), first half** -- the backup log is a TABLE recording ATTEMPTS, not a directory listing, so a schedule that never ran is distinguishable from one that fails. Row written before the copy; the snapshot filename comes from the stamp the INSERT returns; staleness is `julianday('now')` inside SQLite returning an INTERVAL, so **no clock is needed and none was added**. `SCHEMA_VERSION` 4, `SCHEMA_DDL_VER` 2.

## DONE -- the evening, and both gates closed by vc

**AT-00.8 / AC-00.9** (`26dacf1f`) -- the D37 guard, FOUR surfaces. **The faces are the biggest carrier and were in nobody's method**: `intent schema <face>` prints a face verbatim, so a `///` on a modelled type is emitted output. 37 identifiers across four faces -> 0; reasoning moved to `//`, nothing deleted. `owed_by` REMOVED from the model (a library another project links had no business naming our WPs); `owner_wp` guarded for a READER instead of for content. Four mutations, each dying at exactly one test.

**Contract-hash fix** (`c001b639`) -- documentation is not contract. My doc edits moved all three face-version hashes and demanded three bumps. Re-pin measured, not taken: zero contract lines changed across all five faces, verified in a sacrificial worktree.

**AC-06.8 / EXP-05** (`b8491e56`) -- `Flag.disposition` + `ships()`, honoured in the spine. **ic's `surface_check.sh` is at ZERO findings** (21 this morning, 6 at 19:05Z). `pending` sits with `retire`: an undecided flag that ships commits the surface by fait accompli.

**vc closed WP-02 at 8/8 and gate 03 at 10/10**, by running the evidence rather than reading my claim.

## DOING -- nothing; picking up next from TODO

## TODO

1. **AC-06.10(b) -- UNBLOCKED.** ic's `schema --versions` row is authored and already in the binary; the reader is mine and lands with the row rather than before it. `--versions` selects OUTPUT MODE, `face` selects WHICH -- they compose, no arm special-cases the other.
2. **The empty-category remedy** (ic, measured): `remedy: run X --help for the verbs that are` on **nine leaves with zero verbs** -- `info`, `init`, `bootstrap`, `learn`, `fileindex`, `version`, `export`, `ingest`, `mcp`. Promises a category that is empty. Needs a leaf variant of the message.
3. **AC-06.6 export**, then **AC-06.1 surface tail**. **AC-04.1's `TornRollback` arm.**
4. **`intent ingest` has no source argument** (ic, measured). I ruled the shape: `path` at arity `0..1`, recovery DEFAULTS to this project's tree. ic writes the row, I wire it.
5. **D-numbers in prose are not machine-enforceable** -- the faces are swept and guarded; help text and remedies are review-only. With vc; the alternative is a hand-kept roster inside the guard, which is the defect the guard class exists to find.

## Waiting

- **vc**: the D-number exception (measured counter-example in shipped help text). Nothing blocking.
- **ic**: the `ingest` path row when they want it. Boundary ruled AGAINST my recommendation on five measured axes -- `ingest` takes foreign md with a per-file error contract, `sync` moves our own extract with a round-trip guarantee. My argument read the shared gate as the identity; `st new` uses that gate too.
- **dc**: `int prepush` reported "no native/ change" on a push carrying 900 lines of it. FYI, their call.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread** -- three consumers in ic's `gen_dispatch_table.sh`.
- **`doctor --fix` is WITHDRAWN, not deferred** (hv). A diagnostic that NAMES the remedy beats one that performs it. Nothing to remove in `render.rs` -- it was declared and never read; ic removes the row.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

- **A LESSON WRITTEN DOWN IS NOT A LESSON APPLIED, AND THE SECOND INSTANCE CAN BE HOURS LATER IN A SIBLING GUARD.** `store_schema_version.rs` hashes the DDL with comments STRIPPED, and says why in its own doc: a guard that cries wolf on a comment gets re-pinned without reading. I wrote that, then built the face-version hash the same afternoon covering every byte -- so rewording a comment demanded three contract-version bumps. **The fix does not generalise itself; grep the siblings when you write the note.**
- **A REFERENT TRAP CAN LIVE IN A CLASS WITH NO BLESSABLE VALUE.** `ST0000` vs `ST0056` is soluble because one id is universal. `D2-D11` (STZero deliverables, the reader's) vs `D15` (a design decision, ours) is NOT: same shape, no carve-out. **When a class cannot be decided, decide the SURFACE instead** -- counted in the faces where the ambiguity cannot arise, review-only in prose. A guard that fires on correct help text gets switched off.
- **A FIX CAN BE INVISIBLE TO THE ENTIRE SUITE WHILE AN EXTERNAL SCRIPT CATCHES IT.** Removing the flag-disposition skip left all 339 Rust tests green; only ic's `surface_check.sh` noticed, and it is not in CI. **A property whose sole witness lives outside CI regresses on the next refactor.** Mutation-test asks "does anything catch this", and "something outside the suite" is the wrong answer.
- **`intent schema` PRINTS THE FACES, so the `///`-is-published rule reaches further than the repo.** It is not that a doc comment ends up in a committed artefact -- it ends up on a stranger's terminal. The comment exemption in a criterion about OUTPUT does not cover a comment a generator publishes.
- **A PEER PUSHING BETWEEN YOUR TWO PUSHES CAN EMPTY A HOOK'S RANGE.** `@{upstream}` names ONE remote; a hook computing `@{upstream}...HEAD` answers "unpushed to the tracked remote", not "in this push". With two remotes those diverge silently and the gate opens.
- **A GUARD CAN BE COARSER THAN THE DEFECT IT NAMES, AND THEN IT PASSES ON IT.** My `created_at` guard survived reverting `threads` to delete-and-reinsert, because both writes landed inside one second and second-granularity stamps compared equal. **The fix was the STAMP, not the assertion** -- the same collision is load-bearing in the product, since D34's cross-machine merge orders by exactly this value. Found by mutation test; unreachable by reading.
- **"DOES THIS THING HAVE ONE" IS THE WRONG QUESTION WHEN IT HAS SEVERAL.** The completeness check asked whether a table had A stamp with a DEFAULT, so stripping `file_index.created_at`'s left `updated_at` to answer for it and the check passed. **Report per column, never per container.**
- **A DEFECT THAT SURFACES SOMEWHERE ELSE IS A DEFECT THIS GUARD DOES NOT COVER.** That same mutation DID break the build -- through three unrelated snapshot tests hitting a NOT NULL violation. A loud failure elsewhere is not coverage here, and it reads like coverage.
- **A MIGRATION FIXTURE MUST BE A STORE THAT COULD ACTUALLY HAVE EXISTED.** The v1 fixture laid down `event_log` ALONE; it passed rung 2 and then met rung 3, which rebuilds seven tables it had never created. **The ladder growing a second rung exposed it, not any reasoning about the fixture.**
- **SQLite REFUSES `ADD COLUMN` FOR A NOT NULL COLUMN WITH A NON-CONSTANT DEFAULT.** Any DB-stamped column means a table REBUILD, not an ALTER. And **`PRAGMA foreign_keys` inside a transaction is a silent no-op** -- set it outside, or the guard looks applied and does nothing.
- **zsh EATS A BACKTICKED FRAGMENT IN A `-m` COMMIT MESSAGE.** `SELECT strftime(` vanished from a commit body and left "the needle set gained :". **Use `-F -` with a heredoc for any message containing backticks or parens.** `--only` protects a commit and NOT an `--amend`, so check `git diff --cached` is EMPTY before amending to repair one.
- **BETTER PROVENANCE IS NOT THE ABSENCE OF A CONFECTION.** I built `Store::now()` to collapse three process clocks, and it was the same defect one layer up. **Three of us landed on "one well-sourced clock" independently, which means the wrong shape is the intuitive one** -- so the enforcement must be structural, never a rule to remember.
- **A SUFFICIENT-LOOKING FIELD ANSWERS A NARROWER QUESTION THAN THE ONE BEING ASKED.** Eight tables shipped with no record timestamp because three columns look like one. Fourth instance of this class in a day.
- **A SWEEP DOES NOT MOVE A FILE, IT SPLITS A CHANGE.** ic's unqualified `--amend` took my test file without the `store.rs` methods it called; HEAD did not build and every file in it looked finished. **After a sweep the check is "does it still build", not "whose file is this".** `--only` protects a commit and NOT an amend.
- **STAGE NOTHING UNTIL THE MOMENT YOU COMMIT.** Staging early to get past a block is what made my file sweepable.
- **`MM` WITH A CLEAN `git diff HEAD` IS A STALE INDEX ENTRY** (issue 0028) -- invisible to the diff, visible only as the left column of `git status --short`. `git reset -- <path>` clears it and touches no byte of the worktree.
- **A DISCARDED `ArgMatches` DROPS EVERY FLAG SILENTLY.** `Some(("doctor", _))` -- clap accepts a declared flag whether or not anything reads it, so `--help` advertises what the renderer denies.
- **A CENSUS BY WHOLE-FILE GREP UNDER-REPORTS.** Missed `st new -s` because its long spelling is `start`. **Check per-ARM.**
- **A FIXTURE CAN PUT A TEST IN THE WRONG WORLD.** `no_match_is_exit_zero_and_silent` used a bare `st new`, so its index was empty: it proved "never searched anything" while claiming "searched and found nothing", and passed either way.
- **A LANE BOUNDARY YOU ASSERT CAN BE BACKWARDS** -- worse than a stale premise, because it moves your work onto someone else's list. **Before naming a peer's block, run the query that proves it.**
- **A TEST THAT ASSERTS A REFUSAL CANNOT TELL WHICH REFUSAL.** `unwired` refuses too.
- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face and async-graphql into the SDL. **Plain `//` for reasoning.**
- **A TEST CAN ASSERT THE DEFECT, and it looks like diligence.** When a ruling lands, grep the tests for what now asserts the old behaviour.
- **A HAND-KEPT ROSTER INSIDE AN INSTRUMENT IS THE DEFECT THE INSTRUMENT LOOKS FOR.** Discover structurally; never enumerate by hand.
- **`git checkout -- <path>` REVERTS TO HEAD, NOT TO BEFORE YOUR MUTATION.** Back up with `cp`, restore with `cp`.
- **AN ERROR SWALLOWED IN A FIXTURE IS A SILENT ERROR.** `expect()` in fixtures, always.
- **`IF NOT EXISTS` MAKES A SCHEMA CHANGE INVISIBLE UNTIL A QUERY FAILS.** Any DDL change bumps `SCHEMA_VERSION`, re-pins `store_schema_version.rs`, and writes the migration rung -- in the same commit. **The guard earned its existence within hours: it forced the first rung on the first schema change after it was built.**
- **VERSION 0 IS NEVER "SCHEMA ZERO".** SQLite defaults `user_version` to 0, so 0 permanently means unstamped.
- **SCHEMARS RENDERS A DOCUMENTED ENUM AS `oneOf` OF `const`s AND AN UNDOCUMENTED ONE AS A FLAT `enum`.** A reader that knows one shape returns an EMPTY roster the day someone edits a comment, and every test over it passes vacuously.
- **`--only` COMMITS WHAT YOU NAME, and a move is TWO facts.** Verify at HEAD (`git ls-tree`), never on disk.
- **TWO symlinks point INTO this repo.** Sacrificial `git worktree` only for `bin/intent*`. **`git stash` is unsafe here.**
- **Cargo runs from `native/rust`.** `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift` is what re-blesses the faces -- not a workspace-wide bless.
- **Two remotes; a peer can push between your two pushes.** Verify both with `git ls-remote`.
