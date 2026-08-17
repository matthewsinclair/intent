---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-17 14:00Z
status: paused
focus: "PAUSED at a localfold, 14:00Z. **THE SUITE IS FULLY GREEN FOR THE FIRST TIME TODAY: 72 ok legs, 515 passed, 0 failed, cargo's own exit 0**, ic's `literal_stdout_parity.rs` included. FOUR commits since the last fold: `4a0c905c` wp rescope (0052), `1721e4bb` issue 0053 + the two walk defects it unmasked, `b759101d` issue 0032 in v3, `d14cd0b5` **the five voices** -- three parity breaks restored (`ac satisfy`'s ` by evidence`, the AT family's dropped `-> ` arrow, `at na`'s `n/a`) and one ratification BUILT (both undo verbs print `back in scope (<landing state>)`, state computed from kind, verified in both kinds through the binary). **THE THREE SPELLINGS WERE FOUR: `views.rs` wrote the WIRE form into generated `acceptance.md`, so the next projection would have rewritten 23 authored rows into a spelling v2's linter rejects at L1.** And that fix was UNWITNESSED -- reverting it left 72 legs green -- so `view_determinism.rs` now stands on it. **Two of my own instruments were wrong today**: a `///` on an enum variant drifted both committed schema faces (doc comments are shipped output, and I broke that rule hours after writing it on this board), and a mutation check run from the wrong directory printed a confident verdict about a suite that never executed. **My `AT-00.5 is yours to move` to vc was a claim about a binary stated as a claim about the contract** -- v2 is the gate that SCORES this thread and it is frozen unfixed; vc moved the row and had to revert it. BOUNCE: the two paired `kind` conversions (needs ic's notation for a multi-field atomic move), then 0051's guards-unmet driver. v3 stays off PATH; push to local only."
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

1. **THE TWO PAIRED `kind` CONVERSIONS.** `Criterion.kind` folds into Machine 3 as a `(kind, state)` pair; `AcceptanceTest.kind` into the **`AcceptanceTest.status`** machine -- **each `kind` folds into the machine of the entity that OWNS it** (vc's correction to their own shorthand; transcribing it literally would have put a field in the wrong machine). **Flipping `kind` alone is schema-INVALID** (`model.rs:414-432`, held by `ac_kind_state_invariant.rs`), so the verb moves two fields in one act. ic has no notation for a multi-field atomic move, so the spelling needs them.
2. **`Thread.acceptance` OFF `Unbuilt`: immutable after creation, no machine, no edge, owed nothing** (hv). `Option<AcceptanceMode>` is an attribute, not a lifecycle, so changing it is authoring. Needs a third `Disposition` variant; `Entry::Authored` stays the right measurement either way, because it measures a property rather than a classification.
3. **`Table` GETS FOUR DERIVES, AND I TOOK IT FROM ic.** `Serialize` on `Table`, `Family`, `Invariant`, `Vocab`. ic measured the premise: 24 top-level keys, 10 read, 14 stable structural blocks -- nothing like `Target`'s 40-against-2. Six keys are silent AND unwitnessed; deleting `flag_dispositions` reports 95 flags undeclared, **and a wall of offending rows sends its reader to fix the DATA when the fault is one missing key.**
4. **0051's REAL FIX: drive every declared edge a SECOND time with each guard UNMET and require a refusal.** The walk only ever drives guards satisfied, which is why `GatePass` and `ac unsatisfy` both hid there. Needs a per-`(verb, guard)` unmet-fixture table, and **that table is the reviewable artefact: a pair with no fixture is a guard nobody can test and must REFUSE, not skip** -- like `execute` refusing a `State` field with no drive arm.
5. **AC-06.1's surface tail** (NOT the installer/canon block).
6. **WP-10 PHASE B** -- fixtures and a sacrificial copy only. **Not against this estate, and v3 does not go on PATH.**
7. **WP-03 and WP-05 pass their gates and are still `WIP`** (vc measured). Whether the work is done is mine to say; the contract already says yes.

## Operational -- carry into the next session

- **AC-04.6: `Issue.status` IS PAID (Machine 4). Three `Unbuilt` rows remain and all three are TODO 1-2.** The row stays red until they land AND the walk agrees. **vc SHARPENED THE CLOSING CONDITION on my finding and it is stated by BEHAVIOUR, not test name** -- table matches ratified, every declared edge drives, every undeclared pair refuses -- because I had renamed the test the old condition named. **Three tests carry that property and no one of them carries it alone; do NOT merge them.**
- **AND THE POPULATION THAT CONDITION RANGES OVER WAS TWO THIRDS OF THE MACHINES UNTIL 2026-08-17.** `RATIFIED` in `mutation_completeness.rs` is now bound to `transitions::FIELDS`, so a machine cannot land in the code without reaching all three walks. Six State fields exist; `AcceptanceTest.status` and `WorkPackage.scope` are ratified IN PROSE by `data-model.md` ("it does not need a table because the graph is 'any value, one verb, any value'") and are held to that claim by `a_machine_ratified_in_prose_is_actually_trivial` -- **one verb, no from-restriction, reaching every state, and the failure message says WRITE THE TABLE rather than fix the code.** vc ruled that this checks the ratification rather than altering it (`0f4c7f27`, provisional pending hv). **`data-model.md:487` says FIVE and I measured six: correct at its own revision, staled by my own Machine 4 commit hours before I measured against it.**
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
- **ONE VALUE HAD FOUR SPELLINGS AND THE FOURTH DAMAGED A FILE RATHER THAN A LINE.** `at na` printed `na` (the subcommand name echoed as the movement phrase), `n-a` (the wire tag, via `AlreadyThere`), against v2's `n/a` -- and `views.rs` wrote the wire form into GENERATED `acceptance.md`, so the next projection would have rewritten 23 authored rows into a spelling v2's linter rejects at L1. **Reverting that fix left 72 legs green: unwitnessed until `view_determinism.rs` stood on it.** `green` and `red` are byte-identical to v2's tokens, so echoing the wrong source was correct twice and wrong once.
- **A DEFECT CAN MASK ITS OWN DETECTOR, AND THEN THE FIX AND THE TEST MUST LAND TOGETHER.** `ac_rescope`'s hand-written from-state check refused everything, which kept green a walk whose filter excluded a verb's target PER EDGE -- so a kind-dependent verb was being asked to refuse from its own OTHER target. 0053's defect is what kept the walk that should have caught 0053 green.
- **A TEST WHOSE NAME ASSERTS THE DEFECT IS THE PUREST FORM, BECAUSE ONLY THE BODY RUNS.** `reinstating_an_in_scope_criterion_is_refused` named the behaviour hv's ruling retired. Two more today: `a_no_op_scope_change_is_refused_rather_than_silently_accepted` said the opposite of its own (correct) body, and `a_test_backed_criterion_is_satisfied_only_by_a_green_test` required a criterion to STAY satisfied with one covering AT red. **A test that passes under both the defect and the correct implementation is not weak, it is unbuilt** -- assert both halves.
- **A STRUCTURAL ZERO AND AN UNVISITED SUBJECT PRODUCE THE SAME OUTPUT: NONE.** Declare the zero with its reason, require the declared-total subject to have been VISITED, and require every undeclared subject to contribute some. Both directions, or the declaration covers an absence.
- **A TEST THAT PINS A DEFECT AS EXPECTED OUTPUT MAKES THE DEFECT PERMANENT.** `contains("status: wip")` was asserting that the reopen moved the package -- a correct subject -- and captured the wrong spelling on the way.

### AN ENUMERATION THAT HAS GONE SHORT CANNOT BE CHECKED AGAINST ITSELF

- **A LINE-ORIENTED SCAN MISSES A CALL BROKEN ACROSS LINES.** 0050 counted nineteen dropped sites; `ac descope` and `ac withdraw` split `open()?\n  .verb(..)`, so **the real count was twenty-one**. Found by driving every verb twice, which is the only method that could have found them.
- **`measured` BOUNDS WHETHER AN ACT HAPPENED AND NOTHING BOUNDS WHETHER THE ACT REACHED THE FIELD THE ROW THEN DESCRIBES** (ic). `issues add` claimed a success-path stdout from a four-probe matrix that never supplies a title; `critic` declared its findings report, ie the non-empty case, which is the one a green CI never hits.
- **A DECLARED GUARD ENFORCED BY HAND AT THE CALL SITE IS A DECORATIVE DECLARATION** -- and there were TWO. `Guard::GatePass` in two verbs (deleting it from the table changed nothing) and **`ac_unsatisfy`'s from-state check, which made the shared setter's self-loop test unreachable and refused a legal self-loop at exit 1.** Enforcing from the declaration also fixes an ORDERING nobody would have to remember. Filed as 0051.
- **THREE COUNTS OF ONE POPULATION, THREE ENUMERATORS, EACH SHORT OF THE NEXT.** `arg_values_note` says three slots declare `values`; my probe walked `entry.args` + `entry.flags` and said four; a recursive walk of the raw table text says FIVE. **None of the three was careless** -- ic's named slots in prose, mine named two container keys by hand, and the fifth (`config`'s `backup.schedule`) lives in a third. The population must be a walk, never a list of the places a thing is expected to be.
- **A TYPED MODEL IS STRUCTURALLY UNABLE TO REPORT A FIELD IT DROPS.** `dispatch::Flag` has no `values`, so a `values` array on a flag row deserialises into nothing and a sweep over `dispatch::table()` would report the population complete. **Read the raw text when the question is "what does the register declare".**
- **AN ENTITY LIST AND A VERB LIST ARE DIFFERENT AXES, AND HARDENING ONE READS AS HARDENING BOTH.** `mutation_completeness.rs` derived its VERB axis from the ratified tables after `Criterion` went missing from a loop -- an ENTITY omission -- and its comment then read as though both were done. Six State machines existed, four had transcriptions, and **AT-04.6's central clause was being discharged over two thirds of its population.**
- **A MACHINE CAN BE CLOSED AND UNREACHABLE AT THE SAME TIME.** Closure is a question about EXITS, so no graph instrument could catch a verb the surface cannot reach -- `wp_rescope` satisfied the closure property and had no dispatch row. **The register cannot see a verb neither side declares** (0037's blind spot, one layer along).

### INFERRING A MECHANISM FROM A FIELD'S EXISTENCE

- **GOING TO THE SOURCE IS NOT THE SAME AS READING THE SOURCE** (ic). `grep` finds the field; only the reader finds what consumes it. Three instances in an afternoon: ic's `values` "builds the enum" (it does not); my `wp.rescope` comment premise "a value the caller supplies at creation" (false at HEAD, and stated as the premise of the fix it justified); my `issues add` test asserting `severity: null` on the architecture I wanted rather than the one that exists.
- **A TEST WRITTEN FROM THE FACADE'S VERB NAMES DOES NOT REACH THE BINARY.** `at_set` is one facade verb and THREE surface verbs (`green` / `red` / `na`). `at set` answers `unrecognized subcommand`.
- **ONE FACADE CALL REACHED TWO WAYS MUST GIVE ONE ANSWER.** `todo done` read the outcome and `st done` did not, so the WRAPPER was more honest than the thing it wrapped. That delegation is what made 0050 a defect rather than a style preference.

### Prose that is a build input, or a claim nothing checks

- **A `///` DOC COMMENT IS SHIPPED OUTPUT.** schemars lifts it into the JSON Schema face, async-graphql into the SDL, both COMMITTED and drift-checked. **Plain `//` for reasoning.**
- **A DOC COMMENT THAT AGREES WITH ITS OWN DEFECT IS HOW THE DEFECT SURVIVES REVIEW.** `satisfied_by_tests`'s comment was accurate about `.any` and false about the requirement, so auditing the function against its documentation returned agreement -- **and the test covering it asserted the same wrong rule.** Fixed at `b759101d`; the standing rule is that a doc comment states the REQUIREMENT and the code is what has to match it.
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

### A CLAIM ABOUT A BINARY IS NOT A CLAIM ABOUT A BEHAVIOUR

- **IN A DUAL-IMPLEMENTATION ESTATE, "LANDED" NEEDS A SUBJECT.** I told vc `.all` had landed and AT-00.5 was theirs to move. True of v3; **the gate that SCORES this thread is v2, on PATH, frozen and unfixed by ruling.** vc verified the fix correctly, released the row, and the gate scored the false green the row exists to prevent. **During a rewrite the incumbent governs for as long as it is the one on PATH** -- and "v3 stays off PATH" was already on this board as a rule about publication, not read as a rule about what my verdicts mean.
- **"NOTHING RAN" AND "NOTHING FAILED" RENDER IDENTICALLY.** A mutation check run after a `cd` into a scratch project never invoked cargo, and `grep -E FAILED || echo "nothing red"` fired the reassuring branch because `grep -c` on an empty log is `0`. **The conclusion happened to be TRUE, so a broken instrument would have produced the right answer and taught me nothing.** Assert the run's own leg count before reading its result.
- **A `///` IS A BUILD INPUT, AND I BROKE THAT RULE HOURS AFTER WRITING IT ON THIS BOARD.** Reasoning in a doc comment on an enum variant drifted both committed schema faces and reddened three tests. schemars and async-graphql lift TYPE and VARIANT docs; method docs are not lifted. The drift check catching it in a minute is the system working.
- **A FIXTURE WHERE THE CORRECT AND INCORRECT IMPLEMENTATIONS AGREE CANNOT CARRY THE RULING** (ic, on their own fixture choice). `back in scope (unsatisfied)` is byte-identical to v2 on a NON-test criterion, so a declaration written there would assert as-observed behaviour under a `corrected` label. Same shape as the `green`/`red` coincidence, one layer up.

### THE ANSWER MAY ALREADY BE IN THE CODEBASE, AND REASONING TALKS OVER IT

- **BEFORE ROUTING AN AMBIGUITY TO A PEER, LOOK FOR THE RULE THAT ALREADY DECIDES IT.** 0032's ruling is ambiguous under AND about whether an `n-a` row blocks. I had drafted a refinement and was about to send it as an open question -- **and `lint`'s L5 had already ruled it**, refusing a non-test AT over a test-backed criterion because "a non-test AT is never green, so it can never satisfy it". My refinement would have been a second, weaker rule invented to avoid a conflict that did not exist.
- **A SEARCH FOR A FIXTURE CAN FIND A SUBJECT THAT ANSWERS A DIFFERENT QUESTION.** Looking for "a test-kind criterion with no covering AT" found a DESCOPED one, so `resolve` answered on scope and the arm handed a `Descoped` verdict to an assertion about the empty guard -- **and it would have been green with the guard absent.** Construct the case; do not search for it.

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
