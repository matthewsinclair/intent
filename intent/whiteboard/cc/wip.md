---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 12:13Z
status: active
focus: "ACKED db-is-SSOT + the three ratified machines. Nine old-model sites remain in my lane, FOUR in source -- yesterday's fix pass was under-scoped and I reported it done. Next: correct those, transcribe the ratified graph into transitions.rs, then AC-02.6."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified, supersedes everything earlier

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object, and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it, not a way round it** -- a file becomes a well-formed item because the API refused everything it was not. Conformance is structural, not checked. **Sync has two directions and they are different operations**: db -> disk re-derives the extract and cannot lose anything; disk -> db is a RESTORE that replaces truth and loses whatever is newer than the extract. **Re-creation from an extract is a CAPABILITY, not a licence** -- `rm intent.db` costs exactly what the extract does not carry, and today that includes the whole event log. **Migrations are NORMAL**; "no migrations, ever" is deleted and was never asked for. **The requirement it stood in front of is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. That is what bidirectional sync is FOR.

**The three ratified machines are in `data-model.md:223-317`.** ST: `Triage` (entry) -> `NotStarted` -> `Wip` -> `Completed`, `Hold` off `NotStarted`/`Wip`, `Cancelled` from everywhere, exits from BOTH terminals. WP: `NotStarted` -> `Wip` -> `Done` plus `reopen`/`unstart`, no `Hold`/`Cancelled`. AC: **ONE enum** `Satisfied | Unsatisfied | Descoped | Withdrawn`, entry `Unsatisfied`, no direct `Descoped <-> Withdrawn` edge. **AC-04.6 is now CONFORMANCE, not closure** -- the implemented graph must match the ratified tables exactly. A closed graph can still be the wrong graph.

## DOING

1. **Nine old-model sites in my lane, FOUR in source.** `store.rs:353-354` (false, and it documents the DESTRUCTIVE direction), `sync.rs:132` (D29's derivation -- conclusion survives, reason void; same text in `design.md:221`, vc's), `sync.rs:39` ("indexing the index"), **`event.rs:5-10` -- my own repair left it incoherent, a sentence broken mid-phrase, which reads as canon**. Plus five test files: `canon_round_trip:6`, `store_rebuild:108`, `store_round_trip:13`, `ignored_paths_corpus:13`, `sync_scan:123`. **I reported this class fixed yesterday at sixteen sites. It was not.**
2. **`transitions.rs` gets transcribed from the ratified tables.** Every edge I declared carries `from: &[]` -- any state -- so the graph I proved closed is WIDER than the ratified one (`st.done` from anywhere, not only `Wip`). Closure could never have seen that. `EdgeKind::Incidental` loses its only user under the AC collapse (checked: outside `transitions.rs` it appears only in the test proving it does not discharge a trap) -- delete it unless a non-AC user appears.
3. **Seven verbs do not exist**: `st triage`, `st hold`, `st resume`, `st reopen`, `st reinstate`, `wp reopen`, `wp unstart`. **`wp reopen` first** -- without it the live status/gate disagreement can only be repaired by hand-editing the file the CLI exists to own.
4. **`ThreadStatus::Tbc` -> `Triage`**: 7 refs, 5 files. **`st new` enters at `NotStarted` (`facade.rs:609`); ratified says `Triage`.**

## TODO

1. **The AC collapse** -- `satisfied: Option<bool>` + `AcScope` -> one enum. **19 files and all three generated faces** (`ddl.sql`, `schema.graphql`, `thread.schema.json`). A model change with an openness consequence, so it lands WITH AC-02.6, not beside it.
2. **AC-02.6 -- openness** (WP-02 reopens 5/6). `AT-02.6` = `openness.rs`. **Enumerate tables FROM THE GENERATED DDL FACE**, never a hand roster. Each table needs a file form OR an exemption **declared with its reason**. Round-trip both directions. **The discriminating case is ADDING a table with no file form and no exemption and watching it go red** -- a test over the tables that already have forms passes on the defect, which is how `event_log` survived. `event_log` -> `events.jsonl`; **`file_index` is exempt on derivability but is NOT a discardable cache** -- hv ruled it the `.treeindex` replacement (file index + content search, tree-sitter later), so it is a product feature.
3. **The marked-legacy `scope` field** (`data-model.md:83-89`): unit-only non-optional enum + sibling optional. Driven by `Medium-Large`, 1 of 129. Recheck against the AC collapse before building.
4. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026 + 0027 are mine under hv's DEFAULT-DEFER.

## Waiting on vc or hv

- **Three model questions, in `vc/inbox.cc.md` at 12:13Z.** (a) Under the collapse, does `evidence` move INTO `Satisfied { evidence }` -- making satisfied-without-evidence unconstructible, ie the guard made structural -- and what does that do to the JSON form? (b) The test-backed asymmetry: **structural (the type carries it) or runtime (a guard on the verb)?** Ratified text implies structural; it is the bigger change and I will not pick it silently. (c) Does `st new --start` COMPOSE through `triage` + `start` or JUMP to `Wip`? Indistinguishable in the final status, completely different in the audit trail.
- **ic**: the `sync` direction spelling (asked 10:16Z -- the facade has both directions, `sync`'s dispatch row has no flags). And the `at` guard ruling: **v3 has NONE of v2's four `at` set-time guards**; the gate recovers two, and _green-only-from-red cannot be recovered at gate time ever_ because it is a property of history. v2's `at` graph is conditional on `kind`, so a union view false-passes.

## Lane boundary, from 2026-08-15 -- PROPOSED (vc), not ruled

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` are vc's. **`bin/` is the one real collision**: `bin/intent*` is cc's, `bin/int` + `bin/.devbin/**` are dc's. **dc accepted at 09:00Z and corrected the argument**: the load-bearing half is not ownership but **the FREEZE** -- `bin/intent*` is the baseline ic's burn figures and register rows are measured from, so if it moves they measure a moving target and silently stop meaning what they say. "cc's" means "cc is the one who has to refuse" -- frozen by contract, and it may deserve a control rather than an agreement.

## Standing rulings

- **`treeindex` and handover RETIRE** -- the db obviates both; state moves out of per-session `.md` into the intentdb (D30/WP-14). **A retired command is PRESENT AND REFUSING, not absent** (my ruling): no functionality, only an explanation, per AC-04.4. **It breaks a guard and the fix ships with the feature**: `dispatch::is_shipped()` excludes `retire` rows, so a retired-but-refusing command is absent from `shipped_entries()` and present in the spine, and `dispatch_ssot.rs` asserts both directions. **`fileindex` is NOT covered** -- different mechanism; its `pending-hv` INV-07 question stands.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

Anything amounting to "remember to" is archived; it failed twice on entries this board already carried. These are facts about the estate.

- **A REPORT OF N SITES IS A SAMPLE UNTIL SOMEONE COUNTS, and it caught me one day after I wrote it.** I fixed sixteen false-canon sites, reported the class closed, and left nine -- four in source -- because the grep keyed on _"no migrations"_ and _"rm intent.db"_ and never asked about _"durable truth"_, _"disposable"_ or _"rebuildable"_ standing alone. **Grep the phrase FAMILY, then READ every hit instead of counting.** Same class as vc's four-of-sixteen and their `hooksPath` grep, my `| head`, my hyphen regex, my `type == "boolean"` probe.
- **AN EMPTY RESULT CAN MEAN THE QUERY NEVER RAN.** zsh ate unquoted `--include=*.rs` and the grep failed into a clean zero I nearly believed. Quote the globs; distrust a zero that arrives too easily.
- **THREE REMEDIES IN THIS ESTATE INSTRUCTED DATA LOSS**, all found the same day, all by checking a remedy's premise as it was written. `ViewsNotWritten` said "run `intent sync`" (disk -> db: overwrites the SSOT with the stale copy); `FacadeError::Store` said "delete `intent/.cache/intent.db`" (that IS the SSOT, shown on every store error); `doctor`'s module doc said "`rm` is always safe". Each was TRUE under the model it was written against. **A remedy naming a command outlives the reasoning that made it correct.**
- **`--only` commits what you NAME, and a move is TWO facts** (vc). `a1a949c` committed 58 additions and left 55 deletions staged, on both remotes, where a fresh clone would have built the OLD tree from five divergent files. **Every working-tree check passed** -- 234 tests, fmt, clippy, lint, six gates -- because the tree was right and only the repository was wrong. `--only` stays: it is what stopped that commit sweeping a peer's inbox. **Name the deletion side too, and verify at HEAD (`git ls-tree`), never on disk. After any move, clone fresh and build.** A green suite is evidence about the tree you HAVE, never the tree you PUSHED.
- **`git commit --only <paths>` takes whatever is in the working tree at those paths** -- no protection on a file a peer is also editing, and the index has carried staged-only pre-formatter content more than once. Read the diff first. **Six peer-owned paths are staged-dirty right now.**
- **TWO symlinks point INTO this repo**: `which -a intent` returns three reachable copies, `~/.local/bin/intent` and `~/bin/intent` both landing on `Intent/bin/intent`. Mutating `bin/intent*` in place changes the tool every live session runs -- sacrificial `git worktree` only. (`bin/.devbin/**` is exposed through neither, which is where the lane line falls.)
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **Cargo runs from `native/rust`.** A repo-root `cargo` finds no manifest. **A build cache can be stale in a way its own freshness check cannot see** -- cargo compares timestamps and inputs, not the manifest ROOT, so a path move bakes into artefacts invisibly. Tell: passes in isolation, fails in the suite. That is a conclusion, not flakiness. Cost 1.2G and an hour.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
