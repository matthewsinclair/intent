---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 15:29Z
status: active
focus: "Seven verbs wired, AC-03.9 selector built (D36 cleanup taken), st new -s composes, AC-06.4 + AC-06.7 guarded. WP-02 is 7/7. Next: AC-03.10 backup, then the declared-flag guard."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **Migrations are NORMAL.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35** (as vc sharpened it): snapshot = same-schema rollback ONLY; the recovery path for an outdated store is the EXTRACT, never a snapshot -- a snapshot from before a schema change restores the schema you were escaping. **D36**: `rm intent.db` is not an operation, including as a test-fixture idiom. **D37**: our ST/WP/AC ids never reach Intent's output, and that includes the published schema faces (`intent schema` prints them).

## DONE this session -- five commits, `01079fd5` both remotes, 288 pass

- **Seven verbs wired** (`546c06ef`). The rows were ic's and shipped; the wiring was mine. Mutation-proved: deleting an arm fails the drive on `not implemented yet`.
- **AC-03.9 selector built** (`d7f3afdb`) -- `--to-disk` / `--to-store`, both D36 sites retired. **No store-deletion site remains.**
- **`st new -s` composes** (`b0641c8b`) -- test reads the EVENT LOG, per vc's discriminating case.
- **AT-02.7 marker** (`9df18b10`) -- **WP-02 gates 7/7 PASS.**
- **AC-06.4 + AC-06.7 guarded** (`01079fd5`) -- empty index no longer answers like a miss; WP-body search tested.

## DOING -- next up

1. **AC-03.10** (DB backup). `VACUUM INTO`, never a serialiser. **Do not invent the `.backup/` namespace (dc's) or the `intent config` keys (ic's).** D35 as sharpened: do NOT describe the snapshot as the recovery path for a corrupt or outdated store.
2. **The declared-flag guard.** Nothing mechanically links a table-declared flag to a renderer that reads it, so the next silent drop looks identical to a working flag from the help text. **Asked vc whether it is theirs to contract or mine to build.** A whole-file grep UNDER-REPORTS -- must be per-arm.

## TODO

1. **AT-00.8 -- the D37 guard is MINE** (vc ruled; ic owns the dispatch table as an INPUT, dc gets the pre-commit hook later). **The hard part is referent, not regex**: an Intent WP id in `owner_wp` is RED, `ST0000` in help text is GREEN because it names a thing in the reader's own project. A regex over `ST0\d{3}` passes neither honestly.
2. **D37 in the published faces, ~30 hits** -- vc RULED they are in scope and is doing the read themselves. Await their list. **One I walked past and did NOT fix, deliberately, to avoid half a two-ended sweep: `event.rs:60`, `/// Natural id, eg ``ST0056``, ``ST0056/02``, ``0021``` on a `JsonSchema` type.** Pattern already set: keep the description, neutralise to `ST0001`.
3. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026-0029 DEFAULT-DEFER; check AC-03.6 before touching 0029.
4. **AC-04.1's `TornRollback` arm** -- independent of everything.

## Waiting

- **vc**: (a) whether `search`'s empty-index remedy may name `sync --to-store` -- **I decided it rather than stalling** and asked them to check. My reading: AC-03.9 forbids a remedy sending an operator to a RESTORE **to recover from a failure**, and prose is disk-native (D02), so disk -> db is not a recovery path for it but its only path. (b) **`doctor --fix` is theirs** -- a flag whose name promises mutation, declared and unread; what it may touch is a contract question and I am not building it. (c) AC-03.9's own text carries a stale measurement (says db-to-disk "does not exist at all"; `sync_to_disk` exists). (d) `data-model.md`'s AC entity + the two `status_reason` fields.
- **THE FIFTH STATE IS WITH hv, NOT SETTLED.** vc reversed their own ruling in its favour on the record and says keep building; if hv rules against it the cost is one enum value and two edges. **Do not stall on it.**
- **ic**: nothing owed either way. Told them about `wp rescope` (in the facade, no dispatch row) as an observation with the query, not an ask -- the omission may be deliberate.
- **dc**: FYI only, no action. A `cargo build --release` ad-hoc signs the binary and silently de-notarises it; `codesign --verify --strict` returns 0 on it. `int macos verify` is the cheap check.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- which includes wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- it is the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE** -- the db obviates both (D30/WP-14). **A retired command is PRESENT AND REFUSING, not absent**: `dispatch::is_shipped()` excludes `retire` rows, so it is absent from `shipped_entries()` and present in the spine. **`fileindex` is NOT covered** -- its `pending-hv` INV-07 question stands.
- **`EdgeKind::Incidental` STAYS despite having no user.** `Edge::exits` is `leaves() && kind == Direct`, so deleting it collapses `exits` into `leaves` and the trap check silently starts accepting technicality exits again.
- **`owner_wp` stays carried and unread** in the dispatch deserialiser -- it has three consumers in ic's `gen_dispatch_table.sh`.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

- **A DISCARDED `ArgMatches` DROPS EVERY FLAG ON THE COMMAND, SILENTLY.** `Some(("sync", _))` and `Some(("doctor", _))` threw the matches away, so six declared flags could not be read even in principle -- and clap accepts a declared flag whether or not anything reads it, so `--help` advertises what the renderer denies. **Six found; five were real.** No mechanism links the two.
- **A CENSUS BY WHOLE-FILE GREP UNDER-REPORTS.** My first flag census missed `st new -s` because its long spelling is `start`, which is everywhere in the renderer as a verb name. **Check per-ARM, never per-file.**
- **A FIXTURE CAN PUT THE TEST IN THE WRONG WORLD.** `no_match_is_exit_zero_and_silent` used a bare `st new`, so its index was empty: it believed it was proving "searched and found nothing" and was exercising "never searched anything" -- the exact two cases its own criterion exists to separate, and it passed either way.
- **A LANE BOUNDARY YOU ASSERT CAN BE BACKWARDS, and that is worse than a stale premise** -- it moves your work onto someone else's list, where it sits undone. I reported the seven verbs as owed by ic twice, in writing. **Before naming a peer's block, run the query that proves it.** ic's form: _"verify this rather than take my word"_.
- **A TEST THAT ASSERTS A REFUSAL CANNOT TELL WHICH REFUSAL.** The lifecycle test passed identically whether the verbs were wired or not, because `unwired` also refuses. **A test written to make an ask concrete made the ask invisible.**
- **A REPORT OF N SITES IS A SAMPLE UNTIL SOMEONE COUNTS.** Grep the phrase FAMILY, then READ every hit instead of counting.
- **A GREP FINDS THE SPELLING YOU ASKED FOR, AND A CLAIM HAS MORE THAN ONE.** "disposability" survived two passes against greps for "disposable".
- **A GREP OVER COMMENTS FINDS THE WRONG HALF.** The D37 sites that mattered were in string LITERALS. Grep `"[^"]*PATTERN[^"]*"` and exclude comment lines.
- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face and async-graphql into the SDL; `intent schema` prints both. I put an AC id, a node name, a date and a test path into two published faces while closing a different hole -- in the file that carries this warning three fields down. **Plain `//` for reasoning.**
- **A TEST CAN ASSERT THE DEFECT, and it looks like diligence.** `an_unbuilt_command_names_the_work_package_that_owes_it` pinned a D37 leak, having been written as the fix to a WORSE version of it. **When a ruling lands, grep the tests for what now asserts the old behaviour.**
- **A COMMENT CAN DOCUMENT THE BUG AS A FEATURE.** `store.rs:181` said "Reopening an existing DB is a no-op apply (IF NOT EXISTS)" -- accurate, and describing the hole.
- **A HAND-KEPT ROSTER INSIDE AN INSTRUMENT IS THE DEFECT THE INSTRUMENT LOOKS FOR.** Three instances now, the last in the CONTRACT: AC-02.6 said eight tables and the DDL has nine, inside the criterion that forbids hand-maintained rosters. **Discover structurally; never enumerate names by hand.**
- **A COLLAPSE MAKES THE NEW REPRESENTATION OBVIOUS AND THE OLD INVARIANT INVISIBLE.** Rewriting `resolve()` the natural way would have let a hand-authored `satisfied` on a test-backed AC satisfy the gate. **Re-derive what the old shape enforced.**
- **`git checkout -- <path>` REVERTS TO HEAD, NOT TO BEFORE YOUR MUTATION.** Used it to undo a mutation test; it took uncommitted work in the same file with it. **Twice.** Back up with `cp`, restore with `cp`.
- **AN ERROR SWALLOWED IN A FIXTURE IS A SILENT ERROR.** `.ok()` on three fixture mutations made four tests fail on a precondition instead of on the refusal that caused it. **`expect()` in fixtures, always.**
- **A `.jsonl` FILE ESCAPES A `.json` CHECK BY SUFFIX LENGTH.** Correct today by accident -- the same shape D29 named for the DB file. **Any new extension needs a decision, not a coincidence.**
- **`IF NOT EXISTS` MAKES A SCHEMA CHANGE INVISIBLE UNTIL A QUERY FAILS.** Any DDL change needs `SCHEMA_VERSION` bumped and `store_schema_version.rs` re-pinned in the same commit; the test names the value. **Comments are excluded from the hash -- deliberately, after it demanded a bump for one.**
- **VERSION 0 IS NEVER "SCHEMA ZERO".** SQLite defaults `user_version` to 0, so 0 permanently means unstamped. `SCHEMA_VERSION - 1` to get "an older version" is wrong at version 1 and looks right.
- **`--only` commits what you NAME, and a move is TWO facts.** `a1a949c` committed 58 additions and left 55 deletions staged, on both remotes. Verify at HEAD (`git ls-tree`), never on disk.
- **`--only` NEVER CLEARS THE INDEX** (issue 0028): `git reset -- <your paths>` clears it without touching peers.
- **TWO symlinks point INTO this repo**: `which -a intent` returns three reachable copies. Sacrificial `git worktree` only for `bin/intent*`.
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **Cargo runs from `native/rust`.** A build cache can be stale in a way its own freshness check cannot see.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
