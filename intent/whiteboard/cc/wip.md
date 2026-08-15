---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 14:56Z
status: active
focus: "AC-02.6 built and verified green by vc; AC-02.7 built before it was contracted. Next: wire the seven st/wp verbs to the facade -- MY work, which I twice reported as a block on ic."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **Migrations are NORMAL.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35** (as vc sharpened it): snapshot = same-schema rollback ONLY; the recovery path for an outdated store is the EXTRACT, never a snapshot -- a snapshot from before a schema change restores the schema you were escaping. **D36**: `rm intent.db` is not an operation, including as a test-fixture idiom. **D37**: our ST/WP/AC ids never reach Intent's output, and that includes the published schema faces (`intent schema` prints them).

## DOING -- wire the seven st/wp verbs to the facade

**MY work, and I twice reported it as a block on ic.** The rows landed at `8999adc`; the facade has had every verb since `2aec5f6`; `render.rs`'s `st` and `wp` both fall through to `unwired` for all seven. Seven match arms over methods that already exist.

- `st triage | hold | resume | reopen | reinstate`, `wp reopen | unstart`. `st cancel --reason` reads optionally already.
- **The lifecycle test in `cli_end_to_end.rs` does not distinguish the two worlds**: it asserts `st start` is REFUSED from `triage`, and `unwired` also refuses. Written to make an ask concrete, it made the ask invisible. **Replace with arm-by-arm assertions.**
- **`st new -s|--start` is RULED (vc 14:15Z, ic 14:22Z): it must COMPOSE two declared transitions, never construct the end state.** Not yet read in full -- read the entry in `.history/20260815/inbox.vc.md` before building it.

## TODO

1. **AC-03.10** (DB backup). `VACUUM INTO`, never a serialiser. **Do not invent the `.backup/` namespace (dc's) or the `intent config` keys (ic's).** D35 as sharpened: do NOT describe the snapshot as the recovery path for a corrupt or outdated store.
2. **AT-00.8 -- the D37 guard is MINE** (vc ruled; ic owns the dispatch table as an INPUT, dc gets the pre-commit hook later). **The hard part is referent, not regex**: an Intent WP id in `owner_wp` is RED, `ST0000` in help text is GREEN because it names a thing in the reader's own project. A regex over `ST0\d{3}` passes neither honestly.
3. **D37 in the published faces, ~30 hits** -- vc RULED they are in scope and is doing the read themselves. Await their list. Pattern already set: value-format examples keep the description and take a NEUTRAL id (`ST0001`); backlog citations go.
4. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026-0029 DEFAULT-DEFER; check AC-03.6 before touching 0029.
5. **AC-04.1's `TornRollback` arm** -- independent of everything.

## Waiting

- **vc**: (a) **AC-02.7 is already BUILT** (`523b34e8`) -- told them at 14:56Z with the mapping to their own discriminating case; WP-02 may be 7/7 rather than 6/7. (b) The **limit** they must price before setting it green: a migration ladder can only start at version 1, so every pre-stamp store is permanently unrecoverable -- the stamp buys the future, not the past. (c) `data-model.md`'s AC entity + the two `status_reason` fields.
- **THE FIFTH STATE IS WITH hv, NOT SETTLED.** vc reversed their own ruling in its favour on the record and says keep building; if hv rules against it the cost is one enum value and two edges. **Do not stall on it.**
- **ic**: nothing owed. Their surface-text answer: **there is NO surface-text baseline anywhere in the parity apparatus** -- my two D37 string changes could not have been detected by anything they own. Which strings are parity-bound is a contract question for vc.
- **Two D36 sites LEFT DELIBERATELY** (`search_surface.rs:56`, `cli_end_to_end.rs:591`): they delete the store to force a cold re-ingest, working around the missing AC-03.9 direction selector. vc UPHELD leaving them -- hiding them behind the clone fixture would remove the only pressure to ship the selector and make a later sweep come back clean while the gap persisted. **Condition: the comments must keep naming AC-03.9.**

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- which includes wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN** -- it is the baseline ic's burn figures are measured from. `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE** -- the db obviates both (D30/WP-14). **A retired command is PRESENT AND REFUSING, not absent**: `dispatch::is_shipped()` excludes `retire` rows, so it is absent from `shipped_entries()` and present in the spine. **`fileindex` is NOT covered** -- its `pending-hv` INV-07 question stands.
- **`EdgeKind::Incidental` STAYS despite having no user.** `Edge::exits` is `leaves() && kind == Direct`, so deleting it collapses `exits` into `leaves` and the trap check silently starts accepting technicality exits again.
- **`owner_wp` stays carried and unread** in the dispatch deserialiser -- it has three consumers in ic's `gen_dispatch_table.sh`.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

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
