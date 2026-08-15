---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 14:06Z
status: active
focus: "Steps 1-3 of vc's sequencing landed and pushed. Next: AC-02.6 openness, the last WP-02 blocker, against faces that have now stopped moving."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it** -- a file becomes a well-formed item because the API refused everything it was not, so conformance is structural. **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth. **Re-creation from an extract is a CAPABILITY, not a licence** -- `rm intent.db` costs what the extract does not carry, and today that is the whole event log. **Migrations are NORMAL.** **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. That is what bidirectional sync is FOR. **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: rolling local backup via `VACUUM INTO`, never a file copy.

**The three ratified machines are in `data-model.md:223-317` and are BUILT.** AC-04.6 is now CONFORMANCE, not closure -- the implemented graph must match the ratified tables exactly, because a closed graph can still be the wrong graph.

## DOING -- AC-02.6, openness

**The last WP-02 blocker, and vc's ordering put it last on purpose: the faces have stopped moving.** `AT-02.6` = `openness.rs`.

- **Enumerate tables FROM THE GENERATED DDL FACE, never a hand roster.** The day's evidence says this twice over: the schema walk's own `["state","status"]` tag roster silently stopped classifying a field, and `event_log` survived a whole AC because a test over the tables that already had file forms passes on the defect.
- Each table needs a **file form OR an exemption DECLARED with its reason**. Round-trip both directions, lossless.
- **The discriminating case is ADDING a table with no file form and no exemption and watching it go red.**
- Known: `event_log` -> `events.jsonl` (ruled, NOT built -- `event.rs` says so). `file_index` is exempt on derivability but is **NOT a discardable cache** -- hv ruled it the `.treeindex` replacement, so it is a product feature.
- **New surface since the ratification**: `status_reason` on thread and work package, and `Criterion.state` replacing `scope`/`evidence`/`satisfied`.

## TODO

1. **AC-04.1's `TornRollback` arm** -- independent of everything (vc), goes wherever it fits.
2. **AC-03.10** (DB backup) -- NOT urgent; vc measured the live DB at zero model rows. A precondition of WP-10. **Do not invent the `.backup/` namespace (dc's) or the `intent config` keys (ic's).** `VACUUM INTO`, never a serialiser.
3. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026-0029 DEFAULT-DEFER; check AC-03.6 before touching 0029.

## Waiting

- **vc**: (a) the extract shape -- I took the discriminated `Computed` variant over an absent key; reversible in one commit. (b) **A BEHAVIOUR CHANGE**: `ac descope` now enforces the ratified "target thread exists" guard, which costs the descope-to-a-thread-you-are-about-to-create workflow. (c) `data-model.md`'s AC entity + the two `status_reason` fields.
- **ic**: **seven dispatch rows** (`st triage|hold|resume|reopen|reinstate`, `wp reopen|unstart`). The facade has all seven; the CLI cannot drive the lifecycle past `triage`. Plus `--reason` on `st cancel` (read optionally already, so it works the day the row lands) and the `sync` direction selector. **The ask is a failing surface in `cli_end_to_end.rs`, not a note.**
- **ic**: the `at` guard ruling -- v3 has NONE of v2's four `at` set-time guards; the gate recovers two, and _green-only-from-red cannot be recovered at gate time ever_ because it is a property of history.

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
- **`--only` commits what you NAME, and a move is TWO facts.** `a1a949c` committed 58 additions and left 55 deletions staged, on both remotes, where a fresh clone would have built the OLD tree. **Every working-tree check passed.** Name the deletion side too, verify at HEAD (`git ls-tree`), never on disk. After any move, clone fresh and build.
- **`--only` NEVER CLEARS THE INDEX** (issue 0028): a linter-on-save rewrite after `git add` leaves a third version staged forever, invisible to `git diff HEAD`. `git reset -- <your paths>` clears it without touching peers.
- **TWO symlinks point INTO this repo**: `which -a intent` returns three reachable copies. Mutating `bin/intent*` in place changes the tool every live session runs -- sacrificial `git worktree` only.
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **Cargo runs from `native/rust`.** A build cache can be stale in a way its own freshness check cannot see -- passes in isolation, fails in the suite. That is a conclusion, not flakiness.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
