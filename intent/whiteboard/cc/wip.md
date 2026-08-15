---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 14:48Z
status: active
focus: "AC-02.6 BUILT -- WP-02's last blocker. events.jsonl exists, the DDL declares how every table's data leaves, and the enumeration caught the contract's own table count. Close claim with vc."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it** -- a file becomes a well-formed item because the API refused everything it was not, so conformance is structural. **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth. **Re-creation from an extract is a CAPABILITY, not a licence** -- `rm intent.db` costs what the extract does not carry, and today that is the whole event log. **Migrations are NORMAL.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. That is what bidirectional sync is FOR. **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: rolling local backup via `VACUUM INTO`, never a file copy.

**The three ratified machines are in `data-model.md:223-317` and are BUILT.** AC-04.6 is now CONFORMANCE, not closure -- the implemented graph must match the ratified tables exactly, because a closed graph can still be the wrong graph.

## DOING -- nothing claimed; WP-02's blocker is cleared

**AC-02.6 is built and the close claim is with vc** (`b8405e2e`, 283 tests). Next from TODO unless vc rules otherwise.

## LANDED post-compact

- **`b8405e2e`** -- **AC-02.6, openness.** `events.jsonl` BUILT (JSONL because the log is append-only; merged on the ULID because the restore direction must not be destructive for the one table nothing derives). Every table declares its route out IN THE DDL, so the published face says which data can leave. **The enumeration caught the contract**: acceptance.md says eight tables, the DDL has nine. Discriminating case is an assertion over a synthetic DDL, not a hand mutation.

- **`ef62cded`** -- the kind/state clause on the JSON Schema face (vc's cost from the AC collapse). One decision in `AcState::permitted_for`, exhaustive; three enforcement points; roster discovered from the schema's `oneOf`. **Posture change raised to vc, unruled: a mismatched pair now stops the whole estate loading, not just doctor reporting.**
- **`523b34e8`** -- the store schema stamp. dc found it by dogfooding: `CREATE TABLE IF NOT EXISTS` makes the DDL apply a no-op, so open SUCCEEDED on a database it could not read. Stamp-before-DDL in one transaction so a crash repairs rather than bricks. **Version 0 is not schema zero, it is the absence of one** -- unstamped stores are refused with no migration, and the remedy says so instead of promising one.
- **`b786ba65`** -- D37, four shipped strings. **One of them was asserted by a test**, which had been written as the fix to a worse version of the same leak.

## TODO

1. **AC-04.1's `TornRollback` arm** -- independent of everything (vc), goes wherever it fits.
2. **AC-03.10** (DB backup) -- NOT urgent; vc measured the live DB at zero model rows. A precondition of WP-10. **Do not invent the `.backup/` namespace (dc's) or the `intent config` keys (ic's).** `VACUUM INTO`, never a serialiser.
3. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026-0029 DEFAULT-DEFER; check AC-03.6 before touching 0029.

## Waiting

- **vc**: (a) **the ingest posture** -- a kind/state mismatch now refuses the whole estate at load, not just at doctor. Correct under D05, but a posture change is vc's not mine. (b) **Does D37 reach the published schema faces?** ~30 more hits in `thread.schema.json` + `schema.graphql`, lifted from doc comments; `intent schema` prints both. Not all are violations -- "eg `ST0056`" as a value-format example is not our backlog -- so it needs reading every hit, not a count. (c) **Whose is AT-00.8's guard**: mine, ic's or vc's. (d) `data-model.md`'s AC entity + the two `status_reason` fields. (e) `ac descope` now enforces the ratified "target thread exists" guard, which costs the descope-to-a-thread-you-are-about-to-create workflow.
- **THE FIFTH STATE IS PENDING, NOT SETTLED.** vc has `computed` in `data-model.md` and has NOT ratified it -- extending an hv-ratified four-value machine is hv's call. I am now building on it in three more places, so a reversal costs more than it did at 14:07Z. vc owns the escalation.
- **ic**: **seven dispatch rows** (`st triage|hold|resume|reopen|reinstate`, `wp reopen|unstart`). The facade has all seven; the CLI cannot drive the lifecycle past `triage`. **A failing surface in `cli_end_to_end.rs`, not a note.** Plus `--reason` on `st cancel` and the `sync` direction selector.
- **ic**: the `at` guard ruling -- v3 has NONE of v2's four `at` set-time guards; the gate recovers two, and _green-only-from-red cannot be recovered at gate time ever_ because it is a property of history.
- **dc**: nothing owed. The migration ladder (their #2) is mine and unbuilt; **it can only start at version 1, so every pre-stamp store is permanently unreachable by it.** The stamp buys the future, not the past.

## Lane boundary -- PROPOSED (vc), not ruled

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/` is the one collision**: `bin/intent*` is cc's, `bin/int` + `bin/.devbin/**` are dc's. dc accepted and corrected the argument: the load-bearing half is not ownership but **the FREEZE** -- `bin/intent*` is the baseline ic's burn figures are measured from, so if it moves they measure a moving target and silently stop meaning what they say.

## Standing rulings

- **`treeindex` and handover RETIRE** -- the db obviates both (D30/WP-14). **A retired command is PRESENT AND REFUSING, not absent** (my ruling): no functionality, only an explanation, per AC-04.4. **It breaks a guard and the fix ships with the feature**: `dispatch::is_shipped()` excludes `retire` rows, so a retired-but-refusing command is absent from `shipped_entries()` and present in the spine, and `dispatch_ssot.rs` asserts both directions. **`fileindex` is NOT covered** -- different mechanism; its `pending-hv` INV-07 question stands.
- **`EdgeKind::Incidental` STAYS despite having no user**, reversing this board's earlier "delete it unless a non-AC user appears". `Edge::exits` is `leaves() && kind == Direct`, so deleting it collapses `exits` into `leaves` and the trap check silently starts accepting technicality exits again for whatever field-crossing verb arrives next.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

Anything amounting to "remember to" is archived; it failed twice on entries this board already carried. These are facts about the estate.

- **A REPORT OF N SITES IS A SAMPLE UNTIL SOMEONE COUNTS.** I fixed sixteen false-canon sites, reported the class closed, and left eleven. **Grep the phrase FAMILY, then READ every hit instead of counting.**
- **A GREP FINDS THE SPELLING YOU ASKED FOR, AND A CLAIM HAS MORE THAN ONE.** "the D01 **disposability** invariant" survived two correction passes against greps for "**disposable**".
- **A GREP HIT IS A LINE; THE FALSEHOOD MAY BE THE PARAGRAPH.** `store.rs:3` matched and read correct while line 1 -- matching nothing -- still asserted the old model, because a previous repair spliced a clause into the middle of the old sentence. **When correcting canon, REWRITE THE PARAGRAPH rather than inserting a clause.**
- **A HAND-KEPT ROSTER INSIDE AN INSTRUMENT IS THE SAME DEFECT THE INSTRUMENT LOOKS FOR.** The schema walk read tag names from `["state","status"]`; renaming a tag to `is` silently stopped it classifying a field, and its own completeness check then reported the field ABSENT FROM THE SCHEMA. **Discover structurally; never enumerate names by hand.**
- **A COLLAPSE MAKES THE NEW REPRESENTATION OBVIOUS AND THE OLD INVARIANT INVISIBLE.** Rewriting `resolve()` the natural way would have let a hand-authored `satisfied` on a test-backed AC satisfy the gate. **Re-derive what the old shape enforced; do not assume it survived.**
- **A DOC CAN OUTLIVE ITS OWN SUBJECT, not just its model.** `facade.rs` said "THERE IS NO DB -> DISK SYNC YET" -- true when written, false the same day by my own work. Alongside the three remedies that named a command after the reasoning behind it had moved.
- **`git checkout -- <path>` REVERTS TO HEAD, NOT TO BEFORE YOUR MUTATION.** Used it to undo a mutation test and it silently took an hour of uncommitted work in the same file with it. **Twice.** Back up with `cp` before mutating; restore with `cp`.
- **A `.jsonl` FILE ESCAPES A `.json` CHECK BY SUFFIX LENGTH.** `"events.jsonl".ends_with(".json")` is false. Correct today, by accident -- the same shape D29 named for the DB file. **Any new extension needs a decision, not a coincidence.**
- **AN ERROR SWALLOWED IN A FIXTURE IS A SILENT ERROR.** `.ok()` on three fixture mutations made four tests fail on a row-count precondition instead of on the refusal that caused it; two verbs were illegal from the fixture's own states. **`expect()` in fixtures, always.**
- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face and async-graphql into the SDL, and `intent schema` prints both to a consumer's terminal. I put an AC id, a node name, a date and a test path into two published faces while closing a different hole -- in the one file that already carries this warning, three fields down. **Plain `//` for reasoning; `///` only for what a stranger needs.**
- **A GREP OVER COMMENTS FINDS THE WRONG HALF.** The D37 sites that mattered were in string LITERALS. Grep `"[^"]*PATTERN[^"]*"` and exclude comment lines, or the four shipped strings hide behind fifty harmless mentions.
- **A TEST CAN ASSERT THE DEFECT, and it looks like diligence.** `an_unbuilt_command_names_the_work_package_that_owes_it` pinned a D37 leak in place -- written as the fix to a WORSE version of the same leak, which is why it read as careful. **When a ruling lands, grep the tests for what now asserts the old behaviour.**
- **A COMMENT CAN DOCUMENT THE BUG AS A FEATURE.** `store.rs:181` said "Reopening an existing DB is a no-op apply (IF NOT EXISTS)" -- accurate, and describing the hole. **An accurate comment is not evidence of correct behaviour.**
- **`IF NOT EXISTS` MAKES A SCHEMA CHANGE INVISIBLE UNTIL A QUERY FAILS.** Open succeeds on a database it cannot read. Any DDL change needs the stamp bumped and `store_schema_version.rs` re-pinned in the same commit; the test names the value.
- **VERSION 0 IS NEVER "SCHEMA ZERO".** SQLite defaults `user_version` to 0, so 0 permanently means unstamped. Arithmetic like `SCHEMA_VERSION - 1` to get "an older version" is wrong at version 1 and silently right-looking.
- **`--only` commits what you NAME, and a move is TWO facts.** `a1a949c` committed 58 additions and left 55 deletions staged, on both remotes, where a fresh clone would have built the OLD tree. **Every working-tree check passed.** Name the deletion side too, verify at HEAD (`git ls-tree`), never on disk. After any move, clone fresh and build.
- **`--only` NEVER CLEARS THE INDEX** (issue 0028): a linter-on-save rewrite after `git add` leaves a third version staged forever, invisible to `git diff HEAD`. `git reset -- <your paths>` clears it without touching peers.
- **TWO symlinks point INTO this repo**: `which -a intent` returns three reachable copies. Mutating `bin/intent*` in place changes the tool every live session runs -- sacrificial `git worktree` only.
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **Cargo runs from `native/rust`.** A build cache can be stale in a way its own freshness check cannot see -- passes in isolation, fails in the suite. That is a conclusion, not flakiness.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
