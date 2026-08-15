---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 13:42Z
status: active
focus: "Steps 1 and 2 landed (5cdebad, 2aec5f6): the false canon corrected, the ratified ST+WP machines built and enforced from ONE declaration, AC-04.6 now conformance. Next: step 3, the AC enum collapse."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified, supersedes everything earlier

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object, and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it, not a way round it** -- a file becomes a well-formed item because the API refused everything it was not. Conformance is structural, not checked. **Sync has two directions and they are different operations**: db -> disk re-derives the extract and cannot lose anything; disk -> db is a RESTORE that replaces truth and loses whatever is newer than the extract. **Re-creation from an extract is a CAPABILITY, not a licence** -- `rm intent.db` costs exactly what the extract does not carry, and today that includes the whole event log. **Migrations are NORMAL**; "no migrations, ever" is deleted and was never asked for. **The requirement it stood in front of is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): every db entity has a lossless `.json`/`.md` form usable without Intent. That is what bidirectional sync is FOR.

**The three ratified machines are in `data-model.md:223-317`.** ST: `Triage` (entry) -> `NotStarted` -> `Wip` -> `Completed`, `Hold` off `NotStarted`/`Wip`, `Cancelled` from everywhere, exits from BOTH terminals. WP: `NotStarted` -> `Wip` -> `Done` plus `reopen`/`unstart`, no `Hold`/`Cancelled`. AC: **ONE enum** `Satisfied | Unsatisfied | Descoped | Withdrawn`, entry `Unsatisfied`, no direct `Descoped <-> Withdrawn` edge. **AC-04.6 is now CONFORMANCE, not closure** -- the implemented graph must match the ratified tables exactly. A closed graph can still be the wrong graph.

## DOING

**Step 3 -- the AC enum collapse.** `satisfied: Option<bool>` + `AcScope` -> one enum. 19 files, three faces. hv's rulings: `Satisfied { evidence }` for non-test ACs, **no payload** for test-backed ones, and **test-backed ACs store NO state at all** -- computed from covering ATs, so there is no field a hand-satisfy could write. vc's Q2 answer settles the mechanism: `(non-test)` is an authored literal on the AC line (`intent_acceptance:90`), so the type carries it soundly rather than deriving it from AT coverage.

**One shape question is with vc** (asked 2026-08-15): with test-backed ACs storing no state, `Criterion` is no longer one shape, so **the JSON form differs by AC kind**. Absent `state` key, or a discriminated `kind` making the absence structural? AC-02.6 must answer it either way. Not blocking the enum work, blocking the extract shape.

## DONE TODAY -- both pushed, clone-check green

1. **The nine old-model sites, plus TWO the grep could not see** (`5cdebad`). `store.rs:1` -- the grep hit line 3, which was correct, while the falsehood sat on line 1, where yesterday's repair spliced a correction into the middle of the old sentence and left both halves. `facade.rs:22` said "THERE IS NO DB -> DISK SYNC YET", made false by my own AC-03.9 work the same day. A third survived two passes by spelling it **"disposability"** while every grep asked for **"disposable"**.
2. **The ratified ST and WP machines** (`2aec5f6`). Seven verbs built; `Tbc` -> `Triage`; `st new` enters at triage; `status_reason` MODELLED on both entities at hv's instruction. **The facade enforces FROM `transitions::permits`** rather than restating the from-states, so declaration/implementation drift is unconstructible -- and the test moved up a level to a SECOND transcription of the ratified tables taken from the document, not the code. Both former orphans (`tbc`, `hold`) are answered by ratification rather than by a build.

## TODO

1. **AC-02.6 -- openness**, against faces that have stopped moving (vc's ordering). Enumerate tables FROM THE GENERATED DDL FACE, never a hand roster. Each table needs a file form OR an exemption **declared with its reason**. **The discriminating case is ADDING a table with no file form and no exemption and watching it go red.** `event_log` -> `events.jsonl`; `file_index` exempt on derivability but NOT a discardable cache -- hv ruled it the `.treeindex` replacement, so it is a product feature. **`status_reason` is now in scope for it too.**
2. **AC-04.1's `TornRollback` arm** -- independent of everything else (vc), so it goes wherever it fits.
3. **AC-03.10** -- NOT urgent, and vc measured that rather than assuming it: the live DB holds zero model rows. A precondition of WP-10, not of today. **Do not invent the `.backup/` namespace (dc's) or the `intent config` keys (ic's).**
4. **AC-06.6 export**, then **AC-06.1 surface tail**. Issues 0026-0029 stay DEFAULT-DEFER; check AC-03.6 before touching 0029.

## Waiting

- **vc**: the AC extract shape above. Not blocking the enum, blocking the file form.
- **ic**: **seven dispatch rows** (`st triage|hold|resume|reopen|reinstate`, `wp reopen|unstart`) -- asked 2026-08-15. The facade has all seven; the CLI cannot drive the lifecycle past `triage` without them. Also `--reason` on `st cancel`'s existing row (the CLI reads it optionally already, so it works the day the row lands), and the `sync` direction selector. **The ask is a failing surface in `cli_end_to_end.rs`, not a note.**
- **ic**: the `at` guard ruling -- v3 has NONE of v2's four `at` set-time guards; the gate recovers two, and _green-only-from-red cannot be recovered at gate time ever_ because it is a property of history.

## Lane boundary, from 2026-08-15 -- PROPOSED (vc), not ruled

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` are vc's. **`bin/` is the one real collision**: `bin/intent*` is cc's, `bin/int` + `bin/.devbin/**` are dc's. **dc accepted at 09:00Z and corrected the argument**: the load-bearing half is not ownership but **the FREEZE** -- `bin/intent*` is the baseline ic's burn figures and register rows are measured from, so if it moves they measure a moving target and silently stop meaning what they say. "cc's" means "cc is the one who has to refuse" -- frozen by contract, and it may deserve a control rather than an agreement.

## Standing rulings

- **`treeindex` and handover RETIRE** -- the db obviates both; state moves out of per-session `.md` into the intentdb (D30/WP-14). **A retired command is PRESENT AND REFUSING, not absent** (my ruling): no functionality, only an explanation, per AC-04.4. **It breaks a guard and the fix ships with the feature**: `dispatch::is_shipped()` excludes `retire` rows, so a retired-but-refusing command is absent from `shipped_entries()` and present in the spine, and `dispatch_ssot.rs` asserts both directions. **`fileindex` is NOT covered** -- different mechanism; its `pending-hv` INV-07 question stands.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

Anything amounting to "remember to" is archived; it failed twice on entries this board already carried. These are facts about the estate.

- **A REPORT OF N SITES IS A SAMPLE UNTIL SOMEONE COUNTS, and it caught me one day after I wrote it.** I fixed sixteen false-canon sites, reported the class closed, and left nine -- four in source -- because the grep keyed on _"no migrations"_ and _"rm intent.db"_ and never asked about _"durable truth"_, _"disposable"_ or _"rebuildable"_ standing alone. **Grep the phrase FAMILY, then READ every hit instead of counting.** Same class as vc's four-of-sixteen and their `hooksPath` grep, my `| head`, my hyphen regex, my `type == "boolean"` probe.
- **A GREP FINDS THE SPELLING YOU ASKED FOR, AND A CLAIM HAS MORE THAN ONE.** `store_rebuild.rs`'s header survived two correction passes carrying "the D01 **disposability** invariant" while every grep asked for "**disposable**". Stem the word or enumerate its forms.
- **A GREP HIT IS A LINE; THE FALSEHOOD MAY BE THE PARAGRAPH.** `store.rs:3` matched and read correct, and line 1 -- which matched nothing -- still asserted the old model, because a previous repair had spliced a correction into the middle of the old sentence and left both halves standing. **Read the block a hit sits in, and when correcting canon REWRITE THE PARAGRAPH rather than inserting a clause**; a half-corrected sentence reads as canon and matches no search for the thing it still says.
- **AN EMPTY RESULT CAN MEAN THE QUERY NEVER RAN.** zsh ate unquoted `--include=*.rs` and the grep failed into a clean zero I nearly believed. Quote the globs; distrust a zero that arrives too easily.
- **THREE REMEDIES IN THIS ESTATE INSTRUCTED DATA LOSS**, all found the same day, all by checking a remedy's premise as it was written. `ViewsNotWritten` said "run `intent sync`" (disk -> db: overwrites the SSOT with the stale copy); `FacadeError::Store` said "delete `intent/.cache/intent.db`" (that IS the SSOT, shown on every store error); `doctor`'s module doc said "`rm` is always safe". Each was TRUE under the model it was written against. **A remedy naming a command outlives the reasoning that made it correct.**
- **`--only` commits what you NAME, and a move is TWO facts** (vc). `a1a949c` committed 58 additions and left 55 deletions staged, on both remotes, where a fresh clone would have built the OLD tree from five divergent files. **Every working-tree check passed** -- 234 tests, fmt, clippy, lint, six gates -- because the tree was right and only the repository was wrong. `--only` stays: it is what stopped that commit sweeping a peer's inbox. **Name the deletion side too, and verify at HEAD (`git ls-tree`), never on disk. After any move, clone fresh and build.** A green suite is evidence about the tree you HAVE, never the tree you PUSHED.
- **`git commit --only <paths>` takes whatever is in the working tree at those paths** -- no protection on a file a peer is also editing, and the index has carried staged-only pre-formatter content more than once. Read the diff first. **Six peer-owned paths are staged-dirty right now.**
- **TWO symlinks point INTO this repo**: `which -a intent` returns three reachable copies, `~/.local/bin/intent` and `~/bin/intent` both landing on `Intent/bin/intent`. Mutating `bin/intent*` in place changes the tool every live session runs -- sacrificial `git worktree` only. (`bin/.devbin/**` is exposed through neither, which is where the lane line falls.)
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **Cargo runs from `native/rust`.** A repo-root `cargo` finds no manifest. **A build cache can be stale in a way its own freshness check cannot see** -- cargo compares timestamps and inputs, not the manifest ROOT, so a path move bakes into artefacts invisibly. Tell: passes in isolation, fails in the suite. That is a conclusion, not flakiness. Cost 1.2G and an hour.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
