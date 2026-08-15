---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 18:15Z
status: active
focus: "AC-02.8, AC-06.10(a)+(c), three of ic's four spine parity breaks, and AC-03.10(d)'s backup registry all landed and pushed. 322 tests. AC-02.8 with vc; AC-06.10(b) with ic. NEXT: the backup module + retention, to close gate 03 at 10/10."
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

## DOING -- AC-03.10 (c) + the rest of (d): closes gate 03 at 10/10

1. **The `backup` module**: `take()` -> `VACUUM INTO intent/.backup/db/<db-stamp>.db`, recording the attempt through `begin_snapshot`/`finish_snapshot`. Its own namespace so it can never collide with `intent upgrade`'s `backup-<TIMESTAMP>/` -- **two mechanisms in one directory with different retention rules, where pruning the wrong one is the loss this AC exists to prevent.**
2. **(c) rolling retention** day/week/month with configurable counts from `intent/.config/config.json`. **Bucket in SQL** (`strftime` over `taken_at`), so the retention decision stays where the stamps are. Two settings are ruled NOT configurable: the snapshot directory, and any key that silences a backup failure.
3. **`doctor` reports staleness** -- `hours_since_last_good_snapshot()` against the configured schedule. The store half is built and tested.
4. **`intent backup --list`** -- the row exists and `--list` is ratified. It answers WHAT EXISTS and is deliberately NOT the health report; one place reports health and it is `doctor`.

## TODO

1. **AC-06.8 -- two live violations ic measured**: `doctor --quiet` and `--verbose` are declared and structurally unreadable (`fn doctor()` takes no `ArgMatches`; `run` dispatches `Some(("doctor", _))`). **44 more declared-and-unread flags** sit on unwired commands and become violations one at a time as each is wired -- the worst arrival schedule for a defect nobody watches for. ic raised the flag-disposition mechanism as EXP-05; **the spine change is mine when it lands.**
2. **AT-00.8 -- the D37 guard is MINE.** The hard part is REFERENT, not regex.
3. **D37 in the published faces** -- vc is doing the read. Two I found and did NOT fix, to avoid half a sweep: `event.rs` `Subject.id` doc (`eg ST0056`), and `FindingClass`'s own doc ("the two WP-03 adds").
4. **AC-03.10 (c)+(d)** -- retention + `doctor` staleness. (a)+(b) are done and green.
5. **AC-06.6 export**, then **AC-06.1 surface tail**. **AC-04.1's `TornRollback` arm.**
6. **The EXP-05 disposition half is WRITTEN AND NOT COMMITTED.** Honouring it withdraws `sync --to-store`, which is built and covered by two tests, and its `pending` value has a stated reason -- the `sync`/`ingest` boundary is undeclared. **Landing it would answer ic's open question by making one answer true in the binary.** With ic and vc; my recommendation is that `sync` owns both directions and `ingest` retires.

## Waiting

- **vc**: nothing blocking. WP-02 reopened to 7/8 by their own audit; WP-03 was 9/10 before AC-02.8.
- **ic**: nothing owed either way. `wp rescope` has no dispatch row (facade has the method) -- reported as an observation, may be deliberate.
- **dc**: FYI only. `int macos verify` is the cheap release-state check; the shared `target/` hazard is structurally absorbed.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread** -- three consumers in ic's `gen_dispatch_table.sh`.
- **`doctor --fix` is WITHDRAWN, not deferred** (hv). A diagnostic that NAMES the remedy beats one that performs it. Nothing to remove in `render.rs` -- it was declared and never read; ic removes the row.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

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
