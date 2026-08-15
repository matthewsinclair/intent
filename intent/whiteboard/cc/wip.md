---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 16:49Z
status: active
focus: "AC-02.8 planned and two rulings put to vc: created_at cannot be BOTH a fact about the DB and the replacement for threads.created, and with every write a DELETE+INSERT an updated_at trigger never fires while created_at silently means updated_at. Unblocked half starts now -- delete Store::now/today, fill created/completed from what the write RETURNED."
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

## DOING -- AC-02.8, and it is ONE indivisible unit

**hv work instruction: "lance all five and widen the guard." Four are done; these are the rest.**

1. **DELETE `Store::now()` and `Store::today()`.** Deleting is the point, not a side effect -- while they exist someone calls them, and a function that hands out a time IS the confection regardless of where it got the value. They also fail hv's surface rule twice over: they RETURN a time that never went through a record.
2. **`created_at`/`updated_at` on ALL EIGHT tables**, written by the DB (DEFAULT or trigger). vc's audit: **zero of eight** have one today. Three columns look like the answer and none is -- `threads.created`/`issues.created` are authored dates, `file_index.mtime` is the FILE's, `event_log.ts` was an argument.
3. **`threads.created`/`completed` are REPLACED by them, not supplemented** -- two fields claiming to say when a thread was created is how they come to disagree. Both are tool-derived: created = when the record was written, completed = when the update that set the status ran.
4. **`issues.created` is the ONE exception and STAYS** -- v2 users author it by hand, so it is a fact about the world, with a DB stamp beside it.
5. **Bumps `SCHEMA_VERSION` to 3 + re-pin the hash + a migration rung**, in the same commit. Existing rows carry their stamps through; re-stamping at migration is the violation, not the fix.
6. **AT-02.8's discriminating case**: the column is populated whether the DB or a caller filled it, so reading it back proves nothing. **Insert through the facade with NO time available to the caller at all**; assert non-null and ordered, and that two sequential writes are non-decreasing -- the property a read-then-write gap cannot give.

**Reopens WP-02 to 7/8**, knowingly, under "file a defect under its own noun even when that reopens a closed WP".

**Then build hv's surface guard**: no `intentsvcs`/CLI function TAKES a time. The one legitimate seam is `restore_event(&Envelope)`, which takes a RECORD carrying a DB-set stamp through the extract -- make that explicit rather than implied.

## TODO

1. **AC-06.8 -- two live violations ic measured**: `doctor --quiet` and `--verbose` are declared and structurally unreadable (`fn doctor()` takes no `ArgMatches`; `run` dispatches `Some(("doctor", _))`). **44 more declared-and-unread flags** sit on unwired commands and become violations one at a time as each is wired -- the worst arrival schedule for a defect nobody watches for. ic raised the flag-disposition mechanism as EXP-05; **the spine change is mine when it lands.**
2. **AC-06.10 / D41** -- two-part face versions `INTENT_VER` / `SCHEMA_<TYPE>_VER`, **constants in code, injected by the generator**. AT asserts against the face AS PUBLISHED, never the constant -- the failure guarded is a generator that stops injecting.
3. **AT-00.8 -- the D37 guard is MINE.** The hard part is REFERENT, not regex.
4. **D37 in the published faces** -- vc is doing the read. Two I found and did NOT fix, to avoid half a sweep: `event.rs` `Subject.id` doc (`eg ST0056`), and `FindingClass`'s own doc ("the two WP-03 adds").
5. **AC-03.10 (c)+(d)** -- retention + `doctor` staleness. (a)+(b) are done and green.
6. **AC-06.6 export**, then **AC-06.1 surface tail**. **AC-04.1's `TornRollback` arm.**

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
