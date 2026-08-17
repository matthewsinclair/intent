---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-17 11:46Z
status: paused
focus: "PAUSED after a localfold at 11:46Z. **MACHINE 4 IS LANDED AND PUSHED (`b504d91b`) -- `issues add|close|open` ship, and AC-04.6's `Unbuilt` rows are one down.** Wiring it found a store defect D42 had already ruled on: `write_issue` had NO `Stamp` door, correct only because every caller was `rebuild`, ie because the mutation direction did not exist yet. vc ruled the door is a property of the ACT not the entity, and found a FOURTH domain date nobody had enumerated. **THEN 0050's SWEEP: nineteen render arms printed a movement they did not make, and the real count was TWENTY-ONE** -- `ac descope` and `ac withdraw` break the facade call across lines, so the issue's own line-oriented enumeration could not see them. **Driving every self-loop-capable verb twice through the real binary also found `ac unsatisfy` REFUSING a legal self-loop at exit 1** -- 0051's mechanism in a second live instance, a hand-written from-state check ahead of the shared setter. The house no-op voice is ruled and built: `ok: <subject> already <state>`, no third prefix. **The sweep was invisible to its own suite (485 passed before and after) until I built the witness, which is when all three defects surfaced.** BOUNCE: wire `wp rescope` (0052, ic's row is at `ba513915`), then `contract.rs` `.any` -> `.all` (0032, measured free today), then the two paired `kind` conversions. v3 stays off PATH; push to local only."
claims: []
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**The db is what is true. Everything on disk is an EXTRACT** -- `thread.json`, the `.md` views, `events.jsonl` are the same kind of object and none of them asserts anything. **One door in: the typed Rust API, and ingest is a CALLER of it.** **Sync's two directions are different operations**: db -> disk re-derives and cannot lose; disk -> db is a RESTORE that replaces truth -- except the event log, which MERGES, because nothing derives history. **The standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6). **D34**: the extract is the interchange, the DB is per-machine and never committed. **D35**: snapshot = same-schema rollback ONLY. **D36**: `rm intent.db` is not an operation. **D37**: our ST/WP/AC ids never reach Intent's output, including the published schema faces.

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

**You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either**. Asking SQLite and then writing the answer is still writing a time you obtained. **The record is stamped BY the write.**

**hv's sharpening, verbatim:** _"intent3 won't have any cli or intentsvcs functions that TAKE a time. There will be cli and intentsvcs functions that RETURN times, but those will have gone end-to-end thru the db where the time was SET BY SQLite, not confected in an LLM hallucination."_ **That is a property of the API surface, not of the call sites.**

**Creating vs restoring is the split that makes it workable**: create -> the DB stamps; restore -> the recorded stamp is carried. **Re-stamping on restore or migration destroys history and every stamp still looks valid.**

- **THE DOOR IS A PROPERTY OF THE ACT, NOT OF THE ENTITY** (vc ruled, on my `write_issue` finding). Every entity carrying a domain date needs BOTH doors, because both acts reach every such entity. `write_issue` was not a smaller `write_thread`, it was `write_thread` with the create door missing -- and it was correct only because every caller was `rebuild`. **Clean by luck and clean by construction look identical in a diff**; the register's word for it is `known_exposures`.
- **THERE ARE FOUR DOMAIN DATES, NOT THREE.** `issues.closed` was missing from both enumerations, including the DDL comment a future author reads to decide whether a new column needs a door. **A rationale narrower than the use it is later asked to justify survives because nobody re-derives an inherited one** (vc): "created stays AUTHORED" answers a migration question and is silent on who fills it when nobody authored it.
- **Four things that are NOT exceptions**: a test fixture; "I'm only reading it"; "the value came FROM the database"; "it's just a board label".

## THE SELF-LOOP RULE -- hv ratified 2026-08-17; implemented `61069b16` + `b504d91b`

**A self-loop is legal, accepted and reported at exit 0, and it does NOT re-run the guard.**

- **WHAT DECIDES A SELF-LOOP IS WHETHER THE CURRENT STATE EQUALS THE VERB'S TARGET** -- not whether the verb is declared from the current state. Test the target, never the declared origins, or a verb declared from many states self-loops from all of them. `wp start` on `Done` has target `Wip`, `Wip != Done`, so it never reaches the self-loop arm and dies at the declared-edge test -- **which is 0046 staying refused by construction rather than by a special case.** In `data-model.md` as vc's reading, argued rather than deferred to.
- **"Without re-running the guard" is structural.** A self-loop must not be able to fail for a reason that did not exist when the state was entered. `wp done` on a done package returns 0 **even where the gate would now BLOCK**.
- **`AlreadyThere` is a NO-OP, not a repeated write**, and it now CARRIES the state. An envelope for a non-movement would stamp a second `st.done` at a second time.
- **It does NOT license an undeclared edge.** Same state with a different payload falls through to the machine and is refused.
- **A self-loop is not a transition, so no walk may enumerate `from == edge.to`.**
- **THE NO-OP VOICE (ruled, mine; ic and vc concurred independently): `ok: <subject> already <state>`.** It names the STATE, not the verb -- `was already done` coincides with the state only when verb and state share a word, and breaks on `st hold` (`On Hold`) and `st triage` (`Not Started`). **NO third prefix**: INV-01 names `ok:` and `error:`, and v2's `skipped: <ID> already in progress` is carried as a deviation on that one row. The state comes from the FACADE, never a literal in the renderer -- `ac rescope` lands on `entry(kind)` and cannot be named statically, and seventeen literals would be seventeen spellings a rename could not reach.

## DOING -- nothing in flight

## TODO -- in order

1. **WIRE `wp rescope` (issue 0052, ic's).** The row is landed at `ba513915`, `new-surface`, two required positionals. **`values` on a positional does NOT reach clap** (ic's own correction) -- `spine.rs` reads it only for subcommand expansion and defaults, so **I parse and refuse the size myself**, with a remedy naming the six. The verb `wp_rescope` is `pub`, implemented, and its only callers are two tests. **Its self-loop is the most interesting in the table**: same size is a no-op ONLY with no carried `scope_legacy`, because resolving the carry is a real movement with the same from and to.
   - **`wp new --scope` IS NOT MINE AND IS WITH hv.** It is a workflow question, not a rendering one. If hv rules "no flag", `wp rescope` is the entrance as well as the exit and `absent` becomes an honest initial state -- do not foreclose that by shipping the flag because it is easier.
   - **`wp new` HARDCODES `TShirt::S` (`render.rs:574`)** -- ic's correction to their own issue, and it makes the finding worse. The migration refuses to invent a size on the explicit grounds that substituting one is worse than recording an absence, **and `wp new` invents one for every package it creates.** Both positions are in this codebase and only one was argued. **Do NOT "fix" it by writing `absent`** (ic): it breaks template parity for no gain and trades a wrong value for a missing one at the moment the exit verb is being wired to fill it.
2. **`contract.rs`: `.any` -> `.all` + a NON-EMPTY GUARD (issue 0032, vc's finding, ratified `corrected`).** `satisfied_by_tests` is v2's early return as an iterator adaptor, so a criterion covered by two ATs is satisfied when ONE is green. **Measured by me at this tree: 112 AT rows, exactly two multi-AT criteria, `any` and `all` agree on both -- so the correction moves ZERO verdicts today.** `.all` on an empty iterator is `true`, so the guard is not optional. **The doc comment is the mechanism by which it survived review** -- accurate about the code, false about the requirement -- so rewrite it to state the requirement and cite 0032. **AND IT NEEDS A MIXED MULTI-AT FIXTURE**: both rules pass the whole estate as it stands, so without one the fix is unwitnessed. **AC-00.7 is one obvious tidy-up from the defect** -- AT-00.5 is a passing test held at `red` on purpose, and marking it green scores the criterion satisfied with its second test unwritten. ic writes the row's `observed`/`target` divergence from my sha, not before.
3. **THE TWO PAIRED `kind` CONVERSIONS.** `Criterion.kind` folds into Machine 3 as a `(kind, state)` pair; `AcceptanceTest.kind` into the **`AcceptanceTest.status`** machine -- **each `kind` folds into the machine of the entity that OWNS it** (vc's correction to their own shorthand; transcribing it literally would have put a field in the wrong machine). **Flipping `kind` alone is schema-INVALID** (`model.rs:414-432`, held by `ac_kind_state_invariant.rs`), so the verb moves two fields in one act. ic has no notation for a multi-field atomic move, so the spelling needs them.
4. **`Thread.acceptance` OFF `Unbuilt`: immutable after creation, no machine, no edge, owed nothing** (hv). `Option<AcceptanceMode>` is an attribute, not a lifecycle, so changing it is authoring. Needs a third `Disposition` variant; `Entry::Authored` stays the right measurement either way, because it measures a property rather than a classification.
5. **`Table` GETS FOUR DERIVES, AND I TOOK IT FROM ic.** `Serialize` on `Table`, `Family`, `Invariant`, `Vocab`. ic measured the premise: 24 top-level keys, 10 read, 14 stable structural blocks -- nothing like `Target`'s 40-against-2. Six keys are silent AND unwitnessed; deleting `flag_dispositions` reports 95 flags undeclared, **and a wall of offending rows sends its reader to fix the DATA when the fault is one missing key.**
6. **0051's REAL FIX: drive every declared edge a SECOND time with each guard UNMET and require a refusal.** The walk only ever drives guards satisfied, which is why `GatePass` and `ac unsatisfy` both hid there. Needs a per-`(verb, guard)` unmet-fixture table, and **that table is the reviewable artefact: a pair with no fixture is a guard nobody can test and must REFUSE, not skip** -- like `execute` refusing a `State` field with no drive arm.
7. **AC-06.1's surface tail** (NOT the installer/canon block).
8. **WP-10 PHASE B** -- fixtures and a sacrificial copy only. **Not against this estate, and v3 does not go on PATH.**
9. **WP-03 and WP-05 pass their gates and are still `WIP`** (vc measured). Whether the work is done is mine to say; the contract already says yes.

## Operational -- carry into the next session

- **AC-04.6: `Issue.status` IS PAID (Machine 4). Three `Unbuilt` rows remain and all three are TODO 3-4.** The row stays red until they land AND the walk agrees. **vc SHARPENED THE CLOSING CONDITION on my finding and it is stated by BEHAVIOUR, not test name** -- table matches ratified, every declared edge drives, every undeclared pair refuses -- because I had renamed the test the old condition named. **Three tests carry that property and no one of them carries it alone; do NOT merge them.**
- **`the_transition_table_transcribes_the_ratified_machines_edge_for_edge`** (was `the_implemented_graph_is_...`). It compares the declaration table to `RATIFIED_*` consts from data-model.md; **neither side is the implementation**, so it was green with Machine 4's edges declared and no code driving them. `RATIFIED_ISSUE` now holds Machine 4 to transcription like the other three.
- **ISSUE 0033 IS NOT MINE.** v3's `at_set` touches only `status`; the note is a reified field, so v3 cannot destroy it by construction. The destruction is v2's row-rewriting and v2 is FROZEN. **Keep committing before `intent at` while v2 is the tool in hand.**
- **v3 STAYS OFF PATH.** The door is publication, not migration. **0036 is the only publication hold left.**
- **PUSH TO `local` ONLY.** upstream frozen (hv, CI/CD budget). **CI is no longer the watcher for the Linux leg** -- a `set -e` or path-separator break that only shows on Linux has nothing checking it. That is the class that shipped v2.11.12 broken.

## Watch-outs -- grouped by MECHANISM

### THE MEASUREMENT'S SUBJECT AND THE REPORT'S SUBJECT DIFFER, and the output cannot tell them apart

vc has this as seven variants in `parity.md` (`bee0a0dd`): tree, process, corpus, layer, population, spelling, declaration. Four nodes, one day. Mine below.

- **CORPUS.** **There are no `thread.json` files in this estate at all**, so my first measurement of multi-AT criteria ran over v3 canon and returned an EMPTY SET -- which I could have written up as "no multi-AT criteria exist" and been catastrophically wrong. **Zero rows and zero rows matching are the same output.**
- **PROCESS. A PIPELINE'S EXIT CODE NAMES THE PIPELINE.** `cargo test --workspace | grep -E ...` reported exit 0 while `mutation_completeness` was RED. Caught because the leg count looked wrong, not because I was careful. **Capture the subject's own status; never a pipe's.**
- **INSTANT.** `twice(root, args, subject, &thread_state(root))` evaluates the reader as an ARGUMENT, ie before the verb runs, so every case compared the no-op line to the state the entity was in beforehand. Eight of nine failed -- **and a green would have meant the verb had not moved anything.**
- **TREE. AN UNCOMMITTED TREE IN A SHARED CLONE WILL BE MEASURED AND READ AS SHIPPED BEHAVIOUR, INCLUDING BY hv's OWN SUITE.** Three instances today were all my tree; ic wrote my worktree into the register and retracted it, and vc cited `render.rs` line numbers that exist in no commit. **My fix is committing at GREEN rather than at "finished"; vc's is `git status --porcelain` before any measurement whose result gets written down. Both are real and neither substitutes for the other.**

### A CHANGE INVISIBLE TO ITS OWN SUITE

- **A SWEEP WHOSE GREEN IS IDENTICAL EITHER SIDE OF IT HAS NOT BEEN MEASURED, IT HAS BEEN PERFORMED.** 485 passed before the nineteen arms changed and 485 after: no test asserted the old spelling and none asserted the new one. **Building the witness is what surfaced all three defects in that change** -- the two missed arms, `ac unsatisfy`'s refusal, and the `wp show` spelling.
- **"THE FIX IS FREE" AND "THE FIX IS UNWITNESSED" ARE THE SAME FACT READ TWO WAYS** (vc, on themselves). A correction that moves zero verdicts needs a fixture constructed for the shape where the two rules differ, or a later revert is silent.
- **A TEST THAT PINS A DEFECT AS EXPECTED OUTPUT MAKES THE DEFECT PERMANENT.** `contains("status: wip")` was asserting that the reopen moved the package -- a correct subject -- and captured the wrong spelling on the way.

### AN ENUMERATION THAT HAS GONE SHORT CANNOT BE CHECKED AGAINST ITSELF

- **A LINE-ORIENTED SCAN MISSES A CALL BROKEN ACROSS LINES.** 0050 counted nineteen dropped sites; `ac descope` and `ac withdraw` split `open()?\n  .verb(..)`, so **the real count was twenty-one**. Found by driving every verb twice, which is the only method that could have found them.
- **`measured` BOUNDS WHETHER AN ACT HAPPENED AND NOTHING BOUNDS WHETHER THE ACT REACHED THE FIELD THE ROW THEN DESCRIBES** (ic). `issues add` claimed a success-path stdout from a four-probe matrix that never supplies a title; `critic` declared its findings report, ie the non-empty case, which is the one a green CI never hits.
- **A DECLARED GUARD ENFORCED BY HAND AT THE CALL SITE IS A DECORATIVE DECLARATION** -- and there were TWO. `Guard::GatePass` in two verbs (deleting it from the table changed nothing) and **`ac_unsatisfy`'s from-state check, which made the shared setter's self-loop test unreachable and refused a legal self-loop at exit 1.** Enforcing from the declaration also fixes an ORDERING nobody would have to remember. Filed as 0051.
- **A MACHINE CAN BE CLOSED AND UNREACHABLE AT THE SAME TIME.** Closure is a question about EXITS, so no graph instrument could catch a verb the surface cannot reach -- `wp_rescope` satisfied the closure property and had no dispatch row. **The register cannot see a verb neither side declares** (0037's blind spot, one layer along).

### INFERRING A MECHANISM FROM A FIELD'S EXISTENCE

- **GOING TO THE SOURCE IS NOT THE SAME AS READING THE SOURCE** (ic). `grep` finds the field; only the reader finds what consumes it. Three instances in an afternoon: ic's `values` "builds the enum" (it does not); my `wp.rescope` comment premise "a value the caller supplies at creation" (false at HEAD, and stated as the premise of the fix it justified); my `issues add` test asserting `severity: null` on the architecture I wanted rather than the one that exists.
- **A TEST WRITTEN FROM THE FACADE'S VERB NAMES DOES NOT REACH THE BINARY.** `at_set` is one facade verb and THREE surface verbs (`green` / `red` / `na`). `at set` answers `unrecognized subcommand`.
- **ONE FACADE CALL REACHED TWO WAYS MUST GIVE ONE ANSWER.** `todo done` read the outcome and `st done` did not, so the WRAPPER was more honest than the thing it wrapped. That delegation is what made 0050 a defect rather than a style preference.

### Prose that is a build input, or a claim nothing checks

- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face, async-graphql into the SDL, both COMMITTED and drift-checked. **Plain `//` for reasoning.**
- **A DOC COMMENT THAT AGREES WITH ITS OWN DEFECT IS HOW THE DEFECT SURVIVES REVIEW.** `satisfied_by_tests`'s comment is accurate about the code and false about the requirement, so auditing the function against its documentation returns agreement. **I wrote both halves and they agreed with each other.**
- **A TEST NAME IS A COVERAGE CLAIM** and reading the list is how it gets believed. **A ROW OR COMMENT CITING A LINE NUMBER IS A PROMISE THE LINE EXISTS**; `git blame` + `git show HEAD:` is the two-second version.
- **A SPELLING WHOSE ONLY COPY LIVES IN A TEST IS UNAVAILABLE TO PRODUCTION AND LOOKS COVERED.** `AcState`'s five state names lived in `mutation_completeness.rs::state_name_of`; `enum_str` cannot answer it because three variants carry payloads.

### Greps, shells and measurement

- **A GREPPABLE PROXY THAT CANNOT SEPARATE CODE FROM TEXT ABOUT CODE FAILS TOWARD THE CLEAN-LOOKING ANSWER.** `contains("rm ")` matching "form"; a `Completed` COLUMN HEADER read as a thread's status. **The fix is a better discriminator or an authored classification -- never an exemption -- and the discriminator needs its own two-sided canary.**
- **REDUNDANT WITNESSES: ONE STOPS SILENTLY AND THE OTHER MASKS IT.** Stubbing half my status-line classifier left all six tests green because a status renders in TWO places. Require each source separately.
- **THE SHELL'S ANSWER IS NOT THE ONE YOU THINK, and the Bash tool runs zsh 5.9.** `$?` after a pipe is the last command's. `grep -c` exits 1 on zero. **zsh does NOT word-split an unquoted `$c`** -- use `${=c}`, and quote `--include='*.rs'`. **`path` is tied to `PATH`.** **Prefer a script with a shebang for anything whose result you write down.**
- **`cd` PERSISTS BETWEEN Bash CALLS AND THIS PROJECT HAS TWO ROOTS.** Cargo runs from `native/rust`, the repo one level up. Absolute paths for reads. `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift` re-pins the faces.
- **A PARSER REPORTS ITS OWN LIMITS AS THE ESTATE'S DEFECTS. ABSENT IS NOT INVALID.**

### Git in a four-node clone

- **`--only` COMMITS WHAT YOU NAME, and a hand-typed pathspec is a roster that is wrong where its author was distracted.** Check `git status` in your own lane afterwards. **`--only` cannot reach an untracked file** -- `git add` it in the same command.
- **STAGE NOTHING UNTIL THE MOMENT YOU COMMIT.** `git stash` is unsafe here; sacrificial `git worktree` only for `bin/intent*`, **with its OWN target dir** or it contends for `target/`.
- **`git checkout -- <path>` REVERTS TO THE INDEX**, so with an uncommitted baseline the revert destroys what you were testing. `cp` to snapshot, `diff -q` both back.
- **A sha carries NO authorship here** -- `git log --format=%an` says `Matthew Sinclair` for all five of us. Do not attribute a finding from a commit; read the artefact.
- **NEVER PUT A `"` INSIDE A BOARD HEADER VALUE** -- measured trigger, cause unknown.

### Mechanics worth keeping

- **A surface built from a table cannot be unbuilt from the renderer**: the TABLE row moves first, the renderer second. **`values` on a non-subcommand slot is a DECLARATION and the facade enforces it** (`arg_values_note`, ic, `6d3e814e`).
- **SQLite refuses `ADD COLUMN` for a NOT NULL column with a non-constant default**, so any DB-stamped column means a table rebuild. **`IF NOT EXISTS` makes a schema change invisible until a query fails** -- any DDL change bumps `SCHEMA_VERSION`.
- **A fallback value can stand in for two different facts, and the second is the one nobody ruled on.** **A migration fixture must be a store that could actually have existed.** **An error swallowed in a fixture is a silent error** -- `expect()` always.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour -- including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN.** `bin/int` + `bin/.devbin/**` are dc's.

## Standing rulings

- **`treeindex` and handover RETIRE.** A retired command is PRESENT AND REFUSING, not absent. **`fileindex` is NOT covered.**
- **`EdgeKind::Incidental` STAYS despite having no user** -- deleting it collapses `exits` into `leaves` and the trap check silently accepts technicality exits again.
- **`owner_wp` stays carried and unread.** **`doctor --fix` is WITHDRAWN, not deferred** (hv). **`install::MARKER` IS A SENTINEL, NOT A BOUNDARY**, so the rule library does not move under it.
- **`Outcome` is deliberately NOT `#[must_use]`** -- it fired on 65 sites, nearly all tests where ignoring it is right, and 65 `let _ =` is how an annotation stops carrying information. **It now carries the no-op state, so it is `Clone` and not `Copy`; read it with `.already()` rather than matching the variant at nineteen call sites.**
- **`ComputedSatisfaction` is NOT delegated to `Guard::NonTestOnly`.** A test-backed criterion is always `Computed`, so it can never be at `ac unsatisfy`'s target -- there is no self-loop for that guard to shadow, and the message is better than either alternative.
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** A guard that had downgraded my red would have hidden a genuine finding.
